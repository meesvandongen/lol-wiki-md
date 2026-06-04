use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "lol-wiki-export",
    about = "LoL Wiki end-to-end exporter (page list + bulk export)"
)]
pub struct Args {
    #[arg(long, default_value = "Category:Browse")]
    pub root_category: String,
    /// Optional newline-delimited file of page titles to export directly,
    /// bypassing category traversal. Transcluded templates/modules are still
    /// pulled in via `templates=1`, so listing the top-level pages (champions,
    /// items) is enough to get a self-contained, convertible subset.
    #[arg(long)]
    pub page_list: Option<PathBuf>,
    #[arg(long, default_value_t = 5)]
    pub max_depth: usize,
    #[arg(long, default_value = "export_out")]
    pub out_dir: PathBuf,
    #[arg(long, default_value_t = 200)]
    pub batch_size: usize,
    #[arg(long, default_value_t = 50)]
    pub traversal_delay_ms: u64,
    #[arg(long, default_value_t = 400)]
    pub batch_delay_ms: u64,
    #[arg(long, default_value_t = 3)]
    pub concurrency: usize,
    #[arg(long, default_value = "meta")]
    pub meta_dir: String,
}
