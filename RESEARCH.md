# Research: “| immediately followed by a character” artifacts in markdown (\|[a-z])

This document catalogs occurrences of the “pipe immediately followed by a character” pattern in generated markdown, analyzes root causes by tracing the converter pipeline, and proposes targeted fixes. The scope covers content generated into `markdown/` using inputs from `out/`.

## What we looked for

- Signature: any of the following in markdown output
  - Unescaped: `|[a-z]` inside prose or bullets
  - Escaped: `\\|[a-z]` at the start of a line (often raw template param leakage)
- Sources vs. outputs:
  - Input: `out/**` (raw wiki extract)
  - Output: `markdown/**` (converted .md)

Quick verification:

- `out/**`: no matches for either pattern (supports “introduced during conversion”).
- `markdown/**`: multiple matches across champions; see examples below.

## Representative examples (abbreviated)

- `markdown/Akshan.md`
  - Inline channel-type (ct) remnants in prose:
    - “... is usable.|items=false|consume=false|spells=false,interrupts...”
  - Raw template parameter lines (escaped):
    - `\\|description4 = ...`
    - `\\|cooldown = 100 / 92.5 / 85 ...`
    - `\\|target range = cr 2500`, `\\|width = er 120`, `\\|speed = 3200`, ...
  - Another ct remnant mid-note:
    - “... is usable.\\|attack=false\\|items=false\\|consume=false...”
- `markdown/Ambessa.md`
  - Multiple ct remnant sequences embedded in bullets, e.g.:
    - “... \*.|move=Registers input for \*\*.|cast=false|items=false|consume=false|spells=true,true,...”

These examples are consistent across other markdown files too (matches truncated here for brevity).

## Root cause categories and where they originate

Below, “SC.py” refers to `simple_converter.py` (primary generator currently in use). Line references are approximate anchors for the functions mentioned.

### 1) Channel Type template (ct) expansion failures/leakage

- Signature in MD:
  - Inline parameter runs such as `|items=...|consume=...|spells=...|interrupts=...` embedded in prose or bullets, sometimes partially escaped (e.g., `\\|attack=false`).
- Source wiki template (conceptual):
  - `{{ct|<type>|attack=...|move=...|cast=...|items=...|spells=...|consume=...|interrupts=...|damage=...}}`
- Responsible code:
  - SC.py: `_expand_channel_type_templates` (~1062–1290)
  - Routing: `_expand_custom_templates` calls `_expand_channel_type_templates` early (~975, ~1478); `_format_notes` also calls `_expand_custom_templates` on notes (~1757).
- Likely failure modes observed:

  - Parser leaves original `{{ct|...}}` content intact when:
    - The first positional arg (type) is missing or malformed → `pos` becomes empty → block is returned unchanged.
    - Nonstandard name or variant (already supports `ct`, `channel type`, `ctable`, case-insensitive). Some pages may embed the param run outside the template or with unexpected nesting, causing `_split_top_level_pipes`/brace parsing to miss parameters.
    - Parameters are present but the function determines there are no “rows” to render (e.g., all content goes to notes/others), so it returns the raw block.
  - Once left unchanged, later passes convert surrounding wiki text to MD, but leave `|k=v` tokens inline. In bullets/lists, pipes are sometimes backslash-escaped by later routines, creating visible `\\|...` artifacts.

- Consequence:
  - Prose bullets with unreadable `|items=...|consume=...` sequences.
  - Duplicated or conflicting narratives (e.g., sentence fragments plus param runs).

### 2) Raw template parameter line leakage (ability/effect params)

- Signature in MD:
  - Standalone lines starting with an escaped pipe and a key: `\\|description4 = ...`, `\\|cooldown = ...`, `\\|cost = ...`, `\\|target range = ...`, etc.
- Responsible pipeline areas:
  - Ability parsing and param mapping (several parts across SC.py):
    - Param key mapping includes `description4` (around ~1556, ~3701 usage).
    - Ability rendering collects descriptions, scalings, and extra tables (~4210–4448).
  - Generic wiki→MD conversion and table handling:
    - `_convert_wiki_to_markdown_ctx` (~1471)
    - ST/scaling blocks builder (~2850–2954, ~2947 for table header construction)
    - A path that detects scaling blocks starting with `|` and tries to render them (~4424–4444). When not recognized as a proper table, lines can survive and be escaped.
- Likely failure modes observed:

  - Param parsing doesn’t fully consume all `|key = value` lines inside ability/effect templates. Unrecognized keys, formatting variations, or scope boundary issues (brace depth) leave original lines in the text.
  - Later, safety escapes for markdown tables convert leading `|` to `\\|` outside proper table contexts, exposing raw “wikitable-like” lines in output.

- Consequence:
  - Visible, noisy `\\|key = value` lines in “Abilities” sections.

### 3) Miscellaneous artifacts (rarer under this regex)

- Some link remnants with pipes can occur elsewhere (e.g., file/image syntax or alternative display titles), but the strict `\\|[a-z]` focus mostly flags ct params and raw template lines rather than those.

