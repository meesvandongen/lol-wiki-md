import { spawnSync } from 'node:child_process';

function hasOption(args, option) {
  return args.includes(option) || args.some((arg) => arg.startsWith(`${option}=`));
}

function ensureOption(args, option, value) {
  if (!hasOption(args, option)) {
    args.push(option, value);
  }
}

function run(command, args) {
  const result = spawnSync(command, args, {
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });

  if (result.error) {
    throw result.error;
  }

  process.exit(result.status ?? 1);
}

const cliArgs = process.argv.slice(2);
ensureOption(cliArgs, '--out-dir', './generated/wiki-export/out');
ensureOption(cliArgs, '--meta-dir', './generated/wiki-export/meta');

run('cargo', ['run', '-p', 'lol-wiki-export', '--', ...cliArgs]);
