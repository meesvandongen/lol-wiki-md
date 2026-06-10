// Builds the LLM pack from the converter's Markdown output.
//
// Inputs:  generated/markdown/{champions,items,runes}/*.md
// Outputs: generated/llm-pack/
//   manifest.json   build metadata, entity index, content hash
//   llms.txt        llms.txt-convention entry point describing the pack
//   digest.md       token-compact roster of every entity (verbatim values)
//   data/*.json     structured per-category datasets
//   chunks.jsonl    retrieval-ready chunks with stable IDs

import { createHash } from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';

import { chunkDoc } from './chunks.mjs';
import { extractChampion, extractItem, extractRune } from './extract.mjs';
import { parseDoc } from './markdown.mjs';

export const CATEGORIES = [
  { key: 'champions', extract: extractChampion },
  { key: 'items', extract: extractItem },
  { key: 'runes', extract: extractRune },
];

export function resolveDefaultInputRoot(cwd = process.cwd()) {
  return path.resolve(cwd, 'generated/markdown');
}

/**
 * Reads and parses every page in the input root.
 * Returns { pages, sourceFiles } where pages carry doc + extracted data.
 */
export function loadPages(inputRoot) {
  const pages = [];
  const sourceFiles = [];

  for (const { key, extract } of CATEGORIES) {
    const categoryDir = path.join(inputRoot, key);
    if (!fs.existsSync(categoryDir)) {
      continue;
    }
    const files = fs
      .readdirSync(categoryDir)
      .filter((file) => file.endsWith('.md'))
      .sort();
    for (const file of files) {
      const filePath = path.join(categoryDir, file);
      const markdown = fs.readFileSync(filePath, 'utf8');
      const doc = parseDoc(markdown);
      pages.push({
        category: key,
        file: `${key}/${file}`,
        doc,
        data: extract(doc),
        markdown,
      });
      sourceFiles.push({ path: `${key}/${file}`, sha256: sha256(markdown) });
    }
  }

  return { pages, sourceFiles };
}

export function buildPack(inputRoot, outputRoot) {
  if (!fs.existsSync(inputRoot)) {
    throw new Error(
      `Input root not found: ${inputRoot}. Run \`npm run convert\` first to generate Markdown.`,
    );
  }
  const { pages, sourceFiles } = loadPages(inputRoot);
  if (pages.length === 0) {
    throw new Error(`No Markdown pages found under ${inputRoot}.`);
  }

  const chunks = pages.flatMap((page) => chunkDoc(page.category, page.doc));

  fs.mkdirSync(path.join(outputRoot, 'data'), { recursive: true });

  const dataFiles = [];
  for (const { key } of CATEGORIES) {
    const entities = pages.filter((page) => page.category === key).map((page) => page.data);
    if (entities.length === 0) {
      continue;
    }
    const relPath = `data/${key}.json`;
    writeFile(outputRoot, relPath, `${JSON.stringify(entities, null, 2)}\n`);
    dataFiles.push(relPath);
  }

  const chunksJsonl = `${chunks.map((chunk) => JSON.stringify(chunk)).join('\n')}\n`;
  writeFile(outputRoot, 'chunks.jsonl', chunksJsonl);

  const digest = renderDigest(pages);
  writeFile(outputRoot, 'digest.md', digest);

  const manifest = {
    name: 'lol-wiki-llm-pack',
    generatedAt: new Date().toISOString(),
    contentHash: contentHash(sourceFiles),
    source: {
      inputRoot: path.relative(process.cwd(), inputRoot) || '.',
      files: sourceFiles,
    },
    counts: {
      pages: pages.length,
      perCategory: Object.fromEntries(
        CATEGORIES.map(({ key }) => [key, pages.filter((page) => page.category === key).length]),
      ),
      chunks: chunks.length,
      corpusTokensEst: chunks.reduce((sum, chunk) => sum + chunk.tokensEst, 0),
    },
    artifacts: ['manifest.json', 'llms.txt', 'digest.md', 'chunks.jsonl', ...dataFiles],
    conventions: {
      chunkId: '<category>/<entity-slug>#<heading-path-slug>[@part]',
      values: 'All game values are verbatim wiki strings; nothing is computed or inferred.',
      tokensEst: 'Heuristic estimate (~4 chars/token), not a tokenizer count.',
    },
  };
  writeFile(outputRoot, 'manifest.json', `${JSON.stringify(manifest, null, 2)}\n`);

  writeFile(outputRoot, 'llms.txt', renderLlmsTxt(pages, manifest));

  return { manifest, pages, chunks };
}

// ---------------------------------------------------------------------------
// Digest and llms.txt rendering