## Proposed fixes (concrete, low-risk to high-impact)

Prioritize (1) and (2) as they create the vast majority of visible artifacts.

### A. Harden Channel Type expansion

- Be lenient when first positional arg (type) is missing:
  - If `pos` is empty but we detect top-level named params for ct (`attack|move|cast|items|spells|consume|interrupts|damage`), treat type as `unknown` and proceed to build the table instead of bailing out.
- “No rows” fallback:
  - When `_expand_channel_type_templates` would return the raw block due to empty `rows`, degrade gracefully by at least extracting `interrupts`, `items`, `spells`, and `consume` into rows. Do not emit the original `|k=v` text.
- Robust param splitting:
  - Ensure `_split_top_level_pipes` (helper) is used consistently and handles nested braces within ct values; add tests for sequences like `interrupts=false,death,root` and for nested templates.
- Post-conversion clean:
  - As a final guard in `_format_notes` or `_postprocess_markdown`, detect trailing ct-parameter runs with a regex and replace them via a second-pass ct-renderer:
    - Detection regex idea (pseudocode): `(?:\||^)(attack|move|cast|items|spells|consume|interrupts|damage)\s*=.+(?:\|(?:attack|move|cast|items|spells|consume|interrupts|damage)\s*=.+)*`
    - If matched outside a recognized table, rerun a strict ct-param-to-table converter; if conversion fails, drop the param run to avoid leaking.

### B. Consume or quarantine raw template param lines

- During ability/effect parsing:
  - After parsing known keys, strip any remaining lines that match wikitemplate param format: `^\s*\|\s*[a-z][^=]{0,40}\s*=` within the current template scope.
  - Keep unrecognized params in a dict if needed for debugging, but do not emit them into the markdown body.
- Table detection before escaping:
  - When a block starts with `|` but doesn’t contain a valid table header or multiple rows, normalize it to a bullet list or drop it. Avoid generic “escape pipes everywhere” on such blocks.
- Last-resort postprocess in `_postprocess_markdown` (~4518+):
  - Add a cleanup to remove lone param lines that are clearly template residue and not MD tables:
    - E.g., lines matching `^(\\\\|\s*)?[a-z][^=]{0,40}=.+$` that are not part of a contiguous table region (no preceding header like `| ---` and fewer than 2 consecutive `|`-led rows).

### C. Safer list/bullet integration for template output

- When injecting ct tables into bullets, ensure a blank line precedes the table (already partially handled) so it doesn’t merge into the list text and create odd escapes.
- If ct output must remain inline (rare), render a concise, pipe-free sentence (e.g., “Items: Allowed / Spells: Interrupts / Consumables: Disabled”).

## Implementation pointers (by function)

- `_expand_channel_type_templates` (SC.py ~1062–1290):
  - Accept missing `pos[0]` by inferring `ctype = 'unknown'` if named ct-params are found.
  - Do not short-circuit when `rows` is empty; attempt to synthesize rows from named params.
  - When failing definitively, replace the original block with a neutral sentence or omit it rather than returning raw `|k=v` strings.
- `_format_notes` (SC.py ~1731+):
  - After `_expand_custom_templates`, add a pass to capture any leftover ct param runs and re-render them or drop them.
- Ability/effect parsing and rendering (SC.py ~1600–1700, ~4200–4450):
  - Ensure unrecognized `|key = value` lines inside effect templates are not emitted.
  - When `ability['extra_scaling']` contains blocks starting with `|`, validate them as proper tables; otherwise convert to bullets or drop.
- `_postprocess_markdown` (SC.py ~4518+):
  - Add a cleanup that removes stray param lines not part of a table and ct param runs missed earlier.

## Risks and safeguards

- Dropping lines: Only drop lines that clearly match template param syntax and are not part of a table region. Keep a DEBUG flag or log of dropped keys during development.
- False positives on real tables: Guard by requiring at least a header row or multiple consecutive `|` rows to treat as a table. Otherwise, treat as residue.
- Regression tests: Create a small set of champion pages (including known offenders like Akshan, Ambessa) and assert: no `|[a-z]` nor `\\|[a-z]` outside legitimate tables.

## Quick verification approach

- After implementing fixes, run the converter and then:
  - Grep for artifacts in `markdown/**`:
    - `|[a-z]` and `\\|[a-z]`
  - Spot-check previously affected files (Akshan, Ambessa) for ct tables rendered as a proper 2-column markdown table under a “Channel Behavior” header, and absence of raw param lines.

## Status and next steps

- Status: Issues confirmed in markdown output; not present in raw input. Root causes localized to ct-expansion fallback and template param leakage.
- Next steps:
  1. Implement A+B above in `simple_converter.py`.
  2. Re-run generation and verify with grep checks.
  3. Iterate on edge cases; add a small regression harness to fail on new `|[a-z]` leaks.

---

If you want, I can implement the minimal hardening described (A+B) directly in `simple_converter.py` and regenerate a few sample champions to validate the cleanup.
