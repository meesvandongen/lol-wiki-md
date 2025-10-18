use crate::error::Result;
use crate::parse::brace::extract_balanced_templates;
use crate::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use crate::wiki_export::WikiExport;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateValidationIssue {
    pub file: String,
    pub line: usize,
    pub col: usize,
    pub name: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateValidationReport {
    pub issues: Vec<TemplateValidationIssue>,
}

/// Validate all template invocations by expanding supported ones and flagging unknowns.
pub fn validate_templates(wiki_root: &Path, precision: u8) -> Result<TemplateValidationReport> {
    let export = WikiExport::new(wiki_root);
    let mut issues: Vec<TemplateValidationIssue> = Vec::new();
    let reg = TemplateRegistry::new();
    let mut unknown_counts: HashMap<String, usize> = HashMap::new();
    // iterate all .txt files under root
    let entries = std::fs::read_dir(&export.root)?;
    for ent in entries.flatten() {
        let path = ent.path();
        if path.extension().and_then(|e| e.to_str()) != Some("txt") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let Ok(spans) = extract_balanced_templates(&text) else {
            continue;
        };
        if spans.is_empty() {
            continue;
        }
        for sp in spans {
            let body = &sp.raw[2..sp.raw.len() - 2];
            let inv = parse_invocation(body);
            let ctx = ExpanderCtx {
                precision,
                vars: Default::default(),
                conversion_ctx: None,
            };
            if !reg.has_name(&inv.name) {
                issues.push(TemplateValidationIssue {
                    file: path.file_name().unwrap().to_string_lossy().into_owned(),
                    line: sp.line,
                    col: sp.col,
                    name: inv.name.clone(),
                    code: "E_UNKNOWN_TEMPLATE".into(),
                    message: format!("template '{}' not supported", inv.name),
                });
                *unknown_counts.entry(inv.name.clone()).or_insert(0) += 1;
                continue;
            }
            // Try expand to catch malformed params
            if let Err(e) = reg.expand(&inv, &ctx) {
                issues.push(TemplateValidationIssue {
                    file: path.file_name().unwrap().to_string_lossy().into_owned(),
                    line: sp.line,
                    col: sp.col,
                    name: inv.name.clone(),
                    code: format!("{e}"),
                    message: format!("failed to expand: {e}"),
                });
            }
        }
    }
    Ok(TemplateValidationReport { issues })
}
