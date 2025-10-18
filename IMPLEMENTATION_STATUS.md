# Implementation Status & Roadmap

This document tracks progress against `requirements.md` for the Rust LoL Wiki → Markdown converter. It is intended for iterative development; check items off as they are fully delivered. Partial items list sub‑tasks so progress remains visible.

Legend:

- [x] Done
- [~] Partially implemented (some sub-points outstanding)
- [ ] Not started

---

## 1. Core Architecture & Infrastructure

- [x] Rust crate scaffold (lib + bin `convert`)
- [x] CLI baseline (champion selection, wiki root, output dir, precision)
- [x] Error model with stable codes (`ConvertError` variants)
- [x] Deterministic write helper (`write_if_changed`)
- [x] Tracing initialization (plain vs JSON placeholder)
- [ ] Global conversion context / shared caches struct
- [ ] Batch mode & parallelism (Rayon integration for multi-entity)
- [ ] Config flags (max_patches, json output, skip sections)

## 2. Data Models

- [x] Champion basic struct & nested types (BasicInfo, Stats, Ability, etc.)
- [x] Ability data structure (cooldowns/costs/ranges, etc.)
- [x] Item placeholder model
- [x] Rune placeholder model
- [ ] AdvancedStats population
- [ ] Pets model usage in pipeline
- [ ] PatchEntry / Change collection integration
- [ ] Full Item model fields (cost breakdown, stats, passives, build)
- [x] Rune extended model (path, description, notes)

## 3. Parsing Primitives

