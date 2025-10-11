pub mod champion;
pub mod item;
pub mod rune;
pub mod util;

use crate::error::{ConvertError, Result};
use std::path::{Path, PathBuf};

pub use champion::convert_champion;
pub use item::convert_item;

/// Outcome metadata (could be extended with timing, counts, etc.)
#[derive(Debug, Clone)]
pub struct ConversionOutcome {
    pub entity: String,
    pub output: PathBuf,
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
