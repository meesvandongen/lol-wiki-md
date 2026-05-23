# Rust Project Requirements: League of Legends Wiki (MediaWiki Dump) → Markdown Converter

Version: 1.0 (Derived from existing `simple_converter.py` behavior)

## 1. Goal & Scope

Build a high‑performance, maintainable Rust CLI tool that converts extracted League of Legends (LoL) Fandom (MediaWiki) content (champions, items, runes) into structured Markdown files with light semantic enrichment. It should replicate and improve upon the core functionality of the provided Python `simple_converter.py`, including:

- Parsing champion, item, and rune pages from an offline dump directory (already “exploded” into a directory structure similar to `./out`)
- Extracting structured data: abilities, stats, advanced stats, pets, patch history, item stats, rune trees, notes, trivia, formulas, and select templates
- Safely evaluating numeric expressions and formulas embedded in wiki templates
- Converting a subset of MediaWiki/Wikitext markup (and LoL‑specific templates) to clean Markdown
- Producing deterministic, idempotent output (re-running without changes yields identical files)

Out of scope for v1:

- Full generic MediaWiki parser (only targeted subset used by LoL wiki)
- Real-time fetching from network APIs (assumes local dump already present)
- Image/media extraction beyond stripping or simple inlining of text labels

### 1.1 Design Principles (Fail Fast – Always Strict)

This Rust implementation permanently abandons the Python prototype's permissive "best-effort" model. There are no flags to relax behavior.

Core principles:

- Fail Fast: Any deviation from the defined structure (templates, Lua blocks, expressions, tables) is an immediate error.
- Deterministic Contracts: Each supported construct has an explicit mini-spec; violations never degrade silently.
- No Implicit Inference: Missing authoritative data (e.g., champion stats) is not guessed or synthesized.
- Zero Leniency Flags: The binary ships only in strict mode; reproducibility > convenience.
- High-Signal Diagnostics: Stable error codes + source path (+ span when feasible) + remediation hint.
- Idempotent & CI-Friendly: Strictness ensures drift is surfaced early.

Rationale: Eliminates silent corruption pathways and reduces cognitive load for maintainers.

### 1.2 Information Coverage vs Feature Parity

Goal: 100% preservation of _information content_ present in the source wiki pages; **not** 100% replication of interactive or stylistic _features_ of the MediaWiki/Fandom platform.

Policy Guidelines:

- Information-Bearing Constructs (MANDATORY): numerical values, scaling formulas, damage types, ranges, costs, targeting rules, cooldown patterns, patch change bullets, ability / item / rune names, variant form distinctions, conditional clauses, status interactions. These must survive conversion in an explicit textual form.
- Interactive-Only Features (NON-GOAL): hover tooltips, dynamic tab UI, collapsible panels. Their underlying text/value payload must be inlined (e.g., `value (tooltip explanation)`), but no attempt is made to recreate interactivity.
- Purely Stylistic Markup (DROPPED): color (e.g., red vs blue text for physical vs magic damage), font size, alignment, padding, decorative spans, icon sizing classes. If the semantic classifier (e.g., “physical damage”) is already stated in plain text, color conveys no additional information and is omitted.
- Semi-Decorative Icons: champion / item / rune icons reduced to their canonical name text (and possessive where appropriate). No inline images are emitted.
- Tabbers / Multi-State Templates: converted into sequential subsections or tables so that all embedded content is present simultaneously (no information hidden behind states).
- Tooltips: always expanded into static text; if both a value and explanation exist, format as: `Value (Explanation)`.
- Formula Templates: preserve both human-readable expression and, where safe, the evaluated numeric sequence (e.g., `15 / 25 / 35 (+40% AP)` or an evaluated range) so no algebraic intent is lost.
- Redundant Presentation: When a template encodes an attribute already stated (e.g., color-coded damage name), only one canonical textual representation is kept.

Acceptance Alignment: A conversion is considered correct when every distinct fact a reader could learn from the wiki page is still discoverable in the markdown output without relying on style, color, or hover interactions.

## 2. Input / Output Contract

### Inputs

