use clap::Parser;
use lol_wiki_md::parse::templates::TemplateRegistry;
use lol_wiki_md::validate::validate_templates;
use lol_wiki_md::wiki_export::WikiExport;
use lol_wiki_md::{error::ConvertError, CliConfig, ConversionContext, ConversionOutcome};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Debug, Clone, Serialize)]
struct BatchFailure {
    name: String,
    error: String,
}

/// Per-entity result of a batch conversion.
enum BatchOutcome {
    Converted,
    Skipped,
    Failed(BatchFailure),
}

#[derive(Debug, Serialize)]
struct BatchReport {
    entity_type: String,
    total: usize,
    converted: usize,
    skipped: usize,
    failed: usize,
    failures: Vec<BatchFailure>,
}

fn init_tracing(json: bool) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    if json {
        let fmt_layer = fmt::layer()
            .json()
            .with_target(false)
            .with_file(true)
            .with_line_number(true);
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    } else {
        let fmt_layer = fmt::layer().with_target(false).with_level(true);
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt_layer)
            .init();
    }
}

fn main() -> Result<(), ConvertError> {
    let cfg = CliConfig::parse();
    init_tracing(cfg.json_log);
    if cfg.threads > 0 {
        #[cfg(feature = "rayon")]
        rayon::ThreadPoolBuilder::new()
            .num_threads(cfg.threads)
            .build_global()
            .ok();
    }
    std::fs::create_dir_all(&cfg.output)?;

    if cfg.validate {
        let report = validate_templates(&cfg.wiki_root, cfg.precision)?;
        let out = cfg.output.join("template_validation_report.json");
        std::fs::write(&out, serde_json::to_string_pretty(&report).unwrap())
            .map_err(ConvertError::Io)?;
        // Also write a compact summary with top unknowns by count and list of supported names
        let mut counts = std::collections::HashMap::<String, usize>::new();
        for i in &report.issues {
            if i.code == "E_UNKNOWN_TEMPLATE" {
                *counts.entry(i.name.clone()).or_insert(0) += 1;
            }
        }
        let mut pairs: Vec<(String, usize)> = counts.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        let reg = TemplateRegistry::new();
        let summary = serde_json::json!({
            "unknown_top": pairs,
            "supported_names": reg.list_names(),
            "total_issues": report.issues.len(),
        });
        let out2 = cfg.output.join("template_validation_summary.json");
        let _ = std::fs::write(&out2, serde_json::to_string_pretty(&summary).unwrap());
        println!(
            "Validation report written to {} ({} issues)",
            out.display(),
            report.issues.len()
        );
    }

    let ctx = ConversionContext::new(&cfg.wiki_root, cfg.precision)?;
    ctx.set_include_removed(cfg.include_removed);

    let conversion_result = if let Some(name) = &cfg.champion {
        let champion_output = category_output_dir(&cfg.output, "champions", false)?;
        ctx.convert_champion(&champion_output, name).map(|_| ())
    } else if cfg.all_champions {
        let export = WikiExport::new(&cfg.wiki_root);
        let names = export.list_champion_names()?;
        let champion_output = category_output_dir(&cfg.output, "champions", true)?;
        run_batch("champion", names, &cfg.output, |name| {
            ctx.convert_champion(&champion_output, name)
        })
    } else if let Some(item) = &cfg.item {
        let item_output = category_output_dir(&cfg.output, "items", false)?;
        ctx.convert_item(&item_output, item).map(|outcome| {
            if outcome.skipped {
                println!(
                    "Skipped removed item '{}'; pass --include-removed to convert it.",
                    item
                );
            }
        })
    } else if cfg.all_items {
        let export = WikiExport::new(&cfg.wiki_root);
        let names = export.list_item_names()?;
        let item_output = category_output_dir(&cfg.output, "items", true)?;
        run_batch("item", names, &cfg.output, |name| {
            ctx.convert_item(&item_output, name)
        })
    } else if let Some(rune) = &cfg.rune {
        let rune_output = category_output_dir(&cfg.output, "runes", false)?;
        ctx.convert_rune(&rune_output, rune).map(|_| ())
    } else if cfg.all_runes {
        let export = WikiExport::new(&cfg.wiki_root);
        let names = export.list_rune_names()?;
        let rune_output = category_output_dir(&cfg.output, "runes", true)?;
        run_batch("rune", names, &cfg.output, |name| {
            ctx.convert_rune(&rune_output, name)
        })
    } else {
        Ok(())
    };
    ctx.write_inventory_reports(&cfg.output)?;
    conversion_result
}

fn category_output_dir(
    output_root: &Path,
    category: &str,
    clean: bool,
) -> Result<PathBuf, ConvertError> {
    let dir = output_root.join(category);
    if clean && dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn run_batch<F>(
    entity_type: &str,
    names: Vec<String>,
    output_dir: &Path,
    convert_one: F,
) -> Result<(), ConvertError>
where
    F: Fn(&str) -> Result<ConversionOutcome, ConvertError> + Sync + Send,
{
    let total = names.len();
    // `ConversionContext` is internally `Arc`-shared and all of its mutable
    // state lives behind `Mutex`/`OnceCell`, so the conversion closure is safe
    // to invoke from multiple threads. Use Rayon when available to parallelize
    // across all champions / items / runes — that's the single largest win for
    // batch runs, since per-page conversion is CPU-bound (heavy wikitext
    // parsing + template expansion).
    let classify = |name: &String| match convert_one(name) {
        Ok(outcome) if outcome.skipped => BatchOutcome::Skipped,
        Ok(_) => BatchOutcome::Converted,
        Err(err) => BatchOutcome::Failed(BatchFailure {
            name: name.clone(),
            error: err.to_string(),
        }),
    };
    let results: Vec<BatchOutcome> = {
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            names.par_iter().map(classify).collect()
        }
        #[cfg(not(feature = "rayon"))]
        {
            names.iter().map(classify).collect()
        }
    };

    let mut converted = 0usize;
    let mut skipped = 0usize;
    let mut failures: Vec<BatchFailure> = Vec::new();
    for result in results {
        match result {
            BatchOutcome::Converted => converted += 1,
            BatchOutcome::Skipped => skipped += 1,
            BatchOutcome::Failed(failure) => failures.push(failure),
        }
    }

    write_batch_report(output_dir, entity_type, total, converted, skipped, &failures)?;

    if failures.is_empty() {
        if skipped > 0 {
            println!(
                "Converted all {}s successfully ({} written, {} skipped of {} total)",
                entity_type, converted, skipped, total
            );
        } else {
            println!(
                "Converted all {}s successfully ({} total)",
                entity_type, converted
            );
        }
        Ok(())
    } else {
        Err(ConvertError::Internal(format!(
            "{} {} conversions failed; see {}",
            failures.len(),
            entity_type,
            output_dir
                .join(format!("{}_conversion_report.json", entity_type))
                .display()
        )))
    }
}

fn write_batch_report(
    output_dir: &Path,
    entity_type: &str,
    total: usize,
    converted: usize,
    skipped: usize,
    failures: &[BatchFailure],
) -> Result<(), ConvertError> {
    let report = BatchReport {
        entity_type: entity_type.to_string(),
        total,
        converted,
        skipped,
        failed: failures.len(),
        failures: failures.to_vec(),
    };
    let path = output_dir.join(format!("{}_conversion_report.json", entity_type));
    let json = serde_json::to_string_pretty(&report).map_err(|err| {
        ConvertError::Internal(format!("failed to serialize batch report: {err}"))
    })?;
    std::fs::write(&path, json).map_err(ConvertError::Io)?;
    Ok(())
}
