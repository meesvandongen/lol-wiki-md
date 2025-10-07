import { chromium, Page, request } from 'playwright';
import { promises as fs } from 'fs';

// Exploration script: visits Special:Export and tests exporting a known page & category members list extraction strategy.
// Run with: npm run explore

async function exportSinglePageDirect(title: string) {
  const ctx = await request.newContext();
  const url = `https://wiki.leagueoflegends.com/en-us/Special:Export?pages=${encodeURIComponent(title)}&action=submit`; // default: current revision only
  const resp = await ctx.get(url, { timeout: 60000 });
  if (!resp.ok()) throw new Error(`Export failed ${resp.status()} for ${title}`);
  const body = await resp.text();
  if (!body.includes('<mediawiki')) throw new Error('Response does not look like MediaWiki export XML');
  await fs.mkdir('exploration', { recursive: true });
  await fs.writeFile(`exploration/${title.replace(/[^A-Za-z0-9_-]+/g,'_')}.xml`, body, 'utf8');
  console.log(`Saved export for ${title}`);
  await ctx.dispose();
}

async function fetchCategoryMembers(page: Page, category: string): Promise<string[]> {
  // Category page listing members: /wiki/Category:<Name>
  const url = `https://wiki.leagueoflegends.com/en-us/Category:${encodeURIComponent(category)}`;
  await page.goto(url);
  await page.waitForLoadState('domcontentloaded');
  const members = new Set<string>();
  async function extractFromPage() {
    const links = await page.$$eval('#mw-pages li > a', as => as.map(a => (a as HTMLAnchorElement).getAttribute('title') || '')); 
    for (const l of links) { if (l) members.add(l); }
  }
  await extractFromPage();
  // Handle paging via next link inside mw-pages nav
  while (await page.locator('#mw-pages a:has-text("next page")').count() > 0) {
    await page.locator('#mw-pages a:has-text("next page")').click();
    await page.waitForLoadState('domcontentloaded');
    await extractFromPage();
  }
  return [...members];
}

async function probeCategories(page: Page, names: string[]) {
  const results: Record<string, number> = {};
  for (const name of names) {
    try {
      const members = await fetchCategoryMembers(page, name);
      results[name] = members.length;
      await fs.writeFile(`exploration/category_${name.replace(/[^A-Za-z0-9_-]+/g,'_')}_members.json`, JSON.stringify(members, null, 2));
      console.log(`Category:${name} -> ${members.length} pages`);
    } catch (err) {
      console.warn(`Failed category ${name}: ${(err as Error).message}`);
      results[name] = -1;
    }
  }
  await fs.writeFile('exploration/category_probe_summary.json', JSON.stringify(results, null, 2));
}

(async () => {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  try {
    await exportSinglePageDirect('Azir');
    await probeCategories(page, ['Champion', 'Champions', 'Item', 'Items', 'Rune', 'Runes']);
  } catch (e) {
    console.error('Exploration failure', e);
  } finally {
    await browser.close();
  }
})();
