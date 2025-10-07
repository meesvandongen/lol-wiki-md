import { request, chromium } from 'playwright';
import { promises as fs } from 'fs';
import * as path from 'path';

interface CrawlOptions {
  limit?: number;
  delayMs: number;
  parallel: number;
  categories: string[];
  outDir: string;
  retry: number;
  fullHistory: boolean; // export all revisions
  includeTemplates: boolean; // include templates recursively
  maxCategoryDepth: number; // recursion depth for subcategories
  noExport: boolean; // list only, do not export page XML
  batchExport: boolean; // export multiple pages in one request
  batchSize: number; // number of titles per batch request
  splitBatches: boolean; // split combined batch XML into per-page files
  combinedPrefix: string; // prefix for combined batch files
}

const DEFAULTS: CrawlOptions = {
  limit: undefined,
  delayMs: 250,
  parallel: 4,
  // Add canonical broad champion category to ensure full coverage.
  categories: ['League_of_Legends_champion', 'Champions', 'Items', 'Runes'],
  outDir: 'wiki_exports',
  retry: 3,
  fullHistory: false,
  includeTemplates: false,
  maxCategoryDepth: 2,
  noExport: false,
  batchExport: false,
  batchSize: 40,
  splitBatches: true,
  combinedPrefix: 'combined_chunk',
};

function parseArgs(): CrawlOptions {
  const opts: CrawlOptions = { ...DEFAULTS };
  for (let i = 2; i < process.argv.length; i++) {
    const arg = process.argv[i];
    if (arg === '--limit') opts.limit = parseInt(process.argv[++i], 10);
    else if (arg === '--delay') opts.delayMs = parseInt(process.argv[++i], 10);
    else if (arg === '--parallel') opts.parallel = parseInt(process.argv[++i], 10);
    else if (arg === '--category') opts.categories.push(process.argv[++i]);
    else if (arg === '--out') opts.outDir = process.argv[++i];
    else if (arg === '--full-history') opts.fullHistory = true;
    else if (arg === '--with-templates') opts.includeTemplates = true;
    else if (arg === '--max-depth') opts.maxCategoryDepth = parseInt(process.argv[++i], 10);
    else if (arg === '--no-export') opts.noExport = true;
    else if (arg === '--batch-export') opts.batchExport = true;
    else if (arg === '--batch-size') opts.batchSize = parseInt(process.argv[++i], 10);
    else if (arg === '--no-split') opts.splitBatches = false;
    else if (arg === '--combined-prefix') opts.combinedPrefix = process.argv[++i];
  }
  if (opts.limit !== undefined && opts.limit <= 0) opts.limit = undefined; // treat non-positive as unlimited
  return opts;
}

async function sleep(ms: number) { return new Promise(r => setTimeout(r, ms)); }

interface CategoryFetchResult { members: string[]; subcategories: string[]; }

async function fetchCategory(browserPage, category: string): Promise<CategoryFetchResult> {
  // Reuse logic: navigate to category page and paginate.
  const url = `https://wiki.leagueoflegends.com/en-us/Category:${encodeURIComponent(category)}`;
  await browserPage.goto(url, { waitUntil: 'domcontentloaded' });
  const members = new Set<string>();
  const subcats = new Set<string>();
  async function extract() {
    const links = await browserPage.$$eval('#mw-pages li > a', as => as.map(a => (a as HTMLAnchorElement).getAttribute('title') || ''));
    for (const l of links) if (l) members.add(l);
    const subcatLinks = await browserPage.$$eval('#mw-subcategories li > a', as => as.map(a => (a as HTMLAnchorElement).textContent || ''));
    for (const s of subcatLinks) {
      if (s) {
        // Category link text may include parentheses counts. Strip trailing count.
        const cleaned = s.replace(/\s*\(.*?\)$/, '');
        subcats.add(cleaned);
      }
    }
  }
  await extract();
  while (await browserPage.locator('#mw-pages a:has-text("next page")').count() > 0) {
    await browserPage.locator('#mw-pages a:has-text("next page")').click();
    await browserPage.waitForLoadState('domcontentloaded');
    await extract();
  }
  return { members: Array.from(members), subcategories: Array.from(subcats) };
}

