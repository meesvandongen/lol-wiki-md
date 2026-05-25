//! Ability template file parameter parser (Template/Data_<Champion>/<Key>/page.txt)
use crate::error::{ConvertError, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

use super::brace::extract_balanced_templates;
use super::templates::parse_invocation;

static HTML_COMMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)<!--.*?-->").expect("valid html comment regex"));

/// Parse an ability data template file content into key -> value map.
/// File format example (simplified):
/// {{AbilityData
/// | name = Dark Flight
/// | cost = 30 / 40 / 50
/// | cooldown = {{#expr: 10 + 2}}
/// | description = Deals {{ap|60}} magic damage
/// }}
pub fn parse_ability_template(content: &str) -> Result<HashMap<String, String>> {
    let sanitized = HTML_COMMENT_RE.replace_all(content, "");
    let spans = extract_balanced_templates(&sanitized)?;
    for span in spans {
        let inner = &span.raw[2..span.raw.len() - 2];
        let inv = parse_invocation(inner);
        if !is_ability_template_name(&inv.name) {
            continue;
        }
        let mut map = HashMap::new();
        for p in inv.params {
            if let Some(eq_idx) = p.find('=') {
                let key_raw = p[..eq_idx].trim();
                if key_raw.is_empty() {
                    continue;
                }
                let value = p[eq_idx + 1..].trim();
                let key = key_raw.to_ascii_lowercase();
                if map.insert(key.clone(), value.to_string()).is_some() {
                    return Err(ConvertError::DuplicateKey(key));
                }
            }
        }
        if !map.is_empty() {
            return Ok(map);
        }
    }
    Err(ConvertError::MalformedTemplate {
        name: "AbilityData".into(),
        detail: "no ability data template found".into(),
    })
}

fn is_ability_template_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("ability") || lower.starts_with("{{{1") || lower.contains("abilitydata")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_basic_ability() {
        let txt = "{{AbilityData| name = Test | cost = 30 / 40 | cooldown = {{#expr: 5+5}} | description = Deals {{ap|60}} magic damage}}";
        let map = parse_ability_template(txt).unwrap();
        assert_eq!(map.get("name").unwrap(), "Test");
        assert_eq!(map.get("cost").unwrap(), "30 / 40");
        assert!(map.get("cooldown").unwrap().contains("#expr"));
    }

    #[test]
    fn parse_wrapped_ability_template() {
        let txt = "{{#vardefine:scale|0.5}}\n{{{{{1|Ability data}}}| name = Test Skill | skill = Q | description = Deals {{ap|{{#var:scale}}*100}} damage | notes = * First note }}";
        let map = parse_ability_template(txt).unwrap();
        assert_eq!(map.get("name").unwrap(), "Test Skill");
        assert!(map.get("description").unwrap().contains("{{#var"));
        assert_eq!(map.get("skill").unwrap(), "Q");
        assert!(map.get("notes").unwrap().contains("First note"));
    }

    #[test]
    fn parse_ability_template_ignores_commented_duplicate_keys() {
        let txt = concat!(
            "{{{{{1|Ability data}}}|Mounted Form|skill=I|description=Visible text",
            "<!--|description=commented duplicate should be ignored-->",
            "|notes=* First note}}"
        );
        let map = parse_ability_template(txt).unwrap();
        assert_eq!(map.get("description").unwrap(), "Visible text");
        assert_eq!(map.get("skill").unwrap(), "I");
    }
}
