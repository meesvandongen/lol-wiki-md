#!/usr/bin/env node
// Model Context Protocol server for the LoL Wiki LLM pack.
//
// Speaks MCP's stdio transport (newline-delimited JSON-RPC 2.0) using only
// the Node standard library, so there is no SDK dependency to keep current.
// Point any MCP client at it:
//
//   { "command": "node", "args": ["apps/llm-pack/src/mcp-server.mjs"],
//     "env": { "LLM_PACK_ROOT": "/path/to/generated/llm-pack" } }
//
// The pack is loaded once at startup; rebuild the pack and restart to pick
// up new wiki content.

import path from 'node:path';
import process from 'node:process';
import readline from 'node:readline';

import { loadPack, resolveDefaultPackRoot } from './pack.mjs';

const PROTOCOL_VERSION = '2024-11-05';

const packRoot = path.resolve(process.env.LLM_PACK_ROOT ?? resolveDefaultPackRoot());
let pack;
try {
  pack = loadPack(packRoot);
} catch (error) {
  console.error(`[llm-pack mcp] ${error.message}`);
  process.exit(1);
}

const TOOLS = [
  {
    name: 'search_wiki',
    description:
      'Full-text (BM25) search over the League of Legends wiki pack. Returns ranked ' +
      'section chunks with stable IDs; fetch full text with get_chunk. Single letters ' +
      'like q/w/e/r are valid ability terms.',
    inputSchema: {
      type: 'object',
      properties: {
        query: { type: 'string', description: 'Search terms, e.g. "aatrox q sweetspot"' },
        category: {
          type: 'string',
          enum: ['champions', 'items', 'runes'],
          description: 'Optional category filter.',
        },
        limit: { type: 'number', description: 'Max results (default 8).' },
      },
      required: ['query'],
    },
    handler: ({ query, category, limit }) =>
      json(pack.searchFormatted(query, { category, limit })),
  },
  {
    name: 'get_chunk',
    description:
      'Fetch the verbatim Markdown of one section chunk by ID ' +
      '(IDs come from search_wiki or get_page).',
    inputSchema: {
      type: 'object',
      properties: {
        id: { type: 'string', description: 'Chunk ID, e.g. "champions/aatrox#abilities/q-the-darkin-blade"' },
      },
      required: ['id'],
    },
    handler: ({ id }) => {
      const chunk = pack.getChunk(id);
      if (!chunk) {
        return fail(`chunk not found: ${id}`);
      }
      return text(chunk.text);
    },
  },
  {
    name: 'get_page',
    description:
      'Fetch a whole wiki page as Markdown (all of its chunks in order). ' +
      'Prefer get_chunk for token economy on long pages.',
    inputSchema: {
      type: 'object',
      properties: {
        category: { type: 'string', enum: ['champions', 'items', 'runes'] },
        name: { type: 'string', description: 'Entity name, e.g. "Aatrox" or "Infinity Edge".' },
      },
      required: ['category', 'name'],
    },
    handler: ({ category, name }) => {
      const chunks = pack.getPageChunks(category, name);
      if (chunks.length === 0) {
        return fail(`page not found: ${category}/${name}`);
      }
      return text(chunks.map((chunk) => chunk.text).join('\n\n'));
    },
  },
  {
    name: 'get_entity',
    description:
      'Fetch the structured JSON record for a champion, item, or rune: stats, ' +
      'abilities, patch history. All game values are verbatim wiki strings ' +
      '(e.g. "14 / 12 / 10 / 8 / 6"); nothing is computed or inferred.',
    inputSchema: {
      type: 'object',
      properties: {
        category: { type: 'string', enum: ['champions', 'items', 'runes'] },
        name: { type: 'string', description: 'Entity name, e.g. "Aatrox".' },
      },
      required: ['category', 'name'],
    },
    handler: ({ category, name }) => {
      const entity = pack.getEntity(category, name);
      if (!entity) {
        return fail(`entity not found: ${category}/${name}`);
      }
      return json(entity);
    },
  },
  {
    name: 'list_entities',
    description: 'List entity names in the pack, optionally for one category.',
    inputSchema: {
      type: 'object',
      properties: {
        category: { type: 'string', enum: ['champions', 'items', 'runes'] },
      },
    },
    handler: ({ category } = {}) => json(pack.listEntities(category)),
  },
  {
    name: 'get_digest',
    description:
      'Fetch the roster digest: one line per entity with names, roles, and base ' +
      'stats. The cheapest way to ground the whole game roster.',
    inputSchema: { type: 'object', properties: {} },
    handler: () => text(pack.digest),
  },
  {
    name: 'get_manifest',
    description: 'Fetch pack build metadata: counts, content hash, source files.',
    inputSchema: { type: 'object', properties: {} },
    handler: () => json(pack.manifest),
  },
];

