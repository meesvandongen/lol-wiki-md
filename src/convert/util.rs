use crate::convert::context::ConversionContext;
use crate::error::Result;
use crate::model::{Change, PatchEntry};
use crate::parse::extract_balanced_templates;
use crate::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use std::collections::HashMap;
use std::sync::Arc;

/// Collect `#vardefine` assignments from the provided raw wikitext into a map.
pub fn collect_page_vars(raw: &str) -> Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    if let Ok(spans) = extract_balanced_templates(raw) {
        for span in spans {
            let body = &span.raw[2..span.raw.len() - 2];
            let inv = parse_invocation(body);
            if inv.name.eq_ignore_ascii_case("#vardefine") && inv.params.len() >= 2 {
                vars.insert(inv.params[0].clone(), inv.params[1].clone());
            }
        }
    }
    Ok(vars)
}

/// Expand all top-level templates in `raw` using the provided registry and variable map.
pub fn expand_with_vars(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<String> {
    expand_inline_templates(raw, precision, vars, registry, conversion_ctx)
}

/// Iteratively expand inline templates inside a value until a fixed point or iteration cap is reached.
pub fn expand_inline_templates(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<String> {
    let mut curr = raw.to_string();
    let ctx = ExpanderCtx {
        precision,
        vars: vars.clone(),
        conversion_ctx,
    };
    for _ in 0..6 {
        let Ok(spans) = extract_balanced_templates(&curr) else {
            break;
        };
        if spans.is_empty() {
            break;
        }
        let mut out = String::new();
        let mut last = 0usize;
        for sp in spans {
            out.push_str(&curr[last..sp.start]);
            let body = &sp.raw[2..sp.raw.len() - 2];
            let inv = parse_invocation(body);
            let exp = registry.expand(&inv, &ctx)?;
            out.push_str(&exp.expanded);
            last = sp.end;
        }
        out.push_str(&curr[last..]);
        if out == curr {
            break;
        }
        curr = out;
        if !curr.contains("{{") {
            break;
        }
    }
    Ok(curr)
}

/// Extract the body of a second-level heading from expanded wikitext.
pub fn extract_section(expanded: &str, heading: &str) -> Option<String> {
    let target = heading.to_ascii_lowercase();
    let mut in_section = false;
    let mut body: Vec<&str> = Vec::new();
    for line in expanded.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("==") {
            let heading_text = trimmed.trim_matches('=').trim().to_ascii_lowercase();
            if heading_text == target {
                in_section = true;
                continue;
            }
            if in_section {
                break;
            }
        }
        if in_section {
            body.push(line);
        }
    }
    if body.is_empty() {
        None
    } else {
        Some(body.join("\n").trim().to_string())
    }
}

/// Collect list items within a section, preserving nesting markers (`*`, `#`).
pub fn collect_list_items(section: &str) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    let mut current: Option<String> = None;
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('*') || trimmed.starts_with('#') {
            if let Some(existing) = current.take() {
                items.push(existing);
            }
            let normalized = normalize_list_marker(trimmed);
            current = Some(normalized);
        } else if let Some(existing) = current.as_mut() {
            existing.push(' ');
            existing.push_str(trimmed);
        }
    }
    if let Some(existing) = current {
        items.push(existing);
    }
    items
}

fn normalize_list_marker(line: &str) -> String {
    let mut prefix = String::new();
    for ch in line.chars() {
        if matches!(ch, '*' | '#') {
            prefix.push('*');
        } else {
            break;
        }
    }
    let remainder = line.trim_start_matches(['*', '#']).trim_start();
    if prefix.is_empty() {
        line.to_string()
    } else if remainder.is_empty() {
        prefix
    } else {
        format!("{} {}", prefix, remainder)
    }
}

