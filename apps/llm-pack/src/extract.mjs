// Structured extraction from the converter's Markdown output.
//
// Extraction is deliberately conservative: it lifts the converter's stable
// section/table conventions into JSON without transforming any game value —
// numbers, ranges, and scaling expressions are copied as verbatim strings
// (see AGENTS.md: "Numbers round-trip exactly"). Sections this module does
// not model are preserved verbatim under `sections`, so the JSON view never
// silently drops content.

import { plainLabel } from './markdown.mjs';

const BOLD_LABEL_ITEM = /^\*\*(.+?):\*\*\s*(.*)$/;

/** @param {import('./markdown.mjs').Doc} doc */
export function extractChampion(doc) {
  const champion = {
    name: doc.title,
    blurb: doc.blurb,
    overview: {},
    stats: {},
    advancedStats: {},
    specialStatistics: {},
    abilities: [],
    validation: [],
    sections: {},
  };

  for (const section of doc.sections) {
    switch (section.heading) {
      case 'Overview':
        champion.overview = keyValuesFromList(section.blocks);
        break;
      case 'Stats':
        champion.stats = statsTable(section.blocks);
        break;
      case 'Advanced Stats':
        champion.advancedStats = keyValuesFromTable(section.blocks);
        break;
      case 'Special Statistics':
        for (const mode of section.children) {
          champion.specialStatistics[mode.heading] = keyValuesFromTable(mode.blocks);
        }
        break;
      case 'Abilities':
        champion.abilities = section.children.map(extractAbility);
        break;
      case 'Validation':
        champion.validation = listItems(section.blocks);
        break;
      default:
        champion.sections[section.heading] = section.raw;
        break;
    }
  }

  return champion;
}

/** @param {import('./markdown.mjs').Section} section */
function extractAbility(section) {
  // Headings look like "Q – The Darkin Blade" or "Passive – Deathbringer Stance".
  const [slot, ...nameParts] = section.heading.split(' – ');
  const ability = {
    slot: nameParts.length ? slot.trim() : '',
    name: nameParts.length ? nameParts.join(' – ').trim() : section.heading,
    attributes: {},
    description: '',
    effects: [],
    details: {},
    notes: [],
  };

  const paragraphs = [];
  let pendingNotes = false;

  for (const block of section.blocks) {
    if (block.type === 'table' && block.header.length === 2) {
      const headerLabel = plainLabel(block.header[0]);
      const target = headerLabel === 'Detail' ? ability.details : ability.attributes;
      for (const row of block.rows) {
        if (row.length >= 2) {
          target[plainLabel(row[0])] = row[1];
        }
      }
      continue;
    }
    if (block.type === 'paragraph') {
      if (block.text === '**Notes:**') {
        pendingNotes = true;
        continue;
      }
      paragraphs.push(block.text);
      continue;
    }
    if (block.type === 'list') {
      if (pendingNotes) {
        ability.notes.push(...block.items.map(noteText));
        continue;
      }
      for (const item of block.items) {
        const labelled = item.text.match(BOLD_LABEL_ITEM);
        if (labelled && item.depth === 0) {
          ability.effects.push({ label: labelled[1], value: labelled[2] });
        } else {
          ability.notes.push(noteText(item));
        }
      }
    }
  }

  ability.description = paragraphs.join('\n\n');
  return ability;
}

/** @param {import('./markdown.mjs').Doc} doc */
export function extractItem(doc) {
  const item = {
    name: doc.title,
    blurb: doc.blurb,
    overview: {},
    stats: {},
    buildTree: {},
    notes: [],
    similarItems: [],
    patchHistory: [],
    validation: [],
    sections: {},
  };

  for (const section of doc.sections) {
    switch (section.heading) {
      case 'Overview':
        item.overview = keyValuesFromList(section.blocks);
        break;
      case 'Stats':
        item.stats = keyValuesFromTable(section.blocks);
        break;
      case 'Build Tree':
        item.buildTree = buildTreeGroups(section.blocks);
        break;
      case 'Notes':
        item.notes = listItems(section.blocks);
        break;
      case 'Similar Items':
        item.similarItems = listItems(section.blocks);
        break;
      case 'Patch History':
        item.patchHistory = patchHistory(section);
        break;
      case 'Validation':
        item.validation = listItems(section.blocks);
        break;
      default:
        item.sections[section.heading] = section.raw;
        break;
    }
  }

  return item;
}

