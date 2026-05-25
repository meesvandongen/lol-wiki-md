use clap::Parser;
use lol_wiki_md::parse::templates::TemplateRegistry;
use lol_wiki_md::validate::validate_templates;
use lol_wiki_md::wiki_export::WikiExport;
use lol_wiki_md::{error::ConvertError, CliConfig, ConversionContext};
use serde::Serialize;
use std::path::Path;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Debug, Clone, Serialize)]
struct BatchFailure {
    name: String,
    error: String,
}

#[derive(Debug, Serialize)]
struct BatchReport {
    entity_type: String,
    total: usize,
    converted: usize,
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

    let conversion_result = if let Some(name) = &cfg.champion {
        ctx.convert_champion(&cfg.output, name).map(|_| ())
    } else if cfg.all_champions {
        let export = WikiExport::new(&cfg.wiki_root);
        let names = export.list_champion_names()?;
        run_batch("champion", names, &cfg.output, |name| {
            ctx.convert_champion(&cfg.output, name).map(|_| ())
        })
    } else if let Some(item) = &cfg.item {
        ctx.convert_item(&cfg.output, item).map(|_| ())
    } else if cfg.all_items {
        let export = WikiExport::new(&cfg.wiki_root);
        let names = export.list_item_names()?;
        run_batch("item", names, &cfg.output, |name| {
            ctx.convert_item(&cfg.output, name).map(|_| ())
        })
    } else if let Some(rune) = &cfg.rune {
        ctx.convert_rune(&cfg.output, rune).map(|_| ())
    } else if cfg.all_runes {
        let export = WikiExport::new(&cfg.wiki_root);
        let names = export.list_rune_names()?;
        run_batch("rune", names, &cfg.output, |name| {
            ctx.convert_rune(&cfg.output, name).map(|_| ())
        })
    } else {
        Ok(())
    };
    ctx.write_inventory_reports(&cfg.output)?;
    conversion_result
}

fn run_batch<F>(
    entity_type: &str,
    names: Vec<String>,
    output_dir: &Path,
    mut convert_one: F,
) -> Result<(), ConvertError>
where
    F: FnMut(&str) -> Result<(), ConvertError>,
{
    let total = names.len();
    let mut converted = 0usize;
    let mut failures: Vec<BatchFailure> = Vec::new();

    for name in names {
        match convert_one(&name) {
            Ok(()) => converted += 1,
            Err(err) => failures.push(BatchFailure {
                name,
                error: err.to_string(),
            }),
        }
    }

    write_batch_report(output_dir, entity_type, total, converted, &failures)?;

    if failures.is_empty() {
        println!(
            "Converted all {}s successfully ({} total)",
            entity_type, converted
        );
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
    failures: &[BatchFailure],
) -> Result<(), ConvertError> {
    let report = BatchReport {
        entity_type: entity_type.to_string(),
        total,
        converted,
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
