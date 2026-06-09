use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;

use crate::convert::util::{
    collect_list_items, collect_page_vars, expand_inline_templates, expand_inline_templates_mut,
    extract_patch_history, lua_table_to_pretty_json,
};
use crate::convert::{write_markdown_with_plain_text, ConversionOutcome};
use crate::error::{ConvertError, Result};
use crate::model::{Item, ItemEffect, SourceAppendix};
use crate::parse::lua::{lua_value_to_string, lua_value_to_string_vec, LuaValue};
use crate::parse::tables::wikitext_table_to_markdown;
use crate::parse::templates::{parse_invocation, TemplateRegistry};
use crate::render::markdown::{detect_renderer_cleanup_template_names, render_item_markdown};

use super::context::ConversionContext;

/// Convert a single item into Markdown using Module:ItemData for structured fields.
pub(super) fn convert_item(
    ctx: &ConversionContext,
    output_dir: &Path,
    name: &str,
) -> Result<ConversionOutcome> {
    let export = ctx.export();
    let raw = export.read_item_main(name)?;
    let module = ctx.item_module_map()?;
    let (module_name, entry) = resolve_item_entry(module, name, &raw)
        .ok_or_else(|| ConvertError::ItemNotFound(name.to_string()))?;
    let out_file = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    // Removed items live in `Module:ItemData/data/removed` and carry a `removed`
    // marker. They are excluded from the output by default; only emit them when
    // the caller explicitly opts in via `--include-removed`.
    if entry.contains_key("removed") && !ctx.include_removed() {
        return Ok(ConversionOutcome {
            entity: name.to_string(),
            output: out_file,
            skipped: true,
        });
    }
    let registry = ctx.registry();
    let precision = ctx.precision();
    let vars = collect_page_vars(&raw)?;
    if let Ok(spans) = crate::parse::extract_balanced_templates(&raw) {
        if !spans.is_empty() {
            let names = spans
                .into_iter()
                .map(|span| span.name.split('|').next().unwrap_or("").trim().to_string())
                .filter(|n| !n.is_empty())
                .collect::<Vec<_>>();
            ctx.record_templates(names);
        }
    }
    let mut item = build_item_from_entry(ctx, name, &raw, &entry, precision, &vars, registry)?;
    item.source_appendices =
        collect_item_source_appendices(export, name, &raw, module_name, entry)?;
    let markdown = render_item_markdown(&item, &raw);
    write_markdown_with_plain_text(&out_file, &markdown)?;
    if let Ok(rel) = out_file.strip_prefix(output_dir) {
        let artifact = rel.to_string_lossy().replace('\\', "/");
        ctx.set_specimen_sample("item", &artifact);
    } else {
        let artifact = out_file.to_string_lossy().replace('\\', "/");
        ctx.set_specimen_sample("item", &artifact);
    }
    Ok(ConversionOutcome {
        entity: name.to_string(),
        output: out_file,
        skipped: false,
    })
}

fn resolve_item_entry<'a>(
    module: &'a HashMap<String, HashMap<String, LuaValue>>,
    page_name: &str,
    raw: &str,
) -> Option<(&'a str, &'a HashMap<String, LuaValue>)> {
    if let Some(title) = item_module_title_from_page(raw) {
        if let Some(found) = find_item_entry(module, &title) {
            return Some(found);
        }
    }
    find_item_entry(module, page_name)
}

fn find_item_entry<'a>(
    module: &'a HashMap<String, HashMap<String, LuaValue>>,
    item: &str,
) -> Option<(&'a str, &'a HashMap<String, LuaValue>)> {
    module
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(item))
        .map(|(key, entry)| (key.as_str(), entry))
        .or_else(|| {
            module
                .iter()
                .find(|(_, entry)| item_alias_matches(entry, item))
                .map(|(key, entry)| (key.as_str(), entry))
        })
}

fn item_alias_matches(entry: &HashMap<String, LuaValue>, item: &str) -> bool {
    for key in ["formatname", "name", "fullname"] {
        if let Some(candidate) = entry.get(key).and_then(lua_value_to_string) {
            if candidate.eq_ignore_ascii_case(item) {
                return true;
            }
        }
    }
    false
}

