use crate::convert::util::{
    collect_list_items, collect_page_vars, expand_inline_templates, expand_with_vars,
    extract_patch_history, extract_section, lua_table_to_pretty_json,
};
use crate::convert::{write_markdown_with_plain_text, ConversionOutcome};
use crate::error::Result;
use crate::model::{
    Ability, AbilityKey, AdvancedStats, BasicInfo, Champion, ChampionSpecialMode,
    ChampionStatVariant, Pet, SourceAppendix, StatLine, Stats,
};
use crate::parse::brace::TemplateSpan;
use crate::parse::lua::{
    lua_value_to_string, lua_value_to_string_vec, parse_champion_entry, parse_champion_module,
    LuaValue,
};
use crate::parse::templates::{parse_invocation, TemplateRegistry};
use crate::parse::{
    evaluate_expression, extract_balanced_templates, parse_ability_template, ExprNumberFormat,
};
use crate::render::markdown::{detect_renderer_cleanup_template_names, render_champion_markdown};
use crate::wiki_export::{url_decode, WikiExport};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use tracing::info;

use super::context::ConversionContext;

#[derive(Debug, Clone, Default)]
struct ChampionInfoboxCandidate {
    info: BasicInfo,
    identifier: Option<String>,
    tab_label: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct LoadedChampionStats {
    stats: Stats,
    advanced: Option<AdvancedStats>,
    constants: HashMap<String, String>,
    constant_variants: HashMap<String, HashMap<String, String>>,
    positions: Vec<String>,
    special_stats: Vec<ChampionSpecialMode>,
    stat_variants: Vec<ChampionStatVariant>,
}

#[derive(Debug, Default)]
struct LoadedAbilities {
    abilities: Vec<Ability>,
    warnings: Vec<String>,
}

/// Convert a single champion (minimal stub). Looks for `Main/<Name>/page.txt`.
pub(super) fn convert_champion(
    ctx: &ConversionContext,
    output_dir: &Path,
    name: &str,
) -> Result<ConversionOutcome> {
    let export = ctx.export();
    if std::env::var("LOL_MD_DEBUG_LIST_ROOT").ok().as_deref() == Some("1") {
        println!(
            "[convert_champion] wiki_root={} export.root={}",
            ctx.wiki_root().display(),
            export.root.display()
        );
        if let Ok(rd) = std::fs::read_dir(&export.root) {
            for e in rd.flatten() {
                println!("[convert_champion] root has: {}", e.path().display());
            }
        }
    }
    let raw = export.read_champion_main(name)?;
    let registry = ctx.registry();
    let precision = ctx.precision();
    let initial_vars = collect_page_vars(&raw)?;
    let infoboxes = collect_champion_infoboxes(
        &raw,
        precision,
        &initial_vars,
        registry,
        Some(Arc::new(ctx.clone())),
    )?;
    let preferred_stat_identifier = infoboxes
        .first()
        .and_then(|candidate| candidate.identifier.clone());
    let primary_stat_label = infoboxes.first().and_then(|candidate| {
        candidate
            .tab_label
            .clone()
            .or_else(|| candidate.identifier.clone())
    });
    let stat_entry_order = build_stat_entry_order(&infoboxes);
    let stat_label_overrides = build_stat_label_overrides(&infoboxes);
    let mut basic = infoboxes
        .first()
        .map(|candidate| candidate.info.clone())
        .unwrap_or_default();

    let LoadedChampionStats {
        stats,
        advanced,
        constants,
        constant_variants,
        positions,
        special_stats,
        stat_variants,
    } = load_champion_stats_with_hints(
        export,
        name,
        preferred_stat_identifier.as_deref(),
        &stat_entry_order,
        &stat_label_overrides,
    )?;

    // Collect page-level #vardefine vars first, then expand page raw
    let mut vars = initial_vars;
    for (k, v) in &constants {
        vars.entry(k.clone()).or_insert(v.clone());
    }
    ctx.insert_champion_constants(name, constants.clone());
    for (variant_name, variant_constants) in &constant_variants {
        ctx.insert_champion_constants(variant_name, variant_constants.clone());
    }
    let expanded = expand_with_vars(
        &raw,
        precision,
        &vars,
        registry,
        Some(Arc::new(ctx.clone())),
    )?;
    // Template inventory (names only) written adjacent to output dir once per champion for now (will refactor to context-wide)
    if let Ok(spans) = extract_balanced_templates(&raw) {
        if !spans.is_empty() {
            let mut names: Vec<String> = Vec::new();
            for s in spans {
                let n = s.name.split('|').next().unwrap_or("").trim();
                if !n.is_empty() {
                    names.push(n.to_string());
                }
            }
            ctx.record_templates(names);
        }
    }
    // Load abilities and expand their key fields with same var context
    let LoadedAbilities {
        abilities,
        warnings: mut ability_warnings,
    } = load_abilities(
        &export,
        name,
        precision,
        &vars,
        &registry,
        Some(Arc::new(ctx.clone())),
    )?;
    let abilities = attach_skill_tabs(&expanded, abilities);
    if should_use_module_title(basic.title.as_deref(), name) {
        if let Some(module_title) = constants
            .get("title")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            basic.title = Some(module_title.to_string());
        }
    }
    if basic.resource.is_none() {
        if let Some(res) = constants
            .get("resource")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            basic.resource = Some(res.to_string());
        } else if let Some(res_line) = stats.base.get("resource") {
            if res_line.base == 0.0 {
                basic.resource = guess_resource_name(name).or_else(|| Some("Unknown".into()));
            }
        }
    }
    if basic.roles.is_empty() && !positions.is_empty() {
        basic.roles = positions;
    }
    let notes = extract_section(&expanded, "Notes")
        .map(|section| collect_list_items(&section))
        .unwrap_or_default();
    let summary = resolve_champion_summary(name, &expanded);
    let pets = extract_pets(
        &raw,
        precision,
        &vars,
        registry,
        Some(Arc::new(ctx.clone())),
    )?;
    let mut warnings = collect_champion_source_warnings(export, name, &raw);
    warnings.append(&mut ability_warnings);
    warnings.extend(render_cleanup_diagnostic_warnings(
        "champion page",
        &expanded,
    ));
    warnings.extend(validate_core_ability_coverage(&abilities));
    if expanded.contains("{{") {
        warnings.push("Residual template markup remained after expansion.".to_string());
    }
    if expanded.to_ascii_lowercase().contains("<tabber>") {
        warnings.push("Residual <tabber> markup remained after expansion.".to_string());
    }
    let trivia = collect_champion_trivia(
        ctx,
        &export,
        name,
        basic.title.as_deref(),
        &expanded,
        &vars,
        &mut warnings,
    );
    let patch_history =
        collect_champion_patch_history(ctx, &export, name, &raw, &expanded, &vars, &mut warnings);
    let source_appendices = collect_champion_source_appendices(ctx, &export, name, &raw)?;
    warnings.sort();
    warnings.dedup();

    let champion = Champion {
        name: name.to_string(),
        basic,
        summary,
        stats,
        advanced,
        primary_stat_label,
        special_stats,
        stat_variants,
        abilities,
        pets,
        trivia,
        patch_history,
        notes,
        source_appendices,
        warnings,
    };
    let markdown = render_champion_markdown(&champion, &expanded);
    let out_file = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    write_markdown_with_plain_text(&out_file, &markdown)?;
    if let Ok(rel) = out_file.strip_prefix(output_dir) {
        let artifact = rel.to_string_lossy().replace('\\', "/");
        ctx.set_specimen_sample("champion", &artifact);
    } else {
        let artifact = out_file.to_string_lossy().replace('\\', "/");
        ctx.set_specimen_sample("champion", &artifact);
    }
    info!(champion = name, "converted (minimal placeholder)");
    Ok(ConversionOutcome {
        entity: name.to_string(),
        output: out_file,
    })
}

fn collect_champion_infoboxes(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<Vec<ChampionInfoboxCandidate>> {
    let mut candidates = collect_champion_infoboxes_from_tabbers(
        raw,
        precision,
        vars,
        registry,
        conversion_ctx.clone(),
    )?;
    if !candidates.is_empty() {
        return Ok(candidates);
    }

    let spans = extract_balanced_templates(raw)?;
    for span in spans {
        if !is_champion_infobox(&span) {
            continue;
        }
        if let Some(candidate) = parse_champion_infobox_candidate(
            &span,
            None,
            precision,
            vars,
            registry,
            conversion_ctx.clone(),
        )? {
            candidates.push(candidate);
        }
    }
    Ok(candidates)
}

fn collect_champion_infoboxes_from_tabbers(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<Vec<ChampionInfoboxCandidate>> {
    let mut candidates = Vec::new();
    let lower = raw.to_ascii_lowercase();
    let mut search_start = 0usize;
    const TABBER_OPEN: &str = "<tabber>";
    const TABBER_CLOSE: &str = "</tabber>";

    while let Some(start_rel) = lower[search_start..].find(TABBER_OPEN) {
        let content_start = search_start + start_rel + TABBER_OPEN.len();
        let Some(end_rel) = lower[content_start..].find(TABBER_CLOSE) else {
            break;
        };
        let content_end = content_start + end_rel;
        let block = &raw[content_start..content_end];

        for segment in block.split("|-|") {
            let tab_label = extract_tabber_segment_label(segment);
            let spans = extract_balanced_templates(segment)?;
            for span in spans {
                if !is_champion_infobox(&span) {
                    continue;
                }
                if let Some(candidate) = parse_champion_infobox_candidate(
                    &span,
                    tab_label.clone(),
                    precision,
                    vars,
                    registry,
                    conversion_ctx.clone(),
                )? {
                    candidates.push(candidate);
                }
                break;
            }
        }

        search_start = content_end + TABBER_CLOSE.len();
    }

    Ok(candidates)
}

fn parse_champion_infobox_candidate(
    span: &TemplateSpan,
    tab_label: Option<String>,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<Option<ChampionInfoboxCandidate>> {
    let body = &span.raw[2..span.raw.len() - 2];
    let inv = parse_invocation(body);
    let mut named: HashMap<String, String> = HashMap::new();
    let mut positional: Vec<String> = Vec::new();

    for param in inv.params {
        if let Some(eq) = param.find('=') {
            let (key_raw, value_raw) = param.split_at(eq);
            let key = key_raw.trim().to_ascii_lowercase();
            if key.is_empty() {
                continue;
            }
            let expanded = expand_inline_templates(
                value_raw[1..].trim(),
                precision,
                vars,
                registry,
                conversion_ctx.clone(),
            )?;
            named.insert(key, expanded.trim().to_string());
        } else if !param.trim().is_empty() {
            let expanded = expand_inline_templates(
                param.trim(),
                precision,
                vars,
                registry,
                conversion_ctx.clone(),
            )?;
            positional.push(expanded.trim().to_string());
        }
    }

    let identifier = named
        .get("title")
        .cloned()
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            positional
                .first()
                .cloned()
                .filter(|value| !value.trim().is_empty())
        })
        .or_else(|| tab_label.clone().filter(|value| !value.trim().is_empty()));

    let mut roles = Vec::new();
    for (key, value) in &named {
        if key.starts_with("role") {
            roles.extend(split_roles(value));
        }
    }

    let mut deduped_roles = Vec::new();
    let mut seen_roles = HashSet::new();
    for role in roles {
        let canonical = role.to_ascii_lowercase();
        if seen_roles.insert(canonical) {
            deduped_roles.push(role);
        }
    }

    let info = BasicInfo {
        title: identifier.clone(),
        roles: deduped_roles,
        resource: named
            .get("resource")
            .cloned()
            .filter(|value| !value.trim().is_empty()),
    };

    Ok(Some(ChampionInfoboxCandidate {
        info,
        identifier,
        tab_label,
    }))
}

fn extract_tabber_segment_label(segment: &str) -> Option<String> {
    let trimmed = segment.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(idx) = trimmed.find("={{") {
        let label = trimmed[..idx].trim();
        if !label.is_empty() {
            return Some(label.to_string());
        }
    }

    for line in trimmed.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("{{") {
            break;
        }
        if let Some(label) = line.strip_suffix('=') {
            let label = label.trim();
            if !label.is_empty() {
                return Some(label.to_string());
            }
        }
        if let Some((label, rest)) = line.split_once('=') {
            if rest.trim().is_empty() {
                let label = label.trim();
                if !label.is_empty() {
                    return Some(label.to_string());
                }
            }
        }
    }

    None
}

