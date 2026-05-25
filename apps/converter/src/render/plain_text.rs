use once_cell::sync::Lazy;
use regex::Regex;

static COMMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<!--.*?-->").expect("valid html comment regex"));
static DETAILS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?is)<details[^>]*>\s*<summary>(.*?)</summary>.*?</details>")
        .expect("valid details regex")
});
static HTML_TAG_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)</?[a-z][^>]*>").expect("valid html tag regex"));
static HEADING_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s{0,3}#{1,6}\s*").expect("valid heading regex"));
static BLOCKQUOTE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s{0,3}>\s?").expect("valid blockquote regex"));
static BULLET_LIST_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\s*)[-*+]\s+").expect("valid bullet list regex"));
static ORDERED_LIST_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(\s*)\d+[.)]\s+").expect("valid ordered list regex"));
static TABLE_SEPARATOR_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\s*\|?(?:\s*:?-{3,}:?\s*\|)+\s*:?-{3,}:?\s*\|?\s*$")
        .expect("valid table separator regex")
});
static MULTIBLANK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\n{3,}").expect("valid multiblank regex"));

pub fn markdown_to_plain_text(markdown: &str) -> String {
    let mut current = markdown.replace("\r\n", "\n");
    current = DETAILS_RE
        .replace_all(&current, |caps: &regex::Captures| {
            let summary = caps.get(1).map(|capture| capture.as_str()).unwrap_or("");
            format!("\n{}\n", summary.trim())
        })
        .to_string();
    current = COMMENT_RE.replace_all(&current, "").to_string();
    current = strip_markdown_links(&current);
    current = current
        .replace("&nbsp;", " ")
        .replace("&#160;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&#39;", "'")
        .replace("&quot;", "\"");
    current = HTML_TAG_RE.replace_all(&current, "").to_string();

    let mut out_lines = Vec::new();
    for raw_line in current.lines() {
        let mut line = raw_line.trim_end().to_string();
        if line.trim().is_empty() {
            out_lines.push(String::new());
            continue;
        }
        if line.trim_start().starts_with("```") {
            continue;
        }
        if TABLE_SEPARATOR_RE.is_match(line.trim()) {
            continue;
        }
        if line.trim() == "---" || line.trim() == "***" {
            continue;
        }

        if line.trim_start().starts_with('|') {
            line = normalize_table_line(&line);
        }

        line = HEADING_RE.replace(&line, "").to_string();
        line = BLOCKQUOTE_RE.replace(&line, "").to_string();
        line = BULLET_LIST_RE.replace(&line, "$1- ").to_string();
        line = ORDERED_LIST_RE.replace(&line, "${1}1. ").to_string();
        line = line
            .replace("**", "")
            .replace("__", "")
            .replace("~~", "")
            .replace('`', "")
            .replace('*', "")
            .replace('_', "");

        let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if normalized.is_empty() {
            out_lines.push(String::new());
        } else {
            out_lines.push(normalized);
        }
    }

    let collapsed = MULTIBLANK_RE
        .replace_all(&out_lines.join("\n"), "\n\n")
        .trim()
        .to_string();
    if collapsed.is_empty() {
        String::new()
    } else {
        format!("{}\n", collapsed)
    }
}

fn normalize_table_line(line: &str) -> String {
    line.split('|')
        .map(str::trim)
        .filter(|cell| !cell.is_empty())
        .collect::<Vec<_>>()
        .join(" | ")
}

fn strip_markdown_links(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut out = String::new();
    let mut i = 0usize;
    while i < chars.len() {
        let is_image = chars[i] == '!' && i + 1 < chars.len() && chars[i + 1] == '[';
        if chars[i] == '[' || is_image {
            let label_start = if is_image { i + 2 } else { i + 1 };
            let mut j = label_start;
            let mut bracket_depth = 1i32;
            while j < chars.len() {
                match chars[j] {
                    '[' => bracket_depth += 1,
                    ']' => {
                        bracket_depth -= 1;
                        if bracket_depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
                j += 1;
            }
            if j < chars.len() && j + 1 < chars.len() && chars[j + 1] == '(' {
                let label = chars[label_start..j].iter().collect::<String>();
                out.push_str(&label);
                let mut k = j + 2;
                let mut paren_depth = 1i32;
                while k < chars.len() {
                    match chars[k] {
                        '(' => paren_depth += 1,
                        ')' => {
                            paren_depth -= 1;
                            if paren_depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    k += 1;
                }
                i = if k < chars.len() { k + 1 } else { chars.len() };
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_strips_mismatched_emphasis_markers() {
        let markdown = "**_Aurelion Sol** will be knocked down by any immobilizing crowd control during the dash._\n";
        let plain = markdown_to_plain_text(markdown);
        assert_eq!(
            plain,
            "Aurelion Sol will be knocked down by any immobilizing crowd control during the dash.\n"
        );
    }

    #[test]
    fn plain_text_collapses_details_to_summary_only() {
        let markdown = "Before\n\n<details><summary>Raw excerpt</summary>\n\n```wikitext\nsecret body\n```\n</details>\n\nAfter\n";
        let plain = markdown_to_plain_text(markdown);
        assert_eq!(plain, "Before\n\nRaw excerpt\n\nAfter\n");
        assert!(!plain.contains("secret body"));
    }

    #[test]
    fn plain_text_flattens_links_and_tables() {
        let markdown =
            "| **Cooldown** | 16 |\n|-----------|------:|\nSee [quick cast](./quick_cast.md).\n";
        let plain = markdown_to_plain_text(markdown);
        assert_eq!(plain, "Cooldown | 16\nSee quick cast.\n");
    }

    #[test]
    fn plain_text_preserves_ordered_list_shape() {
        let markdown = "1. First\n2. Second\n";
        let plain = markdown_to_plain_text(markdown);
        assert_eq!(plain, "1. First\n1. Second\n");
    }
}