fn item_module_title_from_page(raw: &str) -> Option<String> {
    let spans = crate::parse::extract_balanced_templates(raw).ok()?;
    let span = spans.into_iter().find(is_item_info_template)?;
    let body = &span.raw[2..span.raw.len() - 2];
    let inv = parse_invocation(body);
    inv.params
        .into_iter()
        .find(|param| !param.contains('='))
        .map(|param| param.trim().to_string())
        .filter(|param| !param.is_empty())
}

fn build_item_from_entry(
    ctx: &ConversionContext,
    name: &str,
    raw: &str,
    entry: &HashMap<String, LuaValue>,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<Item> {
    let mut item = Item {
        name: name.to_string(),
        ..Item::default()
    };
    let mut warnings: Vec<String> = Vec::new();

    if let Some(val) = entry.get("tier").and_then(lua_value_to_string) {
        item.tier = Some(val);
    }
    if let Some(mut categories) = entry.get("type").and_then(lua_value_to_string_vec) {
        categories.retain(|s| !s.trim().is_empty());
        item.categories = normalize_categories(categories);
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
    if let Some(caption) = entry.get("caption").and_then(lua_value_to_string) {
        let expanded = expand_text(
            &caption,
            precision,
            vars,
            registry,
            Some(Arc::new(ctx.clone())),
        )?;
        push_render_cleanup_warning(
            &mut warnings,
            &format!("item caption for `{name}`"),
            &expanded,
        );
        if !expanded.trim().is_empty() {
            item.caption = Some(expanded.trim().to_string());
        }
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
        item.limit = Some(expand_text(
            &limit,
            precision,
            vars,
            registry,
            Some(Arc::new(ctx.clone())),
        )?);
    }
    item.ornn_forged = entry
        .get("ornn")
        .and_then(lua_value_to_bool)
        .unwrap_or(false);
    if let Some(removed_patch) = entry.get("removed").and_then(lua_value_to_string) {
        let expanded = expand_text(
            &removed_patch,
            precision,
            vars,
            registry,
            Some(Arc::new(ctx.clone())),
        )?;
        if !expanded.trim().is_empty() {
            item.removed_patch = Some(expanded.trim().to_string());
        }
    }
    if let Some(modes) = entry.get("modes") {
        let flags = collect_enabled_flags(modes)?;
        item.modes = normalize_modes(flags);
    }
    if let Some(stats) = entry.get("stats") {
        item.stats = collect_stats(
            name,
            stats,
            precision,
            vars,
            registry,
            Some(Arc::new(ctx.clone())),
            &mut warnings,
        )?;
    }
    if let Some(effects) = entry.get("effects") {
        item.effects = collect_effects(
            name,
            effects,
            precision,
            vars,
            registry,
            Some(Arc::new(ctx.clone())),
            &mut warnings,
        )?;
    }

    let mut page_vars = vars.clone();
    page_vars
        .entry("__item_name".to_string())
        .or_insert_with(|| name.to_string());
    seed_item_gold_value_vars(ctx, entry, &mut page_vars)?;

    let page_info = parse_item_page_info(
        name,
        raw,
        precision,
        &page_vars,
        registry,
        Some(Arc::new(ctx.clone())),
    );
    item.gold_value = page_info.gold_value;
    item.gold_efficiency = page_info.gold_efficiency;
    item.similar_items = page_info.similar_items;
    item.background = page_info.background;
    item.strategy = page_info.strategy;
    item.notes = page_info.notes;
    item.trivia = page_info.trivia;
    item.patch_history = page_info.patch_history;
    warnings.extend(page_info.warnings);

    let (combine_cost, mut combine_warnings) =
        compute_combine_cost(ctx, name, &item.recipe, item.cost_total)?;
    item.cost_combine = combine_cost;
    warnings.append(&mut combine_warnings);

    item.upgrades = compute_upgrades(ctx, name)?;
    warnings.sort();
    warnings.dedup();
    item.warnings = warnings;

    Ok(item)
}

#[derive(Debug, Default)]
struct ItemPageInfo {
    gold_value: Option<String>,
    gold_efficiency: Option<String>,
    similar_items: Vec<String>,
    background: Option<String>,
    strategy: Option<String>,
    notes: Vec<String>,
    trivia: Vec<String>,
    patch_history: Vec<crate::model::PatchEntry>,
    warnings: Vec<String>,
}

fn parse_item_page_info(
    item_name: &str,
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> ItemPageInfo {
    let mut info = ItemPageInfo::default();
    let mut page_vars = vars.clone();
    let spans = crate::parse::extract_balanced_templates(raw).unwrap_or_default();
    let Some(span) = spans.into_iter().find(is_item_info_template) else {
        return info;
    };

    let body = &span.raw[2..span.raw.len() - 2];
    let inv = parse_invocation(body);
    for param in inv.params {
        let Some(eq) = param.find('=') else {
            continue;
        };
        let key = param[..eq].trim().to_ascii_lowercase();
        let value_raw = param[eq + 1..].trim();
        if key.is_empty() || value_raw.is_empty() {
            continue;
        }

        match key.as_str() {
            "goldvalue" => {
                info.gold_value = expand_item_page_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                );
            }
            "goldefficiency" => {
                info.gold_efficiency = expand_item_page_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                );
            }
            "similaritems" => {
                if let Some(expanded) = expand_item_page_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                ) {
                    info.similar_items = expanded
                        .split(';')
                        .map(str::trim)
                        .filter(|entry| !entry.is_empty())
                        .map(ToString::to_string)
                        .collect();
                    dedup_preserve_order(&mut info.similar_items);
                }
            }
            "background" => {
                info.background = expand_item_page_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                );
            }
            "strategy" => {
                info.strategy = expand_item_page_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                );
            }
            "notes" => {
                info.notes = expand_item_page_list_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                );
            }
            "trivia" => {
                info.trivia = expand_item_page_list_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                );
            }
            "patchhistory" => {
                if let Some(expanded) = expand_item_page_field(
                    item_name,
                    &key,
                    value_raw,
                    precision,
                    &mut page_vars,
                    registry,
                    conversion_ctx.clone(),
                    &mut info.warnings,
                ) {
                    info.patch_history = extract_patch_history(&expanded);
                }
            }
            "media" | "revisions" => {
                let _ = (item_name, key);
            }
            _ => {}
        }
    }
    info.warnings.sort();
    info.warnings.dedup();
    info
}