fn build_stat_entry_order(candidates: &[ChampionInfoboxCandidate]) -> Vec<String> {
    let mut order = Vec::new();
    let mut seen = HashSet::new();

    for candidate in candidates {
        let value = candidate
            .identifier
            .as_deref()
            .or(candidate.info.title.as_deref())
            .or(candidate.tab_label.as_deref());
        let Some(value) = value else {
            continue;
        };
        let normalized = normalize_lookup_key(value);
        if !normalized.is_empty() && seen.insert(normalized.clone()) {
            order.push(normalized);
        }
    }

    order
}

fn build_stat_label_overrides(candidates: &[ChampionInfoboxCandidate]) -> HashMap<String, String> {
    let mut overrides = HashMap::new();

    for candidate in candidates {
        let Some(label) = candidate
            .tab_label
            .clone()
            .or_else(|| candidate.info.title.clone())
            .filter(|value| !value.trim().is_empty())
        else {
            continue;
        };

        for key in [
            candidate.identifier.as_deref(),
            candidate.info.title.as_deref(),
            candidate.tab_label.as_deref(),
        ] {
            let Some(key) = key else {
                continue;
            };
            let normalized = normalize_lookup_key(key);
            if !normalized.is_empty() {
                overrides.entry(normalized).or_insert_with(|| label.clone());
            }
        }
    }

    overrides
}

fn normalize_lookup_key(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn is_champion_infobox(span: &TemplateSpan) -> bool {
    let name_lower = span.name.to_ascii_lowercase();
    name_lower.contains("infobox champion")
        || name_lower.contains("champion info")
        || name_lower.starts_with("champion infobox")
}

fn split_roles(value: &str) -> Vec<String> {
    let sanitized = value
        .replace("<br>", "\n")
        .replace("<br />", "\n")
        .replace('/', "\n");
    sanitized
        .split(|c| c == ',' || c == '\n' || c == ';')
        .map(|s| {
            s.trim()
                .trim_matches('[')
                .trim_matches(']')
                .trim()
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn extract_champion_summary(expanded: &str) -> Option<String> {
    let mut paragraph: Vec<String> = Vec::new();

    for line in expanded.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("==") {
            break;
        }
        if trimmed.is_empty() {
            if !paragraph.is_empty() {
                break;
            }
            continue;
        }
        if line_cannot_start_champion_summary(trimmed) {
            continue;
        }

        paragraph.push(trimmed.to_string());

        if trimmed.ends_with('.') {
            break;
        }
    }

    if paragraph.is_empty() {
        None
    } else {
        Some(paragraph.join(" "))
    }
}

fn resolve_champion_summary(page_name: &str, expanded: &str) -> Option<String> {
    extract_champion_summary(expanded).or_else(|| fallback_champion_summary(page_name))
}

fn line_cannot_start_champion_summary(trimmed: &str) -> bool {
    trimmed.starts_with("<!--")
        || trimmed.starts_with("{{")
        || trimmed.starts_with("|")
        || trimmed.starts_with("{|")
        || trimmed.starts_with("|}")
        || trimmed.starts_with('!')
        || trimmed.starts_with(':')
        || trimmed.starts_with(';')
        || trimmed.starts_with('*')
        || trimmed.starts_with('#')
        || trimmed.starts_with("[[File:")
        || trimmed.starts_with("[[Image:")
        || trimmed.starts_with("__")
        || trimmed.starts_with('{')
}

fn fallback_champion_summary(title: &str) -> Option<String> {
    let title = title.trim();
    if title.is_empty() {
        None
    } else {
        Some(format!("{title} is a champion in League of Legends."))
    }
}

fn should_use_module_title(current_title: Option<&str>, champion_name: &str) -> bool {
    let Some(current_title) = current_title else {
        return true;
    };
    let current_title = current_title.trim();
    current_title.is_empty() || current_title.eq_ignore_ascii_case(champion_name.trim())
}

fn collect_champion_trivia(
    ctx: &ConversionContext,
    export: &WikiExport,
    name: &str,
    current_title: Option<&str>,
    expanded_main: &str,
    base_vars: &HashMap<String, String>,
    warnings: &mut Vec<String>,
) -> Vec<String> {
    let mut trivia = trivia_items_from_expanded(expanded_main, name, current_title);
    let trivia_page_title = format!("{name}/Trivia");
    match export.read_optional_page(&trivia_page_title) {
        Ok(Some(raw_trivia_page)) => {
            if raw_trivia_page
                .to_ascii_lowercase()
                .contains("trivia compilation")
            {
                for title in trivia_compilation_sources(name) {
                    match export.read_optional_page(&title) {
                        Ok(Some(raw_page)) => {
                            match expand_related_page(ctx, &raw_page, base_vars) {
                                Ok(expanded_page) => {
                                    trivia.extend(trivia_items_from_expanded(
                                        &expanded_page,
                                        name,
                                        current_title,
                                    ));
                                }
                                Err(err) => warnings.push(format!(
                                    "Could not expand trivia source `{title}`: {err}"
                                )),
                            }
                        }
                        Ok(None) => {}
                        Err(err) => {
                            warnings.push(format!("Could not read trivia source `{title}`: {err}"))
                        }
                    }
                }
            } else {
                match expand_related_page(ctx, &raw_trivia_page, base_vars) {
                    Ok(expanded_page) => {
                        let mut page_trivia =
                            trivia_items_from_expanded(&expanded_page, name, current_title);
                        if page_trivia.is_empty() {
                            page_trivia =
                                collect_filtered_trivia_items(&expanded_page, name, current_title);
                        }
                        trivia.extend(page_trivia);
                    }
                    Err(err) => {
                        warnings.push(format!("Could not expand `{trivia_page_title}`: {err}"))
                    }
                }
            }
        }
        Ok(None) => {}
        Err(err) => warnings.push(format!("Could not read `{trivia_page_title}`: {err}")),
    }
    dedup_preserve_order(&mut trivia);
    trivia
}

fn collect_champion_patch_history(
    ctx: &ConversionContext,
    export: &WikiExport,
    name: &str,
    raw_main: &str,
    expanded_main: &str,
    base_vars: &HashMap<String, String>,
    warnings: &mut Vec<String>,
) -> Vec<crate::model::PatchEntry> {
    let mut patch_history = extract_patch_history(expanded_main);
    let patch_page_title = format!("{name}/Patch history");
    match export.read_optional_page(&patch_page_title) {
        Ok(Some(raw_patch_page)) => match expand_related_page(ctx, &raw_patch_page, base_vars) {
            Ok(expanded_patch_page) => {
                let page_patch_history = extract_patch_history(&expanded_patch_page);
                if page_patch_history.is_empty() {
                    warnings.push(format!(
                        "Patch history page `{patch_page_title}` was found but yielded no structured entries."
                    ));
                } else if page_patch_history.len() > patch_history.len() {
                    patch_history = page_patch_history;
                }
            }
            Err(err) => warnings.push(format!("Could not expand `{patch_page_title}`: {err}")),
        },
        Ok(None) => {
            if raw_main.to_ascii_lowercase().contains("patch box") {
                warnings.push(format!(
                    "Patch box transclusion detected for `{name}`, but `{patch_page_title}` was not found."
                ));
            }
        }
        Err(err) => warnings.push(format!("Could not read `{patch_page_title}`: {err}")),
    }
    patch_history
}

fn collect_champion_source_warnings(export: &WikiExport, name: &str, raw: &str) -> Vec<String> {
    let mut warnings = Vec::new();
    if raw.contains("#invoke:SkinData") {
        warnings.push(
            "SkinData transclusion detected, but structured skins rendering is not yet implemented."
                .to_string(),
        );
    }

    let universe_title = format!("Universe:{name}");
    if let Err(err) = export.read_optional_page(&universe_title) {
        warnings.push(format!(
            "Could not inspect related page `{universe_title}`: {err}"
        ));
    }

    warnings
}

fn collect_champion_source_appendices(
    ctx: &ConversionContext,
    export: &WikiExport,
    name: &str,
    raw_main: &str,
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

    let universe_title = format!("Universe:{name}");
    if let Some(raw) = export.read_optional_page(&universe_title)? {
        appendices.push(SourceAppendix {
            title: universe_title,
            format: "wikitext".to_string(),
            content: raw,
        });
    }

    let mut ability_templates = export.list_champion_ability_templates(name)?;
    ability_templates.sort();
    for template_title in ability_templates {
        if let Some(raw) = export.read_template_page(&template_title)? {
            appendices.push(SourceAppendix {
                title: template_title,
                format: "wikitext".to_string(),
                content: raw,
            });
        }
    }

    if let Some(module_raw) = ctx.champion_module_raw()? {
        if let Ok(entry) = parse_champion_entry(module_raw, name) {
            appendices.push(SourceAppendix {
                title: format!("Module:ChampionData/{name}"),
                format: "json".to_string(),
                content: lua_table_to_pretty_json(&entry)?,
            });
        }
    }

    Ok(appendices)
}

fn trivia_items_from_expanded(
    expanded: &str,
    champion_name: &str,
    current_title: Option<&str>,
) -> Vec<String> {
    let trivia_headings = ["Trivia", "Trivia and references", "Trivia and References"];
    trivia_headings
        .iter()
        .find_map(|heading| {
            extract_section(expanded, heading)
                .map(|section| {
                    collect_filtered_trivia_items(&section, champion_name, current_title)
                })
                .filter(|items| !items.is_empty())
        })
        .unwrap_or_default()
}

fn collect_filtered_trivia_items(
    section: &str,
    champion_name: &str,
    current_title: Option<&str>,
) -> Vec<String> {
    let filtered = filter_noncurrent_champion_trivia(section, champion_name, current_title);
    collect_list_items(&filtered)
}

fn filter_noncurrent_champion_trivia(
    section: &str,
    champion_name: &str,
    current_title: Option<&str>,
) -> String {
    let normalized_name = normalize_trivia_group_label(champion_name);
    let Some(normalized_title) = current_title
        .map(normalize_trivia_group_label)
        .filter(|value| !value.is_empty())
    else {
        return section.to_string();
    };

    let mut saw_version_groups = false;
    let mut skip_group = false;
    let mut filtered = Vec::new();

    for line in section.lines() {
        let trimmed = line.trim();
        if let Some(label) = trimmed.strip_prefix(';') {
            let normalized_label = normalize_trivia_group_label(label);
            if trivia_group_mentions_champion(&normalized_label, &normalized_name) {
                saw_version_groups = true;
                skip_group = !trivia_group_matches_current_version(
                    &normalized_label,
                    &normalized_name,
                    &normalized_title,
                );
                continue;
            }
            skip_group = false;
        }

        if skip_group {
            continue;
        }

        filtered.push(line.to_string());
    }

    if saw_version_groups {
        filtered.join("\n")
    } else {
        section.to_string()
    }
}

fn trivia_group_mentions_champion(label: &str, champion_name: &str) -> bool {
    !label.is_empty() && (label == champion_name || label.starts_with(&format!("{champion_name} ")))
}

fn trivia_group_matches_current_version(
    label: &str,
    champion_name: &str,
    current_title: &str,
) -> bool {
    label == champion_name
        || label == current_title
        || label == format!("{champion_name} {current_title}")
        || label.contains(&format!("{champion_name} {current_title}"))
}

fn normalize_trivia_group_label(raw: &str) -> String {
    let mut normalized = String::new();
    let mut last_was_space = true;

    for ch in raw.chars() {
        if ch.is_alphanumeric() {
            for lower in ch.to_lowercase() {
                normalized.push(lower);
            }
            last_was_space = false;
        } else if !last_was_space {
            normalized.push(' ');
            last_was_space = true;
        }
    }

    normalized.trim().to_string()
}

fn trivia_compilation_sources(name: &str) -> Vec<String> {
    vec![
        name.to_string(),
        format!("{name}/Audio"),
        format!("{name}/Cosmetics"),
        format!("{name}/Development"),
        format!("Universe:{name}"),
    ]
}

fn expand_related_page(
    ctx: &ConversionContext,
    raw: &str,
    base_vars: &HashMap<String, String>,
) -> Result<String> {
    let mut vars = base_vars.clone();
    for (key, value) in collect_page_vars(raw)? {
        vars.entry(key).or_insert(value);
    }
    expand_with_vars(
        raw,
        ctx.precision(),
        &vars,
        ctx.registry(),
        Some(Arc::new(ctx.clone())),
    )
}

fn extract_pets(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<Vec<Pet>> {
    let Some(section) = extract_section(raw, "Pets") else {
        return Ok(Vec::new());
    };

    let infobox_pets =
        extract_pet_infoboxes(&section, precision, vars, registry, conversion_ctx.clone())?;
    if !infobox_pets.is_empty() {
        return Ok(infobox_pets);
    }

    let expanded_section =
        expand_inline_templates(&section, precision, vars, registry, conversion_ctx)?;
    let items = collect_list_items(&expanded_section);
    let mut pets = Vec::new();
    for item in items {
        if let Some(pet) = parse_pet_entry(&item) {
            pets.push(pet);
        }
    }
    Ok(pets)
}

fn extract_pet_infoboxes(
    section: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<Vec<Pet>> {
    let spans = extract_balanced_templates(section)?;
    let mut pets = Vec::new();

    for span in spans {
        let name_lower = span.name.trim().to_ascii_lowercase();
        if !(name_lower.starts_with("infobox/pet") || name_lower.starts_with("infobox pet")) {
            continue;
        }

        let inv = parse_invocation(&span.raw[2..span.raw.len() - 2]);
        let mut fields: HashMap<String, String> = HashMap::new();
        for param in inv.params {
            let Some(eq_idx) = param.find('=') else {
                continue;
            };
            let key = param[..eq_idx].trim().to_ascii_lowercase();
            if key.is_empty() {
                continue;
            }
            let value_raw = param[eq_idx + 1..].trim();
            let expanded = expand_inline_templates(
                value_raw,
                precision,
                vars,
                registry,
                conversion_ctx.clone(),
            )?;
            fields.insert(key, expanded.trim().to_string());
        }

        let Some(name) = fields
            .get("name")
            .map(|value| collapse_inline_whitespace(value))
            .filter(|value| !value.is_empty())
        else {
            continue;
        };

        let mut stats: Vec<crate::model::PetStat> = Vec::new();
        for (label, key) in [
            ("Gold", "gold"),
            ("EXP", "exp"),
            ("HP", "hp"),
            ("Damage", "damage"),
            ("Damage Modifier", "damagemodifier"),
            ("Armor", "armor"),
            ("Magic Resist", "magicresist"),
            ("Attack Speed", "attackspeed"),
            ("Move Speed", "movespeed"),
            ("Range", "range"),
            ("CC Resist", "ccresist"),
            ("Control", "control"),
            ("Targeting", "targeting"),
            ("Spell Effects", "spelleffects"),
            ("On-hit", "onhit"),
        ] {
            if let Some(value) = fields.get(key) {
                // A field may itself be a `*` bullet list (e.g. damage modifiers
                // or CC resistances); render those as nested sub-bullets.
                let items = split_star_list_items(value);
                if !items.is_empty() {
                    stats.push(crate::model::PetStat {
                        label: label.to_string(),
                        value: String::new(),
                        items,
                    });
                    continue;
                }
                let normalized = collapse_inline_whitespace(value);
                if !normalized.is_empty() {
                    stats.push(crate::model::PetStat {
                        label: label.to_string(),
                        value: normalized,
                        items: Vec::new(),
                    });
                }
            }
        }

        if let Some(abilities) = fields.get("abilities") {
            let entries = pet_block_entries(abilities);
            if !entries.is_empty() {
                stats.push(crate::model::PetStat {
                    label: "Abilities".to_string(),
                    value: String::new(),
                    items: entries,
                });
            }
        }
        if let Some(notes) = fields.get("notes") {
            let entries = pet_note_entries(notes);
            if !entries.is_empty() {
                stats.push(crate::model::PetStat {
                    label: "Notes".to_string(),
                    value: String::new(),
                    items: entries,
                });
            }
        }

        pets.push(Pet {
            name,
            description: String::new(),
            stats,
        });
    }

    Ok(pets)
}

fn parse_pet_entry(item: &str) -> Option<Pet> {
    let trimmed = item.trim();
    if trimmed.is_empty() {
        return None;
    }
    let mut content = trimmed.trim_start_matches('*').trim();
    if content.is_empty() {
        return None;
    }
    if content.starts_with('-') {
        content = content.trim_start_matches('-').trim();
    }
    let mut name = String::new();
    let mut remainder = content;
    if content.starts_with("'''") {
        if let Some(end) = content[3..].find("'''") {
            name = content[3..3 + end].trim().to_string();
            remainder = content[3 + end + 3..].trim();
        }
    }
    if name.is_empty() {
        if let Some((left, right)) = split_first_delim(content) {
            name = left.trim_matches('"').trim_matches('\'').trim().to_string();
            remainder = right.trim();
        }
    }
    if name.is_empty() {
        name = content
            .split_whitespace()
            .next()
            .unwrap_or("Companion")
            .trim_matches('\'')
            .to_string();
        remainder = content;
    }
    let description = remainder
        .trim_start_matches(|c: char| c == '–' || c == '—' || c == '-' || c == ':' || c == ' ')
        .trim()
        .to_string();
    Some(Pet {
        name,
        description,
        stats: Vec::new(),
    })
}

/// Split a wikitext `*` bullet list value into its item texts. Returns an empty
/// vector when the value is not a `*` list so callers can fall back to a flat
/// rendering.
fn split_star_list_items(raw: &str) -> Vec<String> {
    let collapsed = collapse_inline_whitespace(raw);
    let mut trimmed = collapsed.trim();
    // A field list is sometimes introduced by a leading line break (`|x =<br>`).
    loop {
        let stripped = trimmed
            .strip_prefix("<br>")
            .or_else(|| trimmed.strip_prefix("<br/>"))
            .or_else(|| trimmed.strip_prefix("<br />"))
            .map(str::trim_start);
        match stripped {
            Some(rest) => trimmed = rest,
            None => break,
        }
    }
    if !trimmed.starts_with('*') {
        return Vec::new();
    }
    trimmed
        .split('*')
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .map(|item| item.to_string())
        .collect()
}

fn split_first_delim(input: &str) -> Option<(&str, &str)> {
    for delim in ['—', '–', '-', ':'] {
        if let Some(idx) = input.find(delim) {
            let split_at = idx + delim.len_utf8();
            return Some((&input[..idx], &input[split_at..]));
        }
    }
    None
}

fn collapse_inline_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Parse a `;Name\n* body` definition block (e.g. a pet's `abilities` field)
/// into one `Name — body` entry per term.
fn pet_block_entries(raw: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut current_name: Option<String> = None;
    let mut current_body: Vec<String> = Vec::new();

    let flush = |entries: &mut Vec<String>, name: Option<String>, body: &mut Vec<String>| {
        if let Some(name) = name {
            let joined = collapse_inline_whitespace(&body.join(" "));
            // The body lines are themselves `*` bullets; strip the markers.
            let joined = joined
                .split('*')
                .map(|part| part.trim())
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            if joined.is_empty() {
                entries.push(name);
            } else {
                entries.push(format!("{name} — {joined}"));
            }
        }
        body.clear();
    };

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(name) = trimmed.strip_prefix(';') {
            flush(&mut entries, current_name.take(), &mut current_body);
            current_name = Some(name.trim().to_string());
            continue;
        }
        current_body.push(trimmed.to_string());
    }
    flush(&mut entries, current_name.take(), &mut current_body);

    if !entries.is_empty() {
        return entries;
    }
    let collapsed = collapse_inline_whitespace(raw);
    if collapsed.is_empty() {
        Vec::new()
    } else {
        vec![collapsed]
    }
}

fn pet_note_entries(raw: &str) -> Vec<String> {
    let items = collect_list_items(raw);
    if !items.is_empty() {
        return items
            .into_iter()
            .map(|item| collapse_inline_whitespace(item.trim_start_matches('*').trim()))
            .filter(|item| !item.is_empty())
            .collect();
    }
    let collapsed = collapse_inline_whitespace(raw);
    if collapsed.is_empty() {
        Vec::new()
    } else {
        vec![collapsed]
    }
}

fn attach_skill_tabs(page_expanded: &str, mut abilities: Vec<Ability>) -> Vec<Ability> {
    use std::collections::HashMap;

    #[derive(Default)]
    struct TableBuilder {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    }

    let mut builders: HashMap<String, TableBuilder> = HashMap::new();
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
                        let key = part[..colon].trim();
                        let val = part[colon + 1..].trim();
                        let key_lower = key.to_ascii_lowercase();
                        match key_lower.as_str() {
                            "slot" => slot = Some(val.to_ascii_uppercase()),
                            k if k == "h" || k.starts_with("h") => {
                                headers.push(val.to_string());
                            }
                            k if k == "r" || k.starts_with("r") || k.starts_with("row") => {
                                row.push(val.to_string());
                            }
                            _ => {}
                        }
                    }
                }
                if let Some(slot_label) = slot {
                    let entry = builders.entry(slot_label).or_default();
                    if !headers.is_empty() {
                        if entry.headers.is_empty() || entry.headers.len() < headers.len() {
                            entry.headers = headers.clone();
                        }
                    }
                    if !row.is_empty() {
                        if !entry.headers.is_empty() && row.len() < entry.headers.len() {
                            let mut padded = row.clone();
                            padded.resize(entry.headers.len(), String::new());
                            entry.rows.push(padded);
                        } else {
                            entry.rows.push(row.clone());
                        }
                    }
                }
            }
        }
    }

    for ability in &mut abilities {
        let label = ability_key_label(&ability.key);
        if let Some(builder) = builders.remove(&label) {
            if !builder.headers.is_empty() || !builder.rows.is_empty() {
                ability.leveling_tables.push(crate::model::SkillTable {
                    headers: builder.headers.clone(),
                    rows: builder.rows.clone(),
                });
            }
        }
    }
    abilities
}

