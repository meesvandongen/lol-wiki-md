// Splits a parsed page into retrieval-ready chunks with stable IDs.
//
// Chunk boundaries follow the document's own structure: one chunk per leaf
// section (an H3 where present, otherwise the H2), plus one for the page
// intro. IDs are derived from slugs of the entity name and heading path, so
// they only change when the wiki's own structure changes. Oversized sections
// are split on paragraph boundaries with an `@n` part suffix.

const MAX_CHUNK_CHARS = 6000;

/**
 * @typedef {{ id: string, category: string, entity: string,
 *   headingPath: string[], text: string, tokensEst: number }} Chunk
 */

export function slugify(text) {
  return text
    .toLowerCase()
    .normalize('NFKD')
    .replace(/[̀-ͯ]/g, '')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}

export function estimateTokens(text) {
  // Rough heuristic (~4 chars per token for English prose); labelled as an
  // estimate everywhere it is surfaced.
  return Math.ceil(text.length / 4);
}

/**
 * @param {string} category e.g. "champions"
 * @param {import('./markdown.mjs').Doc} doc
 * @returns {Chunk[]}
 */
export function chunkDoc(category, doc) {
  const entity = doc.title;
  const entitySlug = slugify(entity);
  /** @type {Chunk[]} */
  const chunks = [];

  const push = (headingPath, text) => {
    const body = text.trim();
    if (!body) {
      return;
    }
    const pathSlug = headingPath.length ? headingPath.map(slugify).join('/') : '_intro';
    const baseId = `${category}/${entitySlug}#${pathSlug}`;
    for (const [index, part] of splitLong(body).entries()) {
      chunks.push({
        id: index === 0 ? baseId : `${baseId}@${index + 1}`,
        category,
        entity,
        headingPath,
        text: part,
        tokensEst: estimateTokens(part),
      });
    }
  };

  push([], doc.introRaw);

  for (const section of doc.sections) {
    if (section.children.length === 0) {
      push([section.heading], section.raw);
      continue;
    }
    // Content the parent holds before its first child still gets a chunk.
    if (section.ownRaw.trim()) {
      push([section.heading], `${'#'.repeat(section.level)} ${section.heading}\n\n${section.ownRaw}`);
    }
    for (const child of section.children) {
      push([section.heading, child.heading], child.raw);
    }
  }

  return dedupeIds(chunks);
}

function splitLong(text) {
  if (text.length <= MAX_CHUNK_CHARS) {
    return [text];
  }
  const parts = [];
  let current = '';
  for (const paragraph of text.split(/\n\n+/)) {
    if (current && current.length + paragraph.length + 2 > MAX_CHUNK_CHARS) {
      parts.push(current);
      current = paragraph;
    } else {
      current = current ? `${current}\n\n${paragraph}` : paragraph;
    }
  }
  if (current) {
    parts.push(current);
  }
  return parts;
}

/** Duplicate heading slugs (rare) get a positional suffix to stay unique. */
function dedupeIds(chunks) {
  const seen = new Map();
  for (const chunk of chunks) {
    const count = seen.get(chunk.id) ?? 0;
    seen.set(chunk.id, count + 1);
    if (count > 0) {
      chunk.id = `${chunk.id}~${count + 1}`;
    }
  }
  return chunks;
}
