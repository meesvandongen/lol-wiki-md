import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const docsDir = path.join(rootDir, 'apps', 'docs');
const docsOutDir = path.join(rootDir, 'apps', 'docs', 'out');
const wranglerConfigPath = path.join(docsDir, 'wrangler.jsonc');
const wranglerCliPath = path.join(rootDir, 'node_modules', 'wrangler', 'bin', 'wrangler.js');
const envFilePath = path.join(rootDir, '.env');

loadDotEnv(envFilePath);

if (!fs.existsSync(docsOutDir)) {
  fail(
    `Rspress output was not found at ${docsOutDir}. Run \`npm run docs:build\` before publishing.`,
  );
}

if (!fs.existsSync(wranglerConfigPath)) {
  fail(
    `Wrangler config was not found at ${wranglerConfigPath}. Restore the checked-in docs Worker config before publishing.`,
  );
}

if (!fs.existsSync(wranglerCliPath)) {
  fail(
    `Wrangler CLI entrypoint was not found at ${wranglerCliPath}. Run \`npm install\` before publishing.`,
  );
}

const rawForwardedArgs = process.argv.slice(2);
const forwardedArgs = normalizeLegacyPagesArgs(rawForwardedArgs);
const workerName = resolveFirstConfiguredValue([
  process.env.DOCS_WORKER_NAME,
  process.env.CLOUDFLARE_WORKER_NAME,
  process.env.CLOUDFLARE_PAGES_PROJECT_NAME,
]);
const customDomains = parseDelimitedValues(
  resolveFirstConfiguredValue([
    process.env.DOCS_WORKER_DOMAINS,
    process.env.CLOUDFLARE_WORKER_DOMAINS,
    process.env.DOCS_WORKER_DOMAIN,
  ]),
);
const routes = parseDelimitedValues(process.env.DOCS_WORKER_ROUTES);

if (hasOption(forwardedArgs, '--branch')) {
  fail(
    'Cloudflare Workers deployments do not use Pages branches. Remove `--branch` / `CLOUDFLARE_PAGES_BRANCH` and use `--env` only if you define Wrangler environments.',
  );
}

if (!hasOption(forwardedArgs, '--config')) {
  forwardedArgs.push('--config', wranglerConfigPath);
}

if (!hasOption(forwardedArgs, '--name') && isConfiguredValue(workerName)) {
  forwardedArgs.push('--name', workerName);
}

if (!hasAnyOption(forwardedArgs, ['--domain', '--domains'])) {
  for (const domain of customDomains) {
    forwardedArgs.push('--domain', domain);
  }
}

if (!hasAnyOption(forwardedArgs, ['--route', '--routes'])) {
  for (const route of routes) {
    forwardedArgs.push('--route', route);
  }
}

const wranglerExecutable = process.execPath;
const wranglerArgs = [wranglerCliPath, 'deploy', ...forwardedArgs];

console.log(`Publishing ${path.relative(rootDir, docsOutDir)} as a static Worker with Wrangler...`);

if (isConfiguredValue(workerName) && !hasOption(rawForwardedArgs, '--name')) {
  console.log(`[docs:publish] Worker name: ${workerName}`);
}

if (customDomains.length > 0 && !hasAnyOption(rawForwardedArgs, ['--domain', '--domains'])) {
  console.log(`[docs:publish] Custom domains: ${customDomains.join(', ')}`);
}

if (routes.length > 0 && !hasAnyOption(rawForwardedArgs, ['--route', '--routes'])) {
  console.log(`[docs:publish] Routes: ${routes.join(', ')}`);
}

const result = spawnSync(wranglerExecutable, wranglerArgs, {
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

function hasAnyOption(args, optionNames) {
  return optionNames.some((optionName) => hasOption(args, optionName));
}

function isConfiguredValue(value) {
  if (!value) {
    return false;
  }

  const normalized = value.trim().toLowerCase();
  return (
    normalized.length > 0 &&
    normalized !== 'your-cloudflare-pages-project-name' &&
    normalized !== 'your-cloudflare-worker-name'
  );
}

function resolveFirstConfiguredValue(values) {
  for (const value of values) {
    if (isConfiguredValue(value)) {
      return value.trim();
    }
  }
  return '';
}

function parseDelimitedValues(value) {
  if (!value) {
    return [];
  }

  return value
    .split(/[\r\n,]/)
    .map((entry) => entry.trim())
    .filter(Boolean);
}

function normalizeLegacyPagesArgs(args) {
  return args.map((arg) => {
    if (arg === '--project-name') {
      return '--name';
    }
    if (arg.startsWith('--project-name=')) {
      return arg.replace('--project-name=', '--name=');
    }
    return arg;
  });
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
