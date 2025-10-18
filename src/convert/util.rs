use crate::error::Result;
use crate::model::{Change, PatchEntry};
use crate::parse::extract_balanced_templates;
use crate::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use std::collections::HashMap;

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
) -> Result<String> {
    if let Ok(spans) = extract_balanced_templates(raw) {
        let mut output = String::new();
        let mut last = 0usize;
        let ctx = ExpanderCtx {
            precision,
            vars: vars.clone(),
        };
        for span in spans {
            output.push_str(&raw[last..span.start]);
            let body = &span.raw[2..span.raw.len() - 2];
            let inv = parse_invocation(body);
            let exp = registry.expand(&inv, &ctx)?;
            output.push_str(&exp.expanded);
            last = span.end;
        }
        output.push_str(&raw[last..]);
        Ok(output)
    } else {
        Ok(raw.to_string())
    }
}

/// Iteratively expand inline templates inside a value until a fixed point or iteration cap is reached.
pub fn expand_inline_templates(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<String> {
    let mut curr = raw.to_string();
    let ctx = ExpanderCtx {
        precision,
        vars: vars.clone(),
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
    let Some(section) = extract_section(expanded, "Patch history") else {
        return Vec::new();
    };
    let mut entries: Vec<PatchEntry> = Vec::new();
    let mut current: Option<PatchEntry> = None;
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('*') || trimmed.starts_with('#') {
            let depth = trimmed
                .chars()
                .take_while(|c| *c == '*' || *c == '#')
                .count();
            let content = trimmed.trim_start_matches(['*', '#']).trim();
            if depth <= 1 {
                if let Some(entry) = current.take() {
                    if !entry.changes.is_empty() {
                        entries.push(entry);
                    }
                }
                if content.is_empty() {
                    current = None;
                    continue;
                }
                let (version, text) = parse_patch_line(content);
                let mut changes = Vec::new();
                if !text.trim().is_empty() {
                    changes.push(Change {
                        section: "General".to_string(),
                        text,
                    });
                }
                current = Some(PatchEntry { version, changes });
            } else if let Some(entry) = current.as_mut() {
                if !content.is_empty() {
                    entry.changes.push(Change {
                        section: "General".to_string(),
                        text: content.to_string(),
                    });
                }
            }
        } else if let Some(entry) = current.as_mut() {
            if trimmed.is_empty() {
                continue;
            }
            if let Some(last) = entry.changes.last_mut() {
                if !last.text.is_empty() {
                    last.text.push(' ');
                }
                last.text.push_str(trimmed);
            } else {
                entry.changes.push(Change {
                    section: "General".to_string(),
                    text: trimmed.to_string(),
                });
            }
        }
    }
    if let Some(entry) = current {
        if !entry.changes.is_empty() {
            entries.push(entry);
        }
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
        let expanded = "== Patch History ==\n* '''V14.5:''' Buffed damage\n** Extra detail\n* '''V14.4:'''\n** Secondary adjustment\n";
        let history = extract_patch_history(expanded);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].version, "V14.5");
        assert_eq!(history[0].changes.len(), 2);
        assert_eq!(history[0].changes[0].text, "Buffed damage");
        assert_eq!(history[1].version, "V14.4");
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
