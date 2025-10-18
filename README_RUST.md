# lol_wiki_md (Rust)

Strict, deterministic converter from an extracted League of Legends Fandom / MediaWiki dump to Markdown. This is a ground-up Rust reimplementation of the exploratory Python prototype (`simple_converter.py`). It follows the design principles outlined in `requirements.md` (fail fast, zero silent data loss, idempotent output).

## Status

The Rust converter now performs strict single-entity conversion for champions, items, and runes:

- Champion conversion loads `Module:ChampionData`, expands ability templates (including redirects), and renders Markdown with stats tables, ability info blocks, descriptions, cooldowns/costs, and notes.
- Item conversion consumes `Module:ItemData`, extracting tier/type/recipe/cost/stat/effect fields and rendering structured sections for Overview, Recipe, Stats, and Effects.
- Rune conversion parses the infobox, notes, trivia, and patch history into Markdown.
- Template expansion covers parser functions, formula helpers, icon wrappers, stylistic utilities, and a neutralization list for scaffolding templates; unsupported templates remain hard errors.
- Formatting utilities normalize wiki apostrophes, anchors, internal links (basic), and lists; renderers collapse blank lines and ensure trailing newlines for deterministic output.

## Quick Start

```bash
cargo run --bin convert -- --wiki-root ./out --output ./rust_markdown --champion Akshan
```

## High-Level Architecture

```
src/
 ├── bin/convert.rs      # CLI entry (clap)
 ├── cli.rs              # CLI arg structs + config
 ├── error.rs            # ConvertError enum with stable codes
 ├── model.rs            # Core data structs (Champion, Ability, etc.)
 ├── convert/            # Entity converters
 │    ├── mod.rs
 │    ├── champion.rs
 │    ├── item.rs
 │    └── rune.rs
 └── parse/              # Low-level parsing helpers
      ├── mod.rs
      ├── brace.rs       # Balanced template/table extraction
      ├── expr.rs        # Safe arithmetic/#expr evaluator
      └── lua.rs         # Narrow Lua table parser (stub)
```

## Guiding Principles

1. Fail Fast: Any unsupported or malformed construct returns an error (no best-effort fallbacks).
2. Deterministic: Identical inputs yield bit-for-bit identical markdown.
3. Explicit Semantics: Every template we support has a spec-like handler; unknown templates are errors.
4. Safety: No dynamic code execution; expression evaluator is a whitelisted arithmetic grammar.

## Supported Template Handlers (snapshot)

| Template(s)                                                                                       | Behavior                                                                                                                              |
| ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------ |
| `#expr`, `#if`, `#ifeq`, `#switch`                                                                | Expression evaluation and parser functions with strict error propagation.                                                             |
| `#vardefine`, `#var`                                                                              | Page-scoped variable definition/reference (returned to callers as plain text).                                                        |
| `ap`, `pp`, `pptooltip`, `fd`                                                                     | Ability scaling helpers: emit `(+XX% AP)` style sequences, joined per-level values, and fixed-decimal formatting.                     |
| `tt`                                                                                              | Inlines tooltips as `value (tooltip)`.                                                                                                |
| `tip`, `w`                                                                                        | Surface display labels while dropping icon-only usages.                                                                               |
| `ci`/`cis`, `ai`/`ais`, `ii`/`iis`, `ri`, `bi`, `ui`, `cais`                                      | Icon unwrap: returns the text label (possessive handling basic).                                                                      |
| `sbc`, `ct`, `ft`                                                                                 | Stylistic wrappers → uppercase bold, `(Channel)` label, and reversed text respectively (current behavior; slated for spec alignment). |
| `st`                                                                                              | Emits `[SkillTab key:value                                                                                                            | ...]` marker consumed later for leveling tables. |
| Neutralized scaffolding (`Section top`, `Game banner`, `Champions`, `Patch box`, `#invoke`, etc.) | Removed from output so only information-bearing content remains.                                                                      |

See `docs/refreshed_implementation_plan.md` for a detailed roadmap and template coverage notes.

## Next Increments (Roadmap)

- Capture champion patch history, pets, and trivia; enhance skill-tab consolidation into multi-row leveling tables.
- Resolve `ccd` / `cid` constant lookups from Lua modules so template expansions emit final numeric values.
- Introduce deterministic link + anchor normalization that maps wiki links to local markdown targets without harming tables/code.
- Extend item conversion with combine-cost validation, build tree sections, and classification/mode labelling.
- Bootstrap drift protection: golden hash tests for champion/item/rune fixtures and a template inventory scanner per `requirements.md` §12.1.

## Testing

`cargo test` executes unit, integration, property, and golden-hash tests (champion/item/rune outputs). Expression evaluator fuzzing ensures non-panics and finite results.

## License

Dual-licensed under MIT or Apache-2.0.