/** @param {import('./markdown.mjs').Doc} doc */
export function extractRune(doc) {
  const rune = {
    name: doc.title,
    blurb: doc.blurb,
    overview: {},
    description: '',
    notes: [],
    patchHistory: [],
    validation: [],
    sections: {},
  };

  for (const section of doc.sections) {
    switch (section.heading) {
      case 'Overview':
        rune.overview = keyValuesFromList(section.blocks);
        break;
      case 'Description':
        rune.description = section.ownRaw;
        break;
      case 'Notes':
        rune.notes = listItems(section.blocks);
        break;
      case 'Patch History':
        rune.patchHistory = patchHistory(section);
        break;
      case 'Validation':
        rune.validation = listItems(section.blocks);
        break;
      default:
        rune.sections[section.heading] = section.raw;
        break;
    }
  }

  return rune;
}

// ---------------------------------------------------------------------------
// Shared helpers

/** Bullet lists of the form `- **Label:** value` become a key/value map. */
function keyValuesFromList(blocks) {
  const map = {};
  for (const block of blocks) {
    if (block.type !== 'list') {
      continue;
    }
    for (const item of block.items) {
      const match = item.text.match(BOLD_LABEL_ITEM);
      if (match) {
        map[match[1]] = match[2];
      }
    }
  }
  return map;
}

/** Two-column tables become a key/value map; values stay verbatim. */
function keyValuesFromTable(blocks) {
  const map = {};
  for (const block of blocks) {
    if (block.type !== 'table') {
      continue;
    }
    for (const row of block.rows) {
      if (row.length >= 2) {
        map[plainLabel(row[0])] = row[1];
      }
    }
  }
  return map;
}

/** Champion `| Stat | Base | Growth |` tables. */
function statsTable(blocks) {
  const map = {};
  for (const block of blocks) {
    if (block.type !== 'table' || block.header.length < 3) {
      continue;
    }
    for (const row of block.rows) {
      if (row.length >= 3) {
        map[plainLabel(row[0])] = { base: row[1], growth: row[2] };
      }
    }
  }
  return map;
}

/** Flattens list blocks to one string per item, indenting nested items. */
function listItems(blocks) {
  const items = [];
  for (const block of blocks) {
    if (block.type !== 'list') {
      continue;
    }
    for (const item of block.items) {
      items.push(noteText(item));
    }
  }
  return items;
}

function noteText(item) {
  return item.depth > 0 ? `${'  '.repeat(item.depth)}${item.text}` : item.text;
}

/**
 * Build Tree sections group lists under bold paragraph labels, e.g.
 * `**Components**` followed by a component list.
 */
function buildTreeGroups(blocks) {
  const groups = {};
  let label = 'Components';
  for (const block of blocks) {
    if (block.type === 'paragraph') {
      const match = block.text.match(/^\*\*(.+?)\*\*$/);
      if (match) {
        label = match[1];
      }
      continue;
    }
    if (block.type === 'list') {
      groups[label] = [...(groups[label] ?? []), ...block.items.map(noteText)];
    }
  }
  return groups;
}

/** Patch History: one H3 per version, list items as verbatim change lines. */
function patchHistory(section) {
  return section.children.map((version) => ({
    version: version.heading,
    changes: versionChanges(version),
  }));
}

function versionChanges(version) {
  const changes = [];
  for (const block of version.blocks) {
    if (block.type === 'list') {
      for (const item of block.items) {
        changes.push(noteText(item));
      }
    } else if (block.type === 'paragraph' && block.text) {
      changes.push(block.text);
    }
  }
  return changes;
}
