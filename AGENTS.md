# AGENTS.md - lol_wiki_md Rust Project Guide

## Build/Lint/Test Commands

### Build Commands
- `cargo build` - Debug build
- `cargo build --release` - Optimized release build
- `cargo build --features minimal` - Build with minimal features (no rayon parallelism)

### Test Commands
- `cargo test` - Run all unit, integration, and property tests
- `cargo test <test_name>` - Run a specific test (e.g., `cargo test basic` for expression tests)
- `cargo test --test <integration_test_file>` - Run integration tests (e.g., `cargo test --test golden_champion`)
- `cargo test -- --nocapture` - Run tests with output capture disabled for debugging

### Lint/Format Commands
- `cargo clippy` - Run Clippy linter (pedantic mode expected to pass)
- `cargo fmt` - Format code with rustfmt
- `cargo fmt --check` - Check formatting without modifying files

### Benchmark Commands
- `cargo bench` - Run Criterion benchmarks (expression evaluation, conversion performance)

### Validation Commands
- `cargo run --bin convert -- --validate --wiki-root ./out --output ./validation_reports` - Generate template validation reports

## Architecture & Codebase Structure

### Project Structure
This is a Rust library crate with a binary CLI tool for converting League of Legends MediaWiki dumps to Markdown.

**Key Modules:**
- `src/lib.rs` - Library root, exports main APIs
- `src/bin/convert.rs` - CLI entry point using clap
- `src/cli.rs` - CLI argument parsing and configuration
- `src/error.rs` - Error types with stable codes (thiserror-based)
- `src/model.rs` - Core data structures (Champion, Item, Rune, Ability, etc.)
- `src/parse/` - Low-level parsing utilities
  - `brace.rs` - Balanced template/table extraction
  - `expr.rs` - Safe arithmetic expression evaluator (#expr)
  - `lua.rs` - Narrow Lua table parser for Module data
  - `templates/` - Template expansion system (registry + expanders)
- `src/convert/` - Entity converters (champion, item, rune)
- `src/render/` - Markdown rendering functions
- `src/validate/` - Template validation and reporting
- `src/wiki_export/` - Wiki dump filesystem abstraction

### Key Architectural Patterns
- **Template Registry System**: Trait-based `TemplateExpander` for extensible template handling
- **Context Pattern**: `ConversionContext` holds shared caches and configuration
- **Error Propagation**: Strict fail-fast with stable error codes, no warnings
- **Feature Gating**: Rayon parallelism behind `full` feature flag
- **Caching**: Lua module data pre-parsed and cached in Arc

### Data Flow
1. CLI parses arguments → creates `ConversionContext`
2. Load wiki export structure and Lua module data
3. For each entity: parse wikitext → expand templates → extract structured data → render Markdown
4. Template expansion pipeline: variables → expressions → formulas → cleanup → normalization

### External Dependencies
- Core: clap, serde, thiserror, tracing, regex, once_cell, hashbrown
- Optional: rayon (parallelism), criterion (benchmarking)
- Dev: proptest (property testing)

## Code Style Guidelines

### Language & Edition
- **Rust 2021 edition** with 2024 compatibility planned
- **MSRV**: 1.75+ (follows stable channel)
- **No unsafe code**: `#![deny(unsafe_code)]` at crate root

### Imports & Dependencies
- Group imports: `std`, external crates, then local modules
- Use `use` statements at module level, not function level
- Prefer explicit imports over glob imports (`*`)
- Use `serde::{Deserialize, Serialize}` for data structures
- Leverage `once_cell::sync::Lazy` for static initialization

### Naming Conventions
- **Types/Structs**: PascalCase (Champion, AbilityKey, TemplateRegistry)
- **Functions/Methods**: snake_case (convert_champion, evaluate_expression)
- **Variables**: snake_case (wiki_root, template_name)
- **Constants**: SCREAMING_SNAKE_CASE (MAX_REDIRECT_DEPTH)
- **Modules**: snake_case (parse, convert, render)

### Error Handling
- Use `Result<T, ConvertError>` for fallible operations
- Define error variants in `error.rs` with stable codes (E_UNKNOWN_TEMPLATE, E_EXPR, etc.)
- Prefer `?` operator for propagation
- No `unwrap()` or `expect()` in production code - always proper error handling
- Use `thiserror` for derive-based error types

### Type Safety & Collections
- Prefer `HashMap<K, V>` over raw vectors for key-value data
- Use `hashbrown::HashMap` for performance-critical maps
- Leverage Serde derive for (de)serializable structs
- Use strong typing: `AbilityKey` enum instead of strings for ability slots

### Logging & Tracing
- Use `tracing` crate with spans for operations
- Levels: ERROR (failures), WARN (deprecated), INFO (progress), DEBUG (details), TRACE (internals)
- JSON logging available via `--json-log` flag
- Include file/line numbers in debug builds

### Performance Considerations
- Avoid unnecessary allocations - use `Cow<str>` for string transformations
- Pre-compile regex patterns with lazy static
- Cache expensive operations (Lua parsing, template registry)
- Parallel processing with Rayon for batch operations

### Testing Patterns
- Unit tests for parsing primitives (expressions, brace scanning)
- Property tests with proptest for fuzzing (expression evaluation)
- Integration tests for end-to-end conversion (golden outputs)
- Golden master tests with blake3 hashing for determinism verification
- Template validation tests for coverage assurance

### Formatting & Style
- `cargo fmt` compliant (4-space indentation, 100-char lines)
- Clippy pedantic clean (warnings as errors in CI)
- Use `rustfmt.toml` for custom formatting rules if needed
- Document public APIs with rustdoc comments
- Prefer `///` for item docs, `//!` for module docs

### Memory Management
- Use `Arc` for shared immutable data (module caches)
- Prefer slices (`&str`) over owned strings where possible
- Avoid cloning large structures - pass references
- Use `write_if_changed` helper for deterministic file output
