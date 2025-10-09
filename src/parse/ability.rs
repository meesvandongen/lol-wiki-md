//! Ability template file parameter parser (Template/Data_<Champion>/<Key>/page.txt)
use crate::error::{Result, ConvertError};
use std::collections::HashMap;

/// Parse an ability data template file content into key -> value map.
/// File format example (simplified):
/// {{AbilityData
/// | name = Dark Flight
/// | cost = 30 / 40 / 50
/// | cooldown = {{#expr: 10 + 2}}
/// | description = Deals {{ap|60}} magic damage
/// }}
pub fn parse_ability_template(content: &str) -> Result<HashMap<String,String>> {
    let start = content.find("{{").ok_or_else(|| ConvertError::MalformedTemplate { name: "AbilityData".into(), detail: "missing opening".into() })?;
    let mut depth = 0i32; let bytes = content.as_bytes(); let mut end = None;
    for i in start..bytes.len() { match bytes[i] as char { '{' if i+1 < bytes.len() && bytes[i+1] as char == '{' => { depth +=1; }, '}' if i+1 < bytes.len() && bytes[i+1] as char == '}' => { depth -=1; if depth==0 { end=Some(i+2); break; } }, _=>{} } }
    let end = end.ok_or_else(|| ConvertError::UnbalancedBraces { name: "AbilityData".into() })?;
    let body = &content[start+2..end-2];
    let inv = super::templates::parse_invocation(body);
    // Expect first segment is AbilityData or similar
    if !inv.name.to_lowercase().contains("ability") { return Err(ConvertError::MalformedTemplate { name: inv.name, detail: "expected ability data template".into() }); }
    // Parameters are of form key = value; we already have each param string
    let mut map = HashMap::new();
    for p in inv.params { if let Some(eq_idx) = p.find('=') { let key = p[..eq_idx].trim(); let val = p[eq_idx+1..].trim(); if key.is_empty() { continue; } if map.insert(key.to_string(), val.to_string()).is_some() { return Err(ConvertError::DuplicateKey(key.to_string())); } } }
    Ok(map)
}

#[cfg(test)]
mod tests { use super::*; #[test] fn parse_basic_ability() { let txt = "{{AbilityData| name = Test | cost = 30 / 40 | cooldown = {{#expr: 5+5}} | description = Deals {{ap|60}} magic damage}}"; let map = parse_ability_template(txt).unwrap(); assert_eq!(map.get("name").unwrap(), "Test"); assert_eq!(map.get("cost").unwrap(), "30 / 40"); assert!(map.get("cooldown").unwrap().contains("#expr")); }}