import { spawnSync } from 'node:child_process';

const rawArgs = process.argv.slice(2);
const isWindows = process.platform === 'win32';
const command = isWindows ? 'pwsh' : 'bash';
const scriptPath = isWindows ? './scripts/pipeline.ps1' : './scripts/pipeline.sh';
const userArgs = isWindows
  ? rawArgs.map((arg) => (arg === '--help' || arg === '-h' ? '-?' : arg))
  : rawArgs;
const args = [scriptPath, ...userArgs];

const result = spawnSync(command, args, {
  stdio: 'inherit',
  shell: process.platform === 'win32',
});

if (result.error) {
  throw result.error;
}

process.exit(result.status ?? 1);
