# Gnar Passive Rendering, `pp`/`ap` Formula Visualization, and Options

This document captures findings about why parts of Gnar's passive are not rendered correctly, analyzes the `pp` (Passive progression) and `ap` (Ability progression) templates as exported into our local `out/Module/Ability_progression/page.txt`, and proposes options to expose formula/argument information in final Markdown outputs.

## Summary of the issue observed

- In `markdown/Gnar.md`, the passive section shows raw placeholders and tangled text:
  - Repeated "base + growth" phrasing injected from `pp|...|formula=...` outputs.
  - Residual template-like strings like `(Mega Gnar)+(Mega Gnar-Gnar)*(x-1)*(0.7025+0.0175*(x-1))|formula= base + growth` instead of rendered values or a readable tooltip.
  - Broken mini table under "Mini Gnar" with duplicated table headers mid-row (likely caused by a naive wikitext table-to-Markdown converter running into nested table/pipe structures).
- The root cause is incomplete expansion and rendering of the `pp` template and its `formula` parameter from `Module:Ability_progression`, plus partial support for nested template outputs (e.g., `{{pp ...}}` and `{{pptooltip ...}}`) in `simple_converter.py`.

## Ground truth from exported wiki data

- `out/Template/Passive_progression/page.txt` resolves to `<includeonly>{{#invoke:Ability progression|pp}}</includeonly>`, i.e., the `pp` template is only a bridge to a Lua module call.
- `out/Module/Ability_progression/page.txt` contains the full Lua implementation. Key details:
  - `p.pp(frame)` accepts a wide set of arguments, performs parsing of sequences like `5 to 10 by 1`, ranges with `for N`, supports `x` variable (level-like), and has a special `formula` argument (and auto-formula generation via `string_to_formula`).
  - It outputs HTML with attributes like `data-displayformula`, `data-top_values`, etc., intended to be consumed by wiki front-end JS/CSS tooltips.
  - The `p.pptooltip(frame)` companion renders the visual tooltip table using those data attributes.
- Our converter currently doesn't execute Lua nor parse `#invoke`. It only does pattern-based expansions for a subset of templates. As a result, the module output and its formula logic are not being executed; instead, partially processed text leaks into Markdown.

## Why Gnar's passive breaks specifically

- The exported Mega Gnar passive uses constructs like:
  - `{{pp| (MegaGnarBase + ...) |formula= base + growth |health}}` (simplified for illustration), where `pp` is invoked with:
    - a complex expression including an `x` progression term `(x-1)*(0.7025+0.0175*(x-1))` and differences of champion base/growth values via custom `ccd`/`as` wrappers.
    - an explicit `formula= ...` label to display.
  - In wiki, this produces a tooltip with values across levels and a readable formula label. In our Markdown pipeline, it becomes literal text because:
    1. `#invoke` is not executed.
    2. Our `simple_converter.py` doesn't map `pp` calls to any Markdown equivalent.
    3. We also leave helper wrappers like `{{as|...}}` and `{{pp|...}}` partially expanded, causing raw parameters to enter the final output.
- Additionally, "Mini Gnar" section includes a small table with properties like bonus range/movespeed where the wiki uses nested templates and row constructs. Our `_wikitext_table_to_md` struggles with interleaved templates and produces malformed tables.

## Desired outcome

- All information that exists in `pp` or `ap` invocations (computed value arrays, labels, keys, ranges, the explicit or auto-generated formula, rounding, color, display toggles, etc.) should appear in the final Markdown in a readable way, even without client-side JS tooltips.
- We must decide how to represent:
  - The value progression (single range vs full per-level list)
  - The formula (as text, as inline math, or as a code block)
  - The keys/labels (`key`, `key1`, `label`, `label1`, `type`, etc.)

## Inventory of `pp`/`ap` arguments (from `Module:Ability_progression`)

Note: Names are case sensitive as in the module. Some are validated by `Module:Ability_progression/parameters`.

- Shared concepts

  - Value lists: two channels (bot/top) separated by semicolons within `args[1]` and `args[2]` in `pp` and sequentially in `ap`.
  - Parsing helpers: `to` ranges, `by` and `for`, linear fill, and expressions using `x` variable for level.
  - Rounding: `round`, `round1` (values: a number of decimals; "false" to disable); default is 2 decimals.
  - Display toggles: `changedisplay`='true' to switch between range vs list when few values; `showtype`='false' to hide the "(based on level)" suffix.
  - Labels and keys:
    - `label` (bot_label), `label1` (top_label) — Tooltip axis captions.
    - `key` (bot_key), `key1` (top_key) — Suffixes appended to numbers (e.g., "%", " units").
    - `type` — Displayed as top label when `label1` not present; often "level".
  - Formula exposition:
    - `formula` — Explicit string to show in tooltip (display only).
    - Auto generation via `string_to_formula` when two endpoints use `to` a `finish` and no `x` is present; results in data-displayformula.
    - `useformula` — Optional, appears in `pptooltip` path for deriving presented formula from raw values and start/finish.
  - Start/finish ranges: `start`, `finish` (seem to feed `pptooltip` to render derived formulas).
  - Color: `color` accepted as keyword, used for styling.
  - Count/fill: internal padding to 18 entries default.

