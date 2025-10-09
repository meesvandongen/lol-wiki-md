//! Balanced brace / template extraction utilities.
use crate::error::ConvertError;

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateSpan {
    pub name: String,
    pub start: usize,
    pub end: usize,
    pub raw: String,
    pub line: usize, // 1-based line number of opening '{{'
    pub col: usize,  // 1-based column number of opening '{{'
}

/// Extract top-level `{{...}}` templates with nested depth awareness.
pub fn extract_balanced_templates(input: &str) -> Result<Vec<TemplateSpan>, ConvertError> {
    let bytes = input.as_bytes();
    let mut i = 0; let mut spans = Vec::new();
    let mut line = 1usize; let mut col = 1usize;
    while i + 1 < bytes.len() {
        if bytes[i] == b'{' && bytes[i+1] == b'{' {
            // record current position line/col for span
            let start_line = line; let start_col = col;
            let start = i; i += 2; col += 2; let mut depth = 1;
            while i + 1 < bytes.len() && depth > 0 {
                if bytes[i] == b'\n' { line += 1; col = 1; i += 1; continue; }
                if bytes[i] == b'{' && bytes[i+1] == b'{' { depth += 1; i += 2; col += 2; continue; }
                if bytes[i] == b'}' && bytes[i+1] == b'}' { depth -= 1; i += 2; col += 2; continue; }
                i += 1; col += 1;
            }
            if depth != 0 { return Err(ConvertError::UnbalancedBraces { name: format!("start@{start}") }); }
            let end = i; let raw = &input[start..end];
            let inner = &raw[2..raw.len()-2];
            let name = inner.split(['|', '\n']).next().unwrap_or("").trim().to_string();
            spans.push(TemplateSpan { name, start, end, raw: raw.to_string(), line: start_line, col: start_col });
        } else {
            if bytes[i] == b'\n' { line += 1; col = 1; } else { col += 1; }
            i += 1;
        }
    }
    Ok(spans)
}
