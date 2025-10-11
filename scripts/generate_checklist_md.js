// Generates an expanded markdown file from docs/template_checklist.md
// For each table row it creates a heading, a checkbox for status (checked when Status === 'done'),
// a checkbox for include decision (checked when Decision === 'include'),
// pastes the first 20 lines of a matching file from export_out (matching by URL-encoded Name),
// and copies over any existing Notes.
//
// This script is written in plain JavaScript and works under Bun or Node.

const fs = require('fs').promises;
const path = require('path');

const ROOT = path.resolve(__dirname, '..');
const TEMPLATE = path.join(ROOT, 'docs', 'template_checklist.md');
const EXPORT_DIR = path.join(ROOT, 'export_out');

// CLI-configurable options with sensible defaults
const argv = process.argv.slice(2);
function getArg(name, defaultValue) {
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === `--${name}` && i + 1 < argv.length) return argv[i + 1];
    if (a.startsWith(`--${name}=`)) return a.split('=')[1];
  }
  return defaultValue;
}

const BATCH_SIZE = Number(getArg('batch-size', 20));
const LINES = Number(getArg('lines', 20));
const OUT_DIR = getArg('out-dir', path.join(ROOT, 'docs'));
const TEMPLATE_NAMESPACE = getArg('namespace', 'Template');
const TEMPLATE_SUFFIX = getArg('suffix', '/doc');

function parseTableRows(lines) {
  // find the table header start (line that contains '| Name')
  let start = -1;
  for (let i = 0; i < lines.length; i++) {
    if (/^\|\s*Name\s*\|/i.test(lines[i])) {
      start = i;
      break;
    }
  }
  if (start === -1) return [];

  // rows begin two lines after header (header + separator)
  const rows = [];
  for (let i = start + 2; i < lines.length; i++) {
    const line = lines[i].trim();
    if (!line.startsWith('|')) break; // end of table
    // split by '|' but ignore the first and last empty cells caused by leading/trailing |
    const parts = line.split('|').map(s => s.trim());
    // parts[0] is empty if line starts with '|'
    // normalize to 6 columns: Name, Category, Decision, Priority, Status, Notes
    const cols = parts.slice(1, 7).map(s => s === '-' ? '' : s);
    while (cols.length < 6) cols.push('');
    const [Name, Category, Decision, Priority, Status, Notes] = cols;
    rows.push({ Name, Category, Decision, Priority, Status, Notes });
  }
  return rows;
}

function makeFilenameFromName(name) {
  // The export_out filenames appear URL-encoded and end with .txt
  // Use encodeURIComponent to match that format.
  if (!name) return null;
  // Ensure the page title is in the requested namespace and ends with the requested suffix
  let title = name;
  const nsRegex = new RegExp('^' + TEMPLATE_NAMESPACE.replace(/[-/\\^$*+?.()|[\]{}]/g, '\\$&') + ':', 'i');
  if (!nsRegex.test(title)) {
    title = `${TEMPLATE_NAMESPACE}:${title}`;
  }
  const suffRegex = new RegExp(TEMPLATE_SUFFIX.replace(/[-/\\^$*+?.()|[\]{}]/g, '\\$&') + '$', 'i');
  if (!suffRegex.test(title)) {
    // avoid double-slash
    if (TEMPLATE_SUFFIX.startsWith('/') && title.endsWith('/')) {
      title = title + TEMPLATE_SUFFIX.slice(1);
    } else {
      title = title + TEMPLATE_SUFFIX;
    }
  }
  return encodeURIComponent(title) + '.txt';
}

async function readFirstNLines(filePath, n = 20) {
  try {
    const content = await fs.readFile(filePath, 'utf8');
    const lines = content.split(/\r?\n/).slice(0, n);
    return lines.join('\n');
  } catch (err) {
    return null;
  }
}

