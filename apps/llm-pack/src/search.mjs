// Dependency-free BM25 lexical search over pack chunks.
//
// Lexical search keeps the pack maintenance-free: no embedding model, no API
// key, no index format to migrate. Entity names and heading paths are boosted
// so "aatrox q" lands on the right ability without semantic matching. Single
// letters are kept as tokens because Q/W/E/R are meaningful in League.

const K1 = 1.2;
const B = 0.75;
const TITLE_WEIGHT = 3;

const STOPWORDS = new Set([
  'a', 'an', 'and', 'are', 'as', 'at', 'be', 'by', 'for', 'from', 'has', 'his',
  'in', 'is', 'it', 'its', 'of', 'on', 'or', 'that', 'the', 'this', 'to', 'with',
]);

export function tokenize(text) {
  return text
    .toLowerCase()
    .normalize('NFKD')
    .split(/[^a-z0-9%]+/)
    .filter((token) => token && !STOPWORDS.has(token));
}

/**
 * @param {import('./chunks.mjs').Chunk[]} chunks
 */
export function buildIndex(chunks) {
  const docs = [];
  const documentFrequency = new Map();
  let totalLength = 0;

  for (const chunk of chunks) {
    const termFrequency = new Map();
    const add = (tokens, weight) => {
      for (const token of tokens) {
        termFrequency.set(token, (termFrequency.get(token) ?? 0) + weight);
      }
    };
    add(tokenize(`${chunk.entity} ${chunk.headingPath.join(' ')}`), TITLE_WEIGHT);
    add(tokenize(chunk.text), 1);

    let length = 0;
    for (const count of termFrequency.values()) {
      length += count;
    }
    for (const token of termFrequency.keys()) {
      documentFrequency.set(token, (documentFrequency.get(token) ?? 0) + 1);
    }

    docs.push({ chunk, termFrequency, length });
    totalLength += length;
  }

  return {
    docs,
    documentFrequency,
    averageLength: docs.length ? totalLength / docs.length : 0,
  };
}

/**
 * @param {ReturnType<typeof buildIndex>} index
 * @param {string} query
 * @param {{ category?: string, limit?: number }} [options]
 */
export function search(index, query, options = {}) {
  const limit = options.limit ?? 8;
  const queryTokens = tokenize(query);
  if (queryTokens.length === 0) {
    return [];
  }

  const candidates = options.category
    ? index.docs.filter((doc) => doc.chunk.category === options.category)
    : index.docs;
  const n = index.docs.length;

  const scored = [];
  for (const doc of candidates) {
    let score = 0;
    for (const token of queryTokens) {
      const tf = doc.termFrequency.get(token);
      if (!tf) {
        continue;
      }
      const df = index.documentFrequency.get(token) ?? 0;
      const idf = Math.log(1 + (n - df + 0.5) / (df + 0.5));
      score +=
        (idf * tf * (K1 + 1)) /
        (tf + K1 * (1 - B + (B * doc.length) / index.averageLength));
    }
    if (score > 0) {
      scored.push({ chunk: doc.chunk, score });
    }
  }

  scored.sort((a, b) => b.score - a.score || a.chunk.id.localeCompare(b.chunk.id));
  return scored.slice(0, limit);
}

/** A short single-line preview of a chunk body for search results. */
export function preview(text, maxLength = 220) {
  const flat = text.replace(/\s+/g, ' ').trim();
  return flat.length > maxLength ? `${flat.slice(0, maxLength - 1)}…` : flat;
}
