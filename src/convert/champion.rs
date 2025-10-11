use crate::convert::util::{collect_page_vars, expand_inline_templates, expand_with_vars};
use crate::convert::{write_if_changed, ConversionOutcome};
use crate::error::{ConvertError, Result};
use crate::model::{Ability, AbilityKey, BasicInfo, Champion, StatLine, Stats};
use crate::parse::lua::parse_champion_data;
use crate::parse::templates::TemplateRegistry;
use crate::parse::{extract_balanced_templates, parse_ability_template};
use crate::render::markdown::render_champion_markdown;
use crate::wiki_export::WikiExport;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use tracing::info;

/// Convert a single champion (minimal stub). Looks for `Main/<Name>/page.txt`.
pub fn convert_champion(
    wiki_root: &Path,
    output_dir: &Path,
    name: &str,
    precision: u8,
) -> Result<ConversionOutcome> {
    let export = WikiExport::new(wiki_root);
    if std::env::var("LOL_MD_DEBUG_LIST_ROOT").ok().as_deref() == Some("1") {
        println!(
            "[convert_champion] wiki_root={} export.root={}",
            wiki_root.display(),
            export.root.display()
        );
        if let Ok(rd) = std::fs::read_dir(&export.root) {
            for e in rd.flatten() {
                println!("[convert_champion] root has: {}", e.path().display());
            }
        }
    }
    let raw = export.read_champion_main(name)?;
    let stats = load_champion_stats(&export, name)?;
    // Collect page-level #vardefine vars first, then expand page raw
    let registry = TemplateRegistry::new();
    let vars = collect_page_vars(&raw)?;
    let expanded = expand_with_vars(&raw, precision, &vars, &registry)?;
    // Template inventory (names only) written adjacent to output dir once per champion for now (will refactor to context-wide)
    if let Ok(spans) = extract_balanced_templates(&raw) {
        if !spans.is_empty() {
            let inv_path = output_dir.join("template_inventory.json");
            let mut existing: Vec<String> = if inv_path.exists() {
                serde_json::from_str(&std::fs::read_to_string(&inv_path).unwrap_or_default())
                    .unwrap_or_default()
            } else {
                vec![]
            };
            for s in spans {
                let n = s.name.split('|').next().unwrap_or("").trim();
                if !n.is_empty() && !existing.iter().any(|e| e.eq_ignore_ascii_case(n)) {
                    existing.push(n.to_string());
                }
            }
            existing.sort_unstable();
            existing.dedup();
            let _ = std::fs::write(inv_path, serde_json::to_string_pretty(&existing).unwrap());
        }
    }
    if expanded.contains("{{") {
        return Err(ConvertError::Internal(
            "residual template marker '{{' after expansion".into(),
        ));
    }
    if expanded.to_lowercase().contains("<tabber>") {
        return Err(ConvertError::Internal(
            "residual <tabber> block after expansion".into(),
        ));
    }
    // Load abilities and expand their key fields with same var context
    let abilities = load_abilities(&export, name, precision, &vars, &registry)?;
    let abilities = attach_skill_tabs(&expanded, abilities);
    // Populate basic info (resource if present already gathered inside stats helper)
    let mut basic = BasicInfo::default();
    if let Some(res_line) = stats.base.get("resource") {
        if res_line.base == 0.0 {
            // resource stored as pseudo marker
            basic.resource =
                Some(guess_resource_name(wiki_root, name).unwrap_or_else(|| "Unknown".into()));
        }
    }
    let champion = Champion {
        name: name.to_string(),
        basic,
        stats,
        advanced: None,
        abilities,
        pets: vec![],
        trivia: vec![],
        patch_history: vec![],
        notes: None,
    };
    let markdown = render_champion_markdown(&champion, &expanded);
    let out_file = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    write_if_changed(&out_file, &markdown)?;
    info!(champion = name, "converted (minimal placeholder)");
    Ok(ConversionOutcome {
        entity: name.to_string(),
        output: out_file,
    })
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
                            ab.leveling_tables.push(crate::model::SkillTable {
                                headers: headers.clone(),
                                rows: vec![row.clone()],
                            });
                        }
                    }
                }
            }
        }
    }
    abilities
}

// old minimal renderer removed (superseded by render_champion_markdown)

