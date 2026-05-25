use clap::{ArgGroup, Parser};
use lol_wiki_md::similarity::{
    audit_all_champion_outputs, audit_champion_output, ChampionSimilarityReport,
};
use lol_wiki_md::wiki_export::WikiExport;
use lol_wiki_md::ConvertError;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "audit_similarity",
    version,
    about = "Audit converted champion markdown against source-structure expectations"
)]
#[command(group(ArgGroup::new("scope").required(true).args(["champion", "all_champions"])))]
struct SimilarityCli {
    #[arg(long = "wiki-root", value_name = "PATH", default_value = "./out")]
    wiki_root: PathBuf,

    #[arg(long, value_name = "DIR", default_value = "./markdown_rust")]
    output: PathBuf,

    #[arg(long)]
    champion: Option<String>,

    #[arg(long = "all-champions")]
    all_champions: bool,

    #[arg(long, default_value_t = 0.85)]
    threshold: f32,

    #[arg(long, value_name = "FILE")]
    report: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct SimilarityBatchReport {
    basis: String,
    threshold: f32,
    total: usize,
    suspicious: usize,
    reports: Vec<ChampionSimilarityReport>,
}

fn main() -> Result<(), ConvertError> {
    let cli = SimilarityCli::parse();
    let export = WikiExport::new(&cli.wiki_root);

    let reports = if let Some(name) = &cli.champion {
        vec![audit_champion_output(
            &export,
            &cli.output,
            name,
            cli.threshold,
        )?]
    } else if cli.all_champions {
        audit_all_champion_outputs(&export, &cli.output, cli.threshold)?
    } else {
        Vec::new()
    };

    let suspicious = reports
        .iter()
        .filter(|report| report.needs_followup)
        .count();
    let batch = SimilarityBatchReport {
        basis: "local-export-structural-v1".to_string(),
        threshold: cli.threshold,
        total: reports.len(),
        suspicious,
        reports,
    };

    let report_path = cli
        .report
        .unwrap_or_else(|| cli.output.join("champion_similarity_report.json"));
    let json = serde_json::to_string_pretty(&batch).map_err(|err| {
        ConvertError::Internal(format!("failed to serialize similarity report: {err}"))
    })?;
    std::fs::create_dir_all(
        report_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new(".")),
    )
    .map_err(ConvertError::Io)?;
    std::fs::write(&report_path, json).map_err(ConvertError::Io)?;

    println!(
        "Similarity report written to {} ({} suspicious of {} checked)",
        report_path.display(),
        batch.suspicious,
        batch.total
    );

    if batch.suspicious > 0 {
        return Err(ConvertError::Internal(format!(
            "{} champion outputs scored below the threshold {}; see {}",
            batch.suspicious,
            batch.threshold,
            report_path.display()
        )));
    }

    Ok(())
}
