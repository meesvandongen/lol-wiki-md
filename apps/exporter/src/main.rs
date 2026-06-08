use anyhow::{Context, Result};
use clap::Parser;
use reqwest::Client;
use std::{fs, path::PathBuf};

mod args;
mod constants;
mod export;
mod metrics;
mod parse;
mod traversal;
mod util;

use args::Args;
use export::bulk_export;
use traversal::generate_page_list;
use util::prepare_output_dir;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("================ Export Configuration ================");
    println!(" Start Time (UTC): {}", chrono::Utc::now().to_rfc3339());
    println!(" Root Category   : {}", args.root_category);
    println!(" Max Depth       : {}", args.max_depth);
    println!(" Batch Size      : {}", args.batch_size);
    println!(" Concurrency     : {}", args.concurrency);
    println!(" Traversal Delay : {} ms", args.traversal_delay_ms);
    println!(" Batch Delay     : {} ms", args.batch_delay_ms);
    println!(
        " Output Dir      : {} (will be cleared if exists)",
        args.out_dir.display()
    );
    println!(
        " Meta Dir        : {} (separate, will be cleared if exists)",
        args.meta_dir
    );
    println!("======================================================\n");

    prepare_output_dir(&args.out_dir)?;

    let client = Client::builder()
        .user_agent("LoL Wiki Educational Bulk Scraper (rust) 1.0")
        .gzip(true)
        // Use the OS trust store (which honors SSL_CERT_FILE) rather than only
        // the bundled webpki Mozilla roots, so the exporter works behind
        // TLS-inspecting egress proxies whose CA lives in the system trust store.
        .tls_built_in_native_certs(true)
        .build()?;

    let meta_dir_path = PathBuf::from(&args.meta_dir);
    if meta_dir_path == args.out_dir {
        anyhow::bail!("meta_dir must differ from out_dir to avoid mixing page JSON with metadata");
    }
    if meta_dir_path.exists() {
        println!(
            "Clearing existing metadata dir: {}",
            meta_dir_path.display()
        );
        fs::remove_dir_all(&meta_dir_path).context("Failed clearing metadata directory")?;
    }
    fs::create_dir_all(&meta_dir_path).context("Failed creating metadata directory")?;

    let page_list_path = meta_dir_path.join("page_list.txt");
    let mut all_pages = if let Some(list_path) = &args.page_list {
        println!("Reading page list from {}", list_path.display());
        let contents = fs::read_to_string(list_path)
            .with_context(|| format!("Failed reading page list {}", list_path.display()))?;
        contents
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_string)
            .collect::<Vec<_>>()
    } else {
        println!(
            "Generating page list starting at {} (depth <= {})",
            args.root_category, args.max_depth
        );
        generate_page_list(
            &client,
            &args.root_category,
            args.max_depth,
            args.traversal_delay_ms,
        )
        .await?
    };
    println!("Discovered {} total unique pages", all_pages.len());
    fs::write(
        &page_list_path,
        all_pages
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
    )?;

    all_pages.sort();

    let metrics = bulk_export(&client, &all_pages, &args).await?;

    let metrics_path = meta_dir_path.join("metrics.json");
    fs::write(&metrics_path, serde_json::to_vec_pretty(&metrics)?)?;
    println!("\nMetrics summary written to {}", metrics_path.display());
    println!("Metadata directory: {}", meta_dir_path.display());
    println!(
        "Export summary: pages={} batches={} failed_batches={} bytes={} duration={:.2}s rate={:.2} pages/s",
        metrics.pages_exported,
        metrics.total_batches,
        metrics.failed_batches,
        metrics.bytes_downloaded,
        metrics.duration_secs,
        metrics.pages_per_sec
    );

    println!(
        "\nAll done. Exported pages are in {}",
        args.out_dir.display()
    );
    Ok(())
}