- File system root (`--wiki-root`): directory containing subtrees such as `Main/<PageName>/page.txt`, `Module/ChampionData/data/page.txt`, `Module/ItemData/data/page.txt`, `Template/Data_<Champion>/<AbilityKey>/page.txt`, etc.
- Mutually exclusive selector: `--champion <Name>` | `--item <Name>` | `--rune <Name>` (future: batch mode)
- Output directory (`--output`)
- Optional flags (future): `--all-champions`, `--parallel`, `--json-metadata`, `--debug`, `--strict`.

### Outputs

- A Markdown file per entity: `<Output>/<Normalized_Name>.md`
- (Optional future) Sidecar JSON (`<Name>.json`) capturing fully structured parse graph
- Exit codes: `0` success, `1` recoverable parse failures, `2` invalid CLI usage, `>2` internal unrecoverable errors

## 3. Domain Overview: LoL Wiki Specifics

The League of Legends wiki uses MediaWiki with extensive custom templates and Lua modules.

Key patterns:

- Page extraction layout (common after XML → filesystem transform)
  - `Main/<Champion>/page.txt` – core champion page
  - `Main/<Champion>/Patch_history/page.txt`
  - `Template/Data_<Champion>/<Q|W|E|R|I>/page.txt` – ability data templates
  - `Module/ChampionData/data/page.txt` – monolithic Lua table of champion metadata & stats
  - `Module/ItemData/data/page.txt` – item Lua definitions
  - `Template/Spellblade_info/page.txt`, `Energized_info`, etc. – includeonly informational blocks
  - Items / Runes similarly live in `Main/<ItemName>/page.txt`, `Main/<RuneName>/page.txt`
- Wikitext constructs heavily used: `{{Template|...}}`, `<tabber>...</tabber>`, nested braces, tables `{| ... |}`
- LoL‑specific templates: ability icons (ai/ais), champion icons (ci/cis), item icons (ii), rune icons (ri), formula helpers (`{{ap|...}}`, `{{pp|...}}`, `{{pptooltip|...}}`, `{{fd|...}}`), skill tabs (`{{st|...}}`), channel type (`{{ct|...}}`), quotes (`{{Quote|...}}`), item haste tabber, FlipText (`{{ft|...}}`), small bold caps (`{{sbc|...}}`), numeric expressions (`{{#expr: ...}}`), variable define/reference (`{{#vardefine:...}}`, `{{#var:...}}`).
- Internal links: `[[Page]]`, `[[Page|Display]]`, `[[Page#Section|Display]]`.
- Lua module table structure is a semi-regular subset: `{"Key" = {...}, ...}` with nested subtables.

## 4. Functional Requirements

### 4.1 Champion Conversion

- Detect champion page by presence of `{{Champion info` or entry in `Module/ChampionData`.
- Extract:
  - Basic info (title/banner data if present)
  - Ability list (Q, W, E, R, optionally Passive "I") from template directory or inline `{{Data Champion/Key|Ability}}` calls
  - Ability structured fields: names, blurbs, descriptions (multiple numbered variants), cooldowns, cost, targeting, affects, ranges, stats, notes, leveling tables, additional custom fields.
  - Skill tab (`{{st|...}}`) tables aggregated; free-floating st blocks collected.
  - Stats: base HP, HP growth, MP, MP growth, AD, AD growth, attack speed base/ratio/growth, armor, MR, MS, range, resource type, etc. from module data
  - Advanced stats: windup (derived), missiles, selection/pathing radii, windup modifiers, acquisition radius, projectile data
  - Pets (parse `<tabber>` or sequential `{{Infobox/Pet ...}}` blocks)
  - Patch history (limit to last N=10) parsing semicolon/linked patch headers and bullet lists (transform to structured list)
  - Trivia: list items preserving hierarchical order markers
  - Notes formatting preserving list indentation, heading markers, and template expansions

### 4.2 Item Conversion

