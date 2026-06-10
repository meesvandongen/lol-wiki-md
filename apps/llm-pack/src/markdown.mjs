// Line-based structural parser for the converter's Markdown output.
//
// The converter emits a constrained Markdown dialect (see STYLE.md): one H1
// title, a blurb paragraph, then H2 sections optionally containing H3
// subsections, with pipe tables, bullet lists, block quotes, and fenced code
// blocks. This parser recovers that structure while preserving every source
// line verbatim, so downstream artifacts can stay lossless.

/**
 * @typedef {{ type: 'paragraph', text: string }
 *   | { type: 'table', header: string[], rows: string[][] }
 *   | { type: 'list', items: { text: string, depth: number }[] }
 *   | { type: 'quote', text: string }
 *   | { type: 'code', lang: string, text: string }} Block
 *
 * @typedef {{ heading: string, level: number, blocks: Block[],
 *   children: Section[], raw: string, ownRaw: string }} Section
 *
 * @typedef {{ title: string, blurb: string, intro: Block[], introRaw: string,
 *   sections: Section[], raw: string }} Doc
 */

const TABLE_SEPARATOR = /^\|[\s:|-]+\|$/;
const LIST_ITEM = /^(\s*)[-*]\s+(.*)$/;
const HEADING = /^(#{1,6})\s+(.*)$/;
const FENCE = /^\s*```/;

/** @returns {Doc} */
export function parseDoc(markdown) {
  const lines = markdown.split(/\r?\n/);

  // Pass 1: locate headings, ignoring any inside fenced code blocks.
  const headings = [];
  let inFence = false;
  for (let i = 0; i < lines.length; i += 1) {
    if (FENCE.test(lines[i])) {
      inFence = !inFence;
      continue;
    }
    if (inFence) {
      continue;
    }
    const match = lines[i].match(HEADING);
    if (match) {
      headings.push({ level: match[1].length, heading: match[2].trim(), line: i });
    }
  }

  const title = headings.find((h) => h.level === 1)?.heading ?? '';
  const sectionHeadings = headings.filter((h) => h.level >= 2);

  // Pass 2: derive line ranges. A section spans from its heading line to the
  // next heading of equal or shallower level; `own` stops at its first child.
  const slice = (from, to) => lines.slice(from, to).join('\n').replace(/\n+$/, '').replace(/^\n+/, '');

  /** @type {Section[]} */
  const roots = [];
  /** @type {Section[]} */
  const stack = [];
  for (let i = 0; i < sectionHeadings.length; i += 1) {
    const { level, heading, line } = sectionHeadings[i];
    const end = sectionHeadings.slice(i + 1).find((h) => h.level <= level)?.line ?? lines.length;
    const ownEnd = sectionHeadings[i + 1]?.line ?? lines.length;

    /** @type {Section} */
    const section = {
      heading,
      level,
      raw: slice(line, end),
      ownRaw: slice(line + 1, ownEnd),
      blocks: [],
      children: [],
    };
    section.blocks = parseBlocks(section.ownRaw);

    while (stack.length && stack[stack.length - 1].level >= level) {
      stack.pop();
    }
    if (stack.length) {
      stack[stack.length - 1].children.push(section);
    } else {
      roots.push(section);
    }
    stack.push(section);
  }

  // Everything between the H1 and the first section heading is the intro; its
  // first paragraph is the blurb.
  const introStart = (headings.find((h) => h.level === 1)?.line ?? -1) + 1;
  const introEnd = sectionHeadings[0]?.line ?? lines.length;
  const introRaw = slice(introStart, introEnd);
  const intro = parseBlocks(introRaw);
  const blurb = intro.find((block) => block.type === 'paragraph')?.text ?? '';

  return { title, blurb, intro, introRaw, sections: roots, raw: markdown };
}

/** @returns {Block[]} */
export function parseBlocks(text) {
  const lines = text.split(/\r?\n/);
  /** @type {Block[]} */
  const blocks = [];
  let index = 0;

  while (index < lines.length) {
    const line = lines[index];

    if (!line.trim()) {
      index += 1;
      continue;
    }

    const fence = line.match(/^\s*```(\S*)\s*$/);
    if (fence) {
      const body = [];
      index += 1;
      while (index < lines.length && !/^\s*```\s*$/.test(lines[index])) {
        body.push(lines[index]);
        index += 1;
      }
      index += 1; // closing fence
      blocks.push({ type: 'code', lang: fence[1], text: body.join('\n') });
      continue;
    }

    if (line.startsWith('|')) {
      const tableLines = [];
      while (index < lines.length && lines[index].startsWith('|')) {
        tableLines.push(lines[index]);
        index += 1;
      }
      blocks.push(parseTable(tableLines));
      continue;
    }

    if (LIST_ITEM.test(line)) {
      const items = [];
      while (index < lines.length) {
        const itemMatch = lines[index].match(LIST_ITEM);
        if (itemMatch) {
          items.push({ text: itemMatch[2].trim(), depth: Math.floor(itemMatch[1].length / 2) });
          index += 1;
        } else if (lines[index].trim() && /^\s{2,}/.test(lines[index]) && items.length) {
          // Continuation line of the previous item.
          items[items.length - 1].text += ` ${lines[index].trim()}`;
          index += 1;
        } else {
          break;
        }
      }
      blocks.push({ type: 'list', items });
      continue;
    }

    if (line.startsWith('>')) {
      const quoteLines = [];
      while (index < lines.length && lines[index].startsWith('>')) {
        quoteLines.push(lines[index].replace(/^>\s?/, ''));
        index += 1;
      }
      blocks.push({ type: 'quote', text: quoteLines.join('\n').trim() });
      continue;
    }

    const paragraphLines = [];
    while (
      index < lines.length &&
      lines[index].trim() &&
      !lines[index].startsWith('|') &&
      !lines[index].startsWith('>') &&
      !LIST_ITEM.test(lines[index]) &&
      !FENCE.test(lines[index])
    ) {
      paragraphLines.push(lines[index]);
      index += 1;
    }
    blocks.push({ type: 'paragraph', text: paragraphLines.join('\n').trim() });
  }

  return blocks;
}

/** @returns {Block} */
function parseTable(tableLines) {
  const rows = tableLines
    .filter((line) => !TABLE_SEPARATOR.test(line))
    .map((line) =>
      line
        .replace(/^\|/, '')
        .replace(/\|$/, '')
        .split('|')
        .map((cell) => cell.trim()),
    );
  const [header = [], ...body] = rows;
  return { type: 'table', header, rows: body };
}

/** Strips surrounding bold markers from a table label cell. */
export function plainLabel(cell) {
  return cell.replace(/^\*\*(.*)\*\*$/, '$1').trim();
}