async function exportPage(title: string, outRoot: string, retry: number, opts: CrawlOptions): Promise<boolean> {
  const safeName = title.replace(/[^A-Za-z0-9_.-]+/g, '_');
  const subDir = path.join(outRoot, safeName[0]?.toUpperCase() || '_');
  const outPath = path.join(subDir, safeName + '.xml');
  try {
    await fs.mkdir(subDir, { recursive: true });
    if (await exists(outPath)) return false; // already have it
    const ctx = await request.newContext();
    const params = new URLSearchParams();
    params.set('pages', title);
    params.set('action', 'submit');
    if (!opts.fullHistory) params.set('curonly', '1');
    if (opts.includeTemplates) params.set('templates', '1');
    const url = `https://wiki.leagueoflegends.com/en-us/Special:Export?${params.toString()}`;
    const resp = await ctx.get(url, { timeout: 120000 });
    if (!resp.ok()) throw new Error(`HTTP ${resp.status()}`);
    const body = await resp.text();
    if (!body.includes('<mediawiki')) throw new Error('Missing <mediawiki> root');
    await fs.writeFile(outPath, body, 'utf8');
    await ctx.dispose();
    return true;
  } catch (err) {
    if (retry > 0) {
      console.warn(`Retrying ${title} after error: ${(err as Error).message}`);
      await sleep(500 * (4 - retry));
      return exportPage(title, outRoot, retry - 1, opts);
    } else {
      console.error(`Failed to export ${title}: ${(err as Error).message}`);
      return false;
    }
  }
}

async function exists(p: string) { try { await fs.stat(p); return true; } catch { return false; } }

async function main() {
  const opts = parseArgs();
  await fs.mkdir(opts.outDir, { recursive: true });
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  const allTitles: string[] = [];
  const visitedCategories = new Set<string>();
  interface PendingCat { name: string; depth: number; }
  const queueCats: PendingCat[] = opts.categories.map(c => ({ name: c, depth: 0 }));

  while (queueCats.length) {
    const { name, depth } = queueCats.shift()!;
    if (visitedCategories.has(name) || depth > opts.maxCategoryDepth) continue;
    visitedCategories.add(name);
    console.log(`Collecting Category:${name} (depth=${depth})`);
    try {
      const { members, subcategories } = await fetchCategory(page, name);
      console.log(`  -> ${members.length} titles; ${subcategories.length} subcats`);
      allTitles.push(...members);
      await fs.writeFile(path.join(opts.outDir, `category_${name.replace(/[^A-Za-z0-9_.-]+/g,'_')}.json`), JSON.stringify({ members, subcategories }, null, 2));
      for (const sub of subcategories) {
        if (!visitedCategories.has(sub)) queueCats.push({ name: sub, depth: depth + 1 });
      }
    } catch (e) {
      console.error(`Failed to collect category ${name}: ${(e as Error).message}`);
    }
  }
  await browser.close();

  const uniqueTitles = Array.from(new Set(allTitles)).sort();
  // Filter out obvious non-content / meta namespaces (can reintroduce by supplying direct categories if needed)
  const skipPrefixes = ['File:', 'Category:', 'Template:', 'Module:', 'Help:', 'Special:'];
  const filtered = uniqueTitles.filter(t => !skipPrefixes.some(p => t.startsWith(p)));
  const limited = opts.limit ? filtered.slice(0, opts.limit) : filtered;
  console.log(`Total unique titles collected: ${uniqueTitles.length}`);
  if (filtered.length !== uniqueTitles.length) console.log(`Filtered out ${uniqueTitles.length - filtered.length} meta/namespace pages.`);
  console.log(`Titles selected for export: ${limited.length}`);

  if (opts.noExport) {
    await fs.writeFile(path.join(opts.outDir, 'all_titles.json'), JSON.stringify({ total: limited.length, titles: limited }, null, 2));
    console.log('No-export mode: wrote all_titles.json and exiting.');
    return;
  }

  if (opts.batchExport) {
    console.log(`Batch export enabled (size=${opts.batchSize}, split=${opts.splitBatches}).`);
    await batchExportPages(limited, opts);
    return;
  }

  let completed = 0, newDownloads = 0;
  const queue = [...limited];
  const start = Date.now();

  async function worker(id: number) {
    while (queue.length) {
      const title = queue.shift();
      if (!title) break;
      const ok = await exportPage(title, opts.outDir, opts.retry, opts);
      completed++;
      if (ok) newDownloads++;
      if (opts.delayMs) await sleep(opts.delayMs);
      if (completed % 10 === 0) {
        const elapsed = (Date.now() - start) / 1000;
        console.log(`[${id}] ${completed}/${limited.length} (${(completed/limited.length*100).toFixed(1)}%) new=${newDownloads} elapsed=${elapsed.toFixed(1)}s`);
      }
    }
  }

  const workers = Array.from({ length: opts.parallel }, (_, i) => worker(i + 1));
  await Promise.all(workers);
  console.log(`Done. Exported ${newDownloads} (skipped existing ${completed - newDownloads}). Output in ${opts.outDir}`);
}

