import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const docsOutDir = path.join(rootDir, 'apps', 'docs', 'out');
const envFilePath = path.join(rootDir, '.env');

loadDotEnv(envFilePath);

if (!fs.existsSync(docsOutDir)) {
  fail(
    `Rspress output was not found at ${docsOutDir}. Run \`npm run docs:build\` before publishing.`,
  );
}

const forwardedArgs = process.argv.slice(2);
const projectName = process.env.CLOUDFLARE_PAGES_PROJECT_NAME?.trim() ?? '';
const branchName = process.env.CLOUDFLARE_PAGES_BRANCH?.trim() ?? '';

if (!hasOption(forwardedArgs, '--project-name')) {
  if (!isConfiguredProjectName(projectName)) {
    fail(
      'Set CLOUDFLARE_PAGES_PROJECT_NAME in the repository .env file or pass --project-name <name> when invoking the publish command.',
    );
  }
  forwardedArgs.push('--project-name', projectName);
}

if (!hasOption(forwardedArgs, '--branch') && branchName) {
  forwardedArgs.push('--branch', branchName);
}

const npmExecutable = process.platform === 'win32' ? 'npm.cmd' : 'npm';
const wranglerArgs = ['exec', 'wrangler', 'pages', 'deploy', docsOutDir, ...forwardedArgs];

console.log(`Publishing ${path.relative(rootDir, docsOutDir)} with Wrangler...`);

const result = spawnSync(npmExecutable, wranglerArgs, {
  cwd: rootDir,
  env: process.env,
  stdio: 'inherit',
});

if (result.error) {
  fail(`Failed to launch Wrangler: ${result.error.message}`);
}

process.exit(result.status ?? 1);

function hasOption(args, optionName) {
  return args.some((arg) => arg === optionName || arg.startsWith(`${optionName}=`));
}

function isConfiguredProjectName(value) {
  const normalized = value.trim().toLowerCase();
  return normalized.length > 0 && normalized !== 'your-cloudflare-pages-project-name';
}

function loadDotEnv(filePath) {
  if (!fs.existsSync(filePath)) {
    return;
  }

  const contents = fs.readFileSync(filePath, 'utf8');
  for (const rawLine of contents.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }

    const separatorIndex = line.indexOf('=');
    if (separatorIndex === -1) {
      continue;
    }

    const key = line.slice(0, separatorIndex).trim();
    if (!key || Object.prototype.hasOwnProperty.call(process.env, key)) {
      continue;
    }

    let value = line.slice(separatorIndex + 1).trim();
    if (
      (value.startsWith('"') && value.endsWith('"')) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }
    process.env[key] = value;
  }
}

function fail(message) {
  console.error(`\n[docs:publish] ${message}`);
  console.error('[docs:publish] Authenticate first with `npx wrangler login` or an equivalent Wrangler auth flow.');
  process.exit(1);
}
