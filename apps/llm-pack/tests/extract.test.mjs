import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { extractChampion, extractItem, extractRune } from '../src/extract.mjs';
import { parseDoc } from '../src/markdown.mjs';

const fixturesRoot = fileURLToPath(new URL('./fixtures/markdown', import.meta.url));

function fixture(relPath) {
  return fs.readFileSync(path.join(fixturesRoot, relPath), 'utf8');
}

test('champion extraction lifts overview, stats, and abilities verbatim', () => {
  const doc = parseDoc(fixture('champions/Aatrox.md'));
  const champion = extractChampion(doc);

  assert.equal(champion.name, 'Aatrox');
  assert.equal(champion.blurb, 'Aatrox is a champion in League of Legends.');
  assert.equal(champion.overview.Title, 'The Darkin Blade');
  assert.equal(champion.overview.Roles, 'Top, Jungle');
  assert.equal(champion.overview.Resource, 'Blood Well');

  // Numbers round-trip exactly: strings, no reformatting.
  assert.deepEqual(champion.stats['Attack Speed'], { base: '0.651', growth: '2.5' });
  assert.deepEqual(champion.stats.HP, { base: '650', growth: '114' });
  assert.equal(champion.advancedStats['Windup %'], '19.74%');
  assert.equal(champion.specialStatistics.ARAM['Damage Dealt'], '1.05');
  assert.equal(champion.specialStatistics['Ultra Rapid Fire']['Damage Taken'], '0.7');

  assert.equal(champion.abilities.length, 5);
  assert.deepEqual(
    champion.abilities.map((ability) => ability.slot),
    ['Passive', 'Q', 'W', 'E', 'R'],
  );

  const q = champion.abilities[1];
  assert.equal(q.name, 'The Darkin Blade');
  assert.equal(q.attributes.Cooldown, '14 / 12 / 10 / 8 / 6');
  assert.equal(q.details.Targeting, 'Direction');
  assert.ok(q.description.includes('three times before the ability goes on cooldown'));
  const firstCast = q.effects.find((effect) => effect.label === 'First Cast Damage');
  assert.equal(firstCast.value, '10 / 25 / 40 / 55 / 70 (+ 60 / 67.5 / 75 / 82.5 / 90% AD)');
  assert.ok(q.notes.some((note) => note.includes('sweetspot')));

  // Unmodeled sections are preserved verbatim, not dropped.
  assert.ok(champion.sections.Trivia.includes('login screens'));
  assert.equal(champion.validation.length, 2);
});

test('item extraction lifts overview, stats, build tree, and patch history', () => {
  const doc = parseDoc(fixture('items/Infinity_Edge.md'));
  const item = extractItem(doc);

  assert.equal(item.name, 'Infinity Edge');
  assert.equal(item.overview.Cost, 'Total: 3500 • Combine: 725');
  assert.equal(item.overview.Type, 'Legendary');
  assert.equal(item.stats.Ad, '+75');
  assert.equal(item.stats.Crit, '+25%');
  assert.deepEqual(item.buildTree.Components, ['B. F. Sword', 'Pickaxe', 'Cloak of Agility']);
  assert.ok(item.similarItems.includes('Essence Reaver'));

  const v2601 = item.patchHistory.find((entry) => entry.version === 'V26.01');
  assert.ok(v2601.changes.includes('Total cost increased to 3500 from 3450.'));
  // Nested patch lines keep their nesting via indentation.
  const v1310 = item.patchHistory.find((entry) => entry.version === 'V13.10');
  assert.ok(v1310.changes.some((change) => change.startsWith('  ')));

  assert.ok(item.sections.Background.includes('Shurima'));
  assert.equal(item.validation.length, 3);
});

test('rune extraction lifts overview, description, and patch history', () => {
  const doc = parseDoc(fixture('runes/Electrocute.md'));
  const rune = extractRune(doc);

  assert.equal(rune.name, 'Electrocute');
  assert.equal(rune.overview.Path, 'Domination');
  assert.equal(rune.overview.Slot, 'Keystone');
  assert.ok(rune.description.includes('70 – 240 (based on level) (+ 10% **bonus** AD) (+ 5% AP)'));
  assert.ok(rune.notes.some((note) => note.includes('proc damage')));

  const v8_16 = rune.patchHistory.find((entry) => entry.version === 'V8.16');
  assert.ok(
    v8_16.changes.some((change) =>
      change.includes('formula: 40 + (10×level)'),
    ),
  );
});