fn load_champion_stats(export: &WikiExport, name: &str) -> Result<Stats> {
    // Module/ChampionData/data/page.txt; if missing (flat export), return default Stats for now
    let Some(lua) = export.read_champion_module_data()? else {
        return Ok(Stats::default());
    };
    let map = parse_champion_data(&lua, name)?; // flat map
    let mut stats = Stats::default();
    // Map selected keys → StatLine if both base and growth present, else single value growth=0
    let mapping = [
        ("hp", "hpGrowth"),
        ("mp", "mpGrowth"),
        ("ad", "adGrowth"),
        ("armor", "armorGrowth"),
        ("mr", "mrGrowth"),
        ("ms", "msGrowth"),
        ("asBase", "asGrowth"),
        ("range", "rangeGrowth"),
    ];
    for (base_key, growth_key) in mapping {
        if let Some(base) = map.get(base_key) {
            let growth = map
                .get(growth_key)
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0);
            let base_f = base.parse::<f32>().unwrap_or(0.0);
            stats.base.insert(
                base_key.to_string(),
                StatLine {
                    base: base_f,
                    growth,
                },
            );
        }
    }
    // Resource if present stored as a separate pseudo-stat
    if let Some(res) = map.get("resource") {
        stats.base.insert(
            "resource".into(),
            StatLine {
                base: 0.0,
                growth: 0.0,
            },
        ); // marker; actual string captured separately
           // Also include a derived pseudo key for downstream (string accessible separately)
        stats.base.insert(
            format!("resource__{}", res),
            StatLine {
                base: 0.0,
                growth: 0.0,
            },
        );
    }
    Ok(stats)
}

fn guess_resource_name(_root: &Path, _name: &str) -> Option<String> {
    // Placeholder for future: could inspect page excerpt for canonical resource label if not in module.
    // For now rely solely on module value which was injected as pseudo key.
    None
}

