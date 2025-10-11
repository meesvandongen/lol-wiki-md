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
- [ ] Rune extended model (path, description, notes)

## 3. Parsing Primitives

- [x] Balanced template brace scanner (`extract_balanced_templates`)
- [x] Expression evaluator (#expr) with functions & right-associative exponentiation
- [x] Depth-aware template argument splitter (improved `parse_invocation`)
- [x] Variable definition & reference parsing (#vardefine / #var) two-pass
- [x] Basic wikitext table → markdown converter (simple header & rows)
- [ ] Enhanced table parser (styles, row/col spans, nested constructs)
- [~] Apostrophe bold/italic normalization
- [~] Lua narrow parser (Implemented: flat key=value tables, nested table placeholder, line/block comments, unary minus; Missing: arrays, booleans, deeper nested extraction, item data, advanced stats derivations)

## 4. Template Expansion System

- [x] Registry + trait based expanders
- [x] Strict unknown template error (`E_UNKNOWN_TEMPLATE`)
- [x] Implemented expanders: `#expr`, `tt`, `#vardefine`, `#var`
- [x] Quote (basic blockquote formatting w/ author)
- [x] Channel Type (ct) (stub -> parenthetical)
- [~] Skill Tab (st) parsing & table generation (inline marker + naive row capture)
- [x] Flip Text (ft)
- [x] Small Bold Caps (sbc) (uppercase + bold conversion)
- [~] Formula: ap (multi-value sequence formatting implemented)
- [~] Formula: pp (trimming & cleaned joins implemented)
- [~] Formula: pptooltip (mirrors pp)
- [~] Formula: fd (fixed decimals; CLI precision wired overall)
- [~] Icon unwrap (label pass-through w/ basic possessive + display label support)
- [x] Parser functions: `#if`, `#ifeq`, `#switch` (colon-form parsing + basic semantics)
- [ ] External info includes (Spellblade, Energized, Diminishing gold, etc.)
- [~] Champion / item constant data substitution (ccd/cid) (vars placeholder)
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
- [ ] Patch history extraction (limit 10) with structured Change objects
- [ ] Trivia extraction (lists, nested bullets)
- [ ] Notes section formatting preservation
- [ ] Advanced stats derivation (windup %, derived metrics)

## 6. Item Conversion

- [x] Lua item data parsing (Module/ItemData)
- [~] Item gold cost fields & combine cost validation
- [~] Components & build tree graph
- [~] Stats (flat & percent) extraction
- [~] Passive / active parsing (basic text, template expansion)
- [ ] Item classification (starter/basic/legendary/mythic)
- [ ] Embedded table conversion in item descriptions

## 7. Rune Conversion

- [ ] Rune page loading
- [ ] Basic description extraction
- [ ] Path / slot inference (if determinable)
- [ ] Patch history (structured) reuse champion logic
- [ ] Notes / trivia extraction

## 8. Linking & Anchors

- [ ] Internal wiki link normalization to local markdown
- [ ] Anchor normalization (strip parens, unify dashes, remove commas)
- [ ] Skip File: / image links
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
- [ ] Full champion Markdown renderer (sections: Overview, Abilities, Stats, Advanced Stats, Pets, Trivia, Patch History)
- [ ] Item Markdown layout (cost table, stats table, passives/actives)
- [ ] Rune Markdown layout
- [ ] Structural whitespace normalization (collapse blanks, ensure EOF newline)
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

- [ ] Template inventory scanner (build script or test util)
- [ ] Inventory JSON artifact (`template_inventory.json`)
- [ ] Specimen matrix directory structure (`tests/fixtures/templates/<name>/`)
- [ ] Execution coverage metrics (all registered templates executed)
- [ ] Parameter key coverage verification
- [x] Residual construct detection (unexpanded templates) (basic '{{' scan post-expansion)
- [ ] Drift acceptance flow (`UPDATE_INVENTORY=1`, version bump constant)

## 14. Template Specific Logic (Detail Backlog)

- [ ] `ap` render format (retain human readable + scaling notation)
- [ ] `pp` per-level sequences + optional tooltip bridging
- [ ] `pptooltip` variant aggregator
- [ ] `fd` fixed decimal formatting wrapper around numeric normalization
- [ ] `st` multi-column leveling table composition
- [ ] `ct` channel type textual rendering
- [ ] `ft` flip text stylistic wrapper (static form)
- [ ] `sbc` uppercase + bold transformation
- [ ] Icon unwrapping: champion/ability/item/rune icons → text label; possessive fix
- [ ] External info includes `<includeonly>` extraction & inlining
- [ ] `ccd` / `cid` numeric constant substitution (requires data map)
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

Foundation in place for strict parsing (errors, expression evaluation, template registry) with an expanded subset of template handlers, including core parser functions (`#if`, `#ifeq`, `#switch`) and common utility/neutralizer templates. Lua tokenizer improved (comments, unary minus). A validator pass now emits both a full report and a compact summary (supported names and top unknowns) to guide implementation. Conversion currently produces a minimal placeholder markdown for champions; domain data integration and rendering remain the largest next steps.

## Key Technical Debts / Gaps

1. No integration of Lua-derived stats into `Champion.stats`.
2. Abilities not yet loaded beyond template parsing logic (directory traversal & structured extraction missing).
3. Most domain templates (ap, pp, st, ct, etc.) unimplemented → will cause `E_UNKNOWN_TEMPLATE` and halt real pages.
4. Missing link normalization & apostrophe markup handling; raw wiki formatting will bleed into output.
5. Drift protection framework not started; need early bootstrap before template surface grows.
6. Rendering logic minimal; must implement stable layout prior to golden test introduction.

## Near-Term Priority Plan (Next Sessions)

1. Integrate Lua stats mapping (hp, hpGrowth, mp, mpGrowth, ad, adGrowth, asBase, asRatio, asGrowth, armor, mr, ms, range, resource).
2. Ability loader: enumerate `Template/Data_<Champion>/` keys, parse each, build `Ability` list in canonical order.
3. Implement first wave of template handlers unlocking ability text fidelity: `ap`, `pp`, `pptooltip`, `fd`, icon unwrap basics.
4. Introduce early renderer for Abilities + Stats sections; create synthetic fixture champion golden test.
5. Add link normalization utility & apostrophe style converter.
6. Bootstrap drift inventory script (collect template names only as v0) to lock current set and detect expansion impact.

## Tracking Notes

- Keep expansion ordering explicit; many templates depend on prior evaluation (#expr before formula wrappers).
- When adding new template handlers, also add specimen tests to prepare for future inventory gating.
- Introduce `Result` returns in more pipeline stages early to preserve strict fail-fast property end-to-end.
- Start capturing spans in brace scanner now (store start/end) to allow error diagnostics later.

---

## Change Log (Manual Entries Going Forward)

- Initial status file created with snapshot of implemented components.

---

_Last updated: (initialize on creation; update manually)_
