import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { chunkDoc } from '../src/chunks.mjs';
import { parseDoc } from '../src/markdown.mjs';
import { buildIndex, search } from '../src/search.mjs';

const fixturesRoot = fileURLToPath(new URL('./fixtures/markdown', import.meta.url));

function chunksFor(category, relPath) {
  const markdown = fs.readFileSync(path.join(fixturesRoot, relPath), 'utf8');
  return chunkDoc(category, parseDoc(markdown));
}

test('chunk IDs are stable, structural, and unique', () => {
  const chunks = chunksFor('champions', 'champions/Aatrox.md');
  const ids = chunks.map((chunk) => chunk.id);

  assert.ok(ids.includes('champions/aatrox#_intro'));
  assert.ok(ids.includes('champions/aatrox#stats'));
  assert.ok(ids.includes('champions/aatrox#abilities/q-the-darkin-blade'));
  assert.ok(ids.includes('champions/aatrox#special-statistics/aram'));
  assert.equal(new Set(ids).size, ids.length);
});

test('chunks are lossless: every content line of the page appears in a chunk', () => {
  for (const [category, relPath] of [
    ['champions', 'champions/Aatrox.md'],
    ['items', 'items/Infinity_Edge.md'],
    ['runes', 'runes/Electrocute.md'],
  ]) {
    const markdown = fs.readFileSync(path.join(fixturesRoot, relPath), 'utf8');
    const chunks = chunkDoc(category, parseDoc(markdown));
    const corpus = chunks.map((chunk) => chunk.text).join('\n');
    const headingPaths = chunks.flatMap((chunk) => chunk.headingPath);

    for (const line of markdown.split('\n')) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith('# ')) {
        continue; // page H1 is carried by the chunk IDs/entity field
      }
      const headingMatch = trimmed.match(/^#{2,6}\s+(.*)$/);
      if (headingMatch && headingPaths.includes(headingMatch[1])) {
        continue; // container headings are carried by the chunk heading paths
      }
      assert.ok(corpus.includes(trimmed), `${relPath}: line missing from chunks: ${trimmed}`);
    }
  }
});

test('BM25 search finds the right section, including single-letter ability keys', () => {
  const chunks = [
    ...chunksFor('champions', 'champions/Aatrox.md'),
    ...chunksFor('items', 'items/Infinity_Edge.md'),
    ...chunksFor('runes', 'runes/Electrocute.md'),
  ];
  const index = buildIndex(chunks);

  const sweetspot = search(index, 'aatrox q sweetspot knock up');
  assert.equal(sweetspot[0].chunk.id, 'champions/aatrox#abilities/q-the-darkin-blade');

  const crit = search(index, 'infinity edge critical strike damage', { category: 'items' });
  assert.ok(crit.length > 0);
  assert.ok(crit.every(({ chunk }) => chunk.category === 'items'));
  assert.ok(crit[0].chunk.id.startsWith('items/infinity-edge#'));

  const electrocute = search(index, 'electrocute stacks lightning cooldown');
  assert.ok(
    electrocute
      .slice(0, 2)
      .some(({ chunk }) => chunk.id === 'runes/electrocute#description'),
  );

  assert.deepEqual(search(index, ''), []);
});