- [x] Balanced template brace scanner (`extract_balanced_templates`)
- [x] Expression evaluator (#expr) with functions & right-associative exponentiation
- [x] Depth-aware template argument splitter (improved `parse_invocation`)
- [x] Variable definition & reference parsing (#vardefine / #var) two-pass
- [x] Basic wikitext table → markdown converter (simple header & rows)
- [ ] Enhanced table parser (styles, row/col spans, nested constructs)
- [x] Apostrophe bold/italic normalization (basic wiki run handling via renderer utilities)
- [~] Lua narrow parser (Implemented: flat key=value tables, nested table placeholder, line/block comments, unary minus; Missing: arrays, booleans, deeper nested extraction, item data, advanced stats derivations)

## 4. Template Expansion System

- [x] Registry + trait based expanders
- [x] Strict unknown template error (`E_UNKNOWN_TEMPLATE`)
- [x] Implemented expanders: `#expr`, `tt`, `#vardefine`, `#var`
- [x] Quote (basic blockquote formatting w/ author)
- [x] Channel Type (ct) (stub -> parenthetical)
- [x] Skill Tab (st) parsing & table generation (inline marker + multi-row support)
- [x] Flip Text (ft)
- [x] Small Bold Caps (sbc) (uppercase + bold conversion)
- [x] Formula: ap (multi-value sequence formatting implemented)
- [x] Formula: pp (trimming & cleaned joins implemented)
- [x] Formula: pptooltip (mirrors pp)
- [x] Formula: fd (fixed decimals; CLI precision wired overall)
- [~] Icon unwrap (label pass-through w/ basic possessive + display label support; fuller metadata pending)
- [x] Parser functions: `#if`, `#ifeq`, `#switch` (colon-form parsing + basic semantics)
- [ ] External info includes (Spellblade, Energized, Diminishing gold, etc.)
- [x] Champion / item constant data substitution (ccd/cid)
- [ ] Item haste tabber → subsections
- [~] Residual cleanup / neutralization list (navboxes, categories, module invokes, patch box, champions/categories) — many structural templates neutralized, list still evolving

## 5. Champion Conversion Pipeline

- [x] Page loading (Main/<Champion>/page.txt)
- [x] Template expansion (current subset) integrated
- [x] Ability template file parser (`parse_ability_template`)
- [x] Detect & load champion module stats (basic subset mapping hp/hpGrowth etc.)
- [x] Ability directory scan + ordering Passive/Q/W/E/R (basic loader)
- [x] Ability description multi-field extraction (descriptions, notes, cooldowns, costs, ranges)
- [ ] Skill tab (`st`) consolidation into leveling tables
- [ ] Pets extraction (tabber / infobox) and mapping
- [x] Patch history extraction (limit 10) with structured Change objects
- [ ] Trivia extraction (lists, nested bullets)
- [ ] Notes section formatting preservation
- [ ] Advanced stats derivation (windup %, derived metrics)

## 6. Item Conversion

- [x] Lua item data parsing (Module/ItemData)
- [x] Item gold cost fields & combine cost validation (totals captured; validation implemented)
- [x] Components & build tree graph (recipe list captured; hierarchy represented as flat list)
- [x] Stats (flat & percent) extraction
- [x] Passive / active parsing (basic text, template expansion)
- [x] Item classification (starter/basic/legendary/mythic) via tier field
- [x] Embedded table conversion in item descriptions

## 7. Rune Conversion

- [x] Rune page loading
- [x] Basic description extraction
- [x] Path / slot inference (if determinable)
- [x] Patch history (structured) reuse champion logic
- [x] Notes / trivia extraction

## 8. Linking & Anchors

- [x] Internal wiki link normalization to local markdown
- [x] Anchor normalization (strip parens, unify dashes, remove commas)
- [x] Skip File: / image links
- [ ] Context-aware transformation (avoid inside code/table cells already processed)

## 9. Error Handling & Strictness

- [x] Error variants established
- [x] Fail-fast expansion (errors propagate, no silent template leakage)
- [x] Duplicate key error path exercised (ability parser)
- [~] Span capture for errors (template spans now carry line/col; plumbing into errors pending)
- [ ] Distinguish entity/file in diagnostic format output
- [x] Detect & error on unexpanded residual `{{` / `<tabber>` after full pipeline
- [ ] Redirect loop detection for ability template redirects

## 10. Performance & Determinism

- [x] Right-associative exponent ensures deterministic math
- [ ] Lua module pre-scan index (name → byte span)
- [ ] Parallel batch conversion (Rayon)
- [~] Numeric formatting centralization & precision parameter wiring (precision plumbed from CLI)
- [ ] Avoid intermediate String allocations in expansion (Cow optimization)
- [ ] Benchmarks (criterion) baseline

## 11. Rendering (Markdown)

- [x] Minimal champion placeholder renderer
- [~] Full champion Markdown renderer (Overview/Stats/Abilities implemented; advanced stats, pets, trivia, patch history pending)
- [x] Item Markdown layout (cost table, stats table, passives/actives)
- [x] Rune Markdown layout
- [~] Structural whitespace normalization (collapse blanks + EOF newline in renderers; global pass pending)
- [ ] Optional JSON sidecar serialization

## 12. Testing & Quality Gates

- [x] Unit tests: brace parser, expression evaluator, variable templates, Lua simple table, ability parser
- [x] Property tests: expression fuzz (existing minimal) — (Confirm / extend)
- [ ] Property tests: brace round-trip & random template fuzz
- [ ] Lua parser tests: nested tables, arrays, booleans, trailing commas
- [ ] Ability parsing edge cases (redirect, nested templates in values)
- [ ] Template handlers specimen tests (per expander) once implemented
- [ ] Integration golden test: representative champion (e.g., Azir)
- [ ] Integration golden test: item (Infinity Edge)
- [ ] Integration golden test: rune (Electrocute)
- [ ] Structural hash tests (blake3) for deterministic output
- [ ] Drift inventory generator + locked snapshot
- [ ] Mutation tests (intentional malformed snippets → specific error codes)
- [ ] Coverage reporting (CI hook) & threshold review
- [x] Validation CLI writes full template report and compact summary (supported vs. unknown with counts)

## 13. Drift Protection Suite

- [x] Template inventory scanner (conversion context captures template names during runs)
- [x] Inventory JSON artifact (`_inventory/template_inventory.json` written by CLI)
- [x] Specimen matrix summary (`_inventory/specimen_matrix.json` with sample artifacts)
- [~] Execution coverage metrics (coverage JSON emitted; still need enforcement + 100% execution)
- [~] Parameter key coverage verification (`_inventory/template_parameters.json` listing observed keys)
- [x] Residual construct detection (unexpanded templates) (basic '{{' scan post-expansion)
- [ ] Drift acceptance flow (`UPDATE_INVENTORY=1`, version bump constant)

## 14. Template Specific Logic (Detail Backlog)

- [x] `ap` render format (baseline + scaling notation)
- [x] `pp` per-level sequences + optional tooltip bridging (textual join)
- [x] `pptooltip` variant aggregator
- [x] `fd` fixed decimal formatting wrapper around numeric normalization
- [ ] `st` multi-column leveling table composition
- [x] `ct` channel type textual rendering
- [x] `ft` flip text stylistic wrapper (「 a ⟷ b 」)
- [x] `sbc` uppercase + bold transformation
- [~] Icon unwrapping: champion/ability/item/rune icons → text label; possessive fix (baseline label only)
- [ ] External info includes `<includeonly>` extraction & inlining
- [x] `ccd` / `cid` numeric constant substitution
- [ ] Item haste tabber linearization into subsections

## 15. Advanced Stats & Derived Calculations

- [ ] Derive windup percentages
- [ ] Attack speed ratio normalization
- [ ] Per-level formula evaluation (embedding #expr results) with precision control
- [ ] Handling unique champion exceptions (e.g., Yasuo/Yone crit) if in scope

## 16. Logging & Diagnostics Enhancements

- [ ] Structured per-entity summary (template counts, time)
- [ ] Final run aggregate stats (entities converted, failures, unknown templates)
- [ ] `--debug-dump` intermediate artifacts (raw_sections.json, abilities.json)
- [ ] Optional JSON logging formatting toggle (existing placeholder needs wiring)

## 17. Cleanup & Style

- [ ] Remove unused imports / clippy clean pass
- [x] Enforce `#![deny(unsafe_code)]` at crate root
- [ ] rustfmt / CI style config

## 18. Documentation

- [x] Initial README_RUST / requirements doc present (external)
- [ ] Update README with current feature matrix & usage examples
- [ ] Add `TESTING.md` describing drift suite & golden workflow
- [ ] CHANGELOG tracking deviations & drift acceptances
- [ ] Architecture diagrams / module overview update after refactors

## 19. Security & Safety

- [ ] Explicit denial of unsafe code (crate attribute)
- [ ] Panic audit of parsing modules
- [ ] Input size / recursion depth guards (expression & template nesting)

## 20. Performance Tracking

- [ ] Criterion benchmark: single champion
- [ ] Criterion benchmark: full champion set (synthetic fixture subset)
- [ ] Memory footprint measurement (peak) for baseline
- [ ] Hotspot profiling & targeted micro-optimizations (if needed)

---

## Current Implementation Summary (Snapshot)

Champion, item, and rune converters now load their respective module data, expand the supported template set, and render structured Markdown (stats tables, ability info blocks, rune notes/trivia) with apostrophe/link normalization helpers. The template registry covers parser functions, formula helpers, icon unwraps, stylistic wrappers, and a neutralization list for structural scaffolding. Conversion runs also capture template usage and sample artifacts, emitting `_inventory/template_inventory.json`, `_inventory/specimen_matrix.json`, `_inventory/template_coverage.json`, and `_inventory/template_parameters.json` for drift tracking and coverage insight. The validation CLI produces JSON + summary reports, and integration tests exercise the conversion and parsing primitives.

## Key Technical Debts / Gaps

1. Champion pipeline still needs patch history, pets, trivia, and richer skill-tab consolidation.
2. Template constants (`ccd`/`cid`) and other Lua-sourced numeric substitutions remain stubs.
3. Link and anchor normalization is absent, and `FlipText` behavior needs to be reconciled with the spec.
4. Item combine-cost validation, build tree hierarchy, and classification/mode labelling remain outstanding.
5. Drift protection coverage metrics and acceptance workflow remain outstanding (inventory + specimen summaries now generated).

## Near-Term Priority Plan (Next Sessions)

1. Capture champion patch history, trivia, and pets, and upgrade skill-tab handling to support multi-row leveling tables.
2. Resolve `ccd`/`cid` lookups from Lua constants so template expansions emit final numeric values.
3. Perform link + anchor normalization without disturbing tables/code blocks and document the behaviour.
4. Extend item conversion with combine-cost validation, build tree sections, and classification/mode labelling.
5. Extend drift protection: add golden hash coverage plus execute-time metrics and acceptance flow atop new inventory/specimen reports.

## Tracking Notes

- Keep expansion ordering explicit; many templates depend on prior evaluation (#expr before formula wrappers).
- When adding new template handlers, also add specimen tests to prepare for future inventory gating.
- Introduce `Result` returns in more pipeline stages early to preserve strict fail-fast property end-to-end.
- Start capturing spans in brace scanner now (store start/end) to allow error diagnostics later.

---

## Change Log (Manual Entries Going Forward)

- Initial status file created with snapshot of implemented components.
- Patch history extraction upgraded to structured `Change` objects with 10-entry cap and renderer update.
- Inventory/specimen tracking emits `_inventory/template_inventory.json`, `_inventory/specimen_matrix.json`, `_inventory/template_coverage.json`, and `_inventory/template_parameters.json`.

---

_Last updated: 2025-10-18_
