// Loads a built pack from disk and exposes lookup primitives shared by the
// CLI and the MCP server. The search index is rebuilt from chunks.jsonl at
// load time — chunk counts are small enough that this takes milliseconds and
// it removes a whole class of stale-index bugs.

import fs from 'node:fs';
import path from 'node:path';

import { CATEGORIES } from './build.mjs';
import { buildIndex, search, preview } from './search.mjs';

export function resolveDefaultPackRoot(cwd = process.cwd()) {
  return path.resolve(cwd, 'generated/llm-pack');
}

export function loadPack(packRoot) {
  const manifestPath = path.join(packRoot, 'manifest.json');
  if (!fs.existsSync(manifestPath)) {
    throw new Error(
      `No pack found at ${packRoot}. Run \`npm run llm:build\` first (after \`npm run convert\`).`,
    );
  }
  const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'));

  const chunks = fs
    .readFileSync(path.join(packRoot, 'chunks.jsonl'), 'utf8')
    .split('\n')
    .filter(Boolean)
    .map((line) => JSON.parse(line));

  const data = {};
  for (const { key } of CATEGORIES) {
    const dataPath = path.join(packRoot, 'data', `${key}.json`);
    if (fs.existsSync(dataPath)) {
      data[key] = JSON.parse(fs.readFileSync(dataPath, 'utf8'));
    }
  }

  const digest = readOptional(path.join(packRoot, 'digest.md'));
  const index = buildIndex(chunks);
  const chunksById = new Map(chunks.map((chunk) => [chunk.id, chunk]));

  return {
    packRoot,
    manifest,
    chunks,
    chunksById,
    data,
    digest,
    search(query, options) {
      return search(index, query, options);
    },
    searchFormatted(query, options) {
      return this.search(query, options).map(({ chunk, score }) => ({
        id: chunk.id,
        score: Number(score.toFixed(3)),
        entity: chunk.entity,
        headingPath: chunk.headingPath,
        tokensEst: chunk.tokensEst,
        preview: preview(chunk.text),
      }));
    },
    getChunk(id) {
      return this.chunksById.get(id);
    },
    getEntity(category, name) {
      const entities = this.data[category] ?? [];
      const wanted = name.toLowerCase();
      return entities.find((entity) => entity.name.toLowerCase() === wanted);
    },
    listEntities(category) {
      const categories = category ? [category] : Object.keys(this.data);
      const listing = {};
      for (const key of categories) {
        listing[key] = (this.data[key] ?? []).map((entity) => entity.name);
      }
      return listing;
    },
    getPageChunks(category, name) {
      const wanted = name.toLowerCase();
      return this.chunks.filter(
        (chunk) => chunk.category === category && chunk.entity.toLowerCase() === wanted,
      );
    },
  };
}

function readOptional(filePath) {
  return fs.existsSync(filePath) ? fs.readFileSync(filePath, 'utf8') : '';
}