fn ability_key_label(key: &AbilityKey) -> String {
    match key {
        AbilityKey::Passive => "PASSIVE".to_string(),
        AbilityKey::BasicAttack => "A".to_string(),
        AbilityKey::Q => "Q".to_string(),
        AbilityKey::W => "W".to_string(),
        AbilityKey::E => "E".to_string(),
        AbilityKey::R => "R".to_string(),
        AbilityKey::Other(s) => s.to_ascii_uppercase(),
    }
}

fn describe_ability_key(key: &AbilityKey) -> String {
    match key {
        AbilityKey::Passive => "Passive".to_string(),
        AbilityKey::BasicAttack => "Basic Attack".to_string(),
        AbilityKey::Q => "Q".to_string(),
        AbilityKey::W => "W".to_string(),
        AbilityKey::E => "E".to_string(),
        AbilityKey::R => "R".to_string(),
        AbilityKey::Other(raw) if raw.trim().is_empty() => "(missing)".to_string(),
        AbilityKey::Other(raw) => raw.trim().to_string(),
    }
}

fn render_cleanup_diagnostic_warnings(scope: &str, expanded: &str) -> Vec<String> {
    let leaked_templates = detect_renderer_cleanup_template_names(expanded);
    if leaked_templates.is_empty() {
        Vec::new()
    } else {
        vec![format!(
            "Supported template markup survived parser expansion in {scope} and required render-layer cleanup: {}.",
            leaked_templates.join(", ")
        )]
    }
}

fn validate_core_ability_coverage(abilities: &[Ability]) -> Vec<String> {
    let expected = [
        (AbilityKey::Passive, "Passive"),
        (AbilityKey::Q, "Q"),
        (AbilityKey::W, "W"),
        (AbilityKey::E, "E"),
        (AbilityKey::R, "R"),
    ];

    let missing = expected
        .iter()
        .filter_map(|(expected_key, label)| {
            let present = abilities.iter().any(|ability| {
                std::mem::discriminant(&ability.key) == std::mem::discriminant(expected_key)
            });
            if present {
                None
            } else {
                Some((*label).to_string())
            }
        })
        .collect::<Vec<_>>();

    if missing.is_empty() {
        Vec::new()
    } else {
        vec![format!(
            "Missing core ability slots after ability resolution: {}.",
            missing.join(", ")
        )]
    }
}

// old minimal renderer removed (superseded by render_champion_markdown)