fn seed_item_gold_value_vars(
    ctx: &ConversionContext,
    entry: &HashMap<String, LuaValue>,
    vars: &mut HashMap<String, String>,
) -> Result<()> {
    let gold_values = ctx.gold_value_data_map()?;
    if gold_values.is_empty() {
        return Ok(());
    }

    for (stat_key, data) in gold_values {
        if let Some(value) = data.get("val").and_then(lua_value_to_string) {
            vars.entry(stat_key.clone()).or_insert(value);
        }
    }

    if !vars.contains_key("total") {
        if let Some(total) =
            compute_item_base_gold_value(ctx.item_module_map()?, entry, gold_values)
        {
            vars.insert("total".to_string(), total);
        }
    }

    Ok(())
}

fn compute_item_base_gold_value(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    entry: &HashMap<String, LuaValue>,
    gold_values: &HashMap<String, HashMap<String, LuaValue>>,
) -> Option<String> {
    let LuaValue::Table(stats) = entry.get("stats")? else {
        return None;
    };

    let mut sum = 0.0f64;
    let mut saw_value = false;
    for (stat_key, stat_value) in stats {
        let normalized_key = stat_key.trim_end_matches("unique").to_ascii_lowercase();
        if matches!(normalized_key.as_str(), "gp10" | "spec" | "spec2") {
            continue;
        }
        let gold_entry = get_case_insensitive_value(gold_values, &normalized_key)?;
        let unit_value = gold_entry
            .get("val")
            .and_then(lua_value_to_string)
            .and_then(|value| value.trim().parse::<f64>().ok())?;
        let stat_amount =
            resolve_item_stat_number_from_value(module, stat_key, stat_value, &mut HashSet::new())?;
        sum += stat_amount * unit_value;
        saw_value = true;
    }

    if saw_value {
        Some(format_gold_number(sum))
    } else {
        None
    }
}