fn load_abilities(
    export: &WikiExport,
    champ: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<Vec<Ability>> {
    // Scan the main page for Data <Champion>/<...> templates and load those pages generically.
    let main = export.read_champion_main(champ)?;
    let mut out: Vec<Ability> = Vec::new();
    let spans = match extract_balanced_templates(&main) {
        Ok(s) => s,
        Err(_) => Vec::new(),
    };
    for sp in spans {
        let name_seg = sp.name.trim();
        let lname = name_seg.to_lowercase();
        let champ_l = champ.to_lowercase();
        if !(lname.starts_with("data ") && lname.contains(&format!("{}{}", champ_l, "/"))) {
            continue;
        }
        let template_title = format!("Template:{}", name_seg);
        let Some(raw) = export.read_template_page(&template_title)? else {
            continue;
        };

        let ability_vars = collect_page_vars(&raw)?;
        let mut merged_vars = vars.clone();
        for (k, v) in ability_vars {
            merged_vars.insert(k, v);
        }

        let param_map = match parse_ability_template(&raw) {
            Ok(map) => map,
            Err(_) => continue,
        };
        if param_map.is_empty() {
            continue;
        }

        let mut ability_key = AbilityKey::Other(String::new());
        let mut display_name = param_map.get("name").cloned();
        let mut descriptions_with_order: Vec<(usize, String)> = Vec::new();
        let mut leveling_with_order: Vec<(usize, crate::model::SkillTable)> = Vec::new();
        let mut cooldowns: Vec<String> = Vec::new();
        let mut costs: Vec<String> = Vec::new();
        let mut ranges: Vec<String> = Vec::new();
        let mut notes: Vec<String> = Vec::new();
        let mut extra: HashMap<String, String> = HashMap::new();

        let mut keys: Vec<String> = param_map.keys().cloned().collect();
        keys.sort();

        for key_name in keys {
            let raw_val = param_map.get(&key_name).unwrap();
            let expanded = expand_inline_templates(raw_val, precision, &merged_vars, registry)?;
            let normalized = normalize_ability_value(&expanded);
            match key_name.as_str() {
                "champion" => {}
                "skill" => {
                    if !normalized.is_empty() {
                        ability_key = ability_key_from_slot(&normalized);
                    }
                }
                "name" => {
                    if !normalized.is_empty() {
                        display_name = Some(normalized.clone());
                    }
                }
                k if k.starts_with("description") => {
                    if !normalized.is_empty() {
                        let idx = suffix_index(k, "description");
                        descriptions_with_order.push((idx, normalized.clone()));
                    }
                }
                k if k.starts_with("leveling") => {
                    if let Some(table) = parse_skill_tab_marker(&normalized) {
                        let idx = suffix_index(k, "leveling");
                        leveling_with_order.push((idx, table));
                    } else if !normalized.is_empty() {
                        extra.insert(k.to_string(), normalized.clone());
                    }
                }
                k if is_cooldown_key(k) => {
                    if !normalized.is_empty() {
                        cooldowns.push(normalized.clone());
                        extra.insert(k.to_string(), normalized.clone());
                    }
                }
                k if is_cost_key(k) => {
                    if !normalized.is_empty() {
                        costs.push(normalized.clone());
                        extra.insert(k.to_string(), normalized.clone());
                    }
                }
                k if is_range_key(k) => {
                    if !normalized.is_empty() {
                        ranges.push(normalized.clone());
                        extra.insert(k.to_string(), normalized.clone());
                    }
                }
                k if k.starts_with("notes") => {
                    if !normalized.is_empty() {
                        collect_notes(&normalized, &mut notes);
                    }
                }
                _ => {
                    if !normalized.is_empty() {
                        extra.insert(key_name.to_string(), normalized.clone());
                    }
                }
            }
        }

        if !matches!(
            ability_key,
            AbilityKey::Passive | AbilityKey::Q | AbilityKey::W | AbilityKey::E | AbilityKey::R
        ) {
            continue;
        }

        descriptions_with_order.sort_by_key(|(idx, _)| *idx);
        let descriptions: Vec<String> = descriptions_with_order
            .into_iter()
            .map(|(_, v)| v)
            .collect();

        leveling_with_order.sort_by_key(|(idx, _)| *idx);
        let leveling_tables: Vec<crate::model::SkillTable> =
            leveling_with_order.into_iter().map(|(_, t)| t).collect();

        dedup_preserve_order(&mut cooldowns);
        dedup_preserve_order(&mut costs);
        dedup_preserve_order(&mut ranges);
        dedup_preserve_order(&mut notes);

        let mut display_name = display_name.unwrap_or_else(|| derive_name_from_template(name_seg));
        if display_name.trim().is_empty() {
            display_name = derive_name_from_template(name_seg);
        }
        if display_name.trim().len() <= 1 {
            if let Some(icon_name) = fallback_name_from_icons(&extra) {
                display_name = icon_name;
            }
        }
        let display_name = display_name.trim().to_string();

        let ability_key_final = ability_key.clone();
        let candidate = Ability {
            key: ability_key_final.clone(),
            name: display_name,
            descriptions,
            cooldowns,
            costs,
            ranges,
            leveling_tables,
            notes,
            extra,
        };

        if let Some(existing) = out.iter_mut().find(|a| a.key == ability_key_final) {
            if existing.name.len() <= 1 && candidate.name.len() > 1 {
                existing.name = candidate.name.clone();
            }
            if candidate.descriptions.len() > existing.descriptions.len() {
                existing.descriptions = candidate.descriptions.clone();
            }
            if candidate.leveling_tables.len() > existing.leveling_tables.len() {
                existing.leveling_tables = candidate.leveling_tables.clone();
            }
            if candidate.cooldowns.len() > existing.cooldowns.len() {
                existing.cooldowns = candidate.cooldowns.clone();
            }
            if candidate.costs.len() > existing.costs.len() {
                existing.costs = candidate.costs.clone();
            }
            if candidate.ranges.len() > existing.ranges.len() {
                existing.ranges = candidate.ranges.clone();
            }
            for note in &candidate.notes {
                if !existing.notes.contains(note) {
                    existing.notes.push(note.clone());
                }
            }
            for (k, v) in &candidate.extra {
                existing.extra.entry(k.clone()).or_insert_with(|| v.clone());
            }
        } else {
            out.push(candidate);
        }
    }
    out.sort_by_key(|a| match a.key {
        AbilityKey::Passive => 0,
        AbilityKey::Q => 1,
        AbilityKey::W => 2,
        AbilityKey::E => 3,
        AbilityKey::R => 4,
        AbilityKey::Other(_) => 9,
    });
    Ok(out)
}

fn normalize_ability_value(raw: &str) -> String {
    if raw.trim().is_empty() {
        return String::new();
    }
    let mut s = raw.replace("\r\n", "\n");
    for pattern in ["<br />", "<br/>", "<br>", "<BR />", "<BR/>", "<BR>"] {
        s = s.replace(pattern, "\n");
    }
    s = s.replace("&nbsp;", " ");
    s = s.replace("&#160;", " ");
    s.trim().to_string()
}

fn ability_key_from_slot(slot: &str) -> AbilityKey {
    let trimmed = slot.trim();
    if trimmed.is_empty() {
        return AbilityKey::Other(String::new());
    }
    match trimmed.to_ascii_uppercase().as_str() {
        "I" | "P" | "PASSIVE" => AbilityKey::Passive,
        "Q" => AbilityKey::Q,
        "W" => AbilityKey::W,
        "E" => AbilityKey::E,
        "R" | "ULTIMATE" => AbilityKey::R,
        _ => AbilityKey::Other(trimmed.to_string()),
    }
}

fn suffix_index(key: &str, prefix: &str) -> usize {
    if key == prefix {
        return 0;
    }
    let rest = key.strip_prefix(prefix).unwrap_or("");
    let digits: String = rest
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        1
    } else {
        digits.parse().unwrap_or(usize::MAX - 1)
    }
}