- `pp` specifics

  - Two series (bottom/top), arrays built by interpreting `args[1]` and `args[2]` content.
  - Emits a span with dataset attributes consumed by front-end tooltip JS; fallback text is either "start – finish" or a short list, plus optional "(based on level)".

- `pptooltip` specifics

  - Consumes `bot_values`/`top_values` and labels/keys to render an HTML table and formula section; uses `displayformula`, or computes a formula to display if missing.

- `ap` specifics
  - Progression across ability ranks (W/E/Q/R), with an internal `fill` of 5 or 3 depending on skill R.
  - Returns concise value list via `fdmulti` with " / ".
  - Supports `round` and a positional series of values with `to`/`by` expansions.

## Fail modes in our converter

- We don't execute `#invoke:Ability progression|pp` or `pptooltip`.
- We don't replace the `pp`/`ap` markup into Markdown structures; result is literal text or half-expanded content.
- Our expression evaluator supports a few templates (`ap`, `pp`, `pp`-adjacent) only partially; complex nested template parameters and HTML spans get through as raw.
- Our table converter `_wikitext_table_to_md` breaks when encountering nested templates inside table cells; it also doesn't handle embedded headers in the same row.

## Options to include full `pp`/`ap` info in Markdown

Below are implementation options with pros/cons and examples.

### Option A: Implement a pure-Python `pp`/`ap` renderer (no Lua)

- Approach:
  - Reimplement the essential logic of `Module:Ability_progression` in Python:
    - Parse `pp` arguments: two arrays; support `to`/`by`/`for`, `x` expressions, `round`/`round1`, `key`/`key1`, `label`/`label1`/`type`, `color`, `formula`, `start`/`finish`, `changedisplay`, `showtype`.
    - Produce a structured object: bottom/top arrays, labels, keys, inferred/explicit formula text.
    - Render Markdown:
      - Inline summary: "min – max (based on level)" or full list.
      - Optional details block with a Markdown table showing values by level and the formula.
      - Render formula using plain text or KaTeX to improve readability.
  - Similarly, implement `ap` progression (simpler: ranks, defaults to 5/3 entries).
- Pros:
  - Self-contained, no external runtime.
  - Fine-grained control over formatting; can integrate with existing pipeline.
  - Deterministic and fast.
- Cons:
  - Non-trivial effort to match all edge cases and the wiki module behavior.
  - Future drift if the wiki updates the Lua module.
- Example Markdown for Gnar (excerpt):
  - Mega Gnar base health bonus: values list [delta per level], formula: `base + growth × f(x)` displayed as `$base + growth\cdot((x-1)\cdot(0.7025+0.0175\cdot(x-1)))$`.
  - Visual block:
    - Formula: base + growth (provided via `formula` param)
    - Values by level: 1–18, with key if present (e.g., `HP`).

### Option B: Minimal parser + include raw formula strings as code, and lists when available

- Approach:
  - Without full re-implementation, detect `{{pp ...}}`/`{{ap ...}}` occurrences and extract:
    - The explicit `formula` argument, if present.
    - Any "start to finish" pattern (convert to range) and any enumerated values.
    - Labels/keys if present.
  - Render a compact inline summary and a fenced code block with the formula and parsed arguments.
- Pros:
  - Quick to implement, improves readability immediately.
  - No need to evaluate math or reproduce the module.
- Cons:
  - Not all variants are covered (e.g., `x`-based expressions, `for`, `by` may be missed beyond range extraction).
  - Incomplete values (no proper per-level series unless explicitly enumerated in wikitext).
- Example:
  - Inline: "Mega Gnar base MR: 30 → 60 (based on level)"
  - Code block: pp args and `formula= base + growth`.

### Option D: Hybrid: limited evaluation for pp/ap math only, and keep HTML structure

- Approach:
  - Parse `pp`/`ap` calls, compute per-level values when possible (linear `to/by/for`, explicit arrays), and render Markdown tables.
  - If complex `x` expressions or unsupported wrappers appear, fall back to emitting the provided `formula` string verbatim and mark the row as "formula-only".
- Pros:
  - Good coverage for common cases; transparent fallback when too hard.
  - More maintainable than full Lua.
