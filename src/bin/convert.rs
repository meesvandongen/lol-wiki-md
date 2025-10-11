use clap::Parser;
use lol_wiki_md::parse::templates::TemplateRegistry;
use lol_wiki_md::validate::validate_templates;
use lol_wiki_md::wiki_export::WikiExport;
use lol_wiki_md::{convert_champion, convert_item, error::ConvertError, CliConfig};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

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

    if let Some(name) = &cfg.champion {
        convert_champion(&cfg.wiki_root, &cfg.output, name, cfg.precision)?;
    } else if cfg.all_champions {
        // enumerate via WikiExport abstraction
        let export = WikiExport::new(&cfg.wiki_root);
        let names = export.list_champion_names()?;
        #[cfg(feature = "rayon")]
        {
            use rayon::prelude::*;
            names.par_iter().for_each(|name| {
                let _ = convert_champion(&cfg.wiki_root, &cfg.output, name, cfg.precision);
            });
        }
        #[cfg(not(feature = "rayon"))]
        {
            for name in names {
                let _ = convert_champion(&cfg.wiki_root, &cfg.output, &name, cfg.precision);
            }
        }
    } else if let Some(item) = &cfg.item {
        convert_item(&cfg.wiki_root, &cfg.output, item, cfg.precision)?;
    } else if let Some(rune) = &cfg.rune {
        return Err(ConvertError::Internal(format!(
            "rune conversion not yet implemented: {rune}"
        )));
    }
    Ok(())
}
