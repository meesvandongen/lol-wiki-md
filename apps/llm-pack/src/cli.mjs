#!/usr/bin/env node
// CLI for building and querying the LLM pack.
//
//   build  [--input DIR] [--output DIR]   compile generated/markdown into the pack
//   search <query> [--category C] [--limit N] [--pack DIR]
//   show   <chunk-id> [--pack DIR]        print a chunk's full text
//   entity <category> <name> [--pack DIR] print an entity's structured JSON
//   list   [category] [--pack DIR]        list entity names

import path from 'node:path';
import process from 'node:process';

import { buildPack, resolveDefaultInputRoot } from './build.mjs';
import { loadPack, resolveDefaultPackRoot } from './pack.mjs';

function parseArgs(argv) {
  const positional = [];
  const options = {};
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg.startsWith('--')) {
      const eq = arg.indexOf('=');
      if (eq !== -1) {
        options[arg.slice(2, eq)] = arg.slice(eq + 1);
      } else {
        options[arg.slice(2)] = argv[i + 1];
        i += 1;
      }
    } else {
      positional.push(arg);
    }
  }
  return { positional, options };
}

function usage() {
  console.error(
    [
      'Usage: cli.mjs <command>',
      '  build  [--input DIR] [--output DIR]',
      '  search <query> [--category champions|items|runes] [--limit N] [--pack DIR]',
      '  show   <chunk-id> [--pack DIR]',
      '  entity <category> <name> [--pack DIR]',
      '  list   [category] [--pack DIR]',
    ].join('\n'),
  );
  process.exit(2);
}

const [command, ...rest] = process.argv.slice(2);
const { positional, options } = parseArgs(rest);

const packRoot = () => path.resolve(options.pack ?? resolveDefaultPackRoot());

try {
  run();
} catch (error) {
  console.error(error.message);
  process.exit(1);
}

function run() {
  switch (command) {
  case 'build': {
    const inputRoot = path.resolve(options.input ?? resolveDefaultInputRoot());
    const outputRoot = path.resolve(options.output ?? resolveDefaultPackRoot());
    const { manifest } = buildPack(inputRoot, outputRoot);
    console.log(
      `built llm-pack: ${manifest.counts.pages} pages, ${manifest.counts.chunks} chunks ` +
        `(~${manifest.counts.corpusTokensEst} tokens) -> ${path.relative(process.cwd(), outputRoot)}`,
    );
    break;
  }
  case 'search': {
    const query = positional.join(' ');
    if (!query) {
      usage();
    }
    const pack = loadPack(packRoot());
    const results = pack.searchFormatted(query, {
      category: options.category,
      limit: options.limit ? Number(options.limit) : undefined,
    });
    if (results.length === 0) {
      console.log('no results');
      break;
    }
    for (const result of results) {
      console.log(`${result.score.toFixed(3).padStart(8)}  ${result.id}`);
      console.log(`          ${result.preview}`);
    }
    break;
  }
  case 'show': {
    const id = positional[0];
    if (!id) {
      usage();
    }
    const pack = loadPack(packRoot());
    const chunk = pack.getChunk(id);
    if (!chunk) {
      console.error(`chunk not found: ${id}`);
      process.exit(1);
    }
    console.log(chunk.text);
    break;
  }
  case 'entity': {
    const [category, ...nameParts] = positional;
    const name = nameParts.join(' ');
    if (!category || !name) {
      usage();
    }
    const pack = loadPack(packRoot());
    const entity = pack.getEntity(category, name);
    if (!entity) {
      console.error(`entity not found: ${category}/${name}`);
      process.exit(1);
    }
    console.log(JSON.stringify(entity, null, 2));
    break;
  }
  case 'list': {
    const pack = loadPack(packRoot());
    console.log(JSON.stringify(pack.listEntities(positional[0]), null, 2));
    break;
  }
  default:
    usage();
  }
}
