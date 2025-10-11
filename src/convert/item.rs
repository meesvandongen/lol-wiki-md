use std::collections::HashMap;
use std::path::Path;

use crate::convert::util::{collect_page_vars, expand_inline_templates};
use crate::convert::{write_if_changed, ConversionOutcome};
use crate::error::{ConvertError, Result};
use crate::model::{Item, ItemEffect};
use crate::parse::lua::{lua_value_to_string, lua_value_to_string_vec, parse_item_data, LuaValue};
use crate::parse::templates::TemplateRegistry;
use crate::render::markdown::render_item_markdown;
use crate::wiki_export::WikiExport;

/// Convert a single item into Markdown using Module:ItemData for structured fields.
pub fn convert_item(
    wiki_root: &Path,
    output_dir: &Path,
    name: &str,
    precision: u8,
) -> Result<ConversionOutcome> {
    let export = WikiExport::new(wiki_root);
    let raw = export.read_item_main(name)?;
    let registry = TemplateRegistry::new();
    let vars = collect_page_vars(&raw)?;
    let module = export
        .read_item_module_data()?
        .ok_or_else(|| ConvertError::Internal("Module:ItemData data export not present".into()))?;
    let entry = parse_item_data(&module, name)?;
    let item = build_item_from_entry(name, &entry, precision, &vars, &registry)?;
    let markdown = render_item_markdown(&item, &raw);
    let out_file = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    write_if_changed(&out_file, &markdown)?;
    Ok(ConversionOutcome {
        entity: name.to_string(),
        output: out_file,
    })
}

fn build_item_from_entry(
    name: &str,
    entry: &HashMap<String, LuaValue>,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<Item> {
    let mut item = Item {
        name: name.to_string(),
        ..Item::default()
    };

    if let Some(val) = entry.get("tier").and_then(lua_value_to_string) {
        item.tier = Some(val);
    }
    if let Some(mut categories) = entry.get("type").and_then(lua_value_to_string_vec) {
        categories.retain(|s| !s.trim().is_empty());
        item.categories = categories;
    }
    if let Some(mut recipe) = entry.get("recipe").and_then(lua_value_to_string_vec) {
        recipe.retain(|s| !s.trim().is_empty());
        item.recipe = recipe;
    }
    if let Some(val) = entry
        .get("buy")
        .and_then(lua_value_to_string)
        .and_then(parse_u32)
    {
        item.cost_total = Some(val);
    }
    if let Some(val) = entry
        .get("sell")
        .and_then(lua_value_to_string)
        .and_then(parse_u32)
    {
        item.cost_sell = Some(val);
    }
    if let Some(val) = entry
        .get("sellratio")
        .and_then(lua_value_to_string)
        .and_then(parse_f32)
    {
        item.sell_ratio = Some(val);
        if item.cost_sell.is_none() {
            if let Some(total) = item.cost_total {
                item.cost_sell = Some(((total as f32) * val).round() as u32);
            }
        }
    }
    if let Some(limit) = entry.get("limit").and_then(lua_value_to_string) {
        item.limit = Some(expand_text(&limit, precision, vars, registry)?);
    }
    if let Some(modes) = entry.get("modes") {
        item.modes = collect_enabled_flags(modes)?;
    }
    if let Some(stats) = entry.get("stats") {
        item.stats = collect_stats(stats, precision, vars, registry)?;
    }
    if let Some(effects) = entry.get("effects") {
        item.effects = collect_effects(effects, precision, vars, registry)?;
    }

    Ok(item)
}

fn collect_stats(
    value: &LuaValue,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<HashMap<String, String>> {
    let mut out = HashMap::new();
    if let LuaValue::Table(map) = value {
        for (key, val) in map {
            if let Some(s) = lua_value_to_string(val) {
                let expanded = expand_text(&s, precision, vars, registry)?;
                out.insert(key.clone(), expanded);
            }
        }
    }
    Ok(out)
}

fn collect_effects(
    value: &LuaValue,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<Vec<ItemEffect>> {
    let mut out = Vec::new();
    if let LuaValue::Table(map) = value {
        for (kind_raw, effect_val) in map {
            let LuaValue::Table(body) = effect_val else {
                continue;
            };
            let description = body
                .get("description")
                .and_then(lua_value_to_string)
                .unwrap_or_default();
            let name = body.get("name").and_then(lua_value_to_string);
            let cooldown = body
                .get("cd")
                .or_else(|| body.get("cooldown"))
                .and_then(lua_value_to_string);
            let range = body.get("range").and_then(lua_value_to_string);
            let unique = body
                .get("unique")
                .and_then(lua_value_to_bool)
                .unwrap_or(false);
            let desc_expanded = expand_text(&description, precision, vars, registry)?;
            let name_expanded = match name {
                Some(ref n) => Some(expand_text(n, precision, vars, registry)?),
                None => None,
            };
            let cooldown_expanded = match cooldown {
                Some(ref c) => Some(expand_text(c, precision, vars, registry)?),
                None => None,
            };
            let range_expanded = match range {
                Some(ref r) => Some(expand_text(r, precision, vars, registry)?),
                None => None,
            };
            out.push(ItemEffect {
                kind: normalize_effect_kind(kind_raw),
                name: name_expanded,
                description: desc_expanded,
                cooldown: cooldown_expanded,
                range: range_expanded,
                unique,
            });
        }
    }
    out.sort_by(|a, b| a.kind.cmp(&b.kind));
    Ok(out)
}

fn collect_enabled_flags(value: &LuaValue) -> Result<Vec<String>> {
    let mut enabled = Vec::new();
    if let LuaValue::Table(map) = value {
        for (k, v) in map {
            if lua_value_to_bool(v).unwrap_or(false) {
                let display = k.replace('_', " ");
                enabled.push(display);
            }
        }
    }
    enabled.sort();
    Ok(enabled)
}

fn expand_text(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    expand_inline_templates(trimmed, precision, vars, registry)
}

fn parse_u32(raw: String) -> Option<u32> {
    raw.trim().replace('_', "").parse().ok()
}

fn parse_f32(raw: String) -> Option<f32> {
    raw.trim().parse().ok()
}

fn lua_value_to_bool(value: &LuaValue) -> Option<bool> {
    match value {
        LuaValue::Bool(b) => Some(*b),
        LuaValue::Number(n) => n.parse::<f32>().ok().map(|f| f != 0.0),
        LuaValue::String(s) => {
            let lower = s.trim().to_ascii_lowercase();
            match lower.as_str() {
                "true" | "yes" | "on" => Some(true),
                "false" | "no" | "off" => Some(false),
                _ => None,
            }
        }
        _ => None,
    }
}

fn normalize_effect_kind(raw: &str) -> String {
    let lower = raw.to_ascii_lowercase();
    if lower.starts_with("pass") {
        "Passive".to_string()
    } else if lower.starts_with("act") {
        "Active".to_string()
    } else if lower.starts_with("consume") {
        "Consume".to_string()
    } else {
        raw.to_string()
    }
}
