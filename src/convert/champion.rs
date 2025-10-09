use std::path::Path;
use tracing::info;
use crate::error::{Result, ConvertError};
use crate::model::{Champion, BasicInfo, Stats, StatLine, Ability, AbilityKey};
use crate::convert::{ConversionOutcome, write_if_changed};
use crate::parse::{extract_balanced_templates};
use crate::parse::templates::{parse_invocation, TemplateRegistry, ExpanderCtx};
use crate::parse::lua::parse_champion_data;
use crate::render::markdown::render_champion_markdown;
use std::collections::HashMap;
use crate::wiki_export::WikiExport;

/// Convert a single champion (minimal stub). Looks for `Main/<Name>/page.txt`.
pub fn convert_champion(wiki_root: &Path, output_dir: &Path, name: &str, precision: u8) -> Result<ConversionOutcome> {
    let export = WikiExport::new(wiki_root);
    if std::env::var("LOL_MD_DEBUG_LIST_ROOT").ok().as_deref() == Some("1") {
        println!("[convert_champion] wiki_root={} export.root={}", wiki_root.display(), export.root.display());
        if let Ok(rd) = std::fs::read_dir(&export.root) {
            for e in rd.flatten() { println!("[convert_champion] root has: {}", e.path().display()); }
        }
    }
    let raw = export.read_champion_main(name)?;
    let stats = load_champion_stats(&export, name)?;
    // Collect page-level #vardefine vars first, then expand page raw
    let registry = TemplateRegistry::new();
    let vars = collect_vars(&raw)?;
    let expanded = expand_with_vars(&raw, precision, &vars, &registry)?;
    // Template inventory (names only) written adjacent to output dir once per champion for now (will refactor to context-wide)
    if let Ok(spans) = extract_balanced_templates(&raw) { if !spans.is_empty() { let inv_path = output_dir.join("template_inventory.json");
        let mut existing: Vec<String> = if inv_path.exists() { serde_json::from_str(&std::fs::read_to_string(&inv_path).unwrap_or_default()).unwrap_or_default() } else { vec![] };
        for s in spans { let n = s.name.split('|').next().unwrap_or("").trim(); if !n.is_empty() && !existing.iter().any(|e| e.eq_ignore_ascii_case(n)) { existing.push(n.to_string()); } }
        existing.sort_unstable(); existing.dedup(); let _ = std::fs::write(inv_path, serde_json::to_string_pretty(&existing).unwrap()); } }
    if expanded.contains("{{") { return Err(ConvertError::Internal("residual template marker '{{' after expansion".into())); }
    if expanded.to_lowercase().contains("<tabber>") { return Err(ConvertError::Internal("residual <tabber> block after expansion".into())); }
    // Load abilities and expand their key fields with same var context
    let abilities = load_abilities(&export, name, precision, &vars, &registry)?;
    let abilities = attach_skill_tabs(&expanded, abilities);
    // Populate basic info (resource if present already gathered inside stats helper)
    let mut basic = BasicInfo::default();
    if let Some(res_line) = stats.base.get("resource") { if res_line.base == 0.0 { // resource stored as pseudo marker
        basic.resource = Some(guess_resource_name(wiki_root, name).unwrap_or_else(|| "Unknown".into()));
    }}
    let champion = Champion { name: name.to_string(), basic, stats, advanced: None, abilities, pets: vec![], trivia: vec![], patch_history: vec![], notes: None };
    let markdown = render_champion_markdown(&champion, &expanded);
    let out_file = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    write_if_changed(&out_file, &markdown)?;
    info!(champion = name, "converted (minimal placeholder)");
    Ok(ConversionOutcome { entity: name.to_string(), output: out_file })
}