fn resolve_item_stat_number_from_value(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    stat_key: &str,
    value: &LuaValue,
    visited: &mut HashSet<(String, String)>,
) -> Option<f64> {
    match value {
        LuaValue::Number(raw) | LuaValue::String(raw) => {
            let trimmed = raw.trim();
            if let Some(target) = trimmed.strip_prefix("=>") {
                return resolve_item_stat_number(module, target.trim(), stat_key, visited);
            }
            trimmed.parse::<f64>().ok()
        }
        LuaValue::Bool(true) => Some(1.0),
        LuaValue::Bool(false) => Some(0.0),
        _ => None,
    }
}

fn resolve_item_stat_number(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    item_name: &str,
    stat_key: &str,
    visited: &mut HashSet<(String, String)>,
) -> Option<f64> {
    let visit_key = (
        item_name.trim().to_ascii_lowercase(),
        stat_key.trim().to_ascii_lowercase(),
    );
    if !visited.insert(visit_key) {
        return None;
    }

    let (_, entry) = find_item_entry(module, item_name)?;
    let LuaValue::Table(stats) = entry.get("stats")? else {
        return None;
    };
    let value = get_case_insensitive_value(stats, stat_key)?;
    resolve_item_stat_number_from_value(module, stat_key, value, visited)
}

fn get_case_insensitive_value<'a, T>(map: &'a HashMap<String, T>, key: &str) -> Option<&'a T> {
    map.get(key).or_else(|| {
        map.iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| value)
    })
}

fn format_gold_number(value: f64) -> String {
    let rounded = (value * 100.0).round() / 100.0;
    let mut rendered = format!("{rounded:.2}");
    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }
    rendered
}

fn is_item_info_template(span: &crate::parse::brace::TemplateSpan) -> bool {
    let name = span.name.trim().to_ascii_lowercase();
    name == "item info"
}

fn expand_item_page_field(
    item_name: &str,
    field_name: &str,
    value_raw: &str,
    precision: u8,
    vars: &mut HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
    warnings: &mut Vec<String>,
) -> Option<String> {
    match expand_inline_templates_mut(value_raw, precision, vars, registry, conversion_ctx) {
        Ok(expanded) => {
            push_render_cleanup_warning(
                warnings,
                &format!("`Item info` field `{field_name}` for `{item_name}`"),
                &expanded,
            );
            let trimmed = expanded.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        }
        Err(err) => {
            warnings.push(format!(
                "Could not expand `{field_name}` from `Item info` for `{item_name}`: {err}"
            ));
            None
        }
    }
}

fn expand_item_page_list_field(
    item_name: &str,
    field_name: &str,
    value_raw: &str,
    precision: u8,
    vars: &mut HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
    warnings: &mut Vec<String>,
) -> Vec<String> {
    let Some(expanded) = expand_item_page_field(
        item_name,
        field_name,
        value_raw,
        precision,
        vars,
        registry,
        conversion_ctx,
        warnings,
    ) else {
        return Vec::new();
    };

    let mut items = collect_list_items(&expanded);
    if items.is_empty() {
        items.push(format!("* {}", expanded.trim()));
    }
    dedup_preserve_order(&mut items);
    items
}

fn dedup_preserve_order(values: &mut Vec<String>) {
    let mut seen = HashSet::new();
    values.retain(|value| seen.insert(value.clone()));
}

fn push_render_cleanup_warning(warnings: &mut Vec<String>, scope: &str, expanded: &str) {
    let leaked_templates = detect_renderer_cleanup_template_names(expanded);
    if leaked_templates.is_empty() {
        return;
    }
    warnings.push(format!(
        "Supported template markup survived parser expansion in {scope} and required render-layer cleanup: {}.",
        leaked_templates.join(", ")
    ));
}