- Parse item entry from `Module/ItemData` matching exact name; fallback to page text for supplemental info.
- Extract: gold costs (total, combine, sell), components, build tree, stats (flat & percent), passives/actives (basic text extraction), item classification (mythic, legendary, basic, starter)
- Compute combine cost = total - sum(component totals) (if needed) with validation
- Convert any wikitext tables embedded in descriptive sections
- Preserve or compute formulas appearing in fields (resolve #expr where safe)

### 4.3 Rune Conversion

- Locate rune main page, extract basic description, slot/path info if determinable, patch history (like champion), notes

### 4.4 Template & Markup Handling

- Balanced brace parsing for targeted templates (avoid naive regex for nested constructs)
- Expand custom templates before generic stripping: Quote, Channel Type (ct), Item Haste tabber
- Convert Wikitext tables `{| ... |}` to GitHub Markdown tables (header detection, style stripping)
- Convert `<tabber>` blocks for Item Haste into Markdown subsections
- Convert formula templates:
  - `{{ap|...}}` (ability power scaling formatting)
  - `{{pp|...}}` (per-level progression sequences / tooltips)
  - `{{pptooltip|...}}` variant
  - `{{fd|value}}` fixed decimal formatting
  - `{{tt|value|tooltip}}` inline tooltips -> `value (tooltip)`
  - `{{st|...}}` skill tab tables
  - `{{ft|a|b}}` flip text -> `「 a ⟷ b 」`
  - `{{sbc|...}}` small bold caps -> uppercase + bold
- Unwrap icon/inline templates (ci/cis/ai/ais/cais/ri/ii/bi/ui etc.) to plain text with possessive adjustments
- Remove structural templates (navigation, categories, maintenance) gracefully
- Evaluate `{{#expr: arithmetic }}` safely (AST‑based) supporting +,-,\*,/,^,mod, parentheses, functions: round, floor, ceil, min, max, abs, constants pi/e
- Resolve variable definitions: `{{#vardefine:name|value}}` and references `{{#var:name}}`
- Replace champion constant/item data templates (ccd/cid) where numeric values resolvable
- Expand external info templates (Spellblade info, Energized info, Diminishing gold info, Effect at cast time start/end) by inlining `<includeonly>` sections from extracted template pages
- Apostrophe style parser: convert wiki bold/italic runs respecting overlaps and avoiding interfering with apostrophes inside words

### 4.5 Linking & Anchors

- Internal links: map to `./<Normalized>.md` (spaces → `_`), preserve anchor mapping `#Section` with normalization (strip parens, unify dashes, remove commas)
- Avoid over-conversion inside already processed markdown tables and code segments (context aware)
- Remove/skip File: links & image markup

### 4.6 Error Handling & Resilience (Strict Only)

Behavior is unconditionally strict:

- Missing mandatory artifact (module data block, ability template, champion page, item definition) ⇒ error.
- Malformed Lua / unbalanced braces in any supported template (`st`, `ct`, `ap`, `pp`, `#expr`, tables) ⇒ error.
- Unknown template (not in hardcoded neutralization list) ⇒ error `E_UNKNOWN_TEMPLATE`.
- Expression evaluation failure (syntax / forbidden token / div-by-zero / overflow) ⇒ error `E_EXPR`.
- Any table parse failure (once a table is detected) ⇒ error.
- Duplicate semantic keys (stats, ability slot) ⇒ error `E_DUPLICATE_KEY`.

No continuation mode exists; the first error aborts execution.

Error format:
`[ERROR CODE] <message> (entity=<name> file=<path> span=<start..end>? hint=<suggestion>)`

There are no warnings; unexpected states must be transformed into explicit errors during development.

### 4.7 Performance

- Must support batch conversion of all champions/items (< 400 entities) quickly (< few seconds) with:
  - Concurrent IO (thread pool / Rayon)
  - Reuse of parsed Lua module data (once per run, cached Arc)
  - Avoid repeated regex recompilation (lazy static)
- Memory footprint moderate (under a few hundred MB; no need to load every page simultaneously)

### 4.8 Determinism & Idempotency

- Formula evaluation stable; floating numbers normalized (configurable precision: default 2–3 decimals)
- Sorting abilities by canonical order (Passive/I, Q, W, E, R)

### 4.9 Logging

- Structured logging (JSON optional) with levels: ERROR, WARN, INFO, DEBUG, TRACE
- Summarize counts: warnings per page; emit at end of run

### 4.10 Testing

- Unit tests: brace parser, expression evaluator, template expansions, Lua parser, list/trivia formatting
- Integration tests: known champion (e.g., "Azir") diff vs golden markdown; item ("Infinity Edge"), rune ("Electrocute")
- Property tests: round‑trip balanced brace extraction; expression evaluator non-panicking on fuzzed tokens
- Benchmark (criterion) for bulk champion conversion to monitor regression

## 5. Non-Functional Requirements

| Category      | Requirement                                        |
| ------------- | -------------------------------------------------- |
| Language      | Rust stable (MSRV >= 1.75)                         |
| Build         | Cargo, no nightly-only features for core path      |
| Platform      | Cross-platform (Windows, Linux, macOS)             |
| Performance   | < 50ms per champion average on warm cache (target) |
| Safety        | No unsafe code (or isolated & justified)           |
| Extensibility | Modular template handler registry                  |
| Observability | Feature-gated tracing spans                        |
| Docs          | Rustdoc + README + architecture (this file)        |
| Style         | Clippy clean (pedantic optional), rustfmt          |

## 6. Suggested Architecture

```
crate
 ├── bin/convert (CLI main)
 └── src/
     ├── cli.rs
     ├── fs.rs              (filesystem helpers, path resolution, caching)
     ├── lua/
     │    ├── mod.rs        (Lua table top-level dispatcher)
     │    ├── lexer.rs
     │    └── parser.rs     (lightweight subset for tables)
     ├── wiki/
     │    ├── mod.rs
     │    ├── tokenizer.rs  (balanced brace scanner; top-level splitting)
     │    ├── templates/
     │    │    ├── mod.rs
     │    │    ├── expr.rs
     │    │    ├── quote.rs
     │    │    ├── channel_type.rs
     │    │    ├── item_haste.rs
     │    │    ├── skill_tab.rs
     │    │    ├── flip_text.rs
     │    │    ├── sbc.rs
     │    │    ├── formula_ap.rs
     │    │    ├── formula_pp.rs
     │    │    ├── tooltip.rs
     │    │    └── registry.rs (trait TemplateExpander)
     │    ├── tables.rs     (wikitext table → md)
     │    ├── lists.rs
     │    ├── links.rs
     │    ├── cleanup.rs
     │    └── eval.rs       (#expr AST safe evaluation)
     ├── model/
     │    ├── champion.rs
     │    ├── item.rs
     │    ├── rune.rs
     │    ├── ability.rs
     │    ├── pet.rs
     │    ├── patch.rs
     │    └── common.rs
     ├── render/
     │    ├── markdown.rs   (renderer functions)
     │    └── formatting.rs
     ├── convert/
     │    ├── champion.rs
     │    ├── item.rs
     │    ├── rune.rs
     │    └── context.rs    (shared caches, config)
     ├── cache.rs
     ├── error.rs
     ├── logging.rs
     └── util.rs
```

### Key Traits / Interfaces

```rust
pub trait TemplateExpander {
    fn name(&self) -> &'static str;               // canonical / alias forms
    fn can_handle(&self, invocation: &TemplateInvocation) -> bool;
    fn expand(&self, invocation: TemplateInvocation, ctx: &ExpanderCtx) -> ExpansionResult; // may return unchanged fallback
}
```

### Data Structures (Serde-serializable)

```rust
struct Champion { name: String, basic: BasicInfo, stats: Stats, advanced: AdvancedStats, abilities: Vec<Ability>, pets: Vec<Pet>, trivia: Vec<TriviaItem>, patch_history: Vec<PatchEntry>, notes: Option<String> }
struct Ability { key: AbilityKey, name: String, descriptions: Vec<String>, cooldowns: Vec<String>, costs: Vec<String>, ranges: Vec<String>, leveling_tables: Vec<SkillTable>, notes: Vec<String>, extra: HashMap<String,String> }
struct Item { name: String, cost: ItemCost, stats: HashMap<ItemStat, StatValue>, passives: Vec<String>, actives: Vec<String>, build: BuildTree }
struct Rune { name: String, path: Option<String>, description: String, notes: Vec<String>, patch_history: Vec<PatchEntry> }
```

### Parsing Strategy

1. Load global context (ChampionData, ItemData) once → parse Lua tables to intermediate JSON-like maps
2. For a requested entity, read main page wikitext
3. Pre-pass: extract sections (Abilities, Trivia, Pets, etc.) via heading regex with DOTALL segmentation
4. Ability processing:
   - Directory lookup for `Template/Data_<Champion>/<Key>/page.txt`
   - If redirect (#REDIRECT) follow target (bounded depth) to canonical file
   - Parse parameters with top-level brace/pipe depth tracking (avoid splitting inside nested templates)
5. Template expansion pipeline (ordered):
   - Variable definition extraction (#vardefine)
   - Early numeric (#expr) and constant data (ccd/cid)
   - Structural expansions (Quote, Channel Type, Item Haste, Skill Tabs, FlipText, SBC)
   - Icon unwrapping / link normalization
   - Formula templating (ap/pp/pptooltip/fd/tt)
   - Wikitext table conversion
   - List & inline style normalization (apostrophes to markdown)
   - Cleanup / residual template stripping
6. Markdown renderer assembles front-matter style header or simple sectional headings
7. Post-process: collapse redundant blank lines, ensure newline at EOF

### Lua Parsing Approach

- Implement narrow parser (not full Lua): recognize patterns
  - Keys: `['Key']` / `"Key"`
  - Assignments: `['Key'] = value,`
  - Values: number, string, boolean, nested table, flat arrays `{1,2,3}` or `{ "a", "b" }`
- Use tokenization with brace depth to isolate champion/item block by searching for `[{"ChampionName"}] = {` then scanning until braces balanced
- Avoid executing code; ignore functions, operators
- Provide tolerant fallback: if parse fails, log warning and skip advanced stats

### Balanced Brace Extraction

- Single pass scan maintaining depth; record start index when encountering `{{` or `{|` and extract when depth returns to zero; support nested braces / templates
- Provide utility for selective template capture with predicate: name equals `st`, `ct`, etc.

### Expression Evaluation Safety

- Parse expression tokens into AST using `pest` or handwritten recursive descent limited grammar
- Whitelist numeric operations; disallow identifiers outside allowed set
- Limit recursion depth and expression length (e.g., 2k chars) to avoid pathological cases

## 7. Dependencies (Crates)

| Purpose        | Candidate Crate                 | Notes                                    |
| -------------- | ------------------------------- | ---------------------------------------- |
| CLI            | `clap`                          | Derive-based, subcommands later          |
| Regex          | `regex`                         | Precompile with lazy_static or once_cell |
| Concurrency    | `rayon`                         | Parallel batch conversions               |
| Serialization  | `serde`, `serde_json`           | Optional JSON outputs                    |
| Error mgmt     | `thiserror`                     | Derive Error enums                       |
| Logging        | `tracing`, `tracing-subscriber` | Structured logs, feature gating          |
| Benchmark      | `criterion`                     | Dev only                                 |
| Property tests | `proptest`                      | Brace extractor & expr eval              |
| Hash maps      | `hashbrown`                     | If performance-critical                  |
| Markdown post  | (Optional) `pulldown-cmark`     | Only if future parsing back needed       |
| Config         | `serde_yaml`                    | If external config introduced            |

Minimize dependencies for core parser to keep binary lean.

## 8. Error Model

```rust
#[derive(Debug, thiserror::Error)]
pub enum ConvertError {
  #[error("E_CHAMPION_NOT_FOUND: {0}")] ChampionNotFound(String),
  #[error("E_ITEM_NOT_FOUND: {0}")] ItemNotFound(String),
  #[error("E_RUNE_NOT_FOUND: {0}")] RuneNotFound(String),
  #[error("E_LUA_PARSE: {detail}")] LuaParse { detail: String },
  #[error("E_MALFORMED_TEMPLATE {name}: {detail}")] MalformedTemplate { name: String, detail: String },
  #[error("E_UNBALANCED_BRACES {name}")] UnbalancedBraces { name: String },
  #[error("E_UNKNOWN_TEMPLATE {name}")] UnknownTemplate { name: String },
  #[error("E_EXPR {expr}: {detail}")] Expr { expr: String, detail: String },
  #[error("E_TABLE_PARSE {context}: {detail}")] TableParse { context: String, detail: String },
  #[error("E_IO: {0}")] Io(#[from] std::io::Error),
}
```

// No warning enum: every anomaly escalates to an error.

## 9. Configuration & Extensibility

- `Config` struct (clap + env) controlling: precision, max_patches, parallelism, markdown_style options, skip_sections flags
- Template registry built at startup; new expander added by implementing trait & registering
- Feature flags: `--features full`, `minimal` (disable heavy expansions)

## 10. Algorithmic Considerations

| Concern                    | Strategy                                                                                 |
| -------------------------- | ---------------------------------------------------------------------------------------- |
| Nested template splits     | Use depth-aware scanner instead of regex splits                                          |
| Large module files         | Memory-map or stream slice extraction by scanning indices                                |
| Repeated expansions        | Maintain immutable raw text + layered transformations to facilitate debugging (optional) |
| Floating output stability  | Format with `format!("{:.2}", value)`; strip trailing zeros if desired                   |
| Race conditions            | Read-only caches in `Arc`, no mutation after build                                       |
| Windows path normalization | Use `PathBuf` carefully; unify case for lookups where necessary                          |

## 11. Markdown Output Structure (Champion Example)

```
# <Champion Name>

## Overview
<basic info lines>

## Abilities
### Passive – <Name>
<Description>
<Leveling Table (if any)>
Notes:
- Bullet

### Q – <Name>
...

## Stats
| Stat | Base | Growth | Level 18 |
|------|------|--------|----------|

## Advanced Stats
| Metric | Value |

## Pets
(Present if any)

## Trivia
- ...

## Recent Patch History
- **V14.5**: Changed ...

```

## 12. Testing Strategy Details

- Golden master tests load fixture wiki root (subset) and compare output markdown hashed with blake3; update via `UPDATE_GOLDEN=1` env
- Fuzz test expression evaluator with random numeric token sequences ensuring no panic & result finite
- Lua parser tests with nested braces, arrays, unquoted keys, trailing commas scenario
- Integration test all champions concurrently to measure throughput (bench)

### 12.1 Comprehensive Data Coverage & Future Drift Protection (New Requirement)

Objective: Guarantee that 100% of currently known wiki data constructs (templates, their parameter shapes, structural wikitext patterns we claim to support) are exercised by the automated test suite, and that any future upstream wiki change cannot silently degrade conversion fidelity unless it introduces a genuinely new template (which will intentionally fail fast).

Mechanisms (all mandatory):

1. Template Inventory Snapshot

- A build script scans the full (fixture) wiki dump collecting every template invocation: name (normalized), distinct parameter name sets, arity distributions, and example bodies.
- Emits `tests/fixtures/template_inventory.json` (versioned). Fields: `template_name`, `occurrence_count`, `distinct_param_sets` (hashes + explicit param lists), `sample_invocations` (capped N), `first_seen_hash`.
- CI test re-runs inventory; mismatch (added/removed template, new param key, changed prevalence > configurable threshold) = failure with diff output, instructing maintainer to consciously accept drift by regenerating the fixture (explicit `UPDATE_INVENTORY=1`).

2. Coverage Specimens Matrix

- For each supported template (registered in TemplateExpander registry), derive a minimal set of specimen invocations that together cover: (a) every optional parameter present in inventory; (b) each syntactic variant (positional vs named); (c) at least one nested-template-in-parameter case if observed.
- These specimens live in `tests/fixtures/templates/<name>/case_*.wiki` with expected markdown (golden) outputs. A meta-test asserts every supported template has ≥1 specimen and that every parameter key observed in the inventory for that template appears in at least one specimen file.

3. Exhaustive Page Conversion Test

- Integration test enumerates all pages in the (compressed) test dump and runs the converter in strict mode. Asserts: (a) zero `E_UNKNOWN_TEMPLATE`; (b) zero residual `{{` / `<tabber>` / `{|` constructs in final markdown unless whitelisted as intentionally preserved text.
- Produces a JSON run log mapping page → {"templates_total": X, "templates_expanded": X, "residual": 0}. Test asserts `templates_total == templates_expanded` globally.

4. Token Consumption Accounting

- Balanced-brace tokenizer emits per-template span IDs. After expansion, a verifier walks the transformed string ensuring every recorded span ID produced some output token slice (length >0) and none were silently dropped (unless template is in explicit DROP list with justification). Failure = test error identifying span + template.

5. Expression Evaluator Exhaustive Set & Fuzz

- Deterministic corpus: all distinct `#expr` raw strings from inventory executed; AST node type coverage asserted (require each grammar production seen ≥1).
- Property-based fuzz (proptest) generates random bounded expressions; assertions: parse succeeds OR yields controlled `E_EXPR`; never panics; result finite; formatting stable (round-trip of pretty-print -> parse again).

6. Drift Shield Thresholds

- Inventory drift categories:
  - NEW_TEMPLATE: immediate CI failure with guidance to implement expander or classify as unsupported (intentional rejection list) before merging.
  - NEW_PARAM_KEY on existing template: failure; must update specimen matrix & expander parser.
  - PARAM_REMOVED: warning escalated to failure (requires explicit acceptance) to ensure we reconsider dead code paths.
- Accepting drift requires regenerating inventory + specimens and updating a monotonically increasing `INVENTORY_VERSION` constant used in tests.

7. Mutation / Resilience Checks

- Mutate specimen wiki inputs (e.g., delete a pipe, duplicate a parameter, inject unknown template) expecting deterministic specific error codes (snapshot of error classification). Ensures error taxonomy stability.

8. Hash-Based Golden Stability

- For full-entity conversions, compute a structural hash (e.g., blake3 of JSON model + renderer output). Golden test asserts unchanged. Any difference forces review (prevents subtle formatting regressions that do not break functional assertions).

9. 100% Supported-Template Execution Metric

- Test computes: `supported_templates_executed / supported_templates_registered == 1.0` using specimen + integration executions aggregated. Guards against unreferenced dead expander implementations.

10. CI Enforcement Workflow

- Separate GitHub Actions job `drift-check` runs inventory & exhaustive conversion against latest fixture dump.
- Failing drift cannot be bypassed except by pushing a commit that updates fixtures and increments `INVENTORY_VERSION` (explicit audit trail in CHANGELOG under "Template Drift").

11. Future Wiki Update Handling

- When upstream dump changes, first run script locally; new template triggers immediate `E_UNKNOWN_TEMPLATE` in integration test → implement expander (or add to explicit unsupported list causing chosen failure mode) before accepting dump.
- Guarantee: No silent behavior change for existing templates; any structural upstream modification causes either test failure or unchanged output.

Success Criteria for This Requirement:

- A fresh run over the test dump yields: 0 unknown templates, 0 unexpanded residual constructs, 100% parameter key coverage, 100% expander invocation coverage, stable golden hashes.
- Adding a synthetic new template in a test fixture produces a failing test (proves shield works).

Documentation: Add `TESTING.md` detailing inventory generation, regeneration workflow, and acceptance steps.

Rationale: This layered approach combines static inventory locking, specimen completeness, runtime accounting, and drift gating to provide strong assurance that the converter's supported surface exactly matches the observed wiki input domain and cannot regress silently.

## 13. Performance Optimization Opportunities

- Pre-scan module file once to build index: champion name → byte span (start,end) to avoid repeated linear scans
- Use `memchr` / `aho-corasick` for high-frequency token searches (e.g., `{{` occurrences) if profiling indicates hotspot
- Reuse `RegexSet` for multi-pattern template pruning
- Avoid allocating intermediate Strings for each template expansion (work with slices + `Cow<'_, str>`) where feasible

## 14. Security & Safety

- No dynamic code execution; expression evaluator is pure / sandboxed
- Input trusted locally but resilience against malformed wikitext (prevent panics)
- Deny unsafe code by CI (RUSTFLAGS = -D unsafe_code; allow override per module with justification if needed)

## 15. Logging & Diagnostics

- `--debug-dump <dir>` optional artifact: JSON of intermediate parse stages (raw_sections.json, abilities.json)
- Diagnostic IDs for warnings (`W001`, `W002`, etc.) documented in README

## 16. CLI Specification (Initial)

```
lolmd convert --champion Azir --wiki-root ./out --output ./markdown
lolmd convert --item "Infinity Edge" --wiki-root ./out --output ./markdown
lolmd convert --rune Electrocute --wiki-root ./out --output ./markdown
lolmd batch --all-champions --parallel 8 --wiki-root ./out --output ./markdown
```

Flags (non-relaxing only):

- `--max-patches <N>` (default 10)
- `--precision <N>` (numeric formatting)
- `--json` (emit sidecar structured file)
- `--no-advanced` (skip advanced stats section generation at all; still errors if referenced templates malformed)

## 17. Edge Cases & Handling (Single Strict Path)

| Case                          | Handling (Always)                                 |
| ----------------------------- | ------------------------------------------------- |
| Missing ability template file | Error `E_MALFORMED_TEMPLATE`                      |
| Redirect loops (>5)           | Error `E_MALFORMED_TEMPLATE` (loop detected)      |
| Unbalanced `{{` braces        | Error `E_UNBALANCED_BRACES`                       |
| Unknown template              | Error `E_UNKNOWN_TEMPLATE`                        |
| Empty patch history           | Section omitted (not an error)                    |
| Duplicate stat / ability key  | Error `E_DUPLICATE_KEY`                           |
| Expression division by zero   | Error `E_EXPR`                                    |
| Giant numeric (overflow)      | Error `E_EXPR`                                    |
| Table parse failure           | Error `E_TABLE_PARSE`                             |
| Missing module data entry     | Error `E_CHAMPION_NOT_FOUND` / `E_ITEM_NOT_FOUND` |

## 18. Future Enhancements (Not required for v1)

- Batch mode with progress bar (indicatif)
- Incremental rebuild (hash source & skip unchanged)
- LSP / language server providing hover docs for ability formulas
- WASM build for web UI preview
- Structured front-matter YAML for static site generators
- Translation / locale toggles

## 19. Implementation Phases

1. Skeleton CLI + file loading + logger
2. Lua module parser + caching
3. Champion basic extraction (abilities directory only) → markdown
4. Template brace scanner + simple expansions (Quote, tt, st)
5. Expression evaluator (#expr)
6. Items + stats mapping
7. Advanced stats + pets + patch history
8. Channel type + item haste tabber conversions
9. Cleanup & apostrophe style parser
10. Test suite + golden outputs
11. Benchmarks + optimization pass
12. Documentation polish & release

## 20. Acceptance Criteria (Strict Mode Only)

- Converting a known champion produces a markdown with all required sections populated (Abilities, Stats, Advanced Stats, Patch History ≤ 10 entries, Trivia if present) and no panics
- Abilities ordered correctly, skill tab tables rendered as markdown tables
- Item markdown contains cost breakdown and computed combine cost (where applicable)
- All #expr blocks replaced with numeric results; failure to evaluate any is an error (no preservation fallback)
- Unknown templates cause hard failure; zero silent raw `{{...}}` leakage
- Running twice yields identical output (normalized whitespace) for unchanged inputs
- Any single parsing / expansion error aborts execution immediately (no batch continuation logic)
- Test suite passes (≥95% branch coverage in parsing modules ideal but not mandatory for v1)
- Comprehensive Data Coverage & Drift Protection suite passes: inventory snapshot matches, 0 residual constructs, 100% template & parameter coverage metrics satisfied, structural golden hashes unchanged.

## 21. Risks & Mitigations

| Risk                                     | Mitigation                                          |
| ---------------------------------------- | --------------------------------------------------- |
| LoL wiki template drift                  | Configurable template registry + pattern tests      |
| Parsing complexity creep                 | Keep focused subset; document unsupported cases     |
| Performance regressions                  | Benchmark in CI; track baseline commit              |
| Windows path issues                      | Use canonical case-insensitive compare where needed |
| Float formatting differences             | Centralized numeric formatter function              |
| Over-aggressive cleanup removing content | Multi-stage debug dumps for inspection              |

## 22. Glossary

- Passive / I: Champion innate ability slot
- Module Data: Lua table providing canonical numeric stats
- Tabber: Fandom extension enabling tabbed content, parsed manually
- Skill Tab (`st`): Wikitext template for side-by-side leveling values
- Channel Type (`ct`): Template describing interaction constraints (attack, move, etc.)

## 23. Open Questions (To Validate Before Coding)

- Are there item passives needing formula resolution beyond current subset? (If yes, extend formula engine)
- Should we emit front-matter metadata for static site integration?
- Is rune path mapping derivable reliably from dump (else manual mapping file)?
- Accept duplicate ability keys? (Edge case: forms / transformations) Strategy: allow extended keys (e.g., `Q2`, treat as variant)

## 24. Reference Implementation Observed Behaviors (Python)

- Uses many targeted regex + brace-depth loops instead of full parser
- Favors best-effort fallback: leaves unhandled template text for later generic stripping
- Limits patch history list length to 10
- Derives windup% when both cast and total attack times available
- Normalizes anchors: spaces → underscores, remove parens/commas, unify dashes
- Treats Yasuo/Yone crit differently (adjusted critical damage base)

Replicate these semantics unless justified otherwise; document divergences.

---

This document is the authoritative blueprint for the Rust rewrite. Any deviation should be recorded in CHANGELOG / design notes.
