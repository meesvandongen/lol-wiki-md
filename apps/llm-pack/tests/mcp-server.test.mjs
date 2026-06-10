import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { buildPack } from '../src/build.mjs';

const fixturesRoot = fileURLToPath(new URL('./fixtures/sample-markdown', import.meta.url));
const serverPath = fileURLToPath(new URL('../src/mcp-server.mjs', import.meta.url));

/** Spawns the MCP server and resolves responses to a batch of requests. */
function rpcSession(packRoot, requests) {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, [serverPath], {
      env: { ...process.env, LLM_PACK_ROOT: packRoot },
      stdio: ['pipe', 'pipe', 'pipe'],
    });

    const responses = new Map();
    const expected = requests.filter((request) => request.id !== undefined).length;
    let buffer = '';
    const timer = setTimeout(() => {
      child.kill();
      reject(new Error(`timed out waiting for responses; got ${responses.size}/${expected}`));
    }, 15000);

    child.stdout.on('data', (data) => {
      buffer += data.toString();
      let newlineIndex;
      while ((newlineIndex = buffer.indexOf('\n')) !== -1) {
        const line = buffer.slice(0, newlineIndex);
        buffer = buffer.slice(newlineIndex + 1);
        if (!line.trim()) {
          continue;
        }
        const message = JSON.parse(line);
        responses.set(message.id, message);
        if (responses.size === expected) {
          clearTimeout(timer);
          child.kill();
          resolve(responses);
        }
      }
    });
    child.on('error', (error) => {
      clearTimeout(timer);
      reject(error);
    });

    for (const request of requests) {
      child.stdin.write(`${JSON.stringify({ jsonrpc: '2.0', ...request })}\n`);
    }
  });
}

test('MCP server handshake, tool listing, and tool calls over stdio', async (t) => {
  const packRoot = fs.mkdtempSync(path.join(os.tmpdir(), 'llm-pack-mcp-'));
  t.after(() => fs.rmSync(packRoot, { recursive: true, force: true }));
  buildPack(fixturesRoot, packRoot);

  const responses = await rpcSession(packRoot, [
    {
      id: 1,
      method: 'initialize',
      params: { protocolVersion: '2024-11-05', capabilities: {}, clientInfo: { name: 'test', version: '0' } },
    },
    { method: 'notifications/initialized' },
    { id: 2, method: 'tools/list' },
    {
      id: 3,
      method: 'tools/call',
      params: { name: 'search_wiki', arguments: { query: 'aatrox q sweetspot' } },
    },
    {
      id: 4,
      method: 'tools/call',
      params: { name: 'get_entity', arguments: { category: 'items', name: 'Infinity Edge' } },
    },
    { id: 5, method: 'tools/call', params: { name: 'get_chunk', arguments: { id: 'nope' } } },
    { id: 6, method: 'no/such/method' },
  ]);

  const init = responses.get(1).result;
  assert.equal(init.serverInfo.name, 'lol-wiki-llm-pack');
  assert.ok(init.capabilities.tools);

  const toolNames = responses.get(2).result.tools.map((tool) => tool.name);
  assert.ok(toolNames.includes('search_wiki'));
  assert.ok(toolNames.includes('get_entity'));
  assert.ok(toolNames.includes('get_digest'));

  const searchResults = JSON.parse(responses.get(3).result.content[0].text);
  assert.equal(searchResults[0].id, 'champions/aatrox#abilities/q-the-darkin-blade');

  const infinityEdge = JSON.parse(responses.get(4).result.content[0].text);
  assert.equal(infinityEdge.overview.Cost, 'Total: 3500 • Combine: 725');

  assert.equal(responses.get(5).result.isError, true);
  assert.equal(responses.get(6).error.code, -32601);
});
