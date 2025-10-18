use crate::convert::util::{
    collect_list_items, collect_page_vars, expand_inline_templates, expand_with_vars,
    extract_patch_history, extract_section,
};
use crate::convert::{write_if_changed, ConversionOutcome};
use crate::error::{ConvertError, Result};
use crate::model::Rune;
use crate::parse::brace::TemplateSpan;
use crate::parse::extract_balanced_templates;
use crate::parse::templates::{parse_invocation, TemplateRegistry};
use crate::render::markdown::render_rune_markdown;
use std::collections::HashMap;
use std::path::Path;

use super::context::ConversionContext;

/// Convert a single rune page into Markdown.
pub(super) fn convert_rune(
    ctx: &ConversionContext,
    output_dir: &Path,
    name: &str,
) -> Result<ConversionOutcome> {
    let export = ctx.export();
    let raw = export.read_rune_main(name)?;
    let registry = ctx.registry();
    let precision = ctx.precision();
    let vars = collect_page_vars(&raw)?;
    if let Ok(spans) = extract_balanced_templates(&raw) {
        if !spans.is_empty() {
            let names = spans
                .into_iter()
                .map(|span| span.name.split('|').next().unwrap_or("").trim().to_string())
                .filter(|n| !n.is_empty())
                .collect::<Vec<_>>();
            ctx.record_templates(names);
        }
    }
    let expanded = expand_with_vars(&raw, precision, &vars, registry)?;
    if expanded.contains("{{") {
        return Err(ConvertError::Internal(
            "residual template marker '{{' after rune expansion".into(),
        ));
    }
    if expanded.to_ascii_lowercase().contains("<tabber>") {
        return Err(ConvertError::Internal(
            "residual <tabber> block after rune expansion".into(),
        ));
    }

    let info = parse_rune_infobox(&raw, precision, &vars, &registry)?;
    let description = info
        .as_ref()
        .and_then(|inf| inf.description.clone())
        .or_else(|| extract_first_paragraph(&expanded))
        .unwrap_or_default();
    let path = info.as_ref().and_then(|inf| inf.path.clone());
    let slot = info.as_ref().and_then(|inf| inf.slot.clone());

    let notes = extract_section(&expanded, "Notes")
        .map(|section| collect_list_items(&section))
        .unwrap_or_default();
    let trivia = ["Trivia", "Tips", "Tips and Tricks"]
        .iter()
        .find_map(|heading| {
            extract_section(&expanded, heading)
                .map(|section| collect_list_items(&section))
                .filter(|items| !items.is_empty())
        })
        .unwrap_or_default();
    let patch_history = extract_patch_history(&expanded);
    let warnings = validate_rune_metadata(path.as_deref(), slot.as_deref());

    let rune = Rune {
        name: name.to_string(),
        path,
        slot,
        description,
        notes,
        trivia,
        patch_history,
        warnings,
    };
    let markdown = render_rune_markdown(&rune, &expanded);
    let out_file = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    write_if_changed(&out_file, &markdown)?;
    Ok(ConversionOutcome {
        entity: name.to_string(),
        output: out_file,
    })
}

struct RuneInfobox {
    path: Option<String>,
    slot: Option<String>,
    description: Option<String>,
}

fn parse_rune_infobox(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<Option<RuneInfobox>> {
    let spans = extract_balanced_templates(raw)?;
    for span in spans {
        if !is_rune_infobox(&span) {
            continue;
        }
        let body = &span.raw[2..span.raw.len() - 2];
        let inv = parse_invocation(body);
        let mut named: HashMap<String, String> = HashMap::new();
        let mut positional: Vec<String> = Vec::new();
        for p in inv.params {
            if let Some(eq) = p.find('=') {
                let (k, v) = p.split_at(eq);
                let key = k.trim().to_ascii_lowercase();
                let value_raw = v[1..].trim();
                let expanded = expand_inline_templates(value_raw, precision, vars, registry)?;
                if !key.is_empty() {
                    named.insert(key, expanded.trim().to_string());
                }
            } else if !p.trim().is_empty() {
                let expanded = expand_inline_templates(p.trim(), precision, vars, registry)?;
                positional.push(expanded.trim().to_string());
            }
        }
        if named.is_empty() && positional.is_empty() {
            continue;
        }
        let path = named
            .get("path")
            .cloned()
            .or_else(|| positional.first().cloned());
        let slot = named
            .get("slot")
            .cloned()
            .or_else(|| positional.get(1).cloned());
        let description = named
            .get("description")
            .cloned()
            .or_else(|| positional.get(2).cloned())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        return Ok(Some(RuneInfobox {
            path: path.filter(|s| !s.is_empty()),
            slot: slot.filter(|s| !s.is_empty()),
            description,
        }));
    }
    Ok(None)
}

fn is_rune_infobox(span: &TemplateSpan) -> bool {
    let name_lower = span.name.to_ascii_lowercase();
    if name_lower.contains("rune info") || name_lower.contains("rune box") {
        return true;
    }
    if name_lower == "rune" || name_lower.starts_with("rune ") {
        return true;
    }
    if name_lower.contains("keystone") {
        return true;
    }
    false
}

fn extract_first_paragraph(expanded: &str) -> Option<String> {
    let mut paragraph: Vec<String> = Vec::new();
    for line in expanded.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !paragraph.is_empty() {
                break;
            }
            continue;
        }
        if trimmed.starts_with("==") {
            if !paragraph.is_empty() {
                break;
            } else {
                continue;
            }
        }
        paragraph.push(trimmed.to_string());
        if trimmed.ends_with('.') {
            break;
        }
    }
    if paragraph.is_empty() {
        None
    } else {
        Some(paragraph.join(" "))
    }
}

