# TODO / Follow-up Plan

This document outlines the current state of the "lol-wiki-md" converter project, onboarding context for new contributors, and a prioritized roadmap of improvements.

## Project Overview

- Goal: Convert League of Legends Wiki (MediaWiki) pages into clean Markdown files.
- Inputs: MediaWiki XML dumps (`champions.xml`, `items.xml`, `runes.xml`).
- Extraction: `extract_pages.py` streams XML into a local `out/` folder that mirrors wiki namespaces and titles.
- Conversion: `simple_converter.py` (built-in Python only) reads from `out/` and emits Markdown to `markdown/`.
- Alternative (full) converter: `lol_wiki_converter.py` exists for feature parity when using external dependencies (not used in the simplified flow).

Repository structure (key files):

- `extract_pages.py` — XML → `out/<Namespace>/<Title>/page.txt`
- `simple_converter.py` — `out` → `markdown/<Champion>.md`
- `lol_wiki_converter.py` — richer converter (deps required)
- `templates/` — example Jinja templates for a full converter
- `requirements.txt` — dependencies for the full converter (not needed by `simple_converter.py`)

## Quick Start (Windows / PowerShell)

1. Extract wiki pages from XML to local files:

```powershell
# Optional: activate venv if you want to use it
& .\.venv\Scripts\Activate.ps1

python .\extract_pages.py .\runes.xml .\items.xml .\champions.xml --out .\out --ext txt --verbose
```

2. Convert a champion page to Markdown:

```powershell
# Typical champion
python .\simple_converter.py --champion Azir --wiki-root .\out --output .\markdown

# Champion with an apostrophe in the name (PowerShell quoting)
python .\simple_converter.py --champion \"Bel'Veth\" --wiki-root .\out --output .\markdown
# or
python .\simple_converter.py --champion \`"Bel'Veth\`" --wiki-root .\out --output .\markdown
```

Outputs will appear in `markdown/`, e.g., `markdown/Azir.md`.

## Current Status Summary

- `extract_pages.py` is robust and Windows-friendly: it sanitizes paths, handles namespaces, and writes `page.txt` files in `out/`.
- `simple_converter.py`:
  - Works end-to-end for champions like Azir, Yasuo, LeBlanc, and Bel'Veth.
  - Preserves nested bullets in Patch History and Trivia by parsing leading asterisks into properly indented Markdown lists.
  - Converts common wiki templates/links to Markdown; handles section links and external links.
  - Stats table uses a corrected attack speed growth formula (base × (1 + growth% × 17)).
  - Post-processing is intentionally minimal and champion-agnostic; it cleans a few common artifacts without harming structure.
  - Units appending respects special tokens (e.g., does not add `units` to values containing `Global`).

Known limitations (acceptable for the "simple" converter):

- Some complex Wiki templates leak odd tokens or partial formatting (e.g., residual template keys like `si...`, `ui...`, or malformed italics when templates were nested).
- Not all ability parameters are extracted; some composite rows may still look noisy depending on the champion.
- No full template expansion (by design) — keeps the converter dependency-free and simpler.

## Contribution Guidelines

- Keep changes small and test after each edit:
  - Convert 2–3 champions and inspect Markdown diffs.
  - Avoid global find/replace that could break list indentation or emphasis.
- Favor source-level fixes over output-only hacks:
  - E.g., preserve nested bullets when parsing rather than flattening in post-processing.
- Add safe, champion-agnostic cleanups only; no hard-coded champion logic.

## Troubleshooting

- Apostrophes in champion names (PowerShell quoting):
  - Use `\"Bel'Veth\"` or backtick-escaped `\`"Bel'Veth\`"`for the`--champion` argument.
- Missing champion output:
  - Ensure `out/Main/<Champion>/page.txt` exists; re-run `extract_pages.py` with the proper XML dumps.
- Garbled text / artifacts:
  - This often originates from complex templates. Prefer localized, safe regex cleanups or consider switching to the full converter.

## Reference Commands

```powershell
# Re-extract pages (verbose)
python .\extract_pages.py .\runes.xml .\items.xml .\champions.xml --out .\out --ext txt --verbose

# Convert multiple champions (PowerShell)
$names = @('Azir','Yasuo','LeBlanc','Zed','Aurelion Sol')
foreach ($n in $names) { python .\simple_converter.py --champion $n --wiki-root .\out --output .\markdown }

# Champion with apostrophe
python .\simple_converter.py --champion \`"Bel'Veth\`" --wiki-root .\out --output .\markdown
```