/// Parse a "Patch history" section into structured entries.
pub fn extract_patch_history(expanded: &str) -> Vec<PatchEntry> {
    const MAX_PATCH_ENTRIES: usize = 10;
    let Some(section) = extract_section(expanded, "Patch history") else {
        return Vec::new();
    };

    let mut entries: Vec<PatchEntry> = Vec::new();
    let mut current_entry: Option<PatchEntry> = None;
    let mut current_section: Option<String> = None;
    let mut current_change_idx: Option<usize> = None;
    let mut reached_limit = false;

    for line in section.lines() {
        if reached_limit {
            break;
        }
        let mut trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        trimmed = trimmed
            .trim_start_matches(|c| c == ':' || c == ';')
            .trim_start();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((depth, content)) = extract_bullet_depth(trimmed) {
            match depth {
                1 => {
                    if let Some(mut entry) = current_entry.take() {
                        finalize_patch_entry(&mut entry);
                        if !entry.changes.is_empty() && !entry.version.trim().is_empty() {
                            entries.push(entry);
                            if entries.len() >= MAX_PATCH_ENTRIES {
                                reached_limit = true;
                                break;
                            }
                        }
                    }
                    if reached_limit {
                        break;
                    }
                    let (version, summary) = parse_patch_line(content);
                    current_section = None;
                    current_change_idx = None;
                    let mut entry = PatchEntry {
                        version,
                        changes: Vec::new(),
                    };
                    let summary_text = normalize_detail_line(&summary);
                    if !summary_text.is_empty() {
                        let change = Change {
                            section: "General".to_string(),
                            text: summary_text,
                        };
                        current_section = Some("General".to_string());
                        current_change_idx = Some(entry.changes.len());
                        entry.changes.push(change);
                    }
                    current_entry = Some(entry);
                }
                2 => {
                    if let Some(entry) = current_entry.as_mut() {
                        let (label, immediate_text) = parse_section_line(content);
                        let mut change = Change {
                            section: label.clone(),
                            text: immediate_text.unwrap_or_default(),
                        };
                        if change.section.trim().is_empty() {
                            change.section = "General".to_string();
                        }
                        current_section = Some(change.section.clone());
                        current_change_idx = Some(entry.changes.len());
                        entry.changes.push(change);
                    }
                }
                _ => {
                    if let Some(entry) = current_entry.as_mut() {
                        let detail_text = normalize_detail_line(content);
                        if detail_text.is_empty() {
                            continue;
                        }
                        let idx = if let Some(idx) = current_change_idx {
                            idx
                        } else {
                            let section_name = current_section
                                .clone()
                                .unwrap_or_else(|| "General".to_string());
                            entry.changes.push(Change {
                                section: section_name.clone(),
                                text: String::new(),
                            });
                            current_section = Some(section_name);
                            let new_idx = entry.changes.len() - 1;
                            current_change_idx = Some(new_idx);
                            new_idx
                        };
                        if let Some(change) = entry.changes.get_mut(idx) {
                            append_detail(change, &detail_text);
                        }
                    }
                }
            }
        } else if let Some(entry) = current_entry.as_mut() {
            if let Some(idx) = current_change_idx {
                if let Some(change) = entry.changes.get_mut(idx) {
                    if !change.text.is_empty() && !change.text.ends_with('\n') {
                        change.text.push(' ');
                    }
                    change.text.push_str(trimmed);
                }
            }
        }
    }

    if !reached_limit {
        if let Some(mut entry) = current_entry.take() {
            finalize_patch_entry(&mut entry);
            if !entry.changes.is_empty() && !entry.version.trim().is_empty() {
                entries.push(entry);
            }
        }
    }

    if entries.len() > MAX_PATCH_ENTRIES {
        entries.truncate(MAX_PATCH_ENTRIES);
    }

    entries
}

/// Parse a single patch history bullet into (version, text) pair.
pub fn parse_patch_line(line: &str) -> (String, String) {
    if let Some(rest) = line.strip_prefix("'''") {
        if let Some(end) = rest.find("'''") {
            let version = rest[..end].trim().trim_end_matches(':').trim().to_string();
            let remainder = rest[end + 3..].trim().trim_start_matches(':').trim();
            return (
                if version.is_empty() {
                    "Patch".to_string()
                } else {
                    version
                },
                remainder.to_string(),
            );
        }
    }
    if let Some(idx) = line.find(':') {
        let version = line[..idx].trim();
        let remainder = line[idx + 1..].trim();
        return (
            if version.is_empty() {
                "Patch".to_string()
            } else {
                version.to_string()
            },
            remainder.to_string(),
        );
    }
    ("Patch".to_string(), line.trim().to_string())
}

fn extract_bullet_depth(line: &str) -> Option<(usize, &str)> {
    let mut depth = 0usize;
    let mut byte_idx = 0usize;
    for (idx, ch) in line.char_indices() {
        if ch == '*' || ch == '#' {
            depth += 1;
            byte_idx = idx + ch.len_utf8();
        } else {
            break;
        }
    }
    if depth == 0 {
        None
    } else {
        Some((depth, line[byte_idx..].trim_start()))
    }
}

