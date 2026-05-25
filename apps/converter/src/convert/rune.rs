use crate::convert::util::{
    collect_list_items, collect_page_vars, expand_inline_templates, expand_with_vars,
    extract_patch_history, extract_section,
};
use crate::convert::{write_markdown_with_plain_text, ConversionOutcome};
use crate::error::{ConvertError, Result};
use crate::model::Rune;
use crate::parse::brace::TemplateSpan;
use crate::parse::extract_balanced_templates;
use crate::parse::templates::{parse_invocation, TemplateRegistry};
use crate::render::markdown::render_rune_markdown;
use crate::wiki_export::WikiExport;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

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
    let conversion_ctx = Some(Arc::new(ctx.clone()));
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
    let expanded = expand_with_vars(&raw, precision, &vars, registry, conversion_ctx.clone())?;
    let visible_expanded = strip_non_marker_html_comments(&expanded);
    if visible_expanded.contains("{{") {
        return Err(ConvertError::Internal(
            "residual template marker '{{' after rune expansion".into(),
        ));
    }

    let info = parse_rune_infobox(&raw, precision, &vars, &registry, conversion_ctx.clone())?;
    let rune_data = load_rune_data(
        export,
        name,
        precision,
        &vars,
        &registry,
        conversion_ctx.clone(),
    )?;
    let description = info
        .as_ref()
        .and_then(|inf| inf.description.clone())
        .or_else(|| rune_data.as_ref().and_then(|data| data.description.clone()))
        .or_else(|| extract_first_paragraph(&visible_expanded))
        .unwrap_or_default();
    let path = info
        .as_ref()
        .and_then(|inf| inf.path.clone())
        .or_else(|| rune_data.as_ref().and_then(|data| data.path.clone()));
    let slot = info
        .as_ref()
        .and_then(|inf| inf.slot.clone())
        .or_else(|| rune_data.as_ref().and_then(|data| data.slot.clone()));
    let caption = rune_data.as_ref().and_then(|data| data.caption.clone());

    let notes = extract_section(&visible_expanded, "Notes")
        .map(|section| collect_list_items(&section))
        .unwrap_or_default();
    let map_changes = ["Map-Specific Differences", "Mode-Specific Changes"]
        .iter()
        .find_map(|heading| {
            extract_section(&visible_expanded, heading)
                .map(|section| section.trim().to_string())
                .filter(|section| !section.is_empty())
        });
    let trivia = ["Trivia", "Tips", "Tips and Tricks"]
        .iter()
        .find_map(|heading| {
            extract_section(&visible_expanded, heading)
                .map(|section| collect_list_items(&section))
                .filter(|items| !items.is_empty())
        })
        .unwrap_or_default();
    let patch_history = extract_patch_history(&visible_expanded);
    let warnings = validate_rune_metadata(path.as_deref(), slot.as_deref());

    let rune = Rune {
        name: name.to_string(),
        path,
        slot,
        description,
        caption,
        map_changes,
        notes,
        trivia,
        patch_history,
        source_appendices: Vec::new(),
        warnings,
    };
    let markdown = render_rune_markdown(&rune, &visible_expanded);
    let out_file = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    write_markdown_with_plain_text(&out_file, &markdown)?;
    if let Ok(rel) = out_file.strip_prefix(output_dir) {
        let artifact = rel.to_string_lossy().replace('\\', "/");
        ctx.set_specimen_sample("rune", &artifact);
    } else {
        let artifact = out_file.to_string_lossy().replace('\\', "/");
        ctx.set_specimen_sample("rune", &artifact);
    }
    Ok(ConversionOutcome {
        entity: name.to_string(),
        output: out_file,
    })
}