fn validate_rune_metadata(path: Option<&str>, slot: Option<&str>) -> Vec<String> {
    let mut warnings = Vec::new();
    if path.map(|p| p.trim().is_empty()).unwrap_or(true) {
        warnings.push("Rune path is missing".to_string());
    }
    if slot.map(|s| s.trim().is_empty()).unwrap_or(true) {
        warnings.push("Rune slot is missing".to_string());
    }
    warnings
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert::util::parse_patch_line;

    fn registry() -> TemplateRegistry {
        TemplateRegistry::new()
    }

    #[test]
    fn parse_rune_infobox_named_and_positional() {
        let raw = "{{Rune info| path = Domination | slot = Keystone | description = Burst damage}}";
        let info = parse_rune_infobox(raw, 2, &HashMap::new(), &registry())
            .unwrap()
            .unwrap();
        assert_eq!(info.path.as_deref(), Some("Domination"));
        assert_eq!(info.slot.as_deref(), Some("Keystone"));
        assert_eq!(info.description.as_deref(), Some("Burst damage"));

        let raw2 = "{{Rune|Precision|Keystone|Press the Attack}}";
        let info2 = parse_rune_infobox(raw2, 2, &HashMap::new(), &registry())
            .unwrap()
            .unwrap();
        assert_eq!(info2.path.as_deref(), Some("Precision"));
        assert_eq!(info2.slot.as_deref(), Some("Keystone"));
        assert_eq!(info2.description.as_deref(), Some("Press the Attack"));
    }

    #[test]
    fn extract_section_and_lists() {
        let expanded = "Intro text\n\n== Notes ==\n* First line\n** Sub line continues\nMore details\n\n== Trivia ==\n* Fun fact\n\n== Patch History ==\n* '''V14.5:''' Buffed damage\n** Extra detail\n* '''V14.4:'''\n** Secondary adjustment\n";
        let section = extract_section(expanded, "Notes").unwrap();
        let items = collect_list_items(&section);
        assert_eq!(items.len(), 2);
        assert!(items[0].starts_with("*"));
        assert!(items[1].starts_with("**"));
        let trivia = extract_section(expanded, "Trivia").unwrap();
        let trivia_items = collect_list_items(&trivia);
        assert_eq!(trivia_items.len(), 1);
        assert!(trivia_items[0].starts_with("*"));
        let history = extract_patch_history(expanded);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].version, "V14.5");
        assert_eq!(history[0].changes.len(), 2);
        assert_eq!(history[0].changes[0].text, "Buffed damage");
        assert_eq!(history[0].changes[1].text, "Extra detail");
        assert_eq!(history[1].version, "V14.4");
        assert_eq!(history[1].changes.len(), 1);
        assert_eq!(history[1].changes[0].text, "Secondary adjustment");
    }

    #[test]
    fn trivia_heading_fallback() {
        let expanded = "== Tips ==\n* Helpful hint\n";
        let trivia = ["Trivia", "Tips", "Tips and Tricks"]
            .iter()
            .find_map(|heading| {
                extract_section(expanded, heading)
                    .map(|section| collect_list_items(&section))
                    .filter(|items| !items.is_empty())
            });
        let items = trivia.expect("expected trivia items");
        assert_eq!(items.len(), 1);
        assert!(items[0].contains("Helpful hint"));
    }

    #[test]
    fn parse_patch_line_variants() {
        let (v1, t1) = parse_patch_line("'''V10.1:''' Updated numbers");
        assert_eq!(v1, "V10.1");
        assert_eq!(t1, "Updated numbers");
        let (v2, t2) = parse_patch_line("V9.9: Adjusted scaling");
        assert_eq!(v2, "V9.9");
        assert_eq!(t2, "Adjusted scaling");
        let (v3, t3) = parse_patch_line("General tweaks");
        assert_eq!(v3, "Patch");
        assert_eq!(t3, "General tweaks");
    }
}
