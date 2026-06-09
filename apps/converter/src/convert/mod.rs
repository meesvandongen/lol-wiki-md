pub mod champion;
pub mod context;
pub mod item;
pub mod rune;
pub mod util;

use crate::error::{ConvertError, Result};
use crate::render::plain_text::markdown_to_plain_text;
use std::path::{Path, PathBuf};

pub use context::ConversionContext;

/// Outcome metadata (could be extended with timing, counts, etc.)
#[derive(Debug, Clone)]
pub struct ConversionOutcome {
    pub entity: String,
    pub output: PathBuf,
    /// True when the entity was intentionally not written (e.g. a removed item
    /// excluded by default). No output file exists at `output` in that case.
    pub skipped: bool,
}

/// High-level convenience helpers that mirror the legacy conversion entry points.
/// These instantiate a new [`ConversionContext`] for single conversions. For
/// batch workloads prefer constructing a context once and invoking the
/// `convert_*` methods on it to take advantage of caching.
pub fn convert_champion(
    wiki_root: &Path,
    output_dir: &Path,
    name: &str,
    precision: u8,
) -> Result<ConversionOutcome> {
    let ctx = ConversionContext::new(wiki_root, precision)?;
    ctx.convert_champion(output_dir, name)
}

pub fn convert_item(
    wiki_root: &Path,
    output_dir: &Path,
    name: &str,
    precision: u8,
) -> Result<ConversionOutcome> {
    let ctx = ConversionContext::new(wiki_root, precision)?;
    ctx.convert_item(output_dir, name)
}

pub fn convert_rune(
    wiki_root: &Path,
    output_dir: &Path,
    name: &str,
    precision: u8,
) -> Result<ConversionOutcome> {
    let ctx = ConversionContext::new(wiki_root, precision)?;
    ctx.convert_rune(output_dir, name)
}

/// Utility: ensure path exists else error variant.
pub fn require_exists(path: &Path, err: ConvertError) -> Result<()> {
    if path.exists() {
        Ok(())
    } else {
        Err(err)
    }
}

/// Write helper with idempotent short-circuit (avoid rewriting identical content).
pub fn write_if_changed(path: &Path, content: &str) -> Result<()> {
    if let Ok(existing) = std::fs::read_to_string(path) {
        if existing == content {
            return Ok(());
        }
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

pub fn write_markdown_with_plain_text(path: &Path, markdown: &str) -> Result<()> {
    write_if_changed(path, markdown)?;
    let plain_path = path.with_extension("plain.txt");
    let plain_text = markdown_to_plain_text(markdown);
    write_if_changed(&plain_path, &plain_text)
}