async function batchExportPages(titles: string[], opts: CrawlOptions) {
  const chunks: string[][] = [];
  for (let i = 0; i < titles.length; i += opts.batchSize) {
    chunks.push(titles.slice(i, i + opts.batchSize));
  }
  console.log(`Performing ${chunks.length} batch request(s).`);
  let totalWritten = 0; let skipped = 0; let chunkIndex = 0;
  for (const chunk of chunks) {
    chunkIndex++;
    const params = new URLSearchParams();
    // Titles separated by newlines (MediaWiki Special:Export supports multiline pages param)
    params.set('pages', chunk.join('\n'));
    params.set('action', 'submit');
    if (!opts.fullHistory) params.set('curonly', '1');
    if (opts.includeTemplates) params.set('templates', '1');
    const url = `https://wiki.leagueoflegends.com/en-us/Special:Export?${params.toString()}`;
    const ctx = await request.newContext();
    console.log(`[Batch ${chunkIndex}/${chunks.length}] Requesting ${chunk.length} titles`);
    const resp = await ctx.get(url, { timeout: 300000 });
    if (!resp.ok()) {
      console.error(`Batch ${chunkIndex} failed HTTP ${resp.status()}`);
      await ctx.dispose();
      continue;
    }
    const xml = await resp.text();
    await ctx.dispose();
    if (!xml.includes('<mediawiki')) {
      console.error(`Batch ${chunkIndex} missing <mediawiki> root, skipping.`);
      continue;
    }
    const combinedName = `${opts.combinedPrefix}_${String(chunkIndex).padStart(3,'0')}.xml`;
    const combinedPath = path.join(opts.outDir, combinedName);
    await fs.writeFile(combinedPath, xml, 'utf8');
    console.log(`[Batch ${chunkIndex}] Wrote combined XML ${combinedName} (${xml.length} bytes)`);
    if (opts.splitBatches) {
      const headerEndIdx = xml.indexOf('<page>');
      const footerStartIdx = xml.lastIndexOf('</page>');
      const header = xml.substring(0, headerEndIdx);
      const footer = xml.substring(footerStartIdx + '</page>'.length);
      const pageRegex = /<page>[\s\S]*?<\/page>/g;
      const pages = xml.match(pageRegex) || [];
      for (const pageXml of pages) {
        const titleMatch = pageXml.match(/<title>([\s\S]*?)<\/title>/);
        if (!titleMatch) continue;
        const title = titleMatch[1];
        const safe = title.replace(/[^A-Za-z0-9_.-]+/g, '_');
        const subDir = path.join(opts.outDir, safe[0]?.toUpperCase() || '_');
        const outPath = path.join(subDir, safe + '.xml');
        if (await exists(outPath)) { skipped++; continue; }
        await fs.mkdir(subDir, { recursive: true });
        const fullContent = header + pageXml + footer;
        await fs.writeFile(outPath, fullContent, 'utf8');
        totalWritten++;
      }
      console.log(`[Batch ${chunkIndex}] Split pages: wrote ${totalWritten} (skipped existing ${skipped}).`);
    }
    if (opts.delayMs) await sleep(opts.delayMs);
  }
  console.log(`Batch export complete. New pages written: ${totalWritten}.`);
}

main().catch(e => { console.error(e); process.exit(1); });