fn load_champion_stats_with_hints(
    export: &WikiExport,
    name: &str,
    preferred_entry: Option<&str>,
    entry_order: &[String],
    label_overrides: &HashMap<String, String>,
) -> Result<LoadedChampionStats> {
    let Some(lua) = export.read_champion_module_data()? else {
        return Ok(LoadedChampionStats::default());
    };

    let module = parse_champion_module(&lua)?;
    let Some(primary_key) = preferred_entry
        .and_then(|preferred| find_matching_entry_key(&module, preferred))
        .or_else(|| find_matching_entry_key(&module, name))
    else {
        return Ok(LoadedChampionStats::default());
    };
    let Some(primary_entry) = module.get(&primary_key) else {
        return Ok(LoadedChampionStats::default());
    };

    let primary_apiname = primary_entry
        .get("apiname")
        .and_then(lua_value_to_string)
        .unwrap_or_else(|| name.to_string());

    let mut related_keys = collect_related_entry_keys(
        &module,
        &primary_key,
        primary_entry,
        &primary_apiname,
        entry_order,
    );
    sort_related_entry_keys(&mut related_keys, &module, &primary_key, entry_order);

    let mut constant_variants = HashMap::new();
    for entry_key in &related_keys {
        if let Some(entry) = module.get(entry_key) {
            let constants = collect_entry_constants(entry);
            for alias in entry_constant_aliases(entry_key, entry, label_overrides) {
                constant_variants
                    .entry(alias)
                    .or_insert_with(|| constants.clone());
            }
        }
    }
    let constants = constant_variants
        .get(&primary_key)
        .cloned()
        .unwrap_or_else(|| collect_entry_constants(primary_entry));
    let positions = extract_entry_positions(primary_entry);
    let (mut stats, advanced, special_stats) = extract_stats_from_entry(primary_entry);
    if let Some(res) = constants.get("resource") {
        stats.base.insert(
            "resource".into(),
            StatLine {
                base: 0.0,
                growth: 0.0,
            },
        );
        stats.base.insert(
            format!("resource__{}", res),
            StatLine {
                base: 0.0,
                growth: 0.0,
            },
        );
    }

    let mut stat_variants = Vec::new();
    for entry_key in related_keys {
        if entry_key.eq_ignore_ascii_case(&primary_key) {
            continue;
        }
        let Some(entry) = module.get(&entry_key) else {
            continue;
        };
        let (variant_stats, variant_advanced, variant_special_stats) =
            extract_stats_from_entry(entry);
        let has_advanced = variant_advanced
            .as_ref()
            .map(|advanced| !advanced.metrics.is_empty())
            .unwrap_or(false);
        if variant_stats.base.is_empty() && !has_advanced && variant_special_stats.is_empty() {
            continue;
        }
        stat_variants.push(ChampionStatVariant {
            label: display_entry_label(&entry_key, entry, label_overrides),
            stats: variant_stats,
            advanced: variant_advanced,
            special_stats: variant_special_stats,
        });
    }

    Ok(LoadedChampionStats {
        stats,
        advanced,
        constants,
        constant_variants,
        positions,
        special_stats,
        stat_variants,
    })
}

fn collect_related_entry_keys(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    primary_key: &str,
    primary_entry: &HashMap<String, LuaValue>,
    primary_apiname: &str,
    entry_order: &[String],
) -> Vec<String> {
    let mut related_keys: Vec<String> = module
        .iter()
        .filter(|(entry_key, entry)| {
            entry_key.eq_ignore_ascii_case(primary_key)
                || entry
                    .get("apiname")
                    .and_then(lua_value_to_string)
                    .map(|apiname| apiname.eq_ignore_ascii_case(primary_apiname))
                    .unwrap_or(false)
        })
        .map(|(entry_key, _)| entry_key.clone())
        .collect();

    if related_keys.is_empty() {
        related_keys.push(primary_key.to_string());
    }

    for hint in entry_order {
        if let Some(entry_key) = find_matching_entry_key(module, hint) {
            push_unique_case_insensitive(&mut related_keys, entry_key);
        }
    }

    for hint in entry_skill_hints(primary_entry) {
        let Some(entry_key) = find_matching_entry_key(module, &hint) else {
            continue;
        };
        let Some(entry) = module.get(&entry_key) else {
            continue;
        };
        if champion_entries_share_family(primary_entry, entry) {
            push_unique_case_insensitive(&mut related_keys, entry_key);
        }
    }

    related_keys
}

fn entry_skill_hints(entry: &HashMap<String, LuaValue>) -> Vec<String> {
    let mut hints = Vec::new();
    for field in [
        "skills", "skill_i", "skill_q", "skill_w", "skill_e", "skill_r",
    ] {
        if let Some(values) = entry.get(field).and_then(lua_value_to_string_vec) {
            for value in values {
                let trimmed = value.trim();
                if !trimmed.is_empty() {
                    push_unique_case_insensitive(&mut hints, trimmed.to_string());
                }
            }
        }
    }
    hints
}

fn champion_entries_share_family(
    primary_entry: &HashMap<String, LuaValue>,
    candidate_entry: &HashMap<String, LuaValue>,
) -> bool {
    let same_title = primary_entry
        .get("title")
        .and_then(lua_value_to_string)
        .zip(candidate_entry.get("title").and_then(lua_value_to_string))
        .map(|(left, right)| !left.trim().is_empty() && left.eq_ignore_ascii_case(&right))
        .unwrap_or(false);

    let same_id_family = champion_entry_id_family(primary_entry)
        .zip(champion_entry_id_family(candidate_entry))
        .map(|(left, right)| left == right)
        .unwrap_or(false);

    same_title || same_id_family
}

fn champion_entry_id_family(entry: &HashMap<String, LuaValue>) -> Option<i64> {
    entry
        .get("id")
        .and_then(lua_value_to_f32)
        .map(|value| value.trunc() as i64)
}

fn entry_constant_aliases(
    entry_key: &str,
    entry: &HashMap<String, LuaValue>,
    label_overrides: &HashMap<String, String>,
) -> Vec<String> {
    let mut aliases = Vec::new();
    push_unique_case_insensitive(&mut aliases, entry_key.to_string());

    for candidate in champion_entry_lookup_keys(entry_key, entry) {
        push_unique_case_insensitive(&mut aliases, candidate.clone());
        if let Some(label) = label_overrides.get(&normalize_lookup_key(&candidate)) {
            push_unique_case_insensitive(&mut aliases, label.clone());
        }
    }

    let display_label = display_entry_label(entry_key, entry, label_overrides);
    push_unique_case_insensitive(&mut aliases, display_label);
    aliases
}

fn push_unique_case_insensitive(values: &mut Vec<String>, candidate: String) {
    if values
        .iter()
        .any(|existing| existing.eq_ignore_ascii_case(&candidate))
    {
        return;
    }
    values.push(candidate);
}

fn find_matching_entry_key(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    query: &str,
) -> Option<String> {
    module
        .keys()
        .find(|entry_key| entry_key.eq_ignore_ascii_case(query))
        .cloned()
        .or_else(|| {
            module
                .iter()
                .find(|(entry_key, entry)| entry_matches_query(entry_key, entry, query))
                .map(|(entry_key, _)| entry_key.clone())
        })
}

fn entry_matches_query(entry_key: &str, entry: &HashMap<String, LuaValue>, query: &str) -> bool {
    champion_entry_lookup_keys(entry_key, entry)
        .iter()
        .any(|candidate| candidate.eq_ignore_ascii_case(query))
}

fn champion_entry_lookup_keys(entry_key: &str, entry: &HashMap<String, LuaValue>) -> Vec<String> {
    let mut keys = vec![entry_key.to_string()];
    for field in ["apiname", "name", "fullname"] {
        if let Some(value) = entry.get(field).and_then(lua_value_to_string) {
            if !value.trim().is_empty() {
                keys.push(value);
            }
        }
    }

    let mut deduped = Vec::new();
    let mut seen = HashSet::new();
    for key in keys {
        let normalized = normalize_lookup_key(&key);
        if !normalized.is_empty() && seen.insert(normalized) {
            deduped.push(key);
        }
    }
    deduped
}

fn sort_related_entry_keys(
    keys: &mut Vec<String>,
    module: &HashMap<String, HashMap<String, LuaValue>>,
    primary_key: &str,
    entry_order: &[String],
) {
    keys.sort_by(|left, right| {
        let left_entry = module.get(left).expect("sorted entry should exist");
        let right_entry = module.get(right).expect("sorted entry should exist");
        entry_sort_priority(left, left_entry, primary_key, entry_order)
            .cmp(&entry_sort_priority(
                right,
                right_entry,
                primary_key,
                entry_order,
            ))
            .then_with(|| left.to_ascii_lowercase().cmp(&right.to_ascii_lowercase()))
    });
}

fn entry_sort_priority(
    entry_key: &str,
    entry: &HashMap<String, LuaValue>,
    primary_key: &str,
    entry_order: &[String],
) -> (usize, usize) {
    let primary_priority = if entry_key.eq_ignore_ascii_case(primary_key) {
        0
    } else {
        1
    };
    let order_priority = entry_order_position(entry_key, entry, entry_order).unwrap_or(usize::MAX);
    (primary_priority, order_priority)
}

fn entry_order_position(
    entry_key: &str,
    entry: &HashMap<String, LuaValue>,
    entry_order: &[String],
) -> Option<usize> {
    let lookups = champion_entry_lookup_keys(entry_key, entry);
    entry_order.iter().position(|expected| {
        lookups
            .iter()
            .any(|candidate| normalize_lookup_key(candidate) == *expected)
    })
}

fn display_entry_label(
    entry_key: &str,
    entry: &HashMap<String, LuaValue>,
    label_overrides: &HashMap<String, String>,
) -> String {
    for candidate in champion_entry_lookup_keys(entry_key, entry) {
        if let Some(label) = label_overrides.get(&normalize_lookup_key(&candidate)) {
            return label.clone();
        }
    }
    entry_key.to_string()
}

fn collect_entry_constants(entry: &HashMap<String, LuaValue>) -> HashMap<String, String> {
    let mut constants = HashMap::new();
    for (key, value) in entry {
        if let Some(raw) = lua_value_to_string(value) {
            constants.insert(key.clone(), raw);
        }
    }
    if let Some(LuaValue::Table(stats)) = entry.get("stats") {
        for (key, value) in stats {
            if let Some(raw) = lua_value_to_string(value) {
                constants.entry(key.clone()).or_insert(raw);
            }
        }
        let windup = derive_entry_windup(stats);
        if let Some(windup) = windup {
            constants
                .entry("windup".to_string())
                .or_insert_with(|| format_number(windup));
        }
    }
    // NB: we deliberately do *not* inject a `crit_base` (or any other) default
    // here. Champions missing a field fall back to the wiki's own value at
    // lookup time via `ConversionContext::champion_constant_default`, which
    // reads `Module:ChampionData/getter` (`crit_base or 200`). Keeping the
    // default in one place — sourced from the dumped module — avoids the two
    // code paths drifting apart.
    constants
}

fn extract_entry_positions(entry: &HashMap<String, LuaValue>) -> Vec<String> {
    let mut positions = Vec::new();
    let mut seen = HashSet::new();

    for field in ["external_positions", "client_positions"] {
        if let Some(values) = entry.get(field).and_then(lua_value_to_string_vec) {
            for value in values {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let normalized = normalize_lookup_key(trimmed);
                if seen.insert(normalized) {
                    positions.push(trimmed.to_string());
                }
            }
        }
    }

    if positions.is_empty() {
        if let Some(values) = entry.get("role").and_then(lua_value_to_string_vec) {
            for value in values {
                let trimmed = value.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let normalized = normalize_lookup_key(trimmed);
                if seen.insert(normalized) {
                    positions.push(trimmed.to_string());
                }
            }
        }
    }

    positions
}

fn parse_stat_value(raw: &str) -> Option<f32> {
    if let Ok(val) = raw.parse::<f32>() {
        return Some(val);
    }
    let evaluated = evaluate_expression(raw, ExprNumberFormat::Float(6)).ok()?;
    evaluated.parse::<f32>().ok()
}

fn extract_stats_from_entry(
    entry: &HashMap<String, LuaValue>,
) -> (Stats, Option<AdvancedStats>, Vec<ChampionSpecialMode>) {
    let mut stats = Stats::default();
    let Some(LuaValue::Table(stat_map)) = entry.get("stats") else {
        return (stats, None, Vec::new());
    };
    let mut consumed: HashSet<String> = HashSet::new();
    const STAT_MAPPINGS: &[(&str, &str, Option<&str>)] = &[
        ("HP", "hp_base", Some("hp_lvl")),
        ("MP", "mp_base", Some("mp_lvl")),
        ("Armor", "arm_base", Some("arm_lvl")),
        ("Magic Resist", "mr_base", Some("mr_lvl")),
        ("HP Regen", "hp5_base", Some("hp5_lvl")),
        ("MP Regen", "mp5_base", Some("mp5_lvl")),
        ("Attack Damage", "dam_base", Some("dam_lvl")),
        ("Attack Speed", "as_base", Some("as_lvl")),
        ("Range", "range", None),
        ("Move Speed", "ms", None),
    ];
    for (label, base_key, growth_key) in STAT_MAPPINGS {
        if let Some(base_value) = stat_map.get(*base_key).and_then(lua_value_to_f32) {
            let growth_value = growth_key
                .and_then(|g| stat_map.get(g).and_then(lua_value_to_f32))
                .unwrap_or(0.0);
            stats.base.insert(
                (*label).to_string(),
                StatLine {
                    base: base_value,
                    growth: growth_value,
                },
            );
            consumed.insert((*base_key).to_string());
            if let Some(g_key) = growth_key {
                consumed.insert((*g_key).to_string());
            }
        }
    }
    let advanced = extract_advanced_metrics(stat_map, &consumed);
    let special_stats = extract_special_stats(stat_map);
    (stats, advanced, special_stats)
}