fn strip_non_marker_html_comments(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut cursor = 0usize;
    let mut i = 0usize;

    while i + 3 < bytes.len() {
        if bytes[i] == b'<' && bytes[i + 1] == b'!' && bytes[i + 2] == b'-' && bytes[i + 3] == b'-'
        {
            out.push_str(&text[cursor..i]);
            let comment_start = i;
            i += 4;
            while i + 2 < bytes.len() {
                if bytes[i] == b'-' && bytes[i + 1] == b'-' && bytes[i + 2] == b'>' {
                    let comment_end = i + 3;
                    let comment_body = text[comment_start + 4..i].trim_start();
                    if comment_body.starts_with("UNHANDLED TEMPLATE") {
                        out.push_str(&text[comment_start..comment_end]);
                    }
                    cursor = comment_end;
                    i = comment_end;
                    break;
                }
                i += 1;
            }
            if cursor == comment_start {
                cursor = text.len();
                break;
            }
            continue;
        }
        i += 1;
    }

    if cursor < text.len() {
        out.push_str(&text[cursor..]);
    }

    out
}

struct RuneInfobox {
    path: Option<String>,
    slot: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Default)]
struct RuneData {
    path: Option<String>,
    slot: Option<String>,
    description: Option<String>,
    caption: Option<String>,
}

