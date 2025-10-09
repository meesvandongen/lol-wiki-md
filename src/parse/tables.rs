//! Wikitext table → Markdown conversion (very small subset)
use crate::error::{ConvertError, Result};

pub fn wikitext_table_to_markdown(src: &str) -> Result<String> {
    // Expect lines between {| and |}
    if !src.starts_with("{|") || !src.ends_with("|}") { return Err(ConvertError::TableParse { context: "table".into(), detail: "missing opening/closing".into() }); }
    let inner = &src[2..src.len()-2];
    let mut headers: Vec<String> = Vec::new();
    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in inner.lines() {
        let l = line.trim();
        if l.starts_with("!") { // header row variant
            let content = l.trim_start_matches('!').trim();
            headers = content.split("!!").map(|s| s.trim().trim_start_matches('!').trim().to_string()).collect();
            continue;
        }
        if l.starts_with("|") && !l.starts_with("||") { // row or directive
            let content = l.trim_start_matches('|').trim();
            if content.starts_with("-") { continue; }
            if content.contains("||") { rows.push(content.split("||").map(|s| s.trim().to_string()).collect()); }
        }
    }
    if headers.is_empty() && !rows.is_empty() { headers = (0..rows[0].len()).map(|i| format!("Col{i}" )).collect(); }
    let mut out = String::new();
    out.push_str("| "); out.push_str(&headers.join(" | ")); out.push_str(" |\n");
    out.push_str(&format!("|{}|\n", headers.iter().map(|_| "---").collect::<Vec<_>>().join("|")));
    for r in rows { out.push_str("| "); out.push_str(&r.join(" | ")); out.push_str(" |\n"); }
    Ok(out)
}

#[cfg(test)]
mod tests { use super::*; #[test] fn simple_table() { let src = "{|\n|+ caption\n! A !! B\n|-\n| 1 || 2\n|}"; let md = wikitext_table_to_markdown(src).unwrap(); assert!(md.contains("A") && md.contains("1")); }}