fn collect_stats(
    item_name: &str,
    value: &LuaValue,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
    warnings: &mut Vec<String>,
) -> Result<HashMap<String, String>> {
    let mut out = HashMap::new();
    if let LuaValue::Table(map) = value {
        for (key, val) in map {
            if let Some(s) = lua_value_to_string(val) {
                let expanded = expand_text(&s, precision, vars, registry, conversion_ctx.clone())?;
                push_render_cleanup_warning(
                    warnings,
                    &format!("item stat `{key}` for `{item_name}`"),
                    &expanded,
                );
                out.insert(key.clone(), expanded);
            }
        }
    }
    Ok(out)
}

fn collect_effects(
    item_name: &str,
    value: &LuaValue,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
    warnings: &mut Vec<String>,
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
            let desc_expanded = expand_text(
                &description,
                precision,
                vars,
                registry,
                conversion_ctx.clone(),
            )?;
            push_render_cleanup_warning(
                warnings,
                &format!("effect description `{kind_raw}` for `{item_name}`"),
                &desc_expanded,
            );
            let desc_with_tables =
                wikitext_table_to_markdown(&desc_expanded).unwrap_or(desc_expanded);
            let name_expanded = match name {
                Some(ref n) => {
                    let expanded =
                        expand_text(n, precision, vars, registry, conversion_ctx.clone())?;
                    push_render_cleanup_warning(
                        warnings,
                        &format!("effect name `{kind_raw}` for `{item_name}`"),
                        &expanded,
                    );
                    Some(expanded)
                }
                None => None,
            };
            let cooldown_expanded = match cooldown {
                Some(ref c) => {
                    let expanded =
                        expand_text(c, precision, vars, registry, conversion_ctx.clone())?;
                    push_render_cleanup_warning(
                        warnings,
                        &format!("effect cooldown `{kind_raw}` for `{item_name}`"),
                        &expanded,
                    );
                    Some(expanded)
                }
                None => None,
            };
            let range_expanded = match range {
                Some(ref r) => {
                    let expanded =
                        expand_text(r, precision, vars, registry, conversion_ctx.clone())?;
                    push_render_cleanup_warning(
                        warnings,
                        &format!("effect range `{kind_raw}` for `{item_name}`"),
                        &expanded,
                    );
                    Some(expanded)
                }
                None => None,
            };
            out.push(ItemEffect {
                kind: normalize_effect_kind(kind_raw),
                name: name_expanded,
                description: desc_with_tables,
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
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }
    expand_inline_templates(trimmed, precision, vars, registry, conversion_ctx)
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

fn normalize_categories(categories: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for cat in categories {
        let cleaned = cat.replace('_', " ").trim().to_string();
        if cleaned.is_empty() {
            continue;
        }
        let canonical = match cleaned.to_ascii_lowercase().as_str() {
            "starter" => "Starter".to_string(),
            "basic" => "Basic".to_string(),
            "advanced" => "Advanced".to_string(),
            "legendary" => "Legendary".to_string(),
            "mythic" => "Mythic".to_string(),
            "epic" => "Epic".to_string(),
            "consumable" => "Consumable".to_string(),
            other => title_case(other),
        };
        if !out.iter().any(|s| s.eq_ignore_ascii_case(&canonical)) {
            out.push(canonical);
        }
    }
    out.sort();
    out
}

fn normalize_modes(modes: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for mode in modes {
        let cleaned = mode.trim();
        if cleaned.is_empty() {
            continue;
        }
        let canonical = match cleaned.to_ascii_lowercase().as_str() {
            "classic sr 5v5" | "classic sr" | "sr" | "summoner's rift" | "summoners rift" => {
                "Summoner's Rift".to_string()
            }
            "aram" | "ha" | "howling abyss" => "Howling Abyss".to_string(),
            "nb" => "Nexus Blitz".to_string(),
            "ar" | "arena" => "Arena".to_string(),
            "urf" => "Ultra Rapid Fire".to_string(),
            other => title_case(other),
        };
        if !out.iter().any(|s| s.eq_ignore_ascii_case(&canonical)) {
            out.push(canonical);
        }
    }
    out.sort();
    out
}

fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                let mut upper = first.to_uppercase().collect::<String>();
                upper.push_str(&chars.as_str().to_lowercase());
                upper
            } else {
                String::new()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn compute_combine_cost(
    ctx: &ConversionContext,
    item_name: &str,
    components: &[String],
    total_cost: Option<u32>,
) -> Result<(Option<u32>, Vec<String>)> {
    if components.is_empty() || total_cost.is_none() {
        return Ok((None, Vec::new()));
    }
    let total = total_cost.unwrap();
    let module = ctx.item_module_map()?;
    let mut sum: u32 = 0;
    let mut warnings: Vec<String> = Vec::new();
    for comp in components {
        if let Some(data) = module.get(comp) {
            let cost = data
                .get("buy")
                .and_then(lua_value_to_string)
                .and_then(parse_u32);
            match cost {
                Some(val) => sum = sum.saturating_add(val),
                None => warnings.push(format!(
                    "Missing cost for component `{comp}` while computing combine cost for `{item_name}`"
                )),
            }
        } else {
            warnings.push(format!(
                "Component `{comp}` not found in Module:ItemData while computing combine cost for `{item_name}`"
            ));
        }
    }
    if total < sum {
        warnings.push(format!(
            "Total cost {total} is less than sum of component costs {sum} for `{item_name}`"
        ));
    }
    let combine = total.saturating_sub(sum);
    Ok((Some(combine), warnings))
}

fn compute_upgrades(ctx: &ConversionContext, name: &str) -> Result<Vec<String>> {
    let module = ctx.item_module_map()?;
    let mut upgrades: Vec<String> = Vec::new();
    for (item_name, data) in module.iter() {
        if item_name.eq_ignore_ascii_case(name) {
            continue;
        }
        if let Some(recipe_vals) = data.get("recipe").and_then(lua_value_to_string_vec) {
            if recipe_vals
                .iter()
                .any(|entry| entry.eq_ignore_ascii_case(name))
            {
                upgrades.push(item_name.clone());
            }
        }
    }
    upgrades.sort();
    upgrades.dedup();
    Ok(upgrades)
}

fn collect_item_source_appendices(
    export: &crate::wiki_export::WikiExport,
    name: &str,
    raw_main: &str,
    module_title: &str,
    entry: &HashMap<String, LuaValue>,
) -> Result<Vec<SourceAppendix>> {
    let mut appendices = vec![SourceAppendix {
        title: name.to_string(),
        format: "wikitext".to_string(),
        content: raw_main.to_string(),
    }];

    let mut subpages = export.list_titles_with_prefix(&format!("{name}/"))?;
    subpages.sort();
    for title in subpages {
        if let Some(raw) = export.read_optional_page(&title)? {
            appendices.push(SourceAppendix {
                title,
                format: "wikitext".to_string(),
                content: raw,
            });
        }
    }

    appendices.push(SourceAppendix {
        title: if entry.contains_key("removed") {
            format!("Module:ItemData/data/removed/{module_title}")
        } else {
            format!("Module:ItemData/data/{module_title}")
        },
        format: "json".to_string(),
        content: lua_table_to_pretty_json(entry)?,
    });

    Ok(appendices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_item_page_info_collects_rich_fields() {
        let raw = "{{Item info|goldvalue=1000|goldefficiency=110%|similaritems=Essence Reaver;Yun Tal Wildarrows|background=Forged in [[Shurima]].|notes=* First note|trivia=* Fun fact|patchhistory=;[[V1.0]]\n* Added to the shop.}}";
        let vars = HashMap::new();
        let registry = TemplateRegistry::new();
        let info = parse_item_page_info("Test Blade", raw, 2, &vars, &registry, None);

        assert_eq!(info.gold_value.as_deref(), Some("1000"));
        assert_eq!(info.gold_efficiency.as_deref(), Some("110%"));
        assert_eq!(
            info.similar_items,
            vec![
                "Essence Reaver".to_string(),
                "Yun Tal Wildarrows".to_string()
            ]
        );
        assert_eq!(info.background.as_deref(), Some("Forged in [[Shurima]]."));
        assert_eq!(info.notes, vec!["* First note".to_string()]);
        assert_eq!(info.trivia, vec!["* Fun fact".to_string()]);
        assert_eq!(info.patch_history.len(), 1);
        assert_eq!(info.patch_history[0].version, "V1.0");
        assert!(info.warnings.is_empty());
    }

    #[test]
    fn parse_item_page_info_preserves_vars_across_fields() {
        let raw = concat!(
            "{{Item info",
            "|goldvalue={{#vardefineecho:firstlight|25}}",
            "|goldefficiency=* Bonus value: {{#var:firstlight}}",
            "}}"
        );
        let vars = HashMap::new();
        let registry = TemplateRegistry::new();
        let info = parse_item_page_info("Test Blade", raw, 2, &vars, &registry, None);

        assert_eq!(info.gold_value.as_deref(), Some("25"));
        assert_eq!(info.gold_efficiency.as_deref(), Some("* Bonus value: 25"));
        assert!(info.warnings.is_empty());
    }

    #[test]
    fn normalize_modes_maps_live_dump_values() {
        let normalized = normalize_modes(vec![
            "classic sr 5v5".to_string(),
            "aram".to_string(),
            "nb".to_string(),
            "ar".to_string(),
        ]);
        assert_eq!(
            normalized,
            vec![
                "Arena".to_string(),
                "Howling Abyss".to_string(),
                "Nexus Blitz".to_string(),
                "Summoner's Rift".to_string(),
            ]
        );
    }

    #[test]
    fn item_module_title_from_page_prefers_item_info_target() {
        let raw = "{{Item info|Arcane Sweeper (Trinket)|automaticgv=false}}";
        assert_eq!(
            item_module_title_from_page(raw).as_deref(),
            Some("Arcane Sweeper (Trinket)")
        );
    }

    #[test]
    fn convert_item_surfaces_removed_module_metadata() {
        let td = tempfile::tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(flat.join("Ataraxia.txt"), "{{Item info}}\n").unwrap();
        std::fs::write(
            flat.join("Module%3AItemData%2Fdata%2Fremoved.txt"),
            r#"return {
    ["Ataraxia"] = {
        ["buy"] = 3000,
        ["caption"] = "{{sbc|{{ai|Master Craftsman|Ornn}}:}} All stats have been improved.",
        ["ornn"] = true,
        ["removed"] = "V14.11",
        ["tier"] = 4,
        ["type"] = {
            [1] = "Legendary",
        },
    },
}"#,
        )
        .unwrap();

        let ctx = crate::convert::ConversionContext::new(td.path(), 2).unwrap();
        ctx.set_include_removed(true);
        let out_dir = td.path().join("out");
        std::fs::create_dir_all(&out_dir).unwrap();

        ctx.convert_item(&out_dir, "Ataraxia").unwrap();

        let md = std::fs::read_to_string(out_dir.join("Ataraxia.md")).unwrap();
        assert!(md.contains("This article or section may contain obsolete information, but exists here for historical purposes."));
        assert!(md.contains("This item was removed on patch V14.11."));
        assert!(md.contains(
            "Ataraxia was a legendary item in League of Legends. Could only be forged by Ornn."
        ));
        assert!(md.contains("All stats have been improved."));
    }

    #[test]
    fn convert_item_skips_removed_items_by_default() {
        let td = tempfile::tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(flat.join("Ataraxia.txt"), "{{Item info}}\n").unwrap();
        std::fs::write(
            flat.join("Module%3AItemData%2Fdata%2Fremoved.txt"),
            r#"return {
    ["Ataraxia"] = {
        ["buy"] = 3000,
        ["ornn"] = true,
        ["removed"] = "V14.11",
        ["tier"] = 4,
        ["type"] = {
            [1] = "Legendary",
        },
    },
}"#,
        )
        .unwrap();

        let ctx = crate::convert::ConversionContext::new(td.path(), 2).unwrap();
        let out_dir = td.path().join("out");
        std::fs::create_dir_all(&out_dir).unwrap();

        let outcome = ctx.convert_item(&out_dir, "Ataraxia").unwrap();
        assert!(outcome.skipped, "removed item should be skipped by default");
        assert!(
            !out_dir.join("Ataraxia.md").exists(),
            "no markdown file should be written for a skipped removed item"
        );

        // Opting in writes the file as before.
        ctx.set_include_removed(true);
        let outcome = ctx.convert_item(&out_dir, "Ataraxia").unwrap();
        assert!(!outcome.skipped);
        assert!(out_dir.join("Ataraxia.md").exists());
    }

    #[test]
    fn convert_item_resolves_gold_value_formulas_from_module_data() {
        let td = tempfile::tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(
            flat.join("Test%20Blade.txt"),
            concat!(
                "{{Item info",
                "|goldvalue=* 4 [[ability power]] = {{g|{{#vardefineecho:onestack|{{#expr:{{#var:ap}}*4}}}}}}\n",
                "** '''Total Gold Value''' = {{g|{{#expr:{{#var:total}}+{{#var:onestack}}}}}}\n",
                "|goldefficiency=* {{iis|Test Blade}} gold efficiency is increased by {{gec|Test Blade|+{{#var:onestack}}}}.\n",
                "|notes=* Health total = {{fd|{{#expr:{{cid|Test Blade|health}}+10}}}}\n",
                "}}\n"
            ),
        )
        .unwrap();
        std::fs::write(
            flat.join("Module%3AItemData%2Fdata.txt"),
            r#"return {
    ["Test Blade"] = {
        ["buy"] = 350,
        ["sellratio"] = 0.4,
        ["tier"] = 1,
        ["type"] = {
            [1] = "Starter",
        },
        ["stats"] = {
            ["ap"] = 15,
            ["hp"] = 50,
        },
    },
}"#,
        )
        .unwrap();
        std::fs::write(
            flat.join("Module%3AGold%20value%2Fdata.txt"),
            r#"return {
    ["ap"] = {
        ["val"] = 20,
    },
    ["hp"] = {
        ["val"] = 2.666667,
    },
}"#,
        )
        .unwrap();

        let ctx = crate::convert::ConversionContext::new(td.path(), 2).unwrap();
        let out_dir = td.path().join("out");
        std::fs::create_dir_all(&out_dir).unwrap();

        ctx.convert_item(&out_dir, "Test Blade").unwrap();

        let md = std::fs::read_to_string(out_dir.join("Test_Blade.md")).unwrap();
        assert!(md.contains("4 [ability power](./ability_power.md) = 80"));
        assert!(md.contains("513.33"));
        assert!(md.contains("22.86% (+80g)"));
        // {{fd|60}} renders "60", not the padded "60.00" (Module:Fd never pads).
        assert!(md.contains("Health total = 60"));
        assert!(!md.contains("Health total = 60.00"));
    }

    #[test]
    fn convert_item_uses_current_item_name_for_bare_gec() {
        let td = tempfile::tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(
            flat.join("Test%20Elixir.txt"),
            concat!(
                "{{Item info",
                "|goldvalue={{gold value}}\n",
                "* 300 [[health]] = {{g|{{#expr:{{#var:hp}}*300}}}}\n",
                "** '''Total Gold Value''' = {{g|{{#expr:{{#var:hp}}*300}}}}\n",
                "|goldefficiency=* After consuming {{ii|Test Elixir}}, it becomes {{gec}} gold efficient.\n",
                "}}\n"
            ),
        )
        .unwrap();
        std::fs::write(
            flat.join("Module%3AItemData%2Fdata.txt"),
            r#"return {
    ["Test Elixir"] = {
        ["buy"] = 500,
        ["sellratio"] = 0.4,
        ["tier"] = 1,
        ["type"] = {
            [1] = "Consumable",
        },
        ["stats"] = {
            ["hp"] = 300,
        },
    },
}"#,
        )
        .unwrap();
        std::fs::write(
            flat.join("Module%3AGold%20value%2Fdata.txt"),
            r#"return {
    ["hp"] = {
        ["val"] = 2.666667,
    },
}"#,
        )
        .unwrap();

        let ctx = crate::convert::ConversionContext::new(td.path(), 2).unwrap();
        let out_dir = td.path().join("out");
        std::fs::create_dir_all(&out_dir).unwrap();

        ctx.convert_item(&out_dir, "Test Elixir").unwrap();

        let md = std::fs::read_to_string(out_dir.join("Test_Elixir.md")).unwrap();
        assert!(md.contains("160% (+300g) gold efficient"));
    }
}