- Cons:
  - Still needs a robust parser; some corner cases will remain.

## Formatting proposals for Markdown

- Inline style:
  - "value_min – value_max (based on level)"; respect `showtype` toggle.
- Details block for progression (opt-in by a CLI switch, e.g., `--pp-details`):
  - Markdown table with columns: Level, Value (bottom), optional Value (top); suffix keys applied.
  - A "Formula" section:
    - If `formula` provided: render as text or `$...$` math via KaTeX.
    - If not, show derived formula (simple linear only).
  - Include labels: `label`/`label1`, and `type`.

## Concrete next steps in code

1. Extend `simple_converter.py` with a `pp`/`ap` parser:
   - Add a function to locate and parse `{{pp ...}}` and `{{ap ...}}` invocations, returning a structured dict with:
     - type: pp|ap
     - series: [bottom_values], [top_values]
     - labels/keys: label, label1, key, key1, type
     - rounding, color, changedisplay, showtype
     - formula: explicit string if found; auto-generated linear formula if simple `start to finish` without `x`.
   - Replace the template in text with a renderer that outputs:
     - Inline concise summary + a hidden details block (HTML details/summary) with the full table and formula.
   - Reuse existing `_evaluate_expr_templates` scaffolding where possible, or add a new stage before `_convert_wiki_to_markdown`.
2. Improve `_wikitext_table_to_md` to be robust to nested templates in cells:
   - Pre-clean cell text by expanding inner templates first and escaping pipe-like sequences safely.
   - Alternatively, detect malformed rows and downgrade to list formatting.
3. Add KaTeX math rendering for formulas in Markdown:
   - Our README already states KaTeX support; we can wrap formulas in `$...$` or `$$...$$`.

---

## Additional root cause detail: template expansion order

Our converter appears to expand some outer wrappers (like `{{as|...}}`, `{{ci|...}}`) before processing `{{pp ...}}`. When an outer template replaces `{{pp ...}}` with plain text (e.g., stripping braces or reformatting pipes), the `pp` call no longer matches our regex/template expansion and is left as a literal fragment (e.g., `|formula= base + growth`). Fixing the order so inner `pp`/`ap` expansions happen first (or re-queuing nested templates after outer expansion) is key.

Places to adjust in `simple_converter.py`:

- Run a pre-pass to locate `{{pp`/`{{ap` templates and replace them with structured placeholders before other expansions (e.g., in `_convert_wiki_to_markdown_ctx`).
- After other expansions, render those placeholders into final Markdown blocks (values, labels, formula as math/text).
- Alternatively, modify `_resolve_simple_data_templates` to defer expansion of `as/ci/...` when they contain `{{pp` or `{{ap` inside; then re-run the inner pipeline.

## Consistency with docs/pp.md (sanity check)

What matches:

- Purpose and outputs: inline text + tooltip/table driven by pp (Module:Ability_progression) and a tooltip companion (Template:Tooltip/Pp / pptooltip) — captured in this doc.
- Parameters overview: 1 (bottom values), 2 (top values), changedisplay, showtype, label/label1/type, key/key1, round/round1, color, formula — captured in the argument catalog.
- Behavior: auto-completion via `to`/`by`/`for`, support for `x` expressions, default fill to 18, rounding defaults to 2 decimals, reverse-display with changedisplay=true, keys appended, and color theming — all noted.

Nuances to adjust in this doc/implementation based on pp.md:

- Positional vs named: pp.md shows a positional calling convention `{{pp|<1>|<2>|<changedisplay>|<showtype>|<label1>|<type>|<label>|<formula>|<key>|<key1>|<round>|<round1>|<color>}}`. Our parser should accept both positional and named forms, mapping positions 3+ into their named counterparts.
- label1 vs type precedence: pp.md clarifies that label1 sets the tooltip top-row label and type sets the inline “based on …” text; when both are present, type overrides the inline text, label1 overrides the top-row label. Ensure we follow that (earlier notes implied type is used only if label1 is absent; refine to this precedence rule).
- Keys only on numeric values: keys (key/key1) are appended only if the value parses as a number. Keep this guard to avoid attaching units to non-numeric cells.
- Formula operator display: pp.md notes operator symbols are replaced with typographic operator characters for display (×, ÷, −). Our Markdown rendering should either preserve those characters or map `*`→×, `/`→÷, `-`→− in the “display formula” section.
- Images in labels: examples show `[[File:...]]` inside label/label1/type. We should either pass through as-is (rendered as plain text/alt) or strip to alt text; note this limitation in Markdown.
- Max entries: pp.md mentions a maximum of 41 items per row; our notes already reflect the module’s count=41. Keep this limit in mind when validating or slicing long sequences.

Implementation tweaks to track:

- Add positional-to-named mapping for arguments (3..13) to align with pp.md’s parameter order.
- Apply precedence: inline scaling text from `type` when present; tooltip top-row label from `label1` when present.
- Append keys only to numeric outputs; avoid adding to expressions/strings.
- When rendering formulas, replace ASCII operators with typographic variants for readability, matching wiki behavior.
- For labels with images/links, degrade gracefully to text.

## Concrete pp/ap examples and target renderings

- Example 1 (pp, simple linear):

  - Wikitext: `{{pp|10 to 100 by 5|label=Damage|key=|type=level|formula=linear}}`
  - Desired inline: `10 – 100 (based on level)`
  - Desired details:
    - Formula: `linear`
    - Table (Levels 1–18): values 10, 15, ..., 100

- Example 2 (pp, two axes with keys):

  - Wikitext: `{{pp|5;10;15;20|1;2;3;4|label=Value|label1=Level|key=%|key1=|formula=V0 + growth}}`
  - Desired inline: `5% – 20% (based on Level)`
  - Details: two columns for bottom/top series, suffixes applied, and the formula shown.

- Example 3 (ap, ability ranks):

  - Wikitext: `{{ap|60 to 140 by 20}}`
  - Desired inline: `60 / 80 / 100 / 120 / 140`
  - Optional details: a small table "Rank 1–5" with values.

- Example 4 (pp with x and explicit formula):
  - Wikitext (pattern seen in Gnar): `{{pp|(Δbase) + (Δlvl) * (x-1) * (0.7025 + 0.0175*(x-1))|formula= base + growth}}`
  - Desired inline: value range if computable; otherwise show an expression badge.
  - Desired formula display: show `base + growth` and optionally math `$\Delta base + \Delta lvl\cdot(x-1)\cdot(0.7025+0.0175\cdot(x-1))$`.

## Visualization of `formula` for pp (general approach)

- Always surface `formula` when present:
  - Inline: a small italic hint like `(formula: base + growth)` or a superscript info icon.
  - Details block: a "Formula" section with either text or KaTeX math `$...$`.
- When `formula` is absent but a simple `start to finish by step` is detected, auto-generate and display `start + (finish - start)/(times - 1) * (x - 1)` for levels (or analogous for ranks), clearly labeled as derived.
- If the pp contains complex `x` expressions we cannot compute, display the expression verbatim in a code/math block, and include any provided labels/keys for context.

## Appendix: Argument catalog to support

- `pp` (Passive progression):

  - Positional:
    - `1`: bottom values list or expression; semicolon-separated or range syntax (e.g., `10 to 100 by 5`).
    - `2`: top values list or expression (optional).
  - Named:
    - `round`: decimals for bottom series (number or "false").
    - `round1`: decimals for top series (number or "false").
    - `label`: bottom label (displayed label; used in `pptooltip` as `bot_label`).
    - `label1`: top label (used in `pptooltip` as `top_label`).
    - `type`: axis label, often "level" (used when `label1` absent).
    - `key`: suffix appended to bottom values (e.g., `%`, ` units`).
    - `key1`: suffix appended to top values.
    - `color`: keyword color for styling (only visual; we can ignore or map to CSS).
    - `start`: starting value or index for formula derivation (feeds `pptooltip`).
    - `finish`: ending value or index for formula derivation (feeds `pptooltip`).
    - `useformula`: hint used by `pptooltip` when building formula strings.
    - `formula`: explicit display formula string.
    - `changedisplay`: `"true"` toggles between range vs list fallback text.
    - `showtype`: `"false"` hides the `(based on <type>)` suffix.
  - Behavior: supports `to`/`by`/`for` expansions, `x` expressions, and linear filling to 18 entries by default.

- `pptooltip` (companion we may emulate):

  - Named: `bot_label`, `top_label`, `bot_key`, `top_key`, `displayformula`, `useformula`, `start`, `finish`, `bot_values`, `top_values`.

- `ap` (Ability progression):
  - Positional: a sequence of values that can include `to/by/for` expansions (typically 5 for Q/W/E, 3 for R via internal `fill`).
  - Named: `round` (decimals or "false").

## Next steps (dev)

1. Fix expansion order so nested `pp`/`ap` are handled before outer templates; re-run conversion until no `{{pp`/`{{ap` remain.
2. Implement a hybrid pp/ap parser-renderer:
   - Compute lists for simple ranges; carry explicit lists verbatim; preserve labels/keys.
   - Surface `formula` always; wrap math-capable expressions in `$...$`.
   - Provide a details block via HTML `<details><summary>…</summary>…</details>` so Markdown remains readable.
3. Harden table conversion by pre-expanding nested templates in cells and escaping `|` that are not table separators.