fn attach_skill_tabs(page_expanded: &str, mut abilities: Vec<Ability>) -> Vec<Ability> {
    // Very naive extraction: scan for markers produced by SkillTabExpander: [SkillTab key:value | key:value]
    for line in page_expanded.lines() {
        if let Some(idx) = line.find("[SkillTab ") {
            let rest = &line[idx + 10..];
            if let Some(end) = rest.find(']') {
                let body = &rest[..end];
                let mut slot: Option<String> = None;
                let mut headers: Vec<String> = Vec::new();
                let mut row: Vec<String> = Vec::new();
                for part in body.split('|') {
                    let part = part.trim();
                    if let Some(colon) = part.find(':') {
                        let key = part[..colon].trim().to_lowercase();
                        let val = part[colon + 1..].trim();
                        match key.as_str() {
                            "slot" => slot = Some(val.to_string()),
                            "h" => headers.push(val.to_string()),
                            "r" => row.push(val.to_string()),
                            _ => {}
                        }
                    }
                }
                if let Some(slot_name) = slot {
                    let ability_key = match slot_name.as_str() {
                        s if s.eq_ignore_ascii_case("passive") => AbilityKey::Passive,
                        s if s.eq_ignore_ascii_case("q") => AbilityKey::Q,
                        s if s.eq_ignore_ascii_case("w") => AbilityKey::W,
                        s if s.eq_ignore_ascii_case("e") => AbilityKey::E,
                        s if s.eq_ignore_ascii_case("r") => AbilityKey::R,
                        other => AbilityKey::Other(other.to_string()),
                    };
                    if let Some(ab) = abilities.iter_mut().find(|a| a.key == ability_key) {
                        if !headers.is_empty() && !row.is_empty() {
                            ab.leveling_tables.push(crate::model::SkillTable { headers: headers.clone(), rows: vec![row.clone()] });
                        }
                    }
                }
            }
        }
    }
    abilities
}

// old minimal renderer removed (superseded by render_champion_markdown)

fn collect_vars(raw: &str) -> Result<HashMap<String,String>> {
    let mut vars = HashMap::new();
    if let Ok(spans) = extract_balanced_templates(raw) {
        for span in spans {
            let body = &span.raw[2..span.raw.len()-2];
            let inv = parse_invocation(body);
            if inv.name.eq_ignore_ascii_case("#vardefine") && inv.params.len() >= 2 { vars.insert(inv.params[0].clone(), inv.params[1].clone()); }
        }
    }
    Ok(vars)
}

fn expand_with_vars(raw: &str, precision: u8, vars: &HashMap<String,String>, registry: &TemplateRegistry) -> Result<String> {
    if let Ok(spans) = extract_balanced_templates(raw) {
        let mut output = String::new(); let mut last=0usize; let ctx = ExpanderCtx { precision, vars: vars.clone() };
        for span in spans {
            output.push_str(&raw[last..span.start]);
            let body = &span.raw[2..span.raw.len()-2];
            let inv = parse_invocation(body);
            let exp = registry.expand(&inv, &ctx)?;
            output.push_str(&exp.expanded);
            last = span.end;
        }
        output.push_str(&raw[last..]);
        Ok(output)
    } else { Ok(raw.to_string()) }
}

fn load_champion_stats(export: &WikiExport, name: &str) -> Result<Stats> {
    // Module/ChampionData/data/page.txt; if missing (flat export), return default Stats for now
    let Some(lua) = export.read_champion_module_data()? else { return Ok(Stats::default()) };
    let map = parse_champion_data(&lua, name)?; // flat map
    let mut stats = Stats::default();
    // Map selected keys → StatLine if both base and growth present, else single value growth=0
    let mapping = [
        ("hp","hpGrowth"), ("mp","mpGrowth"), ("ad","adGrowth"), ("armor","armorGrowth"), ("mr","mrGrowth"), ("ms", "msGrowth"), ("asBase","asGrowth"), ("range","rangeGrowth")
    ];
    for (base_key,growth_key) in mapping { if let Some(base) = map.get(base_key) { let growth = map.get(growth_key).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0); let base_f = base.parse::<f32>().unwrap_or(0.0); stats.base.insert(base_key.to_string(), StatLine { base: base_f, growth }); } }
    // Resource if present stored as a separate pseudo-stat
    if let Some(res) = map.get("resource") { stats.base.insert("resource".into(), StatLine { base: 0.0, growth: 0.0 }); // marker; actual string captured separately
        // Also include a derived pseudo key for downstream (string accessible separately)
        stats.base.insert(format!("resource__{}", res), StatLine { base: 0.0, growth: 0.0 });
    }
    Ok(stats)
}

fn guess_resource_name(_root: &Path, _name: &str) -> Option<String> {
    // Placeholder for future: could inspect page excerpt for canonical resource label if not in module.
    // For now rely solely on module value which was injected as pseudo key.
    None
}

fn load_abilities(_export: &WikiExport, _champ: &str, _precision: u8, _vars: &HashMap<String,String>, _registry: &TemplateRegistry) -> Result<Vec<Ability>> {
    // Flat export_out no longer contains nested Template/Data_<Champion>/... ability templates.
    // Future work: parse abilities directly from main page sections.
    Ok(vec![])
}
