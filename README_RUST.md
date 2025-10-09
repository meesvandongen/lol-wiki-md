# lol_wiki_md (Rust)

Strict, deterministic converter from an extracted League of Legends Fandom / MediaWiki dump to Markdown. This is a ground-up Rust reimplementation of the exploratory Python prototype (`simple_converter.py`). It follows the design principles outlined in `requirements.md` (fail fast, zero silent data loss, idempotent output).

## Status

Early scaffolding: only minimal champion page passthrough with structural parsing utilities (brace scanner, expression evaluator) and error model. Incrementally expand template handlers and entity extraction.

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

## Next Increments (Roadmap)

- Implement Lua narrow parser for ChampionData/ItemData modules.
- Template registry & handler trait; implement core icon unwrap + formula templates.
- Markdown rendering layer (abilities, stats, patch history, items, runes).
- Golden tests (Azir, Infinity Edge, Electrocute) with blake3 hashes.
- Inventory snapshot build script (template invocation coverage) per requirements §12.1.

## Testing

`cargo test` executes unit + property tests. Expression evaluator is fuzzed (non-panics & finite results). Brace parser round-tripped.

## License

Dual-licensed under MIT or Apache-2.0.