const toolsByName = new Map(TOOLS.map((tool) => [tool.name, tool]));

function text(value) {
  return { content: [{ type: 'text', text: value }] };
}

function json(value) {
  return text(JSON.stringify(value, null, 2));
}

function fail(message) {
  return { content: [{ type: 'text', text: message }], isError: true };
}

const HANDLERS = {
  initialize: (params) => ({
    protocolVersion: params?.protocolVersion ?? PROTOCOL_VERSION,
    capabilities: { tools: {} },
    serverInfo: {
      name: 'lol-wiki-llm-pack',
      version: pack.manifest.contentHash.slice(0, 12),
    },
    instructions:
      'League of Legends wiki knowledge pack. Start with get_digest or ' +
      'list_entities to ground names, search_wiki to find mechanics, ' +
      'get_entity for structured numbers. All values are verbatim wiki ' +
      'strings — treat them as the source of truth, not model memory.',
  }),
  ping: () => ({}),
  'tools/list': () => ({
    tools: TOOLS.map(({ name, description, inputSchema }) => ({ name, description, inputSchema })),
  }),
  'tools/call': (params) => {
    const tool = toolsByName.get(params?.name);
    if (!tool) {
      return fail(`unknown tool: ${params?.name}`);
    }
    try {
      return tool.handler(params.arguments ?? {});
    } catch (error) {
      return fail(`tool ${params.name} failed: ${error.message}`);
    }
  },
  // Lenient empty answers for capability surfaces this server does not offer.
  'resources/list': () => ({ resources: [] }),
  'resources/templates/list': () => ({ resourceTemplates: [] }),
  'prompts/list': () => ({ prompts: [] }),
};

const rl = readline.createInterface({ input: process.stdin, terminal: false });

rl.on('line', (line) => {
  const trimmed = line.trim();
  if (!trimmed) {
    return;
  }

  let message;
  try {
    message = JSON.parse(trimmed);
  } catch {
    send({ jsonrpc: '2.0', id: null, error: { code: -32700, message: 'parse error' } });
    return;
  }

  const { id, method, params } = message;
  const isNotification = id === undefined || id === null;

  if (typeof method !== 'string') {
    if (!isNotification) {
      send({ jsonrpc: '2.0', id, error: { code: -32600, message: 'invalid request' } });
    }
    return;
  }
  if (method.startsWith('notifications/')) {
    return;
  }

  const handler = HANDLERS[method];
  if (!handler) {
    if (!isNotification) {
      send({ jsonrpc: '2.0', id, error: { code: -32601, message: `method not found: ${method}` } });
    }
    return;
  }

  try {
    const result = handler(params);
    if (!isNotification) {
      send({ jsonrpc: '2.0', id, result });
    }
  } catch (error) {
    if (!isNotification) {
      send({ jsonrpc: '2.0', id, error: { code: -32603, message: error.message } });
    }
  }
});

function send(message) {
  process.stdout.write(`${JSON.stringify(message)}\n`);
}

console.error(
  `[llm-pack mcp] serving ${pack.manifest.counts.pages} pages / ${pack.manifest.counts.chunks} chunks from ${packRoot}`,
);
