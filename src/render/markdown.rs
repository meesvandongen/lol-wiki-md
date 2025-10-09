use crate::model::{Champion, AbilityKey};
use regex::Regex;

pub fn render_champion_markdown(champ: &Champion, raw_excerpt: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", champ.name));
    // Stats section (basic subset)
    if !champ.stats.base.is_empty() {
        out.push_str("## Stats\n\n| Stat | Base | Growth |\n|------|------|--------|\n");
        let mut keys: Vec<_> = champ.stats.base.keys().filter(|k| !k.starts_with("resource__")).collect();
        keys.sort();
        for k in keys { if let Some(v)=champ.stats.base.get(k) { if k=="resource" { continue; } out.push_str(&format!("| {} | {} | {} |\n", k, v.base, v.growth)); } }
        out.push('\n');
    }
    // Abilities
    if !champ.abilities.is_empty() {
        out.push_str("## Abilities\n\n");
        for a in &champ.abilities {
            let key_label = match &a.key { AbilityKey::Passive => "Passive", AbilityKey::Q=>"Q", AbilityKey::W=>"W", AbilityKey::E=>"E", AbilityKey::R=>"R", AbilityKey::Other(s)=> s };
            out.push_str(&format!("### {} – {}\n\n", key_label, a.name));
            if let Some(desc) = a.descriptions.first() { out.push_str(desc); out.push_str("\n\n"); }
        }
    }
    out.push_str("<!-- Raw excerpt (first 20 lines) -->\n\n<details><summary>Raw excerpt</summary>\n\n```wikitext\n");
    let excerpt_norm = normalize_internal_links(&normalize_anchors(&normalize_apostrophes(raw_excerpt)));
    out.push_str(&excerpt_norm.lines().take(20).collect::<Vec<_>>().join("\n"));
    out.push_str("\n```\n</details>\n");
    ensure_trailing_newline(&collapse_blank_lines(&out))
}

pub fn normalize_internal_links(s: &str) -> String {
    // Simple pattern [[Page|Display]] or [[Page]] -> Display or Page (underscores)
    let mut out = String::new();
    let mut i=0; let bytes = s.as_bytes();
    while i < bytes.len() { if i+1 < bytes.len() && bytes[i]==b'[' && bytes[i+1]==b'[' { i+=2; let mut inner = String::new(); while i+1 < bytes.len() && !(bytes[i]==b']' && bytes[i+1]==b']') { inner.push(bytes[i] as char); i+=1; } if i+1 >= bytes.len() { out.push_str(&format!("[[{}", inner)); break; } i+=2; // skip closing
            if inner.starts_with("File:") { continue; }
            let display = if let Some(bar)= inner.find('|') { let page=&inner[..bar]; let disp=&inner[bar+1..]; if disp.is_empty() { page } else { disp } } else { &inner };
            out.push_str(display.replace(' ', "_").as_str());
        } else { out.push(bytes[i] as char); i+=1; } }
    out
}

pub fn normalize_apostrophes(s: &str) -> String {
    // Stateful conversion of wiki apostrophes to Markdown markers.
    // Handles 2 (italic), 3 (bold), 5 (bold+italic) with simple toggles.
    let mut out = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut bold_on = false;
    let mut italic_on = false;
    while i < bytes.len() {
        if bytes[i] == b'\'' {
            let mut j = i;
            while j < bytes.len() && bytes[j] == b'\'' { j += 1; }
            let count = j - i;
            match count {
                5 => {
                    // Toggle both, preserve order for closing.
                    if !bold_on && !italic_on { out.push_str("**_"); bold_on = true; italic_on = true; }
                    else { out.push_str("_**"); bold_on = false; italic_on = false; }
                }
                3 => {
                    out.push_str("**"); bold_on = !bold_on;
                }
                2 => {
                    out.push('_'); italic_on = !italic_on;
                }
                _ => { for _ in 0..count { out.push('\''); } }
            }
            i = j; continue;
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

pub fn normalize_anchors(s: &str) -> String {
    // Replace patterns [[Page#Section Title]] -> Page_Section-Title (simplistic)
    let re = Regex::new(r"\[\[([^\]|#]+)#([^\]|]+)\]\]").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let page = caps.get(1).unwrap().as_str().replace(' ', "_");
        let sec = caps.get(2).unwrap().as_str().replace(' ', "-");
        format!("{page}#{sec}")
    }).to_string()
}

fn collapse_blank_lines(s: &str) -> String {
    let mut out = String::new();
    let mut last_blank=false;
    for line in s.lines() {
        let blank = line.trim().is_empty();
        if blank && last_blank { continue; }
        out.push_str(line); out.push('\n');
        last_blank = blank;
    }
    out
}

fn ensure_trailing_newline(s: &str) -> String { if s.ends_with('\n') { s.to_string() } else { format!("{s}\n") } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apostrophes_normalization_basic() {
        assert_eq!(normalize_apostrophes("''italic''"), "_italic_");
        assert_eq!(normalize_apostrophes("'''bold'''"), "**bold**");
        assert_eq!(normalize_apostrophes("'''''both'''''"), "**_both_**");
    }

    #[test]
    fn internal_links_and_anchors() {
        assert_eq!(normalize_internal_links("See [[Aatrox]]"), "See Aatrox");
        assert_eq!(normalize_internal_links("See [[Akshan/Cosmetics|Akshan (Collection)]]"), "See Akshan_(Collection)");
        let s = normalize_anchors("[[Page#Section Title]]");
        assert_eq!(s, "Page#Section-Title");
    }
}
