import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { buildPack } from '../src/build.mjs';
import { loadPack } from '../src/pack.mjs';

const fixturesRoot = fileURLToPath(new URL('./fixtures/markdown', import.meta.url));

function buildToTemp() {
  const outputRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'llm-pack-test-'));
  const result = buildPack(fixturesRoot, outputRoot);
  return { outputRoot, result };
}

test('buildPack writes all artifacts with verbatim values', (t) => {
  const { outputRoot, result } = buildToTemp();
  t.after(() => fs.rmSync(outputRoot, { recursive: true, force: true }));

  for (const artifact of result.manifest.artifacts) {
    assert.ok(fs.existsSync(path.join(outputRoot, artifact)), `missing artifact: ${artifact}`);
  }
  assert.deepEqual(result.manifest.counts.perCategory, { champions: 1, items: 1, runes: 1 });
  assert.equal(result.manifest.counts.pages, 3);
  assert.ok(result.manifest.counts.chunks > 10);

  // Game values survive the whole build verbatim.
  const champions = fs.readFileSync(path.join(outputRoot, 'data/champions.json'), 'utf8');
  assert.ok(champions.includes('"base": "0.651"'));
  assert.ok(champions.includes('14 / 12 / 10 / 8 / 6'));
  const items = fs.readFileSync(path.join(outputRoot, 'data/items.json'), 'utf8');
  assert.ok(items.includes('Total: 3500 • Combine: 725'));

  const digest = fs.readFileSync(path.join(outputRoot, 'digest.md'), 'utf8');
  assert.ok(digest.includes('Aatrox — The Darkin Blade | Top, Jungle | Blood Well'));
  assert.ok(digest.includes('HP 650 (+114/lvl)'));
  assert.ok(digest.includes('Electrocute — Domination | Keystone'));

  const llms = fs.readFileSync(path.join(outputRoot, 'llms.txt'), 'utf8');
  assert.ok(llms.includes('# LoL Wiki — LLM Pack'));
  assert.ok(llms.includes('data/champions.json'));
});

test('content hash is stable across rebuilds of identical input', (t) => {
  const first = buildToTemp();
  const second = buildToTemp();
  t.after(() => {
    fs.rmSync(first.outputRoot, { recursive: true, force: true });
    fs.rmSync(second.outputRoot, { recursive: true, force: true });
  });

  assert.equal(first.result.manifest.contentHash, second.result.manifest.contentHash);
});

test('loadPack round-trips the built pack and serves lookups', (t) => {
  const { outputRoot } = buildToTemp();
  t.after(() => fs.rmSync(outputRoot, { recursive: true, force: true }));

  const pack = loadPack(outputRoot);

  assert.equal(pack.chunks.length, pack.manifest.counts.chunks);
  assert.deepEqual(pack.listEntities('champions'), { champions: ['Aatrox'] });

  const aatrox = pack.getEntity('champions', 'aatrox');
  assert.equal(aatrox.name, 'Aatrox');
  assert.equal(aatrox.stats.HP.base, '650');

  const chunk = pack.getChunk('items/infinity-edge#patch-history/v26-01');
  assert.ok(chunk.text.includes('Total cost increased to 3500 from 3450.'));

  const results = pack.searchFormatted('umbral dash heals', { limit: 3 });
  assert.equal(results[0].id, 'champions/aatrox#abilities/e-umbral-dash');
  assert.ok(results[0].preview.length > 0);

  const pageChunks = pack.getPageChunks('runes', 'Electrocute');
  assert.ok(pageChunks.length >= 4);
});

test('buildPack fails loudly when input is missing or empty', (t) => {
  assert.throws(() => buildPack('/nonexistent/path', '/tmp/unused'), /Input root not found/);

  const emptyRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'llm-pack-empty-'));
  t.after(() => fs.rmSync(emptyRoot, { recursive: true, force: true }));
  assert.throws(() => buildPack(emptyRoot, '/tmp/unused'), /No Markdown pages found/);
});