fn load_rune_data(
    export: &WikiExport,
    name: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<Option<RuneData>> {
    let title = format!("Template:Rune data {name}");
    let Some(raw) = export.read_template_page(&title)? else {
        return Ok(None);
    };
    let Some(named) = read_template_named_params(&raw) else {
        return Ok(None);
    };

    let path = expand_template_named_value(
        &named,
        "path",
        precision,
        vars,
        registry,
        conversion_ctx.clone(),
    )?;
    let slot = expand_template_named_value(
        &named,
        "slot",
        precision,
        vars,
        registry,
        conversion_ctx.clone(),
    )?;
    let caption = expand_template_named_value(
        &named,
        "caption",
        precision,
        vars,
        registry,
        conversion_ctx.clone(),
    )?;

    let mut description_parts = Vec::new();
    for key in numbered_template_value_keys("description") {
        if let Some(value) = expand_template_named_value(
            &named,
            &key,
            precision,
            vars,
            registry,
            conversion_ctx.clone(),
        )? {
            description_parts.push(value);
        }
    }
    if let Some(cooldown) = expand_template_named_value(
        &named,
        "cooldown",
        precision,
        vars,
        registry,
        conversion_ctx.clone(),
    )? {
        description_parts.push(format!("Cooldown: {cooldown} seconds."));
    }
    if let Some(range) =
        expand_template_named_value(&named, "range", precision, vars, registry, conversion_ctx)?
    {
        description_parts.push(format!("Range: {range}."));
    }

    Ok(Some(RuneData {
        path,
        slot,
        description: if description_parts.is_empty() {
            None
        } else {
            Some(description_parts.join("\n\n"))
        },
        caption,
    }))
}

fn read_template_named_params(raw: &str) -> Option<HashMap<String, String>> {
    let span = extract_balanced_templates(raw).ok()?.into_iter().next()?;
    let body = &span.raw[2..span.raw.len() - 2];
    let inv = parse_invocation(body);
    let mut named = HashMap::new();
    for param in inv.params {
        let Some(eq) = param.find('=') else {
            continue;
        };
        let key = param[..eq].trim().to_ascii_lowercase();
        let value = param[eq + 1..].trim();
        if key.is_empty() || value.is_empty() {
            continue;
        }
        named.insert(key, value.to_string());
    }
    Some(named)
}

fn expand_template_named_value(
    named: &HashMap<String, String>,
    key: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<Option<String>> {
    let Some(raw) = named.get(&key.to_ascii_lowercase()) else {
        return Ok(None);
    };
    let expanded = expand_inline_templates(raw, precision, vars, registry, conversion_ctx)?;
    let trimmed = expanded.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

fn numbered_template_value_keys(base: &str) -> Vec<String> {
    let mut keys = vec![base.to_string()];
    for idx in 2..=8 {
        keys.push(format!("{base}{idx}"));
    }
    keys
}

fn parse_rune_infobox(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
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
                let expanded = expand_inline_templates(
                    value_raw,
                    precision,
                    vars,
                    registry,
                    conversion_ctx.clone(),
                )?;
                if !key.is_empty() {
                    named.insert(key, expanded.trim().to_string());
                }
            } else if !p.trim().is_empty() {
                let expanded = expand_inline_templates(
                    p.trim(),
                    precision,
                    vars,
                    registry,
                    conversion_ctx.clone(),
                )?;
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
    matches!(
        span.name.trim().to_ascii_lowercase().as_str(),
        "rune" | "rune info" | "rune box" | "rune infobox"
    )
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
        if trimmed.starts_with("<!--") || trimmed.starts_with("[Unhandled template:") {
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
        let info = parse_rune_infobox(raw, 2, &HashMap::new(), &registry(), None)
            .unwrap()
            .unwrap();
        assert_eq!(info.path.as_deref(), Some("Domination"));
        assert_eq!(info.slot.as_deref(), Some("Keystone"));
        assert_eq!(info.description.as_deref(), Some("Burst damage"));

        let raw2 = "{{Rune|Precision|Keystone|Press the Attack}}";
        let info2 = parse_rune_infobox(raw2, 2, &HashMap::new(), &registry(), None)
            .unwrap()
            .unwrap();
        assert_eq!(info2.path.as_deref(), Some("Precision"));
        assert_eq!(info2.slot.as_deref(), Some("Keystone"));
        assert_eq!(info2.description.as_deref(), Some("Press the Attack"));
    }

    #[test]
    fn rune_header_is_not_treated_as_infobox() {
        let raw = "{{rune header|Electrocute}}";
        let info = parse_rune_infobox(raw, 2, &HashMap::new(), &registry(), None).unwrap();
        assert!(info.is_none());
    }

    #[test]
    fn load_rune_data_reads_metadata_and_caption() {
        let td = tempfile::tempdir().unwrap();
        let export_dir = td.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        std::fs::write(
            export_dir.join("Template%3ARune%20data%20Electrocute.txt"),
            concat!(
                "{{{{{1|Rune data}}}|Electrocute|{{{2|}}}|\n",
                "|path=Domination\n",
                "|slot=Keystone\n",
                "|description={{sbc|Passive:}} Deals damage.\n",
                "|description2=Second paragraph.\n",
                "|cooldown=20\n",
                "|caption={{quote|Storm quote|Rune caption}}\n",
                "}}"
            ),
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let ctx = Arc::new(ConversionContext::new(td.path(), 2).unwrap());
        let data = load_rune_data(
            &export,
            "Electrocute",
            2,
            &HashMap::new(),
            &registry(),
            Some(ctx),
        )
        .unwrap()
        .unwrap();

        assert_eq!(data.path.as_deref(), Some("Domination"));
        assert_eq!(data.slot.as_deref(), Some("Keystone"));
        assert!(data
            .description
            .as_deref()
            .unwrap()
            .contains("**PASSIVE:** Deals damage."));
        assert!(data
            .description
            .as_deref()
            .unwrap()
            .contains("Cooldown: 20 seconds."));
        assert!(data.caption.as_deref().unwrap().contains("Storm quote"));
    }

    #[test]
    fn convert_rune_uses_rune_data_and_map_changes() {
        let td = tempfile::tempdir().unwrap();
        let export_dir = td.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();

        std::fs::write(
            export_dir.join("Electrocute.txt"),
            concat!(
                "{{rune header|Electrocute}}\n\n",
                "== Map-Specific Differences ==\n",
                "{{map changes|Electrocute}}\n\n",
                "== Trivia ==\n",
                "{{Rune data Electrocute|pst2|caption}}\n",
                "* Extra fact\n"
            ),
        )
        .unwrap();
        std::fs::write(
            export_dir.join("Template%3ARune%20data%20Electrocute.txt"),
            concat!(
                "{{{{{1|Rune data}}}|Electrocute|{{{2|}}}|\n",
                "|path=Domination\n",
                "|slot=Keystone\n",
                "|description={{sbc|Passive:}} Deals damage.\n",
                "|caption={{quote|Storm quote|Rune caption}}\n",
                "}}"
            ),
        )
        .unwrap();
        std::fs::write(
            export_dir.join("Template%3AMap%20changes%2Fdata%2Faram.txt"),
            concat!(
                "{{{{{1}}}|aram|{{{2|}}}|\n",
                "|Electrocute =\n",
                "* Cooldown changed to 10 seconds.\n",
                "}}"
            ),
        )
        .unwrap();
        std::fs::write(
            export_dir.join("Template%3AMap%20changes%2Fdata%2Far.txt"),
            concat!(
                "{{{{{1}}}|ar|{{{2|}}}|\n",
                "|Electrocute =\n",
                "* _Obtained from the Combo Master augment._\n",
                "** Damage changed to 60 – 300 (based on level).\n",
                "** Bonus AD ratio changed to 55% '''bonus''' AD.\n",
                "}}"
            ),
        )
        .unwrap();

        let ctx = ConversionContext::new(td.path(), 2).unwrap();
        let out_dir = td.path().join("out");
        convert_rune(&ctx, &out_dir, "Electrocute").unwrap();
        let markdown = std::fs::read_to_string(out_dir.join("Electrocute.md")).unwrap();
        let rendered_body = markdown
            .split("<!-- Raw excerpt (first 20 lines) -->")
            .next()
            .unwrap_or(&markdown);

        assert!(markdown.contains("- **Path:** Domination"));
        assert!(markdown.contains("- **Slot:** Keystone"));
        assert!(markdown.contains("**PASSIVE:** Deals damage."));
        assert!(markdown.contains("## Map-Specific Differences"));
        assert!(markdown.contains("### Howling Abyss"));
        assert!(markdown.contains("Cooldown changed to 10 seconds."));
        assert!(markdown.contains("### Arena"));
        assert!(markdown.contains("- _Obtained from the Combo Master augment._"));
        assert!(markdown.contains("  - Damage changed to 60 – 300 (based on level)."));
        assert!(markdown.contains("  - Bonus AD ratio changed to 55% **bonus** AD."));
        assert!(markdown.contains("Storm quote"));
        assert!(!markdown.contains("[Unhandled template: rune header]"));
        assert!(!markdown.contains("Rune slot is missing"));
        assert!(!rendered_body.contains("\n** Damage changed"));
    }

    #[test]
    fn convert_rune_ignores_plain_comments_and_flattens_item_var_tabbers() {
        let td = tempfile::tempdir().unwrap();
        let export_dir = td.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();

        std::fs::write(
            export_dir.join("Perfect Timing.txt"),
            concat!(
                "{{rune header|Perfect Timing}}\n\n",
                "<!--\n",
                "== Map-Specific Differences ==\n",
                "{{map changes|Perfect Timing}}\n",
                "-->\n\n",
                "== Perfectly Timed Stopwatch ==\n",
                "<tabber>Perfectly Timed=\n",
                "{{Item info/var|Perfectly Timed Stopwatch|automaticgv=false}}\n",
                "|-|Commencing=\n",
                "{{Item info/var|Commencing Stopwatch|automaticgv=false}}\n",
                "</tabber>\n\n",
                "== Notes ==\n",
                "* {{g|40}} sell value.\n",
                "{{Rune footer}}\n"
            ),
        )
        .unwrap();

        let ctx = ConversionContext::new(td.path(), 2).unwrap();
        let out_dir = td.path().join("out");
        convert_rune(&ctx, &out_dir, "Perfect Timing").unwrap();
        let markdown = std::fs::read_to_string(out_dir.join("Perfect_Timing.md")).unwrap();
        let rendered_body = markdown
            .split("<!-- Raw excerpt (first 20 lines) -->")
            .next()
            .unwrap_or(&markdown);

        assert!(rendered_body.contains("Perfectly Timed Stopwatch"));
        assert!(rendered_body.contains("Commencing Stopwatch"));
        assert!(!rendered_body.contains("{{Item info/var"));
        assert!(!rendered_body.contains("{{map changes|Perfect Timing}}"));
        assert!(!rendered_body.contains("## Map-Specific Differences"));
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
        assert_eq!(history[0].changes[0].section, "General");
        assert_eq!(history[0].changes[0].text, "Buffed damage");
        assert_eq!(history[0].changes[1].section, "General");
        assert_eq!(history[0].changes[1].text, "Extra detail");
        assert_eq!(history[1].version, "V14.4");
        assert_eq!(history[1].changes.len(), 1);
        assert_eq!(history[1].changes[0].section, "General");
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