fn parse_skill_tab_marker(value: &str) -> Option<crate::model::SkillTable> {
    if !value.starts_with("[SkillTab ") || !value.ends_with(']') {
        return None;
    }
    let inner = &value[10..value.len() - 1];
    let mut headers = Vec::new();
    let mut row = Vec::new();
    for part in inner.split('|') {
        let part = part.trim();
        if let Some(colon) = part.find(':') {
            let key = part[..colon].trim();
            let val = part[colon + 1..].trim();
            if key.eq_ignore_ascii_case("h") {
                headers.push(val.to_string());
            } else if key.eq_ignore_ascii_case("r") {
                row.push(val.to_string());
            }
        }
    }
    if headers.is_empty() || row.is_empty() {
        return None;
    }
    Some(crate::model::SkillTable {
        headers,
        rows: vec![row],
    })
}

fn is_cooldown_key(key: &str) -> bool {
    key == "cd" || key == "cdstart" || key.contains("cooldown")
}

fn is_cost_key(key: &str) -> bool {
    key.contains("cost")
}

fn is_range_key(key: &str) -> bool {
    matches!(
        key,
        "range"
            | "target range"
            | "width"
            | "effect radius"
            | "ai range"
            | "attack range"
            | "collision radius"
            | "inner radius"
            | "tether radius"
    )
}

fn collect_notes(value: &str, out: &mut Vec<String>) {
    let mut current: Option<String> = None;
    for line in value.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('*') {
            if let Some(existing) = current.take() {
                if !existing.trim().is_empty() {
                    out.push(existing);
                }
            }
            current = Some(trimmed.to_string());
        } else if let Some(existing) = current.as_mut() {
            existing.push(' ');
            existing.push_str(trimmed);
        } else {
            current = Some(trimmed.to_string());
        }
    }
    if let Some(existing) = current {
        if !existing.trim().is_empty() {
            out.push(existing);
        }
    }
}

fn dedup_preserve_order(values: &mut Vec<String>) {
    let mut seen = HashSet::new();
    values.retain(|v| seen.insert(v.clone()));
}

fn fallback_name_from_icons(extra: &HashMap<String, String>) -> Option<String> {
    for key in ["blurbicon", "icon", "icon2", "icon3"] {
        if let Some(val) = extra.get(key) {
            let trimmed = val.trim();
            if trimmed.is_empty() {
                continue;
            }
            let stem = trimmed
                .trim_end_matches(".png")
                .trim_end_matches(".jpg")
                .trim_end_matches(".jpeg")
                .replace('_', " ");
            if !stem.trim().is_empty() {
                return Some(stem.trim().to_string());
            }
        }
    }
    None
}

fn derive_name_from_template(name_seg: &str) -> String {
    if let Some(pos) = name_seg.rfind('/') {
        name_seg[pos + 1..].replace('_', " ")
    } else {
        String::new()
    }
}
