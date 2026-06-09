# Test mining

Tooling for deriving template-combination tests from the real wiki, and for
differential-testing the converter against the wiki's own rendered HTML (the
authoritative ground truth per `AGENTS.md`).

The tests these scripts produced live in:

- `apps/converter/tests/template_combinations.rs` — differential + contract +
  nesting cases, plus an `#[ignore]`d `known_divergences` module.
- `apps/converter/tests/real_page_fixtures.rs` — end-to-end conversion of a real
  champion/item/rune from committed fixtures (`tests/fixtures/mini_export/`),
  fully self-contained (no download, no skip).

Findings are summarized in `DIVERGENCES.md`.

## Scripts

All scripts hit `https://wiki.leagueoflegends.com/en-us/api.php` and need only a
stock Python 3 plus `mwparserfromhell` (`pip install mwparserfromhell`).

| Script | Purpose |
|--------|---------|
| `fetch_titles.py` | Build a page list of every champion/item/rune from the data modules. |
| `mine.py <export_dir>` | Parse a downloaded export, rank template frequency, find parent→child nesting, and dump diverse real invocations to `mining.json`. |
| `render.py '<wikitext>'` | Render one wikitext snippet on the wiki and print the plain-text result (ground truth). |
| `batch_render.py <file>` | Render many snippets (one per line), cached to `render_cache.json`, as `INPUT<TAB>PLAIN`. |

## Workflow

```bash
# 1. Download the convertible corpus (all champions/items/runes + transcluded
#    templates/modules) into the local cache.
python3 scripts/test-mining/fetch_titles.py > /tmp/page_list.txt   # needs module data; see script
cargo run -p lol-wiki-export -- --page-list /tmp/page_list.txt \
  --out-dir ./export_out --meta-dir ./export_out_meta

# 2. Mine template usage and nesting.
python3 scripts/test-mining/mine.py export_out --top 80

# 3. For any candidate snippet, get authoritative ground truth.
python3 scripts/test-mining/render.py '{{ap|60 to 310 6}}'

# 4. Confirm the converter agrees, then bake the case into a Rust test.
```

Bulky/disposable artifacts (the dumped `*.lua` modules, `mining.json`,
`render_cache.json`, page lists, logs) are written under the gitignored
`validation_reports/test-mining/`, not committed.
