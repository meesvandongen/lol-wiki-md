use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn prepare_output_dir(out_dir: &PathBuf) -> Result<()> {
    if out_dir.exists() {
        println!("Clearing existing output dir: {}", out_dir.display());
        fs::remove_dir_all(out_dir).context("Failed clearing output directory")?;
    }
    fs::create_dir_all(out_dir).context("Failed creating output directory")?;
    Ok(())
}