/** One verbatim-valued line per entity; the whole game roster in few tokens. */
export function digestLine(page) {
  const data = page.data;
  if (page.category === 'champions') {
    const parts = [data.overview.Title, data.overview.Roles, data.overview.Resource]
      .filter(Boolean)
      .join(' | ');
    const stats = ['HP', 'Attack Damage', 'Armor', 'Magic Resist', 'Move Speed', 'Range']
      .filter((stat) => data.stats[stat])
      .map((stat) => `${stat} ${data.stats[stat].base}${growthSuffix(data.stats[stat].growth)}`)
      .join(', ');
    return [data.name, parts, stats].filter(Boolean).join(' — ');
  }
  if (page.category === 'items') {
    const head = [data.overview.Type, data.overview.Cost].filter(Boolean).join(' | ');
    const stats = Object.entries(data.stats)
      .map(([stat, value]) => `${stat} ${value}`)
      .join(', ');
    return [data.name, head, stats].filter(Boolean).join(' — ');
  }
  const head = [data.overview.Path, data.overview.Slot].filter(Boolean).join(' | ');
  return [data.name, head].filter(Boolean).join(' — ');
}

function growthSuffix(growth) {
  return growth && growth !== '0' ? ` (+${growth}/lvl)` : '';
}

function renderDigest(pages) {
  const lines = [
    '# League of Legends roster digest',
    '',
    'One line per entity; every value is the verbatim wiki string. Champion',
    'base stats show level-1 values with per-level growth in parentheses.',
    '',
  ];
  for (const { key } of CATEGORIES) {
    const categoryPages = pages.filter((page) => page.category === key);
    if (categoryPages.length === 0) {
      continue;
    }
    lines.push(`## ${capitalize(key)} (${categoryPages.length})`, '');
    for (const page of categoryPages) {
      lines.push(`- ${digestLine(page)}`);
    }
    lines.push('');
  }
  return `${lines.join('\n').trimEnd()}\n`;
}

function renderLlmsTxt(pages, manifest) {
  const lines = [
    '# LoL Wiki — LLM Pack',
    '',
    '> Machine-oriented snapshot of the League of Legends Wiki (champions,',
    '> items, runes), compiled from wiki exports by the lol-wiki-md pipeline.',
    '> All game values are verbatim wiki strings; nothing is computed or',
    '> inferred. The wiki, not patch notes or model memory, is the source of',
    '> truth here.',
    '',
    `Content hash: ${manifest.contentHash}`,
    `Pages: ${manifest.counts.pages} · Chunks: ${manifest.counts.chunks} · ~${manifest.counts.corpusTokensEst} tokens`,
    '',
    '## Artifacts',
    '',
    '- [manifest.json](./manifest.json): build metadata, source file hashes, counts',
    '- [digest.md](./digest.md): one-line-per-entity roster — cheapest way to ground names and base numbers',
    '- [chunks.jsonl](./chunks.jsonl): retrieval-ready sections; stable IDs of the form `<category>/<entity-slug>#<heading-path-slug>`',
  ];
  for (const { key } of CATEGORIES) {
    if (pages.some((page) => page.category === key)) {
      lines.push(
        `- [data/${key}.json](./data/${key}.json): structured ${key} dataset (stats, abilities, patch history as verbatim strings)`,
      );
    }
  }
  lines.push(
    '',
    '## How to consume this pack',
    '',
    '- Need names/roles/base stats only: read digest.md.',
    '- Need exact mechanics text: look up chunks.jsonl by chunk ID, or run the',
    '  MCP server (`npm run llm:mcp`) and call `search_wiki` / `get_chunk`.',
    '- Need machine-readable numbers: read data/*.json. Values are strings on',
    '  purpose — ranges like `14 / 12 / 10 / 8 / 6` and scalings like',
    '  `(+ 60% AD)` must round-trip exactly.',
    '- Unmodeled sections are preserved verbatim under each entity\'s',
    '  `sections` map, so the JSON view never silently drops wiki content.',
    '',
  );

  for (const { key } of CATEGORIES) {
    const categoryPages = pages.filter((page) => page.category === key);
    if (categoryPages.length === 0) {
      continue;
    }
    lines.push(`## ${capitalize(key)}`, '');
    for (const page of categoryPages) {
      lines.push(`- ${digestLine(page)}`);
    }
    lines.push('');
  }

  return `${lines.join('\n').trimEnd()}\n`;
}

function capitalize(text) {
  return text.charAt(0).toUpperCase() + text.slice(1);
}

// ---------------------------------------------------------------------------
// IO helpers

function writeFile(root, relPath, content) {
  const filePath = path.join(root, relPath);
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, content);
}

function sha256(content) {
  return createHash('sha256').update(content).digest('hex');
}

/** Hash of all source content; excludes timestamps so identical inputs diff clean. */
function contentHash(sourceFiles) {
  const hash = createHash('sha256');
  for (const file of [...sourceFiles].sort((a, b) => a.path.localeCompare(b.path))) {
    hash.update(file.path);
    hash.update('\0');
    hash.update(file.sha256);
    hash.update('\n');
  }
  return hash.digest('hex');
}
