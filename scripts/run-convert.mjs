import fs from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const entityOptions = [
  '--champion',
  '--item',
  '--rune',
  '--all-champions',
  '--all-items',
  '--all-runes',
];

const fullBatchTargets = [
  { label: 'champions', flag: '--all-champions' },
  { label: 'items', flag: '--all-items' },
  { label: 'runes', flag: '--all-runes' },
];

function hasOption(args, option) {
  return args.includes(option) || args.some((arg) => arg.startsWith(`${option}=`));
}

function ensureOption(args, option, value) {
  if (!hasOption(args, option)) {
    args.push(option, value);
  }
}

function stripFlag(args, option) {
  const stripped = [];

  for (let index = 0; index < args.length; index += 1) {
    const arg = args[index];
    if (arg === option) {
      continue;
    }

    if (arg.startsWith(`${option}=`)) {
      continue;
    }

    stripped.push(arg);
  }

  return stripped;
}

function resolveDefaultWikiRoot() {
  const generatedRoot = path.resolve('generated/wiki-export/out');
  if (fs.existsSync(generatedRoot)) {
    return './generated/wiki-export/out';
  }

  const exportCacheRoot = path.resolve('export_out');
  if (fs.existsSync(exportCacheRoot)) {
    return './export_out';
  }

  return './generated/wiki-export/out';
}

function runConvert(args) {
  const result = spawnSync('cargo', ['run', '-p', 'lol_wiki_md', '--bin', 'convert', '--', ...args], {
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });

  if (result.error) {
    throw result.error;
  }

  return result.status ?? 1;
}

const rawArgs = process.argv.slice(2);
const helpOnly = rawArgs.some((arg) => ['-h', '--help', '-V', '--version'].includes(arg));
const hasEntitySelection = entityOptions.some((option) => hasOption(rawArgs, option));
const sharedArgs = [...rawArgs];

ensureOption(sharedArgs, '--wiki-root', resolveDefaultWikiRoot());
ensureOption(sharedArgs, '--output', './generated/markdown');

if (helpOnly || hasEntitySelection) {
  process.exit(runConvert(sharedArgs));
}

console.log('No entity selection provided; converting champions, items, and runes by default.');

let exitCode = 0;
const failedTargets = [];

for (const [index, target] of fullBatchTargets.entries()) {
  console.log(`\n==> Converting ${target.label}`);
  const perRunArgs = [...(index === 0 ? sharedArgs : stripFlag(sharedArgs, '--validate')), target.flag];
  const status = runConvert(perRunArgs);
  if (status !== 0) {
    exitCode = status;
    failedTargets.push(target.label);
  }
}

if (failedTargets.length > 0) {
  console.error(`\nDefault convert run finished with failures in: ${failedTargets.join(', ')}`);
}

process.exit(exitCode);
