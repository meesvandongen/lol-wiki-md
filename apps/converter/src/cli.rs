use clap::{ArgGroup, Parser};
use std::path::PathBuf;

/// Command line configuration (strict only mode).
#[derive(Parser, Debug)]
#[command(name = "lol_wiki_md", version, about = "LoL Wiki dump -> Markdown strict converter", long_about = None)]
#[command(group(ArgGroup::new("entity")
    .required(true)
    .args(["champion", "item", "rune", "all_champions", "all_items", "all_runes"])))]
pub struct CliConfig {
    /// Root directory of exploded wiki dump (e.g. ./out)
    #[arg(long = "wiki-root", value_name = "PATH", default_value = "./out")]
    pub wiki_root: PathBuf,

    /// Output directory for Markdown artifacts
    #[arg(long, value_name = "DIR", default_value = "./markdown_rust")]
    pub output: PathBuf,

    /// Specific champion to convert
    #[arg(long)]
    pub champion: Option<String>,

    /// Specific item to convert
    #[arg(long)]
    pub item: Option<String>,

    /// Specific rune to convert
    #[arg(long)]
    pub rune: Option<String>,

    /// Convert all champions (future: items/runes batch)
    #[arg(long = "all-champions")]
    pub all_champions: bool,

    /// Convert all items
    #[arg(long = "all-items")]
    pub all_items: bool,

    /// Convert all runes
    #[arg(long = "all-runes")]
    pub all_runes: bool,

    /// JSON structured logging instead of pretty text
    #[arg(long, default_value_t = false)]
    pub json_log: bool,

    /// Maximum floating precision for evaluated expressions
    #[arg(long, default_value_t = 2)]
    pub precision: u8,

    /// Parallelism override (threads); 0 = rayon default
    #[arg(long, default_value_t = 0)]
    pub threads: usize,

    /// Validate templates across the wiki export and write a JSON report
    #[arg(long, default_value_t = false)]
    pub validate: bool,
}