fn extract_advanced_metrics(
    stat_map: &HashMap<String, LuaValue>,
    consumed: &HashSet<String>,
) -> Option<AdvancedStats> {
    let mut metrics: HashMap<String, String> = HashMap::new();
    for (key, value) in stat_map {
        if consumed.contains(key) {
            continue;
        }
        if matches!(value, LuaValue::Table(_)) {
            continue;
        }
        if let Some(formatted) = format_metric_value(value) {
            if formatted.trim().is_empty() {
                continue;
            }
            let label = metric_label(key);
            metrics.entry(label).or_insert(formatted);
        }
    }
    if let Some(windup) = derive_entry_windup(stat_map) {
        metrics
            .entry("Windup %".to_string())
            .or_insert(format!("{:.1}%", windup * 100.0));
    }
    if metrics.is_empty() {
        None
    } else {
        Some(AdvancedStats { metrics })
    }
}

fn derive_entry_windup(stats: &HashMap<String, LuaValue>) -> Option<f32> {
    let attack_cast_time = stats.get("attack_cast_time").and_then(lua_value_to_f32);
    let attack_total_time = stats.get("attack_total_time").and_then(lua_value_to_f32);
    let as_base = stats.get("as_base").and_then(lua_value_to_f32);

    match (attack_cast_time, attack_total_time, as_base) {
        (Some(cast), Some(total), _) if total > 0.0 => Some(cast / total),
        (Some(cast), _, Some(base_as)) if base_as > 0.0 => Some(cast * base_as),
        _ => stats
            .get("attack_delay_offset")
            .and_then(lua_value_to_f32)
            .map(|offset| 0.3 + offset)
            .filter(|windup| *windup > 0.0),
    }
}

fn extract_special_stats(stat_map: &HashMap<String, LuaValue>) -> Vec<ChampionSpecialMode> {
    let mut special_stats = Vec::new();
    let mut seen_modes = HashSet::new();

    for (mode_key, mode_label) in [
        ("swift", "Swiftplay"),
        ("aram", "ARAM"),
        ("ha", "ARAM"),
        ("ar", "Arena"),
        ("nb", "Nexus Blitz"),
        ("ofa", "One For All"),
        ("urf", "Ultra Rapid Fire"),
        ("usb", "Ultimate Spellbook"),
    ] {
        let Some(LuaValue::Table(mode_map)) = stat_map.get(mode_key) else {
            continue;
        };
        let mut metrics = HashMap::new();
        let mut keys: Vec<_> = mode_map.keys().collect();
        keys.sort();
        for key in keys {
            let Some(value) = mode_map.get(key) else {
                continue;
            };
            if matches!(value, LuaValue::Table(_) | LuaValue::Array(_)) {
                continue;
            }
            if let Some(formatted) = format_metric_value(value) {
                if formatted.trim().is_empty() {
                    continue;
                }
                metrics
                    .entry(special_metric_label(key))
                    .or_insert(formatted);
            }
        }
        if !metrics.is_empty() && seen_modes.insert(mode_label.to_string()) {
            special_stats.push(ChampionSpecialMode {
                mode: mode_label.to_string(),
                metrics,
            });
        }
    }

    special_stats
}

fn special_metric_label(key: &str) -> String {
    match key {
        "dmg_dealt" => "Damage Dealt".to_string(),
        "dmg_taken" => "Damage Taken".to_string(),
        "healing_done" => "Healing Done".to_string(),
        "shielding_done" => "Shielding Done".to_string(),
        "healing_taken" => "Healing Taken".to_string(),
        "shielding_taken" => "Shielding Taken".to_string(),
        "ability_haste" | "haste" => "Ability Haste".to_string(),
        "hp_base" => "HP Base".to_string(),
        "hp_lvl" => "HP Growth".to_string(),
        "mp_base" => "MP Base".to_string(),
        "mp_lvl" => "MP Growth".to_string(),
        "arm_base" => "Armor Base".to_string(),
        "arm_lvl" => "Armor Growth".to_string(),
        "mr_base" => "Magic Resist Base".to_string(),
        "mr_lvl" => "Magic Resist Growth".to_string(),
        "hp5_base" => "HP Regen Base".to_string(),
        "hp5_lvl" => "HP Regen Growth".to_string(),
        "mp5_base" => "MP Regen Base".to_string(),
        "mp5_lvl" => "MP Regen Growth".to_string(),
        "dam_base" => "Attack Damage Base".to_string(),
        "dam_lvl" => "Attack Damage Growth".to_string(),
        "as_base" => "Attack Speed Base".to_string(),
        "as_lvl" => "Attack Speed Growth".to_string(),
        "range" => "Range".to_string(),
        "ms" => "Move Speed".to_string(),
        other => metric_label(other),
    }
}

fn lua_value_to_f32(value: &LuaValue) -> Option<f32> {
    match value {
        LuaValue::Number(raw) | LuaValue::String(raw) => parse_stat_value(raw),
        LuaValue::Bool(true) => Some(1.0),
        LuaValue::Bool(false) => Some(0.0),
        _ => None,
    }
}

fn format_metric_value(value: &LuaValue) -> Option<String> {
    match value {
        LuaValue::Number(raw) | LuaValue::String(raw) => {
            if let Some(num) = parse_stat_value(raw) {
                Some(format_number(num))
            } else {
                let trimmed = raw.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }
        }
        LuaValue::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

fn format_number(value: f32) -> String {
    let mut s = if value.abs() >= 1000.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.4}", value)
    };
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    s
}

fn metric_label(key: &str) -> String {
    match key {
        "as_ratio" | "attack_speed_ratio" => "Attack Speed Ratio".to_string(),
        "attack_cast_time" => "Attack Cast Time (s)".to_string(),
        "attack_total_time" => "Base Attack Time (s)".to_string(),
        "attack_delay_offset" => "Attack Delay Offset (s)".to_string(),
        "missile_speed" => "Missile Speed".to_string(),
        "acquisition_radius" => "Acquisition Radius".to_string(),
        "selection_radius" => "Selection Radius".to_string(),
        "selection_height" => "Selection Height".to_string(),
        "pathing_radius" => "Pathing Radius".to_string(),
        "attack_cast_offset" => "Attack Cast Offset".to_string(),
        other => title_case(other),
    }
}

