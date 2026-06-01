import { existsSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

import { defineConfig } from '@rspress/core';
import { pluginLlms } from '@rspress/plugin-llms';
import { pluginSitemap } from '@rspress/plugin-sitemap';

interface PackageJsonMetadata {
  version?: string;
  repository?: string | { url?: string };
  homepage?: string;
}

const docsPackage = JSON.parse(
  readFileSync(new URL('./package.json', import.meta.url), 'utf8'),
) as PackageJsonMetadata;
const siteVersion = docsPackage.version?.trim() || '0.1.0';
const repoUrl =
  resolveRepositoryUrl(docsPackage.repository) ??
  docsPackage.homepage ??
  'https://github.com/meesvandongen/lol-wiki-md';
const docsRoot = process.env.RSPRESS_DOCS_ROOT?.trim() || 'docs';
const outputDir = process.env.RSPRESS_OUT_DIR?.trim() || 'out';
const siteUrl = resolveSiteUrl();

export default defineConfig({
  root: docsRoot,
  title: 'LoL Wiki',
  description: 'Generated League Wiki champions, items, and runes documentation.',
  lang: 'en',
  logoText: `LoL Wiki v${siteVersion}`,
  outDir: outputDir,
  globalStyles: fileURLToPath(new URL('./styles/global.css', import.meta.url)),
  plugins: [pluginLlms(), pluginSitemap({ siteUrl })],
  route: {
    extensions: ['.md', '.mdx'],
  },
  search: false,
  markdown: {
    link: {
      checkDeadLinks: false,
    },
    image: {
      checkDeadImages: false,
    },
  },
  ssg: false,
  themeConfig: {
    enableScrollToTop: true,
    socialLinks: [
      {
        icon: 'github',
        mode: 'link',
        content: repoUrl,
      },
    ],
    footer: {
      message: `Generated from League Wiki exports and the Rust converter. · Version ${siteVersion}`,
    },
  },
});

function resolveRepositoryUrl(repository: PackageJsonMetadata['repository']): string | undefined {
  if (!repository) {
    return undefined;
  }
  if (typeof repository === 'string') {
    return normalizeRepositoryUrl(repository);
  }
  return normalizeRepositoryUrl(repository.url);
}

function normalizeRepositoryUrl(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }
  return value.replace(/^git\+/, '').replace(/\.git$/, '');
}

function resolveSiteUrl(): string {
  const explicitSiteUrl = resolveEnvValue('DOCS_SITE_URL');
  if (explicitSiteUrl) {
    return withTrailingSlash(explicitSiteUrl);
  }

  const pagesProjectName =
    resolveEnvValue('CLOUDFLARE_PAGES_PROJECT_NAME') || 'your-cloudflare-pages-project-name';
  return withTrailingSlash(`https://${pagesProjectName}.pages.dev`);
}

function resolveEnvValue(name: string): string | undefined {
  const runtimeValue = process.env[name]?.trim();
  if (runtimeValue) {
    return runtimeValue;
  }

  const envFileValue = loadDotEnvValue(name);
  return envFileValue?.trim() || undefined;
}

function loadDotEnvValue(name: string): string | undefined {
  const envFileUrl = new URL('../../.env', import.meta.url);
  if (!existsSync(envFileUrl)) {
    return undefined;
  }

  const envFile = readFileSync(envFileUrl, 'utf8');
  for (const rawLine of envFile.split(/\r?\n/)) {
    const line = rawLine.trim();
    if (!line || line.startsWith('#')) {
      continue;
    }
    const separatorIndex = line.indexOf('=');
    if (separatorIndex === -1) {
      continue;
    }
    const key = line.slice(0, separatorIndex).trim();
    if (key !== name) {
      continue;
    }
    let value = line.slice(separatorIndex + 1).trim();
    if (
      (value.startsWith('"') && value.endsWith('"')) ||
      (value.startsWith("'") && value.endsWith("'"))
    ) {
      value = value.slice(1, -1);
    }
    return value;
  }
  return undefined;
}

function withTrailingSlash(value: string): string {
  return value.endsWith('/') ? value : `${value}/`;
}