fn parse_section_line(content: &str) -> (String, Option<String>) {
    let cleaned = normalize_detail_line(content);
    if cleaned.is_empty() {
        return ("General".to_string(), None);
    }
    if cleaned.starts_with("'''") {
        if let Some(end) = cleaned[3..].find("'''") {
            let label = cleaned[3..3 + end].trim();
            let remainder = cleaned[3 + end + 3..].trim().trim_start_matches(':').trim();
            let label = clean_section_label(label);
            return if remainder.is_empty() {
                (label, None)
            } else {
                (label, Some(remainder.to_string()))
            };
        }
    }
    if let Some(idx) = cleaned.find(':') {
        let label = clean_section_label(&cleaned[..idx]);
        let remainder = cleaned[idx + 1..].trim();
        if remainder.is_empty() {
            (label, None)
        } else {
            (label, Some(remainder.to_string()))
        }
    } else if is_title_case(&cleaned) {
        (clean_section_label(&cleaned), None)
    } else {
        ("General".to_string(), Some(cleaned))
    }
}

fn clean_section_label(label: &str) -> String {
    let trimmed = label
        .trim()
        .trim_matches(|c: char| matches!(c, '-' | '–' | '—' | '•'))
        .trim();
    let trimmed = trimmed
        .trim_end_matches(|c: char| matches!(c, '.' | ':' | '-' | '–' | '—' | '•'))
        .trim();
    if trimmed.is_empty() {
        "General".to_string()
    } else {
        trimmed.to_string()
    }
}

fn is_title_case(input: &str) -> bool {
    let mut has_alpha = false;
    for word in input.split_whitespace() {
        let mut chars = word.chars().filter(|c| c.is_alphabetic());
        if let Some(first) = chars.next() {
            has_alpha = true;
            if !first.is_uppercase() {
                return false;
            }
            if !chars.all(|c| c.is_lowercase()) {
                return false;
            }
        }
    }
    has_alpha
}

fn normalize_detail_line(line: &str) -> String {
    let cleaned = line
        .trim()
        .trim_start_matches(|c: char| matches!(c, '-' | '–' | '—' | '•'))
        .trim_start();
    cleaned.to_string()
}

fn append_detail(change: &mut Change, detail: &str) {
    if change.text.trim().is_empty() {
        if change.text.is_empty() {
            change.text.push_str(detail);
        } else {
            change.text.push(' ');
            change.text.push_str(detail);
        }
    } else {
        change.text.push('\n');
        change.text.push_str(detail);
    }
}

fn finalize_patch_entry(entry: &mut PatchEntry) {
    for change in &mut entry.changes {
        change.section = if change.section.trim().is_empty() {
            "General".to_string()
        } else {
            change.section.trim().to_string()
        };
        change.text = change.text.trim_end().to_string();
    }
    entry
        .changes
        .retain(|change| !change.section.trim().is_empty() || !change.text.trim().is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_and_collect_lists() {
        let expanded = "Intro\n\n== Notes ==\n* First\n** Second\nMore\n\n== Trivia ==\n* Fact\n";
        let notes = extract_section(expanded, "Notes").unwrap();
        let items = collect_list_items(&notes);
        assert_eq!(items.len(), 2);
        assert!(items[0].starts_with('*'));
        let trivia = extract_section(expanded, "Trivia").unwrap();
        let trivia_items = collect_list_items(&trivia);
        assert_eq!(trivia_items.len(), 1);
    }

    #[test]
    fn patch_history_parsing() {
        let expanded = "== Patch History ==\n* '''V14.5:''' Dirty Fighting tweaks\n** '''Dirty Fighting:''' Base damage increased\n*** Additional detail\n** Heroic Swing\n*** Gains unstoppable while swinging\n* '''V14.4:'''\n** '''General:''' Adjusted icons\n";
        let history = extract_patch_history(expanded);
        assert_eq!(history.len(), 2);

        let first = &history[0];
        assert_eq!(first.version, "V14.5");
        assert_eq!(first.changes.len(), 3);
        assert_eq!(first.changes[0].section, "General");
        assert_eq!(first.changes[0].text, "Dirty Fighting tweaks");
        assert_eq!(first.changes[1].section, "Dirty Fighting");
        assert_eq!(
            first.changes[1].text,
            "Base damage increased\nAdditional detail"
        );
        assert_eq!(first.changes[2].section, "Heroic Swing");
        assert_eq!(first.changes[2].text, "Gains unstoppable while swinging");

        let second = &history[1];
        assert_eq!(second.version, "V14.4");
        assert_eq!(second.changes.len(), 1);
        assert_eq!(second.changes[0].section, "General");
        assert_eq!(second.changes[0].text, "Adjusted icons");
    }

    #[test]
    fn patch_history_limits_to_ten_entries() {
        let mut expanded = String::from("== Patch History ==\n");
        for idx in 0..12 {
            expanded.push_str(&format!("* '''V1.{}:''' Summary {}\n", idx, idx));
        }
        let history = extract_patch_history(&expanded);
        assert_eq!(history.len(), 10);
        assert_eq!(history[0].version, "V1.0");
        assert_eq!(history.last().unwrap().version, "V1.9");
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