fn title_case(input: &str) -> String {
    let mut out = String::new();
    let mut upper = true;
    for c in input.chars() {
        if c == '_' {
            out.push(' ');
            upper = true;
            continue;
        }
        if upper {
            out.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

fn guess_resource_name(_name: &str) -> Option<String> {
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
    conversion_ctx: Option<Arc<ConversionContext>>,
) -> Result<LoadedAbilities> {
    let main = export.read_champion_main(champ)?;
    let mut template_titles = collect_referenced_ability_templates(&main, champ);
    if template_titles.is_empty() {
        template_titles = export.list_champion_ability_templates(champ)?;
    }

    // Some slots are transcluded via {{Grouped ability|Champion|Slot}} instead of a
    // direct {{Data Champion/Slot}} reference. A grouped slot bundles several distinct
    // sub-abilities (e.g. Fizz's E = "Playful" / "Trickster") whose individual data
    // templates are never named on the champion page, so the plain reference scan above
    // misses them entirely. Mirror the wiki template's own logic: read the constituent
    // ability names from the champion module's `skill_<slot>` list and append the
    // corresponding `Data Champion/<name>` templates. This stays fully data-driven — no
    // champion- or slot-specific names are baked in.
    let grouped_refs = collect_grouped_ability_refs(&main);
    if !grouped_refs.is_empty() {
        if let Some(lua) = export.read_champion_module_data()? {
            if let Ok(module) = parse_champion_module(&lua) {
                for reference in &grouped_refs {
                    let names = if !reference.explicit_skills.is_empty() {
                        reference.explicit_skills.clone()
                    } else {
                        grouped_ability_skill_names(&module, &reference.champion, &reference.slot)
                    };
                    for name in names {
                        let title = format!("Template:Data {}/{}", reference.champion, name);
                        if !template_titles
                            .iter()
                            .any(|existing| existing.eq_ignore_ascii_case(&title))
                        {
                            template_titles.push(title);
                        }
                    }
                }
            }
        }
    }

    // On the wiki every ability data template is transcluded into the champion
    // page and shares one `#vardefine` scope, so a variable defined in one
    // ability (e.g. a Wild Rift stat) is visible to the others. Pre-collect
    // those definitions so per-ability expansion can resolve cross-references.
    let mut shared_ability_vars: HashMap<String, String> = HashMap::new();
    for template_title in &template_titles {
        if let Ok(Some(raw)) = export.read_template_page(template_title) {
            if let Ok(template_vars) = collect_page_vars(&raw) {
                for (k, v) in template_vars {
                    shared_ability_vars.insert(k, v);
                }
            }
        }
    }

    let mut out: Vec<Ability> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    for template_title in template_titles {
        let name_seg = template_title.trim_start_matches("Template:").trim();
        let Some(raw) = export.read_template_page(&template_title)? else {
            warnings.push(format!(
                "Referenced ability template `{template_title}` is missing."
            ));
            continue;
        };
        let template_name_fallback = ability_name_from_template_source(&template_title, &raw);

        let ability_vars = collect_page_vars(&raw)?;
        let mut merged_vars = vars.clone();
        for (k, v) in &shared_ability_vars {
            merged_vars.insert(k.clone(), v.clone());
        }
        if raw.contains("#invoke:Gold value|wikivaluedefine") {
            if let Some(conv_ctx) = conversion_ctx.as_ref() {
                for (key, data) in conv_ctx.gold_value_data_map()? {
                    if let Some(value) = data.get("val").and_then(lua_value_to_string) {
                        merged_vars.entry(key.clone()).or_insert(value);
                    }
                }
            }
        }
        for (k, v) in ability_vars {
            merged_vars.insert(k, v);
        }

        let param_map = match parse_ability_template(&raw) {
            Ok(map) => map,
            Err(err) => {
                warnings.push(format!(
                    "Could not parse ability template `{template_title}`: {err}"
                ));
                continue;
            }
        };
        if param_map.is_empty() {
            warnings.push(format!(
                "Ability template `{template_title}` expanded to an empty parameter map and was skipped."
            ));
            continue;
        }

        let mut ability_key = AbilityKey::Other(String::new());
        let mut display_name = param_map
            .get("name")
            .cloned()
            .filter(|name| !is_placeholder_ability_name(name));
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
            let expanded = expand_inline_templates(
                raw_val,
                precision,
                &merged_vars,
                registry,
                conversion_ctx.clone(),
            )?;
            let leaked_templates = detect_renderer_cleanup_template_names(&expanded);
            if !leaked_templates.is_empty() {
                warnings.push(format!(
                    "Supported template markup survived parser expansion in `{template_title}` field `{key_name}` and required render-layer cleanup: {}.",
                    leaked_templates.join(", ")
                ));
            }
            let normalized = normalize_ability_value(&expanded);
            match key_name.as_str() {
                "champion" => {}
                "skill" => {
                    if !normalized.is_empty() {
                        ability_key = ability_key_from_slot(&normalized);
                    }
                }
                "name" => {
                    if !normalized.is_empty() && !is_placeholder_ability_name(&normalized) {
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
            AbilityKey::Passive
                | AbilityKey::BasicAttack
                | AbilityKey::Q
                | AbilityKey::W
                | AbilityKey::E
                | AbilityKey::R
        ) {
            // Auxiliary-slot abilities (e.g. Aphelios' weapon slot `A`) carry a
            // real, non-empty slot label and ability content; render those as
            // their own entries instead of dropping them. Templates whose slot
            // does not resolve (empty label) or that have no description are
            // decorative/duplicate and are still skipped.
            let renderable = matches!(&ability_key, AbilityKey::Other(slot) if !slot.trim().is_empty())
                && !descriptions_with_order.is_empty();
            if !renderable {
                warnings.push(format!(
                    "Ability template `{template_title}` resolved to unsupported skill slot `{}` and was skipped.",
                    describe_ability_key(&ability_key)
                ));
                continue;
            }
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

        let mut display_name = display_name.unwrap_or_default();
        if is_placeholder_ability_name(&display_name) {
            display_name.clear();
        }
        if display_name.trim().is_empty() {
            if let Some(template_name) = template_name_fallback.as_ref() {
                display_name = template_name.clone();
            } else if let Some(icon_name) = fallback_name_from_icons(&extra) {
                display_name = icon_name;
            } else {
                display_name = derive_name_from_template(name_seg);
            }
        }
        if display_name.trim().len() <= 1 {
            if let Some(icon_name) = fallback_name_from_icons(&extra) {
                if icon_name.trim().len() > 1 {
                    display_name = icon_name;
                }
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

        if let Some(existing) = out
            .iter_mut()
            .find(|a| a.key == ability_key_final && a.name.eq_ignore_ascii_case(&candidate.name))
        {
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
        // Basic attack first, mirroring the wiki's ability ordering for
        // champions whose auto-attack is a distinct ability (Senna, Aphelios).
        AbilityKey::BasicAttack => 0,
        AbilityKey::Passive => 1,
        AbilityKey::Q => 2,
        AbilityKey::W => 3,
        AbilityKey::E => 4,
        AbilityKey::R => 5,
        AbilityKey::Other(_) => 9,
    });
    warnings.sort();
    warnings.dedup();
    Ok(LoadedAbilities {
        abilities: out,
        warnings,
    })
}

fn collect_referenced_ability_templates(raw: &str, champ: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    collect_referenced_ability_templates_inner(
        raw,
        &champ.to_ascii_lowercase(),
        &mut seen,
        &mut out,
    );
    out
}

fn collect_referenced_ability_templates_inner(
    raw: &str,
    champ_lower: &str,
    seen: &mut HashSet<String>,
    out: &mut Vec<String>,
) {
    let Ok(spans) = extract_balanced_templates(raw) else {
        return;
    };

    for span in spans {
        let name_seg = span.name.trim();
        let lower_name = name_seg.to_ascii_lowercase();
        if lower_name.starts_with("data ") && lower_name.contains(&format!("{champ_lower}/")) {
            let title = format!("Template:{}", name_seg);
            if seen.insert(title.to_ascii_lowercase()) {
                out.push(title);
            }
        }

        if span.raw.len() > 4 {
            let inner = &span.raw[2..span.raw.len() - 2];
            if inner.contains("{{") {
                collect_referenced_ability_templates_inner(inner, champ_lower, seen, out);
            }
        }
    }
}

/// A `{{Grouped ability}}` transclusion discovered on a champion page.
#[derive(Debug, Clone)]
struct GroupedAbilityRef {
    champion: String,
    slot: String,
    /// Explicit `skill1..skill4` overrides, when the page supplies them directly
    /// instead of relying on the champion module's `skill_<slot>` list.
    explicit_skills: Vec<String>,
}

/// Scan a page (recursing into nested templates) for `{{Grouped ability|Champion|Slot}}`
/// transclusions, returning the champion/slot pairs that need their constituent
/// sub-ability data templates resolved from the champion module.
fn collect_grouped_ability_refs(raw: &str) -> Vec<GroupedAbilityRef> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    collect_grouped_ability_refs_inner(raw, &mut seen, &mut out);
    out
}

fn collect_grouped_ability_refs_inner(
    raw: &str,
    seen: &mut HashSet<String>,
    out: &mut Vec<GroupedAbilityRef>,
) {
    let Ok(spans) = extract_balanced_templates(raw) else {
        return;
    };

    for span in spans {
        if span.name.trim().eq_ignore_ascii_case("Grouped ability") {
            let inner = &span.raw[2..span.raw.len() - 2];
            if let Some(reference) = parse_grouped_ability_ref(inner) {
                let key = format!(
                    "{}|{}",
                    reference.champion.to_ascii_lowercase(),
                    reference.slot.to_ascii_lowercase()
                );
                if seen.insert(key) {
                    out.push(reference);
                }
            }
        }

        if span.raw.len() > 4 {
            let inner = &span.raw[2..span.raw.len() - 2];
            if inner.contains("{{") {
                collect_grouped_ability_refs_inner(inner, seen, out);
            }
        }
    }
}

fn parse_grouped_ability_ref(body: &str) -> Option<GroupedAbilityRef> {
    let inv = parse_invocation(body);
    let mut positional: Vec<String> = Vec::new();
    let mut champion: Option<String> = None;
    let mut slot: Option<String> = None;
    let mut explicit_skills: Vec<String> = Vec::new();

    for param in inv.params {
        if let Some(eq) = param.find('=') {
            let key = param[..eq].trim().to_ascii_lowercase();
            let value = param[eq + 1..].trim().to_string();
            match key.as_str() {
                "champion" => champion = Some(value),
                "skill" => slot = Some(value),
                "skill1" | "skill2" | "skill3" | "skill4" => {
                    if !value.is_empty() {
                        explicit_skills.push(value);
                    }
                }
                _ => {}
            }
        } else {
            let trimmed = param.trim();
            if !trimmed.is_empty() {
                positional.push(trimmed.to_string());
            }
        }
    }

    let champion = champion
        .or_else(|| positional.first().cloned())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())?;
    // The grouped-ability template defaults the slot to `Q` when omitted.
    let slot = slot
        .or_else(|| positional.get(1).cloned())
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "Q".to_string());

    Some(GroupedAbilityRef {
        champion,
        slot,
        explicit_skills,
    })
}

/// Resolve the constituent sub-ability names for a grouped slot from the champion
/// module's `skill_<slot>` list (e.g. `skill_e = {"Playful", "Trickster"}`).
fn grouped_ability_skill_names(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    champion: &str,
    slot: &str,
) -> Vec<String> {
    let Some(entry_key) = find_matching_entry_key(module, champion) else {
        return Vec::new();
    };
    let Some(entry) = module.get(&entry_key) else {
        return Vec::new();
    };
    let field = format!("skill_{}", slot.trim().to_ascii_lowercase());
    entry
        .get(&field)
        .and_then(lua_value_to_string_vec)
        .unwrap_or_default()
        .into_iter()
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
        .collect()
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
        "A" | "BASIC ATTACK" => AbilityKey::BasicAttack,
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
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut current_row_index = 0;
    for part in inner.split('|') {
        let part = part.trim();
        if let Some(colon) = part.find(':') {
            let key = part[..colon].trim();
            let val = part[colon + 1..].trim();
            if key.eq_ignore_ascii_case("h") {
                headers.push(val.to_string());
            } else if key.to_lowercase().starts_with("r") {
                let row_num = if key.eq_ignore_ascii_case("r") {
                    1
                } else if let Ok(num) = key[1..].parse::<usize>() {
                    num
                } else {
                    continue;
                };
                if row_num == current_row_index + 1 {
                    if !current_row.is_empty() {
                        rows.push(current_row);
                        current_row = Vec::new();
                    }
                    current_row_index = row_num;
                }
                current_row.push(val.to_string());
            }
        }
    }
    if !current_row.is_empty() {
        rows.push(current_row);
    }
    if headers.is_empty() || rows.is_empty() {
        return None;
    }
    Some(crate::model::SkillTable { headers, rows })
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
        if trimmed.starts_with('*') || is_note_block_header(trimmed) {
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

/// A note line that introduces a sub-section rather than continuing the
/// previous bullet: a definition-list term (`;Zombie info`) or a standalone
/// bold paragraph used as a header (`'''Interactions & Other'''`). In wikitext
/// a newline ends the preceding block, so these must start a new note entry
/// instead of being appended to the prior bullet's text.
fn is_note_block_header(trimmed: &str) -> bool {
    if trimmed.starts_with(';') {
        return true;
    }
    if let Some(inner) = trimmed
        .strip_prefix("'''")
        .and_then(|rest| rest.strip_suffix("'''"))
    {
        return !inner.is_empty() && !inner.contains("'''");
    }
    false
}

fn dedup_preserve_order(values: &mut Vec<String>) {
    let mut seen = HashSet::new();
    values.retain(|v| seen.insert(v.clone()));
}

fn is_placeholder_ability_name(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "" | "false" | "n/a" | "na" | "none" | "unknown"
    )
}

fn ability_name_from_template_source(template_title: &str, raw_template: &str) -> Option<String> {
    extract_redirect_target(raw_template)
        .as_deref()
        .and_then(ability_name_from_template_path)
        .or_else(|| ability_name_from_template_wrapper(raw_template))
        .or_else(|| ability_name_from_template_path(template_title))
}

fn extract_redirect_target(raw_template: &str) -> Option<String> {
    let trimmed = raw_template.trim();
    if !trimmed.to_ascii_lowercase().starts_with("#redirect") {
        return None;
    }
    let start = trimmed.find("[[")? + 2;
    let end = trimmed[start..].find("]]")? + start;
    Some(trimmed[start..end].trim().to_string())
}

fn ability_name_from_template_path(path: &str) -> Option<String> {
    let name = url_decode(path.rsplit('/').next()?.trim()).replace('_', " ");
    if name.len() <= 1 || is_placeholder_ability_name(&name) {
        None
    } else {
        Some(name)
    }
}

fn ability_name_from_template_wrapper(raw_template: &str) -> Option<String> {
    let trimmed = raw_template.trim_start();
    let start = trimmed.find("}}}|")? + 4;
    let rest = &trimmed[start..];
    let end = rest.find('|')?;
    ability_name_from_template_path(rest[..end].trim())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::lua::parse_champion_entry;
    use crate::wiki_export::url_encode;
    use tempfile::tempdir;

    #[test]
    fn collect_notes_splits_definition_terms_and_bold_headers() {
        // Sion's "Glory in Death" notes interleave bullets with a bold
        // sub-header (`'''Interactions & Other'''`) and a definition-list term
        // (`;Zombie info`). Each must become its own note entry rather than
        // being glued onto the trailing text of the previous bullet.
        let notes = concat!(
            "'''Details'''\n",
            "* First detail.\n",
            "* Last detail.\n",
            "'''Interactions & Other'''\n",
            "* An interaction.\n",
            ";Zombie info\n",
            "* Zombie states trigger upon lethal damage.\n",
        );
        let mut out = Vec::new();
        collect_notes(notes, &mut out);
        assert_eq!(
            out,
            vec![
                "'''Details'''".to_string(),
                "* First detail.".to_string(),
                "* Last detail.".to_string(),
                "'''Interactions & Other'''".to_string(),
                "* An interaction.".to_string(),
                ";Zombie info".to_string(),
                "* Zombie states trigger upon lethal damage.".to_string(),
            ]
        );
    }

    #[test]
    fn extracts_stats_and_advanced_metrics() {
        let lua = r#"return {
  ["Tester"] = {
    ["stats"] = {
      ["hp_base"] = 600,
      ["hp_lvl"] = 100,
      ["dam_base"] = 60,
      ["dam_lvl"] = 3,
      ["as_base"] = 0.65,
      ["as_lvl"] = 2.0,
      ["range"] = 175,
      ["ms"] = 340,
      ["as_ratio"] = 0.625,
      ["attack_cast_time"] = 0.3,
      ["attack_total_time"] = 1.5,
      ["acquisition_radius"] = 525,
      ["selection_radius"] = 100,
      ["selection_height"] = 120,
      ["pathing_radius"] = 35,
    }
  }
}"#;

        let entry = parse_champion_entry(lua, "Tester").unwrap();
        let (stats, advanced, special_stats) = extract_stats_from_entry(&entry);

        let hp = stats.base.get("HP").expect("HP stat missing");
        assert!((hp.base - 600.0).abs() < f32::EPSILON);
        assert!((hp.growth - 100.0).abs() < f32::EPSILON);

        let ad = stats
            .base
            .get("Attack Damage")
            .expect("Attack Damage missing");
        assert!((ad.base - 60.0).abs() < f32::EPSILON);
        assert!((ad.growth - 3.0).abs() < f32::EPSILON);

        let as_line = stats
            .base
            .get("Attack Speed")
            .expect("Attack Speed missing");
        assert!((as_line.base - 0.65).abs() < f32::EPSILON);
        assert!((as_line.growth - 2.0).abs() < f32::EPSILON);

        let advanced = advanced.expect("advanced metrics missing");
        assert_eq!(
            advanced.metrics.get("Attack Speed Ratio"),
            Some(&"0.625".to_string())
        );
        assert_eq!(
            advanced.metrics.get("Acquisition Radius"),
            Some(&"525".to_string())
        );
        assert_eq!(advanced.metrics.get("Windup %"), Some(&"20.0%".to_string()));
        assert!(special_stats.is_empty());
    }

    #[test]
    fn parse_champion_infobox_ignores_positional_asset_params() {
        let raw = "{{Champion info|Kled & Skaarl|Kled1|splash=Kled OriginalSkin.jpg}}";
        let registry = TemplateRegistry::new();
        let candidates = collect_champion_infoboxes(raw, 2, &HashMap::new(), &registry, None)
            .expect("infobox parsing should succeed");

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].info.title.as_deref(), Some("Kled & Skaarl"));
        assert!(candidates[0].info.roles.is_empty());
    }

    #[test]
    fn load_champion_stats_prefers_primary_tab_and_collects_special_variants() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(
            flat.join("Module%3AChampionData%2Fdata.txt"),
            r#"return {
    ["Kled"] = {
        ["apiname"] = "Kled",
        ["resource"] = "Courage",
        ["external_positions"] = {"Top"},
        ["stats"] = {
            ["ms"] = 305,
            ["range"] = 250,
            ["aram"] = {
                ["dmg_dealt"] = 1.05,
            },
            ["urf"] = {
                ["dmg_taken"] = 0.9,
            },
        },
    },
    ["Kled & Skaarl"] = {
        ["apiname"] = "Kled",
        ["resource"] = "Courage",
        ["external_positions"] = {"Top"},
        ["stats"] = {
            ["ms"] = 345,
            ["range"] = 125,
            ["aram"] = {
                ["dmg_taken"] = 0.9,
            },
        },
    },
}"#,
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let entry_order = vec![
            normalize_lookup_key("Kled & Skaarl"),
            normalize_lookup_key("Kled"),
        ];
        let mut label_overrides = HashMap::new();
        label_overrides.insert(
            normalize_lookup_key("Kled & Skaarl"),
            "Kled & Skaarl".to_string(),
        );
        label_overrides.insert(
            normalize_lookup_key("Kled"),
            "Kled (Dismounted)".to_string(),
        );

        let loaded = load_champion_stats_with_hints(
            &export,
            "Kled",
            Some("Kled & Skaarl"),
            &entry_order,
            &label_overrides,
        )
        .unwrap();

        assert_eq!(loaded.positions, vec!["Top".to_string()]);
        assert_eq!(loaded.stats.base.get("Move Speed").unwrap().base, 345.0);
        assert_eq!(loaded.stats.base.get("Range").unwrap().base, 125.0);
        // crit_base is not stamped onto the constants; champions missing it fall
        // back to the wiki's getter default at lookup time, not here.
        assert_eq!(loaded.constants.get("crit_base"), None);
        assert_eq!(loaded.special_stats[0].mode, "ARAM");
        assert_eq!(
            loaded.special_stats[0].metrics.get("Damage Taken"),
            Some(&"0.9".to_string())
        );
        assert_eq!(loaded.stat_variants.len(), 1);
        assert_eq!(loaded.stat_variants[0].label, "Kled (Dismounted)");
        assert_eq!(
            loaded.stat_variants[0]
                .stats
                .base
                .get("Move Speed")
                .unwrap()
                .base,
            305.0
        );
        assert!(loaded.stat_variants[0]
            .special_stats
            .iter()
            .any(|mode| mode.mode == "Ultra Rapid Fire"));
    }

    #[test]
    fn collect_entry_constants_flattens_stats_without_injecting_crit_base() {
        let lua = r#"return {
    ["Graves"] = {
        ["title"] = "the Outlaw",
        ["stats"] = {
            ["missile_speed"] = 3800,
            ["range"] = 425,
            ["attack_cast_time"] = 0.3,
            ["attack_total_time"] = 1.5,
        }
    }
}"#;

        let entry = parse_champion_entry(lua, "Graves").unwrap();
        let constants = collect_entry_constants(&entry);

        assert_eq!(constants.get("title"), Some(&"the Outlaw".to_string()));
        assert_eq!(constants.get("missile_speed"), Some(&"3800".to_string()));
        assert_eq!(constants.get("range"), Some(&"425".to_string()));
        assert_eq!(constants.get("windup"), Some(&"0.2".to_string()));
        // No default is stamped in: crit_base is resolved from the getter module
        // at lookup time, so a champion without it simply has no entry here.
        assert_eq!(constants.get("crit_base"), None);
    }

    #[test]
    fn collect_entry_constants_derives_windup_from_attack_delay_offset() {
        let lua = r#"return {
    ["Smolder"] = {
        ["stats"] = {
            ["as_base"] = 0.638,
            ["attack_delay_offset"] = -0.1,
        }
    }
}"#;

        let entry = parse_champion_entry(lua, "Smolder").unwrap();
        let constants = collect_entry_constants(&entry);
        let (_, advanced, _) = extract_stats_from_entry(&entry);

        assert_eq!(constants.get("windup"), Some(&"0.2".to_string()));
        assert_eq!(
            advanced
                .as_ref()
                .and_then(|metrics| metrics.metrics.get("Windup %")),
            Some(&"20.0%".to_string())
        );
    }

    #[test]
    fn load_champion_stats_registers_transform_alias_constants() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(
            flat.join("Module%3AChampionData%2Fdata.txt"),
            r#"return {
    ["Gnar"] = {
        ["id"] = 150,
        ["apiname"] = "Gnar",
        ["title"] = "the Missing Link",
        ["stats"] = {
            ["as_lvl"] = 6,
        },
        ["skills"] = {
            [1] = "Mini Gnar",
            [2] = "Mega Gnar",
        },
    },
    ["Mega Gnar"] = {
        ["id"] = 150.2,
        ["apiname"] = "GnarBig",
        ["title"] = "the Missing Link",
        ["stats"] = {
            ["as_lvl"] = 0.5,
        },
    },
}"#,
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let entry_order = vec![
            normalize_lookup_key("Mini Gnar"),
            normalize_lookup_key("Mega Gnar"),
        ];
        let mut label_overrides = HashMap::new();
        label_overrides.insert(normalize_lookup_key("Gnar"), "Mini Gnar".to_string());
        label_overrides.insert(normalize_lookup_key("Mega Gnar"), "Mega Gnar".to_string());

        let loaded = load_champion_stats_with_hints(
            &export,
            "Gnar",
            Some("Gnar"),
            &entry_order,
            &label_overrides,
        )
        .unwrap();

        assert_eq!(loaded.constants.get("as_lvl"), Some(&"6".to_string()));
        assert_eq!(
            loaded
                .constant_variants
                .get("Mini Gnar")
                .and_then(|constants| constants.get("as_lvl")),
            Some(&"6".to_string())
        );
        assert_eq!(
            loaded
                .constant_variants
                .get("Mega Gnar")
                .and_then(|constants| constants.get("as_lvl")),
            Some(&"0.5".to_string())
        );
        assert_eq!(loaded.stat_variants.len(), 1);
        assert_eq!(loaded.stat_variants[0].label, "Mega Gnar");
    }

    #[test]
    fn detects_placeholder_ability_names() {
        assert!(is_placeholder_ability_name("false"));
        assert!(is_placeholder_ability_name("N/A"));
        assert!(!is_placeholder_ability_name("Heroic Swing"));
    }

    #[test]
    fn prefers_module_title_when_infobox_title_is_name() {
        assert!(should_use_module_title(Some("Akshan"), "Akshan"));
        assert!(should_use_module_title(None, "Akshan"));
        assert!(!should_use_module_title(
            Some("the Rogue Sentinel"),
            "Akshan"
        ));
    }

    #[test]
    fn ability_name_fallback_uses_redirect_target() {
        let raw = "#REDIRECT [[Template:Data Anivia/Frostbite]]";
        assert_eq!(
            ability_name_from_template_source("Template:Data Anivia/E", raw),
            Some("Frostbite".to_string())
        );
    }

    #[test]
    fn ability_name_fallback_uses_template_wrapper_title() {
        let raw = "{{{{{1|Ability data}}}|Weapon Queue System|{{{2|}}}|skill=E}}";
        assert_eq!(
            ability_name_from_template_source("Template:Data Aphelios/E", raw),
            Some("Weapon Queue System".to_string())
        );
    }

    #[test]
    fn load_abilities_collects_nested_transform_templates() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();

        std::fs::write(
            flat.join("Elise.txt"),
            concat!(
                "{{Champion info|Elise}}\n",
                "== Abilities ==\n",
                "{{Data Elise/I|Ability}}\n",
                "{{Image tabber|title1=Human abilities|content1=\n",
                "{{Data Elise/Q|Ability}}\n",
                "|title2=Spider abilities|content2=\n",
                "{{Data Elise/Venomous Bite|Ability}}\n",
                "}}\n",
                "{{Data Elise/R|Ability}}\n"
            ),
        )
        .unwrap();

        for (title, content) in [
            (
                "Template:Data Elise/I",
                "{{{{{1|Ability data}}}|Spider Queen|skill=I|description=Passive}}",
            ),
            (
                "Template:Data Elise/Q",
                "{{{{{1|Ability data}}}|Neurotoxin|skill=Q|description=Human Q}}",
            ),
            (
                "Template:Data Elise/Venomous Bite",
                "{{{{{1|Ability data}}}|Venomous Bite|skill=Q|description=Spider Q}}",
            ),
            (
                "Template:Data Elise/R",
                "{{{{{1|Ability data}}}|Spider Form|skill=R|description=Transform}}",
            ),
        ] {
            std::fs::write(flat.join(format!("{}.txt", url_encode(title))), content).unwrap();
        }

        let export = WikiExport::new(td.path());
        let registry = TemplateRegistry::new();
        let loaded = load_abilities(&export, "Elise", 2, &HashMap::new(), &registry, None).unwrap();
        let abilities = loaded.abilities;

        assert!(abilities
            .iter()
            .any(|ability| ability.name == "Spider Queen"));
        assert!(abilities.iter().any(|ability| ability.name == "Neurotoxin"));
        assert!(abilities
            .iter()
            .any(|ability| ability.name == "Venomous Bite"));
        assert!(abilities
            .iter()
            .any(|ability| ability.name == "Spider Form"));
        assert_eq!(
            abilities
                .iter()
                .filter(|ability| matches!(ability.key, AbilityKey::Q))
                .count(),
            2
        );
        assert!(loaded.warnings.is_empty());
    }

    #[test]
    fn load_abilities_resolves_grouped_ability_slots_from_module() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();

        // Mirrors the live Fizz page: the E slot is transcluded via {{Grouped ability}}
        // rather than a direct {{Data Fizz/E}} reference, so its two sub-abilities are
        // only discoverable through the champion module's skill_e list.
        std::fs::write(
            flat.join("Fizz.txt"),
            concat!(
                "{{Champion info|Fizz}}\n",
                "== Abilities ==\n",
                "{{Data Fizz/I|Ability}}\n",
                "{{Data Fizz/Q|Ability}}\n",
                "{{Data Fizz/W|Ability}}\n",
                "{{Grouped ability|Fizz|E}}\n",
                "{{Data Fizz/R|Ability}}\n"
            ),
        )
        .unwrap();

        std::fs::write(
            flat.join("Module%3AChampionData%2Fdata.txt"),
            concat!(
                "return { ['Fizz'] = { ",
                "skill_i = {'Nimble Fighter'}, skill_q = {'Urchin Strike'}, ",
                "skill_w = {'Seastone Trident'}, skill_e = {'Playful', 'Trickster'}, ",
                "skill_r = {'Chum the Waters'} } }"
            ),
        )
        .unwrap();

        for (title, content) in [
            (
                "Template:Data Fizz/I",
                "{{{{{1|Ability data}}}|Nimble Fighter|skill=I|description=Passive}}",
            ),
            (
                "Template:Data Fizz/Q",
                "{{{{{1|Ability data}}}|Urchin Strike|skill=Q|description=Dash}}",
            ),
            (
                "Template:Data Fizz/W",
                "{{{{{1|Ability data}}}|Seastone Trident|skill=W|description=On-hit}}",
            ),
            (
                "Template:Data Fizz/Playful",
                "{{{{{1|Ability data}}}|Playful|skill=E|description=Hop away}}",
            ),
            (
                "Template:Data Fizz/Trickster",
                "{{{{{1|Ability data}}}|Trickster|skill=E|description=Second hop}}",
            ),
            (
                "Template:Data Fizz/R",
                "{{{{{1|Ability data}}}|Chum the Waters|skill=R|description=Shark}}",
            ),
        ] {
            std::fs::write(flat.join(format!("{}.txt", url_encode(title))), content).unwrap();
        }

        let export = WikiExport::new(td.path());
        let registry = TemplateRegistry::new();
        let loaded = load_abilities(&export, "Fizz", 2, &HashMap::new(), &registry, None).unwrap();
        let abilities = loaded.abilities;

        assert!(abilities.iter().any(|ability| ability.name == "Playful"));
        assert!(abilities.iter().any(|ability| ability.name == "Trickster"));
        assert_eq!(
            abilities
                .iter()
                .filter(|ability| matches!(ability.key, AbilityKey::E))
                .count(),
            2
        );
        assert!(loaded.warnings.is_empty());
    }

    #[test]
    fn load_abilities_ignores_unreferenced_old_rework_templates() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();

        std::fs::write(
            flat.join("Akali.txt"),
            concat!(
                "{{Champion info|Akali}}\n",
                "== Abilities ==\n",
                "{{Data Akali/I|Ability}}\n",
                "{{Data Akali/R|Ability}}\n"
            ),
        )
        .unwrap();

        for (title, content) in [
            (
                "Template:Data Akali/I",
                "#REDIRECT [[Template:Data Akali/Assassin's Mark]]",
            ),
            (
                "Template:Data Akali/R",
                "#REDIRECT [[Template:Data Akali/Perfect Execution]]",
            ),
            (
                "Template:Data Akali/Assassin's Mark",
                "{{{{{1|Ability data}}}|Assassin's Mark|skill=I|description=Current passive}}",
            ),
            (
                "Template:Data Akali/Perfect Execution",
                "{{{{{1|Ability data}}}|Perfect Execution|skill=R|description=Current ultimate}}",
            ),
            (
                "Template:Data Akali/Twin Disciplines",
                "{{{{{1|Ability data}}}|Twin Disciplines|skill=I|description=Old passive}}",
            ),
            (
                "Template:Data Akali/Shadow Dance",
                "{{{{{1|Ability data}}}|Shadow Dance|skill=R|description=Old ultimate}}",
            ),
        ] {
            std::fs::write(flat.join(format!("{}.txt", url_encode(title))), content).unwrap();
        }

        let export = WikiExport::new(td.path());
        let registry = TemplateRegistry::new();
        let loaded = load_abilities(&export, "Akali", 2, &HashMap::new(), &registry, None).unwrap();

        assert!(loaded
            .abilities
            .iter()
            .any(|ability| ability.name == "Assassin's Mark"));
        assert!(loaded
            .abilities
            .iter()
            .any(|ability| ability.name == "Perfect Execution"));
        assert!(!loaded
            .abilities
            .iter()
            .any(|ability| ability.name == "Twin Disciplines"));
        assert!(!loaded
            .abilities
            .iter()
            .any(|ability| ability.name == "Shadow Dance"));
    }

    #[test]
    fn load_abilities_falls_back_to_scanned_templates_when_page_has_no_refs() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();

        std::fs::write(flat.join("Tester.txt"), "{{Champion info|Tester}}\n").unwrap();
        std::fs::write(
            flat.join(format!("{}.txt", url_encode("Template:Data Tester/I"))),
            "{{{{{1|Ability data}}}|Steady Hands|skill=I|description=Fallback passive}}",
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let registry = TemplateRegistry::new();
        let loaded =
            load_abilities(&export, "Tester", 2, &HashMap::new(), &registry, None).unwrap();

        assert_eq!(loaded.abilities.len(), 1);
        assert_eq!(loaded.abilities[0].name, "Steady Hands");
    }

    #[test]
    fn load_abilities_keeps_basic_attack_slot_and_orders_it_first() {
        // Champions whose auto-attack is a distinct ability (Senna, Aphelios)
        // reference a {{Data X/A}} basic-attack slot. It must be rendered, not
        // skipped as an unsupported slot, and ordered ahead of the passive.
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();

        std::fs::write(
            flat.join("Tester.txt"),
            "{{Champion info|Tester}}\n{{Data Tester/A|Ability}}\n{{Data Tester/I|Ability}}\n",
        )
        .unwrap();
        std::fs::write(
            flat.join(format!("{}.txt", url_encode("Template:Data Tester/A"))),
            "{{{{{1|Ability data}}}|skill=A|icon=Basic Attack.png|description=Relic cannon blast}}",
        )
        .unwrap();
        std::fs::write(
            flat.join(format!("{}.txt", url_encode("Template:Data Tester/I"))),
            "{{{{{1|Ability data}}}|Soul Siphon|skill=I|description=Passive}}",
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let registry = TemplateRegistry::new();
        let loaded =
            load_abilities(&export, "Tester", 2, &HashMap::new(), &registry, None).unwrap();

        assert_eq!(loaded.abilities.len(), 2);
        assert_eq!(loaded.abilities[0].key, AbilityKey::BasicAttack);
        assert_eq!(loaded.abilities[1].key, AbilityKey::Passive);
        assert!(
            !loaded
                .warnings
                .iter()
                .any(|w| w.contains("unsupported skill slot")),
            "basic attack slot should not warn as unsupported: {:?}",
            loaded.warnings
        );
    }

    #[test]
    fn trivia_items_filter_noncurrent_champion_version_groups() {
        let expanded = concat!(
            "== Trivia ==\n",
            ";Akali, the Fist of Shadow\n",
            "* Old fact\n",
            ";Akali, the Rogue Assassin\n",
            "* Current fact\n"
        );

        assert_eq!(
            trivia_items_from_expanded(expanded, "Akali", Some("the Rogue Assassin")),
            vec!["* Current fact".to_string()]
        );
    }

    #[test]
    fn extract_champion_summary_prefers_lead_before_sections() {
        let expanded = concat!(
            "{{Champion info|Tester}}\n",
            "<!-- hidden -->\n",
            "Tester is a champion in League of Legends.\n",
            "== Abilities ==\n",
            "Ability text\n"
        );

        assert_eq!(
            extract_champion_summary(expanded),
            Some("Tester is a champion in League of Legends.".to_string())
        );
    }

    #[test]
    fn extract_champion_summary_ignores_section_note_after_heading() {
        let expanded = concat!(
            "{{Champion info|Aatrox}}\n",
            "== Abilities ==\n",
            "Ability text\n",
            "== Champion skins ==\n",
            ":''This article section only contains champion skins. For all associated collection items, see [[Aatrox/Cosmetics|Aatrox (Collection)]].''\n"
        );

        assert_eq!(extract_champion_summary(expanded), None);
    }

    #[test]
    fn resolve_champion_summary_falls_back_to_page_name() {
        let expanded = concat!(
            "{{Champion info|Aatrox}}\n",
            "== Abilities ==\n",
            "Ability text\n",
            "== Champion skins ==\n",
            ":''This article section only contains champion skins.''\n"
        );

        assert_eq!(
            resolve_champion_summary("Aatrox", expanded),
            Some("Aatrox is a champion in League of Legends.".to_string())
        );
    }

    #[test]
    fn fallback_champion_summary_matches_live_wiki_lead_shape() {
        assert_eq!(
            fallback_champion_summary("Aatrox"),
            Some("Aatrox is a champion in League of Legends.".to_string())
        );
    }

    #[test]
    fn load_abilities_warns_on_missing_or_skipped_templates() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();

        std::fs::write(
            flat.join("Tester.txt"),
            concat!(
                "{{Champion info|Tester}}\n",
                "== Abilities ==\n",
                "{{Data Tester/I|Ability}}\n",
                "{{Data Tester/Q|Ability}}\n",
                "{{Data Tester/W|Ability}}\n",
                "{{Data Tester/A|Ability}}\n"
            ),
        )
        .unwrap();

        std::fs::write(
            flat.join(format!("{}.txt", url_encode("Template:Data Tester/I"))),
            "{{{{{1|Ability data}}}|Steady Hands|skill=I|description=Passive}}",
        )
        .unwrap();
        // Auxiliary slot with real content: rendered as its own ability.
        std::fs::write(
            flat.join(format!("{}.txt", url_encode("Template:Data Tester/W"))),
            "{{{{{1|Ability data}}}|Sidearm|skill=A|description=An off-hand weapon.}}",
        )
        .unwrap();
        // Non-standard slot but no content: still skipped.
        std::fs::write(
            flat.join(format!("{}.txt", url_encode("Template:Data Tester/A"))),
            "{{{{{1|Ability data}}}|Decorative|skill=X}}",
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let registry = TemplateRegistry::new();
        let loaded =
            load_abilities(&export, "Tester", 2, &HashMap::new(), &registry, None).unwrap();

        // Passive + the auxiliary-slot ability render; the content-less one is skipped.
        assert_eq!(loaded.abilities.len(), 2);
        assert!(loaded
            .abilities
            .iter()
            .any(|ability| ability.name == "Sidearm"
                && matches!(&ability.key, AbilityKey::Other(slot) if slot == "A")));
        assert!(loaded
            .warnings
            .iter()
            .any(|warning| warning.contains("Template:Data Tester/Q")
                && warning.contains("is missing")));
        assert!(loaded
            .warnings
            .iter()
            .any(|warning| warning.contains("unsupported skill slot `X`")));
    }

    #[test]
    fn load_abilities_seeds_gold_value_vars_for_wikivaluedefine_preambles() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();

        std::fs::write(
            flat.join("Tester.txt"),
            concat!(
                "{{Champion info|Tester}}\n",
                "== Abilities ==\n",
                "{{Data Tester/I|Ability}}\n"
            ),
        )
        .unwrap();
        std::fs::write(
            flat.join(format!("{}.txt", url_encode("Template:Data Tester/I"))),
            concat!(
                "{{#vardefine:i_ad|0.75}}",
                "{{#vardefine:cs|10}}",
                "{{#vardefine:stack|20}}",
                "{{#invoke:Gold value|wikivaluedefine}}",
                "{{{{{1|Ability data}}}|Measured Value|skill=I|notes=* Value = {{g|{{#expr:{{#var:i_ad}}*{{#var:ad}}+{{#var:crit}}*{{#var:cs}}/{{#var:stack}}}}}}}}"
            ),
        )
        .unwrap();
        std::fs::write(
            flat.join("Module%3AGold%20value%2Fdata.txt"),
            r#"return {
    ["ad"] = {
        ["val"] = 35,
    },
    ["crit"] = {
        ["val"] = 40,
    },
}"#,
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let registry = TemplateRegistry::new();
        let conversion_ctx =
            Arc::new(crate::convert::ConversionContext::new(td.path(), 2).unwrap());
        let loaded = load_abilities(
            &export,
            "Tester",
            2,
            &HashMap::new(),
            &registry,
            Some(conversion_ctx),
        )
        .unwrap();

        assert_eq!(loaded.abilities.len(), 1);
        assert!(loaded.abilities[0]
            .notes
            .iter()
            .any(|note| note.contains("46.25")));
    }

    #[test]
    fn validate_core_ability_coverage_warns_on_missing_slots() {
        let warnings = validate_core_ability_coverage(&[Ability {
            key: AbilityKey::Passive,
            name: "Steady Hands".to_string(),
            descriptions: Vec::new(),
            cooldowns: Vec::new(),
            costs: Vec::new(),
            ranges: Vec::new(),
            leveling_tables: Vec::new(),
            notes: Vec::new(),
            extra: HashMap::new(),
        }]);

        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("Q, W, E, R"));
    }

    #[test]
    fn extract_pets_parses_infobox_pet_sections() {
        let raw = concat!(
            "== Pets ==\n",
            "{{Infobox/Pet\n",
            "|name=Spiderling\n",
            "|control=Autonomous\n",
            "|targeting=Ward\n",
            "|abilities=\n",
            ";Pounce\n",
            "Dashes to a target.\n",
            "|notes=\n",
            "* Grants sight\n",
            "}}\n"
        );
        let registry = TemplateRegistry::new();
        let pets = extract_pets(raw, 2, &HashMap::new(), &registry, None).unwrap();

        assert_eq!(pets.len(), 1);
        assert_eq!(pets[0].name, "Spiderling");
        let stat = |label: &str| pets[0].stats.iter().find(|s| s.label == label);
        assert_eq!(stat("Control").unwrap().value, "Autonomous");
        assert_eq!(stat("Targeting").unwrap().value, "Ward");
        assert_eq!(
            stat("Abilities").unwrap().items,
            vec!["Pounce — Dashes to a target.".to_string()]
        );
        assert_eq!(
            stat("Notes").unwrap().items,
            vec!["Grants sight".to_string()]
        );
    }
}

fn derive_name_from_template(name_seg: &str) -> String {
    if let Some(pos) = name_seg.rfind('/') {
        name_seg[pos + 1..].replace('_', " ")
    } else {
        String::new()
    }
}
