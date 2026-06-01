use crate::convert::context::ConversionContext;
use crate::error::Result;
use crate::model::{Change, PatchEntry};
use crate::parse::extract_balanced_templates;
use crate::parse::lua::LuaValue;
use crate::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use serde_json::{Map, Number, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Collect `#vardefine` assignments from the provided raw wikitext into a map.
pub fn collect_page_vars(raw: &str) -> Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    if let Ok(spans) = extract_balanced_templates(raw) {
        for span in spans {
            let body = &span.raw[2..span.raw.len() - 2];
            let inv = parse_invocation(body);
            if (inv.name.eq_ignore_ascii_case("#vardefine")
                || inv.name.eq_ignore_ascii_case("#vardefineecho"))
                && inv.params.len() >= 2
            {
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
    let shared_vars = Arc::new(Mutex::new(vars.clone()));
    expand_inline_templates_with_store(raw, precision, shared_vars, registry, conversion_ctx)
}

pub fn expand_inline_templates_mut(
    raw: &str,
    precision: u8,
    vars: &mut HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<String> {
    let shared_vars = Arc::new(Mutex::new(vars.clone()));
    let expanded = expand_inline_templates_with_store(
        raw,
        precision,
        shared_vars.clone(),
        registry,
        conversion_ctx,
    )?;
    if let Ok(guard) = shared_vars.lock() {
        *vars = guard.clone();
    }
    Ok(expanded)
}

pub fn expand_inline_templates_with_store(
    raw: &str,
    precision: u8,
    vars: Arc<Mutex<HashMap<String, String>>>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<String> {
    let mut curr = raw.to_string();
    let ctx = ExpanderCtx {
        precision,
        vars,
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

pub fn lua_table_to_pretty_json(table: &HashMap<String, LuaValue>) -> Result<String> {
    let value = lua_table_ref_to_json_value(table);
    serde_json::to_string_pretty(&value).map_err(|err| {
        crate::error::ConvertError::Internal(format!(
            "failed to serialize Lua table to JSON: {err}"
        ))
    })
}

fn lua_value_to_json_value(value: &LuaValue) -> Value {
    match value {
        LuaValue::Nil => Value::Null,
        LuaValue::Bool(b) => Value::Bool(*b),
        LuaValue::Number(raw) => number_or_string_json(raw),
        LuaValue::String(s) => Value::String(s.clone()),
        LuaValue::Array(items) => Value::Array(
            items
                .iter()
                .map(lua_value_to_json_value)
                .collect::<Vec<_>>(),
        ),
        LuaValue::Table(table) => lua_table_ref_to_json_value(table),
    }
}

fn lua_table_ref_to_json_value(table: &HashMap<String, LuaValue>) -> Value {
    let mut keys: Vec<&String> = table.keys().collect();
    keys.sort();
    let mut object = Map::new();
    for key in keys {
        if let Some(value) = table.get(key) {
            object.insert(key.clone(), lua_value_to_json_value(value));
        }
    }
    Value::Object(object)
}

fn number_or_string_json(raw: &str) -> Value {
    let trimmed = raw.trim();
    if let Ok(int_val) = trimmed.parse::<i64>() {
        return Value::Number(Number::from(int_val));
    }
    if let Ok(uint_val) = trimmed.parse::<u64>() {
        return Value::Number(Number::from(uint_val));
    }
    if let Ok(float_val) = trimmed.parse::<f64>() {
        if let Some(num) = Number::from_f64(float_val) {
            return Value::Number(num);
        }
    }
    Value::String(trimmed.to_string())
}

/// Extract the body of a second-level heading from expanded wikitext.
pub fn extract_section(expanded: &str, heading: &str) -> Option<String> {
    let target = heading.to_ascii_lowercase();
    let mut in_section = false;
    let mut body: Vec<&str> = Vec::new();
    for line in expanded.lines() {
        let trimmed = line.trim();
        if let Some((level, heading_text)) = parse_heading_line(trimmed) {
            if level == 2 && heading_text == target {
                in_section = true;
                continue;
            }
            if in_section && level <= 2 {
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

fn parse_heading_line(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with('=') {
        return None;
    }

    let leading = trimmed.chars().take_while(|&ch| ch == '=').count();
    let trailing = trimmed.chars().rev().take_while(|&ch| ch == '=').count();
    if leading < 2 || trailing < 2 {
        return None;
    }

    let content = trimmed[leading..trimmed.len() - trailing].trim();
    if content.is_empty() {
        return None;
    }

    Some((leading.min(trailing), content.to_ascii_lowercase()))
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
    let source = extract_section(expanded, "Patch history").unwrap_or_else(|| expanded.to_string());
    let normalized_source = normalize_patch_history_source(&source);

    let mut entries: Vec<PatchEntry> = Vec::new();
    let mut current_entry: Option<PatchEntry> = None;
    let mut current_section: Option<String> = None;
    let mut current_change_idx: Option<usize> = None;
    let mut section_depth_base: usize = 2;
    let mut reached_limit = false;

    for line in normalized_source.lines() {
        if reached_limit {
            break;
        }
        let mut trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("==")
            || trimmed.starts_with("[[Category:")
            || trimmed.starts_with("<noinclude")
            || trimmed.starts_with("</noinclude")
            || looks_like_interwiki_link(trimmed)
        {
            continue;
        }
        trimmed = trimmed.trim_start_matches(':').trim_start();
        if trimmed.is_empty() {
            continue;
        }

        if let Some((version, summary, depth_base)) = parse_patch_version_header(trimmed) {
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

            section_depth_base = depth_base;
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
            continue;
        }

        let Some(entry) = current_entry.as_mut() else {
            continue;
        };

        if let Some((depth, content)) = extract_bullet_depth(trimmed) {
            if depth <= section_depth_base {
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
            } else {
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
        } else {
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

fn looks_like_interwiki_link(line: &str) -> bool {
    let trimmed = line.trim();
    if !trimmed.starts_with("[[") || !trimmed.ends_with("]]") {
        return false;
    }
    let inner = &trimmed[2..trimmed.len().saturating_sub(2)];
    let Some((prefix, _rest)) = inner.split_once(':') else {
        return false;
    };
    let prefix = prefix.trim();
    (2..=3).contains(&prefix.len()) && prefix.chars().all(|ch| ch.is_ascii_lowercase())
}

fn normalize_patch_history_source(source: &str) -> String {
    let source = strip_html_comments(source);
    let mut out: Vec<String> = Vec::new();
    let mut semicolon_mode = false;

    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(';') {
            if let Some((version, summary)) = parse_semicolon_patch_line(rest.trim()) {
                let mut normalized = format!("* '''{version}'''");
                if !summary.trim().is_empty() {
                    normalized = format!("* '''{version}:''' {}", summary.trim());
                }
                out.push(normalized);
                semicolon_mode = true;
                continue;
            }
        }

        if semicolon_mode {
            let trimmed_start = line.trim_start();
            if trimmed_start.starts_with('*') || trimmed_start.starts_with('#') {
                let leading_ws = &line[..line.len() - trimmed_start.len()];
                out.push(format!("{leading_ws}*{trimmed_start}"));
                continue;
            }
            if trimmed_start.starts_with("==") {
                semicolon_mode = false;
            }
        }

        out.push(line.to_string());
    }

    out.join("\n")
}

fn strip_html_comments(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut remaining = source;

    while let Some(start) = remaining.find("<!--") {
        out.push_str(&remaining[..start]);
        let comment_body = &remaining[start + 4..];
        if let Some(end) = comment_body.find("-->") {
            remaining = &comment_body[end + 3..];
        } else {
            remaining = "";
            break;
        }
    }

    out.push_str(remaining);
    out
}

fn parse_patch_version_header(line: &str) -> Option<(String, String, usize)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix(';') {
        let (version, summary) = parse_semicolon_patch_line(rest.trim())?;
        return Some((version, summary, 1));
    }
    if let Some((depth, content)) = extract_bullet_depth(trimmed) {
        if depth == 1 {
            let (version, summary) = parse_patch_line(content);
            if looks_like_patch_version(&version, content) {
                return Some((version, summary, 2));
            }
        }
    }
    None
}

fn parse_semicolon_patch_line(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(rest) = trimmed.strip_prefix("[[") {
        let end = rest.find("]]")?;
        let inner = &rest[..end];
        let after = rest[end + 2..].trim();
        let display = inner.rsplit('|').next().unwrap_or(inner).trim();
        let version = display
            .split('#')
            .next()
            .unwrap_or(display)
            .trim()
            .to_string();
        let summary = after
            .trim_start_matches('-')
            .trim_start_matches(':')
            .trim()
            .to_string();
        if looks_like_patch_version(&version, trimmed) {
            return Some((version, summary));
        }
        return None;
    }

    let (version, summary) = if let Some((left, right)) = trimmed.split_once(" - ") {
        (left.trim().to_string(), right.trim().to_string())
    } else if let Some((left, right)) = trimmed.split_once(':') {
        (left.trim().to_string(), right.trim().to_string())
    } else {
        (trimmed.to_string(), String::new())
    };

    if looks_like_patch_version(&version, trimmed) {
        Some((version, summary))
    } else {
        None
    }
}

fn looks_like_patch_version(version: &str, raw: &str) -> bool {
    let version = version.trim();
    if version.is_empty() {
        return false;
    }
    let lower_version = version.to_ascii_lowercase();
    let lower_raw = raw.trim().to_ascii_lowercase();
    lower_version.starts_with('v')
        || lower_version.contains("patch")
        || lower_raw.starts_with("[[v")
        || lower_raw.contains(" patch")
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
            let remainder =
                normalize_section_remainder(cleaned[3 + end + 3..].trim().trim_start_matches(':'));
            let label = clean_section_label(label);
            return (label, remainder);
        }
    }
    let title_probe = cleaned.split("<ref").next().unwrap_or(&cleaned).trim();
    if title_probe != cleaned && is_title_case(title_probe) {
        return (clean_section_label(&cleaned), None);
    }
    if let Some(idx) = find_section_separator(&cleaned) {
        let label = clean_section_label(&cleaned[..idx]);
        let remainder = normalize_section_remainder(&cleaned[idx + 1..]);
        (label, remainder)
    } else if is_title_case(&cleaned) {
        (clean_section_label(&cleaned), None)
    } else {
        ("General".to_string(), Some(cleaned))
    }
}

fn find_section_separator(content: &str) -> Option<usize> {
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut chars = content.char_indices().peekable();
    while let Some((idx, ch)) = chars.next() {
        match ch {
            '(' => {
                paren_depth += 1;
                continue;
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
                continue;
            }
            '[' => {
                bracket_depth += 1;
                continue;
            }
            ']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                continue;
            }
            ':' if paren_depth == 0 && bracket_depth == 0 => {}
            _ => continue,
        }
        let remainder = &content[idx + ch.len_utf8()..];
        let trimmed = remainder.trim_start_matches(|c: char| matches!(c, '*' | '_' | '\''));
        let next_is_separator = trimmed
            .chars()
            .next()
            .map(|next| next.is_whitespace())
            .unwrap_or(true);
        if next_is_separator {
            return Some(idx);
        }
    }
    None
}

fn normalize_section_remainder(remainder: &str) -> Option<String> {
    let trimmed = remainder
        .trim()
        .trim_start_matches(|c: char| matches!(c, '*' | '_' | '\''))
        .trim_start()
        .trim_end_matches(|c: char| matches!(c, '*' | '_' | '\''))
        .trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn clean_section_label(label: &str) -> String {
    let trimmed = label
        .trim()
        .trim_matches(|c: char| matches!(c, '-' | '–' | '—' | '•' | '*'))
        .trim();
    let trimmed = trimmed
        .trim_end_matches(|c: char| matches!(c, '.' | ':' | '-' | '–' | '—' | '•' | '*'))
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
    fn extract_section_keeps_nested_subheadings() {
        let expanded = concat!(
            "== Pets ==\n",
            "{{Infobox/Pet\n",
            "|name = The Ball\n",
            "=== General ===\n",
            "* Nested detail\n",
            "}}\n",
            "\n== Trivia ==\n",
            "* Fact\n"
        );

        let pets = extract_section(expanded, "Pets").unwrap();
        assert!(pets.contains("=== General ==="));
        assert!(pets.contains("Nested detail"));
        assert!(pets.contains("{{Infobox/Pet"));
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

    #[test]
    fn clean_section_label_strips_markdown_emphasis() {
        assert_eq!(clean_section_label("**NEW:**"), "NEW");
        assert_eq!(
            clean_section_label("**UNIQUE PASSIVE - PERFECTION**"),
            "UNIQUE PASSIVE - PERFECTION"
        );
    }

    #[test]
    fn parse_section_line_ignores_empty_bold_remainder() {
        assert_eq!(parse_section_line("**NEW:**"), ("NEW".to_string(), None));
        assert_eq!(
            parse_section_line("**REMOVED:** No longer has a passive."),
            (
                "REMOVED".to_string(),
                Some("No longer has a passive.".to_string())
            )
        );
    }

    #[test]
    fn parse_section_line_ignores_url_colons_inside_refs() {
        let content = "Broken Wings<ref>Youtube — Phreak (22 July 2025) [https://youtu.be/OG7a43cu-LY?feature=shared&t=1237 Patch 25.15 Preview] ''(20:37)''</ref><ref>YouTube — Vandiril (19 July 2025) [https://www.youtube.com/watch?v=Qa-xqVypeoc Riot messed up AGAIN...]. Bug demonstration.</ref>";
        assert_eq!(parse_section_line(content), (content.to_string(), None));
    }

    #[test]
    fn parse_section_line_ignores_formula_colons_inside_parentheses() {
        let content = "Base damage reduced to 50 – 220 (based on level; formula: 40 + (10xlevel)).";
        assert_eq!(
            parse_section_line(content),
            ("General".to_string(), Some(content.to_string()))
        );
    }

    #[test]
    fn patch_history_parses_semicolon_style_subpages() {
        let expanded = "==Release version==\n;[[V14.22]]\n* Heroic Swing\n** Bug Fix: Empowered shots now follow the same logic as regular shots.\n\n;[[V14.20]]\n* General\n** Bug Fix: Additional shot now correctly applies Serrated Edge.\n";
        let history = extract_patch_history(expanded);
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].version, "V14.22");
        assert_eq!(history[0].changes[0].section, "Heroic Swing");
        assert!(history[0].changes[0]
            .text
            .contains("Empowered shots now follow the same logic"));
        assert_eq!(history[1].version, "V14.20");
        assert_eq!(history[1].changes[0].section, "General");
    }

    #[test]
    fn patch_history_ignores_trailing_interwiki_links() {
        let expanded = "== Patch History ==\n* '''V1.0:''' Updated numbers\n[[de:Elektrisieren]]\n[[ru:Казнь_электричеством_(Руна)]]\n";
        let history = extract_patch_history(expanded);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].version, "V1.0");
        assert_eq!(history[0].changes.len(), 1);
        assert_eq!(history[0].changes[0].text, "Updated numbers");
    }

    #[test]
    fn patch_history_strips_inline_and_block_html_comments() {
        let expanded = concat!(
            "== Patch History ==\n",
            ";[[V14.7]]\n",
            "* '''Test of Spirit:'''<!-- Note: hidden source note -->\n",
            "** Duration reduced.\n",
            "* Upheaval\n",
            "** Slow adjusted.<!--\n",
            "* Hidden Patch\n",
            "** This should not be parsed.\n",
            "-->\n",
        );

        let history = extract_patch_history(expanded);
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].version, "V14.7");
        assert_eq!(history[0].changes.len(), 2);
        assert_eq!(history[0].changes[0].section, "Test of Spirit");
        assert_eq!(history[0].changes[0].text, "Duration reduced.");
        assert_eq!(history[0].changes[1].section, "Upheaval");
        assert_eq!(history[0].changes[1].text, "Slow adjusted.");
    }

    #[test]
    fn lua_table_json_preserves_sorted_keys_and_nested_values() {
        let mut nested = HashMap::new();
        nested.insert("z".to_string(), LuaValue::String("last".to_string()));
        nested.insert("a".to_string(), LuaValue::Number("1".to_string()));

        let mut table = HashMap::new();
        table.insert("beta".to_string(), LuaValue::Bool(true));
        table.insert("alpha".to_string(), LuaValue::Table(nested));

        let json = lua_table_to_pretty_json(&table).unwrap();
        let alpha_idx = json.find("\"alpha\"").unwrap();
        let beta_idx = json.find("\"beta\"").unwrap();
        assert!(alpha_idx < beta_idx);
        assert!(json.contains("\"a\": 1"));
        assert!(json.contains("\"z\": \"last\""));
    }
}
