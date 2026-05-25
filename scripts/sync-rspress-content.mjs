import { promises as fs } from 'node:fs';
import path from 'node:path';

const sourceRoot = path.resolve(process.argv[2] ?? 'generated/markdown');
const targetRoot = path.resolve(process.argv[3] ?? 'apps/docs/docs');

const categories = [
  {
    key: 'champions',
    title: 'Champions',
    description: 'Generated champion markdown pages from the League Wiki converter.',
    icon: '⚔️',
  },
  {
    key: 'items',
    title: 'Items',
    description: 'Generated item markdown pages from the League Wiki converter.',
    icon: '🛡️',
  },
  {
    key: 'runes',
    title: 'Runes',
    description: 'Generated rune markdown pages from the League Wiki converter.',
    icon: '✨',
  },
];

function yamlString(value) {
  return JSON.stringify(String(value));
}

function titleFromFilename(filename) {
  const stem = filename.replace(/\.md$/i, '');
  const maybeDecoded = (() => {
    try {
      return decodeURIComponent(stem);
    } catch {
      return stem;
    }
  })();

  return maybeDecoded.replace(/_/g, ' ');
}

async function ensureCleanDirectory(dir) {
  await fs.rm(dir, { recursive: true, force: true });
  await fs.mkdir(dir, { recursive: true });
}

async function listMarkdownFiles(dir) {
  try {
    const entries = await fs.readdir(dir, { withFileTypes: true });
    return entries
      .filter((entry) => entry.isFile() && entry.name.endsWith('.md'))
      .map((entry) => entry.name)
      .sort((left, right) => left.localeCompare(right));
  } catch (error) {
    if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT') {
      return [];
    }

    throw error;
  }
}

async function writeFile(filePath, content) {
  await fs.mkdir(path.dirname(filePath), { recursive: true });
  await fs.writeFile(filePath, content, 'utf8');
}

function stemFromFile(file) {
  return file.replace(/\.md$/i, '');
}

function routeStemFromTitle(title) {
  const normalized = title
    .normalize('NFKD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/[’']/g, '');

  const slug = normalized
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');

  return slug || 'page';
}

function uniqueRouteStem(baseStem, usedStems) {
  let candidate = baseStem;
  let suffix = 2;

  while (usedStems.has(candidate)) {
    candidate = `${baseStem}-${suffix}`;
    suffix += 1;
  }

  usedStems.add(candidate);
  return candidate;
}

async function main() {
  await ensureCleanDirectory(targetRoot);

  const counts = new Map();
  for (const { key } of categories) {
    counts.set(key, (await listMarkdownFiles(path.join(sourceRoot, key))).length);
  }

  await writeFile(
    path.join(targetRoot, '_nav.json'),
    JSON.stringify(
      [
        { text: 'Home', link: '/' },
        { text: 'Champions', link: '/champions/', activeMatch: '^/champions/' },
        { text: 'Items', link: '/items/', activeMatch: '^/items/' },
        { text: 'Runes', link: '/runes/', activeMatch: '^/runes/' },
      ],
      null,
      2,
    ) + '\n',
  );

  await writeFile(
    path.join(targetRoot, 'index.mdx'),
    `---
pageType: home
title: League Wiki
titleSuffix: '| Export, Convert, Browse'
hero:
  name: League Wiki
  text: Export, convert, and browse.
  tagline: Generated League Wiki documentation powered by the Rust exporter, converter, and Rspress.
  actions:
    - theme: brand
      text: Champions
      link: /champions/
    - theme: alt
      text: Items
      link: /items/
    - theme: alt
      text: Runes
      link: /runes/
features:
  - title: Champions
    details: ${counts.get('champions')} generated champion pages.
    icon: ⚔️
  - title: Items
    details: ${counts.get('items')} generated item pages.
    icon: 🛡️
  - title: Runes
    details: ${counts.get('runes')} generated rune pages.
    icon: ✨
---

## Generated content

- Champions: ${counts.get('champions')}
- Items: ${counts.get('items')}
- Runes: ${counts.get('runes')}

Use the navbar or sidebar to browse the generated documentation.
`,
  );

  for (const { key, title, description, icon } of categories) {
    const categoryInput = path.join(sourceRoot, key);
    const categoryOutput = path.join(targetRoot, key);
    const files = await listMarkdownFiles(categoryInput);
    const relativeSource = path.relative(targetRoot, categoryInput) || key;
    const usedStems = new Set();
    const pageEntries = files.map((file) => {
      const titleValue = titleFromFilename(file);
      const routeStem = uniqueRouteStem(routeStemFromTitle(titleValue), usedStems);

      return {
        file,
        title: titleValue,
        routeStem,
      };
    });

    await fs.mkdir(categoryOutput, { recursive: true });

    await writeFile(
      path.join(categoryOutput, '_meta.json'),
      JSON.stringify(
        [
          'index',
          ...pageEntries.map(({ routeStem, title: pageTitle }) => ({
            type: 'file',
            name: routeStem,
            label: pageTitle,
          })),
        ],
        null,
        2,
      ) + '\n',
    );

    await writeFile(
      path.join(categoryOutput, 'index.mdx'),
      `---
title: ${yamlString(title)}
description: ${yamlString(description)}
---

# ${title}

${icon} Generated pages: ${files.length}

These documents are copied from \`${relativeSource}\` each time the pipeline runs.
`,
    );

    for (const { file, title: titleValue, routeStem } of pageEntries) {
      const sourceFile = path.join(categoryInput, file);
      const raw = await fs.readFile(sourceFile, 'utf8');
      const wrapped = `---\ntitle: ${yamlString(titleValue)}\n---\n\n${raw}`;
      await writeFile(path.join(categoryOutput, `${routeStem}.md`), wrapped);
    }
  }
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});