async function main() {
  const src = await fs.readFile(TEMPLATE, 'utf8');
  const lines = src.split(/\r?\n/);
  const rows = parseTableRows(lines);

  // split into batches
  const BATCH_SIZE = 20;
  const batches = [];
  for (let i = 0; i < rows.length; i += BATCH_SIZE) {
    batches.push(rows.slice(i, i + BATCH_SIZE));
  }

  const writtenFiles = [];
  for (let bi = 0; bi < batches.length; bi++) {
    const batch = batches[bi];
    const outLines = [];
    outLines.push('# Template checklist — expanded');
    outLines.push('');
    outLines.push(`Generated: ${new Date().toISOString()}`);
    outLines.push('');
    outLines.push(`Batch ${bi + 1} of ${batches.length} — items ${bi * BATCH_SIZE + 1}..${bi * BATCH_SIZE + batch.length}`);
    outLines.push('');

    for (const row of batch) {
      const title = row.Name || '(no name)';
      outLines.push(`## ${title}`);
      outLines.push('');

      const statusValue = (row.Status || '').toLowerCase();
      const statusChecked = statusValue === 'done' ? 'x' : ' ';
      outLines.push(`- [${statusChecked}] Status: ${row.Status || ''}`);

      const decisionValue = (row.Decision || '').toLowerCase();
      const decisionChecked = decisionValue === 'include' ? 'x' : ' ';
      outLines.push(`- [${decisionChecked}] Include decision: ${row.Decision || ''}`);

      outLines.push('');
      outLines.push('### Source (first 20 lines)');
      outLines.push('');

      const filename = makeFilenameFromName(title);
      if (filename) {
        outLines.push(`- Source filename: ${filename}`);
        try {
          const decoded = decodeURIComponent(filename.replace(/\.txt$/, ''));
          outLines.push(`- Decoded name: ${decoded}`);
          const nsParts = decoded.split(':');
          if (nsParts.length > 1) {
            outLines.push(`- Namespace: ${nsParts[0]}`);
            outLines.push(`- Remainder: ${nsParts.slice(1).join(':')}`);
          }
          const pathParts = decoded.split('/').map(s => s.trim()).filter(Boolean);
          if (pathParts.length > 1) {
            outLines.push(`- Path parts: ${pathParts.join(' / ')}`);
          } else if (pathParts.length === 1) {
            outLines.push(`- Path part: ${pathParts[0]}`);
          }
        } catch (e) {}
      }

      let srcText = null;
      if (filename) {
        const candidate = path.join(EXPORT_DIR, filename);
        srcText = await readFirstNLines(candidate, 20);
      }
      if (!srcText) {
        outLines.push('');
        outLines.push('_No matching export file found or file empty._');
      } else {
        outLines.push('');
        outLines.push('```');
        outLines.push(srcText);
        outLines.push('```');
      }

      outLines.push('');
      outLines.push('### Notes');
      outLines.push('');
      if (row.Notes && row.Notes.trim() !== '') {
        outLines.push(row.Notes.trim());
      } else {
        outLines.push('_No notes specified._');
      }

      outLines.push('');
      outLines.push('---');
      outLines.push('');
    }

    const partName = `template_checklist_expanded_${String(bi + 1).padStart(3, '0')}.md`;
    const outPath = path.join(ROOT, 'docs', partName);
    await fs.writeFile(outPath, outLines.join('\n'), 'utf8');
    writtenFiles.push(partName);
  }

  // write an index file listing the batches
  const indexLines = [];
  indexLines.push('# Template checklist — expanded (index)');
  indexLines.push('');
  indexLines.push(`Generated: ${new Date().toISOString()}`);
  indexLines.push('');
  indexLines.push(`Total items: ${rows.length}`);
  indexLines.push(`Batch size: ${BATCH_SIZE}`);
  indexLines.push('');
  indexLines.push('Files:');
  for (const f of writtenFiles) {
    indexLines.push(`- [${f}](./${f})`);
  }
  const indexPath = path.join(ROOT, 'docs', 'template_checklist_expanded_index.md');
  await fs.writeFile(indexPath, indexLines.join('\n'), 'utf8');

  console.log(`Wrote ${writtenFiles.length} files to docs/ (total ${rows.length} entries)`);
}

main().catch(err => {
  console.error(err);
  process.exitCode = 1;
});
