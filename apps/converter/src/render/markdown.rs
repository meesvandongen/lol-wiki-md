use crate::model::{AbilityKey, Champion, ChampionSpecialMode, Item, Rune};
use crate::parse::extract_balanced_templates;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static COMMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)<!--\s*(.*?)\s*-->").expect("valid comment regex"));
static REF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?is)<ref(?:\s+name\s*=\s*"([^">/]+)"?)?\s*>(.*?)</ref>"#)
        .expect("valid ref regex")
});
static SELF_CLOSING_REF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?is)<ref(?:\s+name\s*=\s*"([^"]+)")?\s*/>"#).expect("valid ref regex")
});
static MALFORMED_SELF_CLOSING_REF_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?is)<ref\s+name\s*=\s*"([^">/]+)\s*/>"#).expect("valid malformed ref regex")
});
static REFERENCES_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<references\s*/>").expect("valid references regex"));
static SIMPLE_HTML_TAG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?is)</?(small|span|div|sup|sub|center|abbr|font|code|nowiki|i|b|u)[^>]*>")
        .expect("valid html tag regex")
});
static SKIN_TAG_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r";\s*\[Skin:\s*([^\]]+)\]").expect("valid skin tag regex"));
static MULTISPACE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[ \t]{2,}").expect("valid multispace regex"));
static INLINE_ICON_ARTIFACT_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b\d+px(?:\|[A-Za-z-]+(?:=[^|\s]*)?)+\s*").expect("valid icon artifact regex")
});
static SIMPLE_TEMPLATE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\{\{([^{}]+)\}\}").expect("valid simple template regex"));
static GALLERY_OPEN_TAG_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<gallery[^>]*>").expect("valid gallery open regex"));
static GALLERY_CLOSE_TAG_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)</gallery>").expect("valid gallery close regex"));
static TABBER_TAG_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)</?tabber>").expect("valid tabber tag regex"));
static TABBER_SEPARATOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\|\-\|").expect("valid tabber separator regex"));
static INLINE_WIKITEXT_TABLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)\{\|.*?\|\}").expect("valid inline wikitext table regex"));
static TABLE_HEADER_START_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*!\s*").expect("valid table header start regex"));
static TABLE_HEADER_SEPARATOR_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\s!\s").expect("valid table header separator regex"));
static BOLD_POSSESSIVE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\*\*([^*]+?)'\s+\*\*").expect("valid bold possessive regex"));
static ITALIC_POSSESSIVE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"_([^_\n]+?)'(\s+)_").expect("valid italic possessive regex"));
static BOLD_QUAD_APOSTROPHE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\*\*([^*]+?)'{2,}\*\*").expect("valid bold quad apostrophe regex"));
static MISSING_BOLD_POSSESSIVE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\*\*([A-Za-z][^*\n]*?)'(\s)").expect("valid missing bold possessive regex")
});
static BOLD_RUNON_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\*\*([^*]+?)'\*\*(\w)").expect("valid bold runon regex"));
static CROSSED_BOLD_ITALIC_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\*\*_([^*\n]+)\*\*((?:[^_\n]|_[^_\n]+_)+)_")
        .expect("valid crossed bold italic regex")
});
static QUAD_APOSTROPHE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([A-Za-z])'{4}(\s|$)").expect("valid quad apostrophe regex"));
static PARAM_ASSIGNMENT_PARENS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"\((?:[A-Za-z_]+=(?:true|false|[0-9.]+))(?:,\s*[A-Za-z_]+=(?:true|false|[0-9.]+))*\)",
    )
    .expect("valid param assignment regex")
});
static INLINE_TEMPLATE_PARAM_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\s+\|\s*[A-Za-z][A-Za-z0-9_]*\s*=\s*[^|\n]+")
        .expect("valid inline template param regex")
});
static CLOSE_PAREN_WORD_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\)([A-Za-z])").expect("valid close paren word regex"));
static WORD_OPEN_PAREN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"([A-Za-z])\(").expect("valid word open paren regex"));
static LINK_PLURAL_SPACE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\]\([^)]+\))\s+([a-z])\b").expect("valid link plural space regex"));
static GALLERY_FILENAME_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Gallery:\s*([A-Za-z0-9_()' -]+)\.(png|jpg|jpeg|webp|gif)")
        .expect("valid gallery filename regex")
});
static LMB_CLICKING_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\bLMB\s*clicking\b").expect("valid lmb clicking regex"));
static RMB_CLICKING_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\bRMB\s*clicking\b").expect("valid rmb clicking regex"));
static LMB_CLICK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\bLMB\s*click\b").expect("valid lmb click regex"));
static RMB_CLICK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)\bRMB\s*click\b").expect("valid rmb click regex"));

pub fn render_champion_markdown(champ: &Champion, _raw_excerpt: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", champ.name));
    if let Some(summary) = &champ.summary {
        if !summary.trim().is_empty() {
            out.push_str(&normalize_all(summary));
            out.push_str("\n\n");
        }
    }
    let mut overview: Vec<String> = Vec::new();
    if let Some(title) = &champ.basic.title {
        if !title.trim().is_empty() {
            overview.push(format!("- **Title:** {}", normalize_all(title)));
        }
    }
    if !champ.basic.roles.is_empty() {
        let roles = champ
            .basic
            .roles
            .iter()
            .map(|r| normalize_all(r))
            .collect::<Vec<_>>()
            .join(", ");
        if !roles.is_empty() {
            overview.push(format!("- **Roles:** {}", roles));
        }
    }
    if let Some(resource) = &champ.basic.resource {
        if !resource.trim().is_empty() {
            overview.push(format!("- **Resource:** {}", normalize_all(resource)));
        }
    }
    if !overview.is_empty() {
        out.push_str("## Overview\n\n");
        out.push_str(&overview.join("\n"));
        out.push_str("\n\n");
    }
    let has_stat_variants = !champ.stat_variants.is_empty();
    if has_stat_variants {
        out.push_str("## Stats\n\n");
        let primary_label = champ
            .primary_stat_label
            .as_deref()
            .filter(|label| !label.trim().is_empty())
            .unwrap_or(&champ.name);
        render_champion_stat_view(
            &mut out,
            primary_label,
            &champ.stats,
            champ.advanced.as_ref(),
            &champ.special_stats,
        );
        for variant in &champ.stat_variants {
            render_champion_stat_view(
                &mut out,
                &variant.label,
                &variant.stats,
                variant.advanced.as_ref(),
                &variant.special_stats,
            );
        }
    } else {
        if !champ.stats.base.is_empty() {
            out.push_str("## Stats\n\n");
            render_stats_table(&mut out, &champ.stats);
            out.push('\n');
        }
        if let Some(advanced) = &champ.advanced {
            if !advanced.metrics.is_empty() {
                out.push_str("## Advanced Stats\n\n");
                render_metric_table(&mut out, &advanced.metrics);
                out.push('\n');
            }
        }
        if !champ.special_stats.is_empty() {
            out.push_str("## Special Statistics\n\n");
            render_special_stats(&mut out, &champ.special_stats, 3);
        }
    }
    // Abilities
    if !champ.abilities.is_empty() {
        out.push_str("## Abilities\n\n");
        for a in &champ.abilities {
            let key_label = match &a.key {
                AbilityKey::Passive => "Passive",
                AbilityKey::Q => "Q",
                AbilityKey::W => "W",
                AbilityKey::E => "E",
                AbilityKey::R => "R",
                AbilityKey::Other(s) => s,
            };
            out.push_str(&format!(
                "### {} – {}\n\n",
                key_label,
                normalize_all(&a.name)
            ));
            // Ability info block: show key attribute/value fields in the same general order as Template:Ability info
            let info_rows: Vec<(&str, Option<&String>)> = vec![
                ("Range", a.extra.get("range")),
                ("Cast Time", a.extra.get("cast time")),
                ("Target Range", a.extra.get("target range")),
                ("Attack Range", a.extra.get("attack range")),
                ("AI Range", a.extra.get("ai range")),
                ("Windup", a.extra.get("windup")),
                ("Collision Radius", a.extra.get("collision radius")),
                ("Effect Radius", a.extra.get("effect radius")),
                ("Angle", a.extra.get("angle")),
                ("Inner Radius", a.extra.get("inner radius")),
                ("Tether Radius", a.extra.get("tether radius")),
                ("Width", a.extra.get("width")),
                ("Speed", a.extra.get("speed")),
                ("Cost", a.extra.get("cost")),
                ("Cost Type", a.extra.get("costtype")),
                ("Cooldown", a.extra.get("cooldown")),
                ("Cooldown Start", a.extra.get("cdstart")),
                ("Static", a.extra.get("static")),
                ("Recharge", a.extra.get("recharge")),
                ("On-Target CD Static", a.extra.get("ontargetcdstatic")),
                ("On-Target CD", a.extra.get("ontargetcd")),
                ("Queue Time", a.extra.get("queue time")),
            ];
            // Build the rows first so the header is only emitted when at least
            // one value survives normalization (avoids an empty table).
            let mut info_lines: Vec<String> = info_rows
                .into_iter()
                .filter_map(|(label, val_opt)| {
                    let v = normalize_table_value(val_opt?, false)?;
                    Some(format!("| **{}** | {} |\n", label, v))
                })
                .collect();
            for (label_key, info_key) in [
                ("customlabel", "custominfo"),
                ("customlabel2", "custominfo2"),
            ] {
                if let (Some(cl), Some(ci)) = (a.extra.get(label_key), a.extra.get(info_key)) {
                    if !cl.trim().is_empty() {
                        if let Some(v) = normalize_table_value(ci, false) {
                            info_lines.push(format!("| **{}** | {} |\n", cl, v));
                        }
                    }
                }
            }
            if !info_lines.is_empty() {
                out.push_str("| Attribute | Value |\n|-----------|------:|\n");
                for line in info_lines {
                    out.push_str(&line);
                }
                out.push('\n');
            }
            // Descriptions in order: description, description2..description6
            if !a.descriptions.is_empty() {
                for desc in &a.descriptions {
                    let d = normalize_all(desc);
                    out.push_str(&d);
                    out.push_str("\n\n");
                }
            }
            // Ability details block after info (order from Template:Ability details)
            let details_rows: Vec<(&str, Option<&String>)> = vec![
                ("Targeting", a.extra.get("targeting")),
                ("Affects", a.extra.get("affects")),
                ("Damage Type", a.extra.get("damagetype")),
                ("Out of Range", a.extra.get("outofrange")),
                ("Target Warning", a.extra.get("targetwarning")),
                ("Terrain Grace", a.extra.get("terraingrace")),
                ("Spell Effects", a.extra.get("spelleffects")),
                ("Spell Shield", a.extra.get("spellshield")),
                ("Parry", a.extra.get("parry")),
                ("Projectile", a.extra.get("projectile")),
                ("Call For Help", a.extra.get("callforhelp")),
                ("Grounded", a.extra.get("grounded")),
                ("Knockdown", a.extra.get("knockdown")),
                ("Silence", a.extra.get("silence")),
            ];
            let detail_lines: Vec<String> = details_rows
                .into_iter()
                .filter_map(|(label, val_opt)| {
                    let v = normalize_table_value(val_opt?, true)?;
                    Some(format!("| **{}** | {} |\n", label, v))
                })
                .collect();
            if !detail_lines.is_empty() {
                out.push_str("| Detail | Value |\n|--------|------:|\n");
                for line in detail_lines {
                    out.push_str(&line);
                }
                out.push('\n');
            }
            if !a.notes.is_empty() {
                out.push_str("**Notes:**\n\n");
                render_starred_list(&mut out, &a.notes);
                out.push('\n');
            }
            // Leveling tables as simple sub-tables if present
            if !a.leveling_tables.is_empty() {
                for st in &a.leveling_tables {
                    if !st.headers.is_empty() {
                        out.push_str(&format!("| {} |\n", st.headers.join(" | ")));
                        out.push_str(&format!(
                            "|{}|\n",
                            st.headers
                                .iter()
                                .map(|_| "---")
                                .collect::<Vec<_>>()
                                .join("|")
                        ));
                    }
                    for row in &st.rows {
                        out.push_str(&format!("| {} |\n", row.join(" | ")));
                    }
                    out.push('\n');
                }
            }
        }
    }
    if !champ.notes.is_empty() {
        out.push_str("## Notes\n\n");
        render_starred_list(&mut out, &champ.notes);
        out.push('\n');
    }
    if !champ.pets.is_empty() {
        out.push_str("## Pets\n\n");
        for pet in &champ.pets {
            let name = normalize_all(&pet.name);
            if !pet.stats.is_empty() {
                out.push_str(&format!("- **{}**\n", name));
                for stat in &pet.stats {
                    let label = normalize_all(&stat.label);
                    if stat.items.is_empty() {
                        out.push_str(&format!(
                            "  - {}: {}\n",
                            label,
                            normalize_all(&stat.value)
                        ));
                    } else {
                        out.push_str(&format!("  - {}:\n", label));
                        for item in &stat.items {
                            out.push_str(&format!("    - {}\n", normalize_all(item)));
                        }
                    }
                }
            } else {
                let desc = normalize_all(&pet.description);
                if desc.is_empty() {
                    out.push_str(&format!("- {}\n", name));
                } else {
                    out.push_str(&format!("- **{}** — {}\n", name, desc));
                }
            }
        }
        out.push('\n');
    }
    if !champ.trivia.is_empty() {
        out.push_str("## Trivia\n\n");
        render_starred_list(&mut out, &champ.trivia);
        out.push('\n');
    }
    if !champ.warnings.is_empty() {
        out.push_str("## Validation\n\n");
        for warning in &champ.warnings {
            out.push_str(&format!("- {}\n", normalize_all(warning)));
        }
        out.push('\n');
    }
    if !champ.patch_history.is_empty() {
        out.push_str("## Patch History\n\n");
        for entry in &champ.patch_history {
            if entry.version.trim().is_empty() {
                continue;
            }
            out.push_str(&format!("### {}\n\n", normalize_all(&entry.version)));
            for change in &entry.changes {
                let section_label = change.section.trim();
                let text = change.text.trim();
                if section_label.eq_ignore_ascii_case("general") || section_label.is_empty() {
                    if text.is_empty() {
                        continue;
                    }
                    if text.contains('\n') {
                        for line in text.lines() {
                            let detail = line.trim();
                            if detail.is_empty() {
                                continue;
                            }
                            out.push_str(&format!("- {}\n", normalize_all(detail)));
                        }
                    } else {
                        out.push_str(&format!("- {}\n", normalize_all(text)));
                    }
                    continue;
                }

                let normalized_label = normalize_all(section_label);
                if text.is_empty() {
                    out.push_str(&format!("- **{}**\n", normalized_label));
                    continue;
                }
                if text.contains('\n') {
                    out.push_str(&format!("- **{}**\n", normalized_label));
                    for line in text.lines() {
                        let detail = line.trim();
                        if detail.is_empty() {
                            continue;
                        }
                        out.push_str(&format!("  - {}\n", normalize_all(detail)));
                    }
                } else {
                    out.push_str(&format!(
                        "- **{}** — {}\n",
                        normalized_label,
                        normalize_all(text)
                    ));
                }
            }
            out.push('\n');
        }
    }
    ensure_trailing_newline(&collapse_blank_lines(&normalize_rendered_body(&out)))
}

fn render_champion_stat_view(
    out: &mut String,
    label: &str,
    stats: &crate::model::Stats,
    advanced: Option<&crate::model::AdvancedStats>,
    special_stats: &[ChampionSpecialMode],
) {
    let has_advanced = advanced
        .map(|advanced| !advanced.metrics.is_empty())
        .unwrap_or(false);
    if stats.base.is_empty() && !has_advanced && special_stats.is_empty() {
        return;
    }

    out.push_str(&format!("### {}\n\n", normalize_all(label)));
    if !stats.base.is_empty() {
        render_stats_table(out, stats);
        out.push('\n');
    }
    if let Some(advanced) = advanced {
        if !advanced.metrics.is_empty() {
            out.push_str("**Advanced Stats**\n\n");
            render_metric_table(out, &advanced.metrics);
            out.push('\n');
        }
    }
    if !special_stats.is_empty() {
        out.push_str("**Special Statistics**\n\n");
        render_special_stats(out, special_stats, 4);
    }
}

fn render_stats_table(out: &mut String, stats: &crate::model::Stats) {
    out.push_str("| Stat | Base | Growth |\n|------|------|--------|\n");
    let mut keys: Vec<_> = stats
        .base
        .keys()
        .filter(|key| !key.starts_with("resource__"))
        .collect();
    keys.sort();
    for key in keys {
        if key == "resource" {
            continue;
        }
        if let Some(value) = stats.base.get(key) {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                key, value.base, value.growth
            ));
        }
    }
}

fn render_metric_table(out: &mut String, metrics: &HashMap<String, String>) {
    out.push_str("| Metric | Value |\n|--------|-------|\n");
    let mut rows: Vec<_> = metrics.iter().collect();
    rows.sort_by(|(a_label, _), (b_label, _)| a_label.to_lowercase().cmp(&b_label.to_lowercase()));
    for (label, value) in rows {
        out.push_str(&format!(
            "| {} | {} |\n",
            normalize_all(label),
            normalize_all(value)
        ));
    }
}

fn render_special_stats(
    out: &mut String,
    special_stats: &[ChampionSpecialMode],
    heading_level: usize,
) {
    for mode in special_stats {
        if mode.metrics.is_empty() {
            continue;
        }
        out.push_str(&format!(
            "{} {}\n\n",
            "#".repeat(heading_level),
            normalize_all(&mode.mode)
        ));
        render_metric_table(out, &mode.metrics);
        out.push('\n');
    }
}

pub fn render_item_markdown(item: &Item, _raw_excerpt: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", item.name));

    if let Some(removed_patch) = item
        .removed_patch
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        out.push_str(
            "> This article or section may contain obsolete information, but exists here for historical purposes.\n",
        );
        out.push_str(&format!(
            "> This item was removed on patch {}.\n\n",
            normalize_all(removed_patch)
        ));
    }

    if let Some(lead) = synthesize_item_lead(item) {
        out.push_str(&lead);
        out.push_str("\n\n");
    }

    if let Some(caption) = &item.caption {
        if !caption.trim().is_empty() {
            out.push_str(&normalize_all(caption));
            out.push_str("\n\n");
        }
    }

    let mut overview: Vec<String> = Vec::new();
    if let Some(tier) = &item.tier {
        if !tier.trim().is_empty() {
            overview.push(format!("- **Tier:** {}", tier));
        }
    }
    if !item.categories.is_empty() {
        overview.push(format!("- **Type:** {}", item.categories.join(", ")));
    }
    if item.cost_total.is_some() || item.cost_sell.is_some() || item.sell_ratio.is_some() {
        let total = item.cost_total.map(|v| v.to_string());
        let sell = item.cost_sell.map(|v| v.to_string());
        let ratio = item.sell_ratio.map(|v| format!("{:.2}", v));
        let combine = item.cost_combine.map(|v| v.to_string());
        let mut pieces: Vec<String> = Vec::new();
        if let Some(t) = total {
            pieces.push(format!("Total: {}", t));
        }
        if let Some(c) = combine {
            pieces.push(format!("Combine: {}", c));
        }
        if let Some(s) = sell {
            pieces.push(format!("Sell: {}", s));
        }
        if let Some(r) = ratio {
            pieces.push(format!("Sell ratio: {}", r));
        }
        if !pieces.is_empty() {
            overview.push(format!("- **Cost:** {}", pieces.join(" • ")));
        }
    }
    if let Some(limit) = &item.limit {
        if !limit.trim().is_empty() {
            overview.push(format!("- **Limit:** {}", normalize_all(limit)));
        }
    }
    if item.ornn_forged {
        overview.push("- **Forged by:** Ornn".to_string());
    }
    if let Some(removed_patch) = item
        .removed_patch
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        overview.push(format!("- **Removed:** {}", normalize_all(removed_patch)));
    }
    if item.gold_value.is_some() || item.gold_efficiency.is_some() {
        let mut pieces: Vec<String> = Vec::new();
        if let Some(gold_value) = &item.gold_value {
            if !gold_value.trim().is_empty() {
                pieces.push(format!("Gold value: {}", normalize_all(gold_value)));
            }
        }
        if let Some(gold_efficiency) = &item.gold_efficiency {
            if !gold_efficiency.trim().is_empty() {
                pieces.push(format!(
                    "Gold efficiency: {}",
                    normalize_all(gold_efficiency)
                ));
            }
        }
        if !pieces.is_empty() {
            overview.push(format!("- **Economics:** {}", pieces.join(" • ")));
        }
    }
    if !item.modes.is_empty() {
        overview.push(format!("- **Modes:** {}", item.modes.join(", ")));
    }
    if !overview.is_empty() {
        out.push_str("## Overview\n\n");
        out.push_str(&overview.join("\n"));
        out.push_str("\n\n");
    }

    if let Some(background) = &item.background {
        if !background.trim().is_empty() {
            out.push_str("## Background\n\n");
            out.push_str(&normalize_all(background));
            out.push_str("\n\n");
        }
    }

    if !item.recipe.is_empty() || !item.upgrades.is_empty() {
        out.push_str("## Build Tree\n\n");
        if !item.recipe.is_empty() {
            out.push_str("**Components**\n\n");
            for component in &item.recipe {
                out.push_str(&format!("- {}\n", normalize_all(component)));
            }
            out.push('\n');
        }
        if !item.upgrades.is_empty() {
            out.push_str("**Upgrades**\n\n");
            for upgrade in &item.upgrades {
                out.push_str(&format!("- {}\n", normalize_all(upgrade)));
            }
            out.push('\n');
        }
    }

    if !item.stats.is_empty() {
        out.push_str("## Stats\n\n| Stat | Value |\n|------|-------|\n");
        let mut keys: Vec<_> = item.stats.keys().collect();
        keys.sort();
        for key in keys {
            if let Some(val) = item.stats.get(key) {
                let norm_val = normalize_all(val);
                out.push_str(&format!("| {} | {} |\n", key, norm_val));
            }
        }
        out.push('\n');
    }

    if !item.warnings.is_empty() {
        out.push_str("## Validation\n\n");
        for warn in &item.warnings {
            out.push_str(&format!("- {}\n", normalize_all(warn)));
        }
        out.push('\n');
    }

    if !item.effects.is_empty() {
        out.push_str("## Effects\n\n");
        for effect in &item.effects {
            let title = match &effect.name {
                Some(name) if !name.trim().is_empty() => {
                    format!("### {} – {}\n\n", effect.kind, normalize_all(name))
                }
                _ => format!("### {}\n\n", effect.kind),
            };
            out.push_str(&title);
            let mut qualifiers: Vec<String> = Vec::new();
            if effect.unique {
                qualifiers.push("Unique".to_string());
            }
            if let Some(cd) = &effect.cooldown {
                if !cd.trim().is_empty() {
                    qualifiers.push(format!("Cooldown: {}", normalize_all(cd)));
                }
            }
            if let Some(range) = &effect.range {
                if !range.trim().is_empty() {
                    qualifiers.push(format!("Range: {}", normalize_all(range)));
                }
            }
            if !qualifiers.is_empty() {
                out.push_str(&format!("_{}_.\n\n", qualifiers.join(" • ")));
            }
            let desc = normalize_all(&effect.description);
            if !desc.trim().is_empty() {
                out.push_str(&desc);
                out.push_str("\n\n");
            }
        }
    }

    if !item.similar_items.is_empty() {
        out.push_str("## Similar Items\n\n");
        for item_name in &item.similar_items {
            out.push_str(&format!("- {}\n", normalize_all(item_name)));
        }
        out.push('\n');
    }

    if let Some(strategy) = &item.strategy {
        if !strategy.trim().is_empty() {
            out.push_str("## Strategy\n\n");
            out.push_str(&normalize_all(strategy));
            out.push_str("\n\n");
        }
    }

    if !item.notes.is_empty() {
        out.push_str("## Notes\n\n");
        render_starred_list(&mut out, &item.notes);
        out.push('\n');
    }

    if !item.trivia.is_empty() {
        out.push_str("## Trivia\n\n");
        render_starred_list(&mut out, &item.trivia);
        out.push('\n');
    }

    if !item.patch_history.is_empty() {
        out.push_str("## Patch History\n\n");
        for entry in &item.patch_history {
            if entry.version.trim().is_empty() {
                continue;
            }
            out.push_str(&format!("### {}\n\n", normalize_all(&entry.version)));
            for change in &entry.changes {
                let section_label = change.section.trim();
                let text = change.text.trim();
                if section_label.eq_ignore_ascii_case("general") || section_label.is_empty() {
                    if text.is_empty() {
                        continue;
                    }
                    if text.contains('\n') {
                        for line in text.lines() {
                            let detail = line.trim();
                            if detail.is_empty() {
                                continue;
                            }
                            out.push_str(&format!("- {}\n", normalize_all(detail)));
                        }
                    } else {
                        out.push_str(&format!("- {}\n", normalize_all(text)));
                    }
                    continue;
                }

                let normalized_label = normalize_all(section_label);
                if text.is_empty() {
                    out.push_str(&format!("- **{}**\n", normalized_label));
                    continue;
                }
                if text.contains('\n') {
                    out.push_str(&format!("- **{}**\n", normalized_label));
                    for line in text.lines() {
                        let detail = line.trim();
                        if detail.is_empty() {
                            continue;
                        }
                        out.push_str(&format!("  - {}\n", normalize_all(detail)));
                    }
                } else {
                    out.push_str(&format!(
                        "- **{}** — {}\n",
                        normalized_label,
                        normalize_all(text)
                    ));
                }
            }
            out.push('\n');
        }
    }

    ensure_trailing_newline(&collapse_blank_lines(&normalize_rendered_body(&out)))
}

fn synthesize_item_lead(item: &Item) -> Option<String> {
    let status_verb = if item
        .removed_patch
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .is_some()
    {
        "was"
    } else {
        "is"
    };
    let kind = item_lead_kind(item);
    let mut lead = format!(
        "{} {} {} in League of Legends.",
        normalize_all(&item.name),
        status_verb,
        with_indefinite_article(&kind)
    );
    if item.ornn_forged {
        lead.push(' ');
        if status_verb == "was" {
            lead.push_str("Could only be forged by Ornn.");
        } else {
            lead.push_str("Can only be forged by Ornn.");
        }
    }
    Some(lead)
}

fn item_lead_kind(item: &Item) -> String {
    item.categories
        .first()
        .map(|category| format!("{} item", category.trim().to_ascii_lowercase()))
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "item".to_string())
}

fn with_indefinite_article(phrase: &str) -> String {
    let article = if starts_with_vowel_sound(phrase) {
        "an"
    } else {
        "a"
    };
    format!("{} {}", article, phrase.trim())
}

fn starts_with_vowel_sound(phrase: &str) -> bool {
    matches!(
        phrase
            .trim()
            .chars()
            .next()
            .map(|ch| ch.to_ascii_lowercase()),
        Some('a' | 'e' | 'i' | 'o' | 'u')
    )
}

pub fn render_rune_markdown(rune: &Rune, _raw_excerpt: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", rune.name));

    if let Some(lead) = synthesize_rune_lead(rune) {
        out.push_str(&lead);
        out.push_str("\n\n");
    }

    let mut overview: Vec<String> = Vec::new();
    if let Some(path) = &rune.path {
        if !path.trim().is_empty() {
            overview.push(format!("- **Path:** {}", normalize_all(path)));
        }
    }
    if let Some(slot) = &rune.slot {
        if !slot.trim().is_empty() {
            overview.push(format!("- **Slot:** {}", normalize_all(slot)));
        }
    }
    if !overview.is_empty() {
        out.push_str("## Overview\n\n");
        out.push_str(&overview.join("\n"));
        out.push_str("\n\n");
    }

    if !rune.description.trim().is_empty() {
        out.push_str("## Description\n\n");
        out.push_str(&normalize_all(&rune.description));
        out.push_str("\n\n");
    }

    if let Some(caption) = &rune.caption {
        if !caption.trim().is_empty() {
            out.push_str(&normalize_all(caption));
            out.push_str("\n\n");
        }
    }

    if let Some(map_changes) = &rune.map_changes {
        if !map_changes.trim().is_empty() {
            out.push_str("## Map-Specific Differences\n\n");
            out.push_str(&normalize_all(map_changes));
            out.push_str("\n\n");
        }
    }

    if !rune.notes.is_empty() {
        out.push_str("## Notes\n\n");
        render_starred_list(&mut out, &rune.notes);
        out.push('\n');
    }

    if !rune.trivia.is_empty() {
        out.push_str("## Trivia\n\n");
        render_starred_list(&mut out, &rune.trivia);
        out.push('\n');
    }

    if !rune.warnings.is_empty() {
        out.push_str("## Validation\n\n");
        for warning in &rune.warnings {
            out.push_str(&format!("- {}\n", normalize_all(warning)));
        }
        out.push('\n');
    }

    if !rune.patch_history.is_empty() {
        out.push_str("## Patch History\n\n");
        for entry in &rune.patch_history {
            if entry.version.trim().is_empty() {
                continue;
            }
            out.push_str(&format!("### {}\n\n", normalize_all(&entry.version)));
            for change in &entry.changes {
                let text = normalize_all(&change.text);
                if text.trim().is_empty() {
                    continue;
                }
                out.push_str(&format!("- {}\n", text));
            }
            out.push('\n');
        }
    }

    ensure_trailing_newline(&collapse_blank_lines(&normalize_rendered_body(&out)))
}

fn synthesize_rune_lead(rune: &Rune) -> Option<String> {
    let name = rune.name.trim();
    if name.is_empty() {
        None
    } else {
        Some(format!(
            "{} is a rune in League of Legends.",
            normalize_all(name)
        ))
    }
}

pub fn normalize_internal_links(s: &str) -> String {
    // Transform [[Page]] or [[Page|Display]] to [Display](./Page.md)
    let mut out = String::new();
    let mut remaining = s;
    while let Some(open) = remaining.find("[[") {
        out.push_str(&remaining[..open]);
        remaining = &remaining[open + 2..];
        let Some(close) = remaining.find("]]") else {
            out.push_str("[[");
            out.push_str(remaining);
            return out;
        };
        let inner = &remaining[..close];
        remaining = &remaining[close + 2..];
        let parts = inner.split('|').collect::<Vec<_>>();
        let page = parts.first().copied().unwrap_or("");
        let display = if parts.len() >= 2 {
            let disp = parts[1..].join("|");
            if disp.is_empty() {
                page.to_string()
            } else {
                disp
            }
        } else {
            page.to_string()
        };
        if page.starts_with("http://") || page.starts_with("https://") {
            let mut parts = page.splitn(2, char::is_whitespace);
            let url = parts.next().unwrap_or("");
            let label = parts
                .next()
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .unwrap_or(display.trim());
            let label = if label.is_empty() { url } else { label };
            out.push_str(&format!("[{}]({})", label, url));
            continue;
        }
        if page.starts_with("File:") || page.starts_with(":File:") {
            if let Some(caption) = render_file_link_caption(&parts) {
                out.push_str(&caption);
            }
            continue;
        }
        // Normalize page name: spaces and / to _, extract anchor
        let (link_target, anchor) = if let Some(hash_pos) = page.find('#') {
            let target = page[..hash_pos].replace(|c: char| c == ' ' || c == '/', "_");
            let anch = &page[hash_pos + 1..];
            (target, format!("#{}", anch))
        } else {
            (
                page.replace(|c: char| c == ' ' || c == '/', "_"),
                String::new(),
            )
        };
        let link = format!("[{}](./{}.md{})", display, link_target, anchor);
        out.push_str(&link);
    }
    out.push_str(remaining);
    out
}

fn render_file_link_caption(parts: &[&str]) -> Option<String> {
    for part in parts.iter().skip(1).rev() {
        let trimmed = part.trim();
        if trimmed.is_empty() || is_file_link_parameter(trimmed) {
            continue;
        }
        return Some(trimmed.to_string());
    }
    None
}

fn is_file_link_parameter(part: &str) -> bool {
    let lower = part.trim().to_ascii_lowercase();
    if lower.is_empty() {
        return true;
    }
    if lower.contains('=') {
        return true;
    }
    if let Some(px) = lower.strip_suffix("px") {
        if px.chars().all(|ch| ch.is_ascii_digit()) {
            return true;
        }
    }
    matches!(
        lower.as_str(),
        "thumb"
            | "thumbnail"
            | "frame"
            | "frameless"
            | "border"
            | "right"
            | "left"
            | "center"
            | "none"
            | "baseline"
            | "sub"
            | "super"
            | "top"
            | "text-top"
            | "middle"
            | "bottom"
            | "text-bottom"
    )
}

pub fn normalize_apostrophes(s: &str) -> String {
    // Stateful conversion of wiki apostrophes to Markdown markers.
    // Handles 2 (italic), 3 (bold), 5 (bold+italic) with simple toggles.
    let mut out = String::new();
    let chars = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut bold_on = false;
    let mut italic_on = false;
    while i < chars.len() {
        if chars[i] == '\'' {
            let mut j = i;
            while j < chars.len() && chars[j] == '\'' {
                j += 1;
            }
            let count = j - i;
            match count {
                5 => {
                    // Toggle both, preserve order for closing.
                    if !bold_on && !italic_on {
                        out.push_str("**_");
                        bold_on = true;
                        italic_on = true;
                    } else {
                        out.push_str("_**");
                        bold_on = false;
                        italic_on = false;
                    }
                }
                3 => {
                    out.push_str("**");
                    bold_on = !bold_on;
                }
                2 => {
                    out.push('_');
                    italic_on = !italic_on;
                }
                _ => {
                    for _ in 0..count {
                        out.push('\'');
                    }
                }
            }
            i = j;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

pub fn normalize_anchors(s: &str) -> String {
    // Normalize anchors in links: strip parens, unify dashes, remove commas
    let re = Regex::new(r"\[\[([^\]|#]+)#([^\]|]+)\]\]").unwrap();
    re.replace_all(s, |caps: &regex::Captures| {
        let page = caps.get(1).unwrap().as_str();
        let mut sec = caps.get(2).unwrap().as_str().to_string();
        // Normalize section: strip parens, unify dashes, remove commas
        sec = sec.replace("(", "").replace(")", "").replace(",", "");
        // Unify dashes: replace multiple - with single -
        let re_dash = Regex::new(r"-+").unwrap();
        sec = re_dash.replace_all(&sec, "-").to_string();
        // Replace spaces with -
        sec = sec.replace(' ', "-");
        format!("{}#{}", page, sec)
    })
    .to_string()
}

fn collapse_blank_lines(s: &str) -> String {
    let mut out = String::new();
    let mut last_blank = false;
    for line in s.lines() {
        let blank = line.trim().is_empty();
        if blank && last_blank {
            continue;
        }
        out.push_str(line);
        out.push('\n');
        last_blank = blank;
    }
    out
}

fn ensure_trailing_newline(s: &str) -> String {
    if s.ends_with('\n') {
        s.to_string()
    } else {
        format!("{s}\n")
    }
}

fn normalize_excerpt(s: &str) -> String {
    let mut normalized = normalize_apostrophes(s)
        .replace("<br />", "\n")
        .replace("<br/>", "\n")
        .replace("<br>", "\n")
        .replace("ÃÂ ", " ")
        .replace("Â ", " ")
        .replace("â", "–")
        .replace("â", "—")
        .replace("â", "’")
        .replace("â", "‘")
        .replace("â", "“")
        .replace("â", "”")
        .replace("â¢", "•")
        .replace("â¦", "…")
        .replace('\u{00a0}', " ");
    for _ in 0..3 {
        let fixed = fix_common_mojibake(&normalized);
        if fixed == normalized {
            break;
        }
        normalized = fixed;
    }
    normalized = fix_known_mojibake_sequences(&normalized);
    normalize_internal_links(&normalize_anchors(&normalized))
}

fn normalize_all(s: &str) -> String {
    let normalized = normalize_excerpt(s);
    let normalized = normalize_comments(&normalized);
    let normalized = normalize_refs(&normalized);
    let normalized = strip_simple_html_tags(&normalized);
    let normalized = normalize_external_links(&normalized);
    let normalized = normalize_simple_templates(&normalized);
    let normalized = normalize_reader_artifacts(&normalized);
    let normalized = SKIN_TAG_RE
        .replace_all(&normalized, " (Skin: $1)")
        .to_string();
    let normalized = MULTISPACE_RE
        .replace_all(&normalized, " ")
        .trim()
        .to_string();
    if normalized.contains("{{")
        || normalized.contains("<!--")
        || normalized.contains("|yvideo")
    {
        collapse_residual_comment_payload(&normalized)
    } else {
        normalized
    }
}

fn fix_common_mojibake(s: &str) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        let lead = chars[i] as u32;
        let seq_len = if (0xC2..=0xDF).contains(&lead) {
            2
        } else if (0xE0..=0xEF).contains(&lead) {
            3
        } else if (0xF0..=0xF4).contains(&lead) {
            4
        } else {
            0
        };

        if seq_len > 1
            && i + seq_len <= chars.len()
            && chars[i..i + seq_len].iter().all(|ch| (*ch as u32) <= 0xFF)
        {
            let bytes = chars[i..i + seq_len]
                .iter()
                .map(|ch| *ch as u8)
                .collect::<Vec<_>>();
            if let Ok(decoded) = std::str::from_utf8(&bytes) {
                out.push_str(decoded);
                i += seq_len;
                continue;
            }
        }

        out.push(chars[i]);
        i += 1;
    }
    out
}

fn fix_known_mojibake_sequences(s: &str) -> String {
    s.replace("\u{00e2}\u{0080}\u{0093}", "–")
        .replace("\u{00e2}\u{0080}\u{0094}", "—")
        .replace("\u{00e2}\u{0080}\u{0099}", "’")
        .replace("\u{00e2}\u{0080}\u{0098}", "‘")
        .replace("\u{00e2}\u{0080}\u{009c}", "“")
        .replace("\u{00e2}\u{0080}\u{009d}", "”")
        .replace("\u{00e2}\u{0080}\u{00a2}", "•")
        .replace("\u{00e2}\u{0080}\u{00a6}", "…")
        .replace("\u{00e2}\u{009f}\u{00b7}", "•")
        .replace("\u{00e3}\u{0080}\u{008c}", "“")
        .replace("\u{00e3}\u{0080}\u{008d}", "”")
        .replace("\u{00c3}\u{0097}", "×")
        .replace("\u{00c3}\u{0081}", "Á")
        .replace("\u{00c3}\u{0089}", "É")
        .replace("\u{00c3}\u{008d}", "Í")
        .replace("\u{00c3}\u{0093}", "Ó")
        .replace("\u{00c3}\u{0096}", "Ö")
        .replace("\u{00c3}\u{009a}", "Ú")
        .replace("\u{00c3}\u{009c}", "Ü")
        .replace("\u{00c3}\u{00a1}", "á")
        .replace("\u{00c3}\u{00a9}", "é")
        .replace("\u{00c3}\u{00ad}", "í")
        .replace("\u{00c3}\u{00b3}", "ó")
        .replace("\u{00c3}\u{00b6}", "ö")
        .replace("\u{00c3}\u{00ba}", "ú")
        .replace("\u{00c3}\u{00bc}", "ü")
        .replace("\u{00c5}\u{0090}", "Ő")
        .replace("\u{00c5}\u{00b0}", "Ű")
        .replace("\u{00c5}\u{0091}", "ő")
        .replace("\u{00c5}\u{00b1}", "ű")
}

fn make_possessive(label: &str) -> String {
    let trimmed = label.trim();
    if trimmed.is_empty()
        || trimmed.ends_with("'s")
        || trimmed.ends_with("’s")
        || trimmed.ends_with('\'')
        || trimmed.ends_with('’')
    {
        return trimmed.to_string();
    }
    if trimmed.ends_with('s') || trimmed.ends_with('S') {
        format!("{}'", trimmed)
    } else {
        format!("{}'s", trimmed)
    }
}

fn decode_template_entities(s: &str) -> String {
    s.replace("&#123;&#123;", "{{")
        .replace("&#125;&#125;", "}}")
        .replace("&#123;", "{")
        .replace("&#125;", "}")
        .replace("&nbsp;", " ")
}

fn split_simple_named_arg(arg: &str) -> Option<(&str, &str)> {
    let (key, value) = arg.split_once('=')?;
    let key = key.trim();
    if key.is_empty()
        || !key
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-')
    {
        return None;
    }
    Some((key, value.trim()))
}

fn simple_positional_args<'a>(args: &'a [&'a str]) -> Vec<&'a str> {
    args.iter()
        .copied()
        .filter(|arg| split_simple_named_arg(arg).is_none())
        .collect()
}

fn render_csl_template(args: &[&str]) -> String {
    let positional = simple_positional_args(args);
    let first = positional.first().copied().unwrap_or("");
    let second = positional.get(1).copied().unwrap_or("");
    let last = positional.last().copied().unwrap_or(first);
    if positional.len() >= 3 && !last.is_empty() {
        last.to_string()
    } else if positional.len() >= 2 {
        if second.eq_ignore_ascii_case("Original") {
            first.to_string()
        } else if first.is_empty() {
            second.to_string()
        } else if second.is_empty() {
            first.to_string()
        } else {
            format!("{} {}", second, first)
        }
    } else {
        first.to_string()
    }
}

fn render_rd_template(args: &[&str]) -> String {
    let positional = simple_positional_args(args);
    let melee = positional.first().copied().unwrap_or("").trim();
    let ranged = positional.get(1).copied().unwrap_or(melee).trim();
    if melee.is_empty() && ranged.is_empty() {
        String::new()
    } else if ranged.is_empty() || ranged == melee {
        melee.to_string()
    } else {
        format!("{} (melee) / {} (ranged)", melee, ranged)
    }
}

fn render_ig_template(args: &[&str]) -> String {
    let positional = simple_positional_args(args);
    positional
        .get(1)
        .copied()
        .or_else(|| positional.first().copied())
        .unwrap_or("")
        .trim()
        .to_string()
}

fn render_wi_template(args: &[&str]) -> String {
    let positional = simple_positional_args(args);
    positional
        .get(1)
        .copied()
        .or_else(|| positional.first().copied())
        .unwrap_or("")
        .trim()
        .to_string()
}

fn render_mastery_icon_template(args: &[&str]) -> String {
    simple_positional_args(args)
        .first()
        .copied()
        .unwrap_or("")
        .trim()
        .to_string()
}

fn render_lorskin_template(args: &[&str]) -> String {
    let positional = simple_positional_args(args);
    let first = positional.first().copied().unwrap_or("").trim();
    let second = positional.get(1).copied().unwrap_or("").trim();
    let third = positional.get(2).copied().unwrap_or("").trim();
    if !third.is_empty() {
        third.to_string()
    } else if second.is_empty() || second.eq_ignore_ascii_case("Original") {
        first.to_string()
    } else if first.is_empty() {
        second.to_string()
    } else {
        format!("{} {}", second, first)
    }
}

fn render_recurring_template(args: &[&str]) -> String {
    let digits = simple_positional_args(args)
        .first()
        .copied()
        .unwrap_or("")
        .trim();
    let mut rendered = String::new();
    for ch in digits.chars() {
        rendered.push(ch);
        if !ch.is_whitespace() {
            rendered.push('\u{0305}');
        }
    }
    rendered
}

fn render_item_stat_table_fallback(_args: &[&str]) -> Option<String> {
    // The item stat table requires module data and is handled by the template
    // registry; there is no meaningful inline rendering at this stage.
    None
}

fn render_fd_template_fallback(args: &[&str]) -> Option<String> {
    let value = simple_positional_args(args)
        .first()
        .copied()
        .unwrap_or("")
        .trim();
    let numeric = value.strip_suffix('%').map(str::trim).unwrap_or(value);
    if numeric.parse::<f64>().is_ok() {
        Some(value.to_string())
    } else {
        None
    }
}

fn flatten_wikitext_table(table: &str) -> String {
    let mut content = table.replace('\n', " ").replace('\r', " ");
    content = content.trim().trim_start_matches("{|").trim().to_string();
    let Some(first_cell) = content.find(['|', '!']) else {
        return String::new();
    };
    content = content[first_cell..].to_string();
    content = content.replace("|}", "");
    content = content.replace("|-", "|§ROW§|");
    content = TABLE_HEADER_START_RE.replace(&content, "|").to_string();
    content = TABLE_HEADER_SEPARATOR_RE
        .replace_all(&content, " | ")
        .to_string();

    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut current_row: Vec<String> = Vec::new();
    for token in content
        .split('|')
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        if token == "§ROW§" {
            if !current_row.is_empty() {
                rows.push(std::mem::take(&mut current_row));
            }
            continue;
        }
        current_row.push(token.to_string());
    }
    if !current_row.is_empty() {
        rows.push(current_row);
    }

    rows.into_iter()
        .filter(|row| !row.is_empty())
        .map(|row| row.join(" | "))
        .collect::<Vec<_>>()
        .join(" / ")
}

fn render_lll_template(args: &[&str]) -> String {
    let positional = simple_positional_args(args);
    let first = positional.first().copied().unwrap_or("").trim();
    let second = positional.get(1).copied().unwrap_or("").trim();
    let third = positional.get(2).copied().unwrap_or("").trim();
    if !third.is_empty() {
        third.to_string()
    } else if second.is_empty() || second.eq_ignore_ascii_case("Base") {
        first.to_string()
    } else if first.is_empty() {
        second.to_string()
    } else {
        format!("{} ({})", first, second)
    }
}

fn render_note_template(args: &[&str]) -> String {
    let positional = simple_positional_args(args);
    match positional.first().copied().unwrap_or("").trim() {
        "1" => "Editor's note:".to_string(),
        "2" => "Developer's note:".to_string(),
        _ => "Note:".to_string(),
    }
}

fn render_adaptive_template(args: &[&str]) -> Option<String> {
    let raw = simple_positional_args(args)
        .first()
        .copied()
        .unwrap_or("")
        .trim();
    if raw.is_empty() {
        return Some(String::new());
    }

    if let Some((start, separator, end)) = split_adaptive_range(raw) {
        let start_value = parse_adaptive_number(start)?;
        let end_value = parse_adaptive_number(end)?;
        return Some(format!(
            "{} **bonus** Attack Damage or {} Ability Power (Adaptive)",
            format_adaptive_range(start_value * 0.6, separator, end_value * 0.6, true),
            format_adaptive_range(start_value, separator, end_value, false)
        ));
    }

    let value = parse_adaptive_number(raw)?;
    Some(format!(
        "{} **bonus** Attack Damage or {} Ability Power (Adaptive)",
        format_adaptive_damage_value(value * 0.6),
        format_adaptive_ap_value(value)
    ))
}

fn split_adaptive_range(raw: &str) -> Option<(&str, &'static str, &str)> {
    let trimmed = raw.trim();
    for separator in [" to ", " – ", "–", " — ", "—"] {
        if let Some((start, end)) = trimmed.split_once(separator) {
            return Some((start.trim(), separator, end.trim()));
        }
    }
    None
}

fn parse_adaptive_number(raw: &str) -> Option<f64> {
    let trimmed = raw.trim();
    if let Ok(value) = trimmed.parse::<f64>() {
        return Some(value);
    }
    // Endpoints may be simple arithmetic expressions (e.g. `1.8*12`).
    crate::parse::expr::evaluate_expression(trimmed, crate::parse::expr::ExprNumberFormat::Float(6))
        .ok()
        .and_then(|value| value.trim().parse::<f64>().ok())
}

fn format_adaptive_range(start: f64, separator: &str, end: f64, is_damage: bool) -> String {
    let format_value = if is_damage {
        format_adaptive_damage_value as fn(f64) -> String
    } else {
        format_adaptive_ap_value as fn(f64) -> String
    };
    format!("{}{}{}", format_value(start), separator, format_value(end))
}

fn format_adaptive_damage_value(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        format!("{}", value.round() as i64)
    } else {
        format!("{value:.2}")
    }
}

fn format_adaptive_ap_value(value: f64) -> String {
    if (value - value.round()).abs() < 1e-9 {
        return format!("{}", value.round() as i64);
    }
    let mut rendered = format!("{value:.2}");
    while rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }
    rendered
}

fn collapse_noisy_comment_payload(body: &str) -> String {
    let trimmed = body.trim();
    let cut = [
        trimmed.find("{{#"),
        trimmed.find("\n|"),
        trimmed.find("|yvideo"),
    ]
    .into_iter()
    .flatten()
    .min();
    let candidate = cut
        .map(|idx| trimmed[..idx].trim_end())
        .unwrap_or(trimmed)
        .trim();
    if candidate.is_empty() {
        return String::new();
    }
    if cut.is_some() {
        if candidate.eq_ignore_ascii_case("outdated") {
            "Outdated details omitted.".to_string()
        } else {
            format!("{} (details omitted)", candidate)
        }
    } else {
        candidate.to_string()
    }
}

fn collapse_residual_comment_payload(body: &str) -> String {
    let trimmed = body.trim();
    let cut = [
        trimmed.find("{{"),
        trimmed.find("<!--"),
        trimmed.find("-->"),
    ]
    .into_iter()
    .flatten()
    .min();
    let candidate = cut
        .map(|idx| trimmed[..idx].trim_end())
        .unwrap_or(trimmed)
        .trim();
    if candidate.is_empty() {
        return String::new();
    }
    if cut.is_some() {
        format!("{} (details omitted)", candidate)
    } else {
        candidate.to_string()
    }
}

fn normalize_comment_payload(body: &str) -> String {
    let cleaned = decode_template_entities(body.trim());
    let cleaned = collapse_noisy_comment_payload(&cleaned);
    if cleaned.is_empty() {
        return String::new();
    }
    let normalized = normalize_excerpt(&cleaned);
    let normalized = normalize_refs(&normalized);
    let normalized = strip_simple_html_tags(&normalized);
    let normalized = normalize_external_links(&normalized);
    let normalized = normalize_simple_templates(&normalized);
    let normalized = normalize_reader_artifacts(&normalized);
    let normalized = MULTISPACE_RE
        .replace_all(&normalized, " ")
        .trim()
        .to_string();
    if normalized.contains("{{")
        || normalized.contains("}}")
        || normalized.contains("<ref")
        || normalized.contains("|yvideo")
    {
        collapse_residual_comment_payload(&normalized)
    } else {
        normalized
    }
}

/// Canonicalize a template name for matching: MediaWiki treats underscores and
/// spaces as equivalent and collapses whitespace runs, so `skin_tier`,
/// `skin tier`, and `skin  tier` all name the same template.
fn normalize_simple_template_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut prev_was_space = false;
    for ch in name.trim().chars() {
        if ch == '_' || ch.is_whitespace() {
            if !prev_was_space {
                out.push(' ');
                prev_was_space = true;
            }
        } else {
            out.extend(ch.to_lowercase());
            prev_was_space = false;
        }
    }
    out
}

fn is_supported_simple_template_name(name: &str) -> bool {
    matches!(
        normalize_simple_template_name(name).as_str(),
        "as" | "ap"
            | "adaptive"
            | "sti"
            | "ci"
            | "ui"
            | "uis"
            | "ii"
            | "nie"
            | "ris"
            | "cbi"
            | "lor"
            | "gems"
            | "skin tier"
            | "fd"
            | "tftc"
            | "tftt"
            | "wrskin"
            | "cis"
            | "cbis"
            | "iis"
            | "nies"
            | "csl"
            | "fi"
            | "si"
            | "sis"
            | "cai"
            | "cais"
            | "tip"
            | "w"
            | "univ"
            | "citation needed"
            | "equals"
            | "gold"
            | "champion_icon"
            | "champion icon"
            | "ability icon"
            | "zoe spell thief list"
            | "bug"
            | "pending for test"
            | "effect at cast time start"
            | "effect at cast time end"
            | "degree"
            | "minus"
            | "plus"
            | "lmb"
            | "rmb"
            | "times"
            | "arcaneciteep"
            | "ccd"
            | "ai"
            | "ais"
            | "tt"
            | "sbc"
            | "spoiler"
            | "note"
            | "rd"
            | "ig"
            | "lll"
            | "set"
            | "wi"
            | "wrcst"
            | "tfti"
            | "lorskin"
            | "recurring"
            | "item stat table"
            | "mi1"
            | "mi2"
            | "mi3"
            | "mi4"
            | "mi6"
            | "mi7"
    )
}

pub fn detect_renderer_cleanup_template_names(s: &str) -> Vec<String> {
    let Ok(spans) = extract_balanced_templates(s) else {
        return Vec::new();
    };

    let mut names = Vec::new();
    let mut seen = HashSet::new();
    for span in spans {
        let name = span.name.split('|').next().unwrap_or("").trim();
        if name.is_empty() || !is_supported_simple_template_name(name) {
            continue;
        }
        let normalized = name.to_ascii_lowercase();
        if seen.insert(normalized) {
            names.push(name.to_string());
        }
    }
    names
}

fn normalize_simple_templates(s: &str) -> String {
    let mut current = s.to_string();
    for _ in 0..8 {
        let next = SIMPLE_TEMPLATE_RE
            .replace_all(&current, |caps: &regex::Captures| {
                let raw = caps.get(1).map(|capture| capture.as_str()).unwrap_or("");
                let parts = raw.split('|').map(str::trim).collect::<Vec<_>>();
                if parts.is_empty() {
                    return caps.get(0).unwrap().as_str().to_string();
                }
                let name = parts[0];
                let args = &parts[1..];
                render_simple_inline_template(name, args)
                    .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string())
            })
            .to_string();
        if next == current {
            break;
        }
        current = next;
    }
    current
}

/// Render a "simple" inline template (icon/label/text helper) to reader-facing
/// text. Returns `None` for any name this renderer does not handle, so callers
/// can decide how to treat genuinely unknown templates (the template registry
/// fails fast on them).
pub fn render_simple_inline_template(name: &str, args: &[&str]) -> Option<String> {
    let name = normalize_simple_template_name(name);
    let positional = simple_positional_args(args);
    let named = args
        .iter()
        .filter_map(|arg| split_simple_named_arg(arg))
        .map(|(key, value)| (key.to_ascii_lowercase(), value))
        .collect::<std::collections::HashMap<_, _>>();
    let first = positional
        .first()
        .copied()
        .unwrap_or_else(|| args.first().copied().unwrap_or(""));
    let last = positional.last().copied().unwrap_or(first);
    let rendered = match name.as_str() {
        "as" | "ap" | "sti" | "ci" | "ui" | "uis" | "ii" | "nie" | "ris" | "cbi" | "lor"
        | "gems" | "skin tier" | "si" | "tfti" => first.to_string(),
        "adaptive" => return render_adaptive_template(args),
        "fd" => return render_fd_template_fallback(args),
        "tftc" | "tftt" | "wrskin" => {
            if positional.len() >= 2 {
                last.to_string()
            } else {
                first.to_string()
            }
        }
        "cis" | "cbis" | "iis" | "nies" | "sis" => {
            if first.is_empty() {
                String::new()
            } else {
                make_possessive(first)
            }
        }
        "csl" => render_csl_template(args),
        "fi" | "tip" | "wrtip" => last.to_string(),
        "lorskin" => render_lorskin_template(args),
        "w" | "univ" => first.to_string(),
        "citation needed" => "[Citation needed]".to_string(),
        "equals" => " = ".to_string(),
        "gold" => {
            if first.is_empty() {
                " gold".to_string()
            } else {
                format!("{} gold", first)
            }
        }
        "champion_icon" | "champion icon" => named
            .get("champion")
            .copied()
            .or_else(|| positional.first().copied())
            .unwrap_or("")
            .to_string(),
        "ability icon" => {
            if first.is_empty() {
                String::new()
            } else {
                format!("_{}_", first)
            }
        }
        "zoe spell thief list" => "(Spell Thief item-actives list omitted.)".to_string(),
        "bug" => "[Bug]".to_string(),
        "pending for test" | "pft" => "[Pending test]".to_string(),
        "effect at cast time start" => "(effect determined at cast time start)".to_string(),
        "effect at cast time end" => "(effect determined at cast time end)".to_string(),
        "degree" => "°".to_string(),
        "minus" => "-".to_string(),
        "plus" => "+".to_string(),
        "lmb" => "LMB".to_string(),
        "rmb" => "RMB".to_string(),
        "times" => "×".to_string(),
        "arcaneciteep" => {
            if first.is_empty() {
                "Arcane episode".to_string()
            } else {
                format!("Arcane episode {}", first)
            }
        }
        "ai" => {
            let label = if positional.len() >= 3 { last } else { first };
            if label.is_empty() {
                String::new()
            } else {
                format!("_{}_", label)
            }
        }
        "cai" => {
            if first.is_empty() {
                String::new()
            } else {
                format!("_{}_", first)
            }
        }
        "ais" | "cais" => {
            if first.is_empty() {
                String::new()
            } else {
                format!("_{}_", make_possessive(first))
            }
        }
        "sbc" => first.to_string(),
        "spoiler" => {
            if first.is_empty() {
                "(Spoiler warning)".to_string()
            } else {
                format!("(Spoilers: {})", first)
            }
        }
        "note" => render_note_template(args),
        "rd" => render_rd_template(args),
        "ig" => render_ig_template(args),
        "wi" => render_wi_template(args),
        "recurring" => render_recurring_template(args),
        "wrcst" => {
            if positional.len() >= 2 {
                last.to_string()
            } else {
                first.to_string()
            }
        }
        "mi1" | "mi2" | "mi3" | "mi4" | "mi6" | "mi7" => render_mastery_icon_template(args),
        "lll" => render_lll_template(args),
        "set" => {
            if first.is_empty() {
                String::new()
            } else {
                format!("Set: {}", first)
            }
        }
        "item stat table" => return render_item_stat_table_fallback(args),
        "tt" => {
            if first.is_empty() {
                String::new()
            } else if positional.len() >= 2 && !last.is_empty() {
                format!("{} ({})", first, last)
            } else {
                first.to_string()
            }
        }
        _ => return None,
    };
    Some(rendered)
}

fn normalize_reader_artifacts(s: &str) -> String {
    let normalized = INLINE_ICON_ARTIFACT_RE.replace_all(s, "").to_string();
    let normalized = GALLERY_OPEN_TAG_RE
        .replace_all(&normalized, "Gallery: ")
        .to_string();
    let normalized = GALLERY_CLOSE_TAG_RE
        .replace_all(&normalized, "")
        .to_string();
    let normalized = TABBER_TAG_RE.replace_all(&normalized, "").to_string();
    let normalized = TABBER_SEPARATOR_RE
        .replace_all(&normalized, " / ")
        .to_string();
    let normalized = INLINE_WIKITEXT_TABLE_RE
        .replace_all(&normalized, |caps: &regex::Captures| {
            flatten_wikitext_table(caps.get(0).map(|capture| capture.as_str()).unwrap_or(""))
        })
        .to_string();
    let normalized = GALLERY_FILENAME_RE
        .replace_all(&normalized, |caps: &regex::Captures| {
            let label = caps
                .get(1)
                .map(|capture| capture.as_str().replace('_', " "))
                .unwrap_or_else(|| "gallery image".to_string());
            format!("Gallery image: {}", label)
        })
        .to_string();
    let normalized = BOLD_POSSESSIVE_RE
        .replace_all(&normalized, "**$1'**")
        .to_string();
    let normalized = ITALIC_POSSESSIVE_RE
        .replace_all(&normalized, "_$1'_$2")
        .to_string();
    let normalized = BOLD_QUAD_APOSTROPHE_RE
        .replace_all(&normalized, "**$1'**")
        .to_string();
    let normalized = MISSING_BOLD_POSSESSIVE_RE
        .replace_all(&normalized, "**$1'**$2")
        .to_string();
    let normalized = BOLD_RUNON_RE
        .replace_all(&normalized, "**$1'** $2")
        .to_string();
    let normalized = CROSSED_BOLD_ITALIC_RE
        .replace_all(&normalized, "***$1**$2*")
        .to_string();
    let normalized = QUAD_APOSTROPHE_RE
        .replace_all(&normalized, "$1'$2")
        .to_string();
    let normalized = PARAM_ASSIGNMENT_PARENS_RE
        .replace_all(&normalized, "")
        .to_string();
    let normalized = INLINE_TEMPLATE_PARAM_RE
        .replace_all(&normalized, "")
        .to_string();
    let normalized = normalized
        .replace("(Comment: / ", "(Comment: ")
        .replace(".(", ". (")
        .replace("( for", "(for")
        .replace("''''", "'");
    let normalized = CLOSE_PAREN_WORD_RE
        .replace_all(&normalized, ") $1")
        .to_string();
    let normalized = WORD_OPEN_PAREN_RE
        .replace_all(&normalized, "$1 (")
        .to_string();
    let normalized = LINK_PLURAL_SPACE_RE
        .replace_all(&normalized, "$1$2")
        .to_string();
    let normalized = normalize_definition_list_lines(&normalized);
    normalize_math_tags(&normalized)
}

fn normalize_definition_list_lines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut changed = false;

    for segment in s.split_inclusive('\n') {
        let (line, has_newline) = if let Some(line) = segment.strip_suffix('\n') {
            (line, true)
        } else {
            (segment, false)
        };

        if let Some(normalized) = normalize_definition_list_line(line) {
            if normalized != line {
                changed = true;
            }
            out.push_str(&normalized);
        } else {
            out.push_str(line);
        }

        if has_newline {
            out.push('\n');
        }
    }

    if changed {
        out
    } else {
        s.to_string()
    }
}

fn normalize_definition_list_line(line: &str) -> Option<String> {
    let trimmed_start = line.trim_start();
    let leading_ws = &line[..line.len().saturating_sub(trimmed_start.len())];
    let marker_len = trimmed_start.chars().take_while(|ch| *ch == ':').count();
    if marker_len == 0 {
        return None;
    }

    let remainder = trimmed_start[marker_len..].trim_start();
    if remainder.is_empty() {
        return None;
    }

    if matches!(remainder.chars().next(), Some('*' | '#' | ';' | ':' | '|')) {
        return None;
    }

    Some(format!(
        "{}{}{}",
        leading_ws,
        "> ".repeat(marker_len),
        remainder
    ))
}

fn normalize_math_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut remaining = s;

    while let Some(start) = remaining.find("<math>") {
        out.push_str(&remaining[..start]);
        let after_open = &remaining[start + "<math>".len()..];
        let Some(end) = after_open.find("</math>") else {
            out.push_str(&remaining[start..]);
            return out;
        };

        out.push_str(&render_math_fragment(&after_open[..end]));
        remaining = &after_open[end + "</math>".len()..];
    }

    out.push_str(remaining);
    out
}

fn render_math_fragment(raw: &str) -> String {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix("\\frac") {
        let rest = rest.trim_start();
        if let Some((numerator, rest)) = consume_braced_math(rest) {
            if let Some((denominator, _)) = consume_braced_math(rest.trim_start()) {
                let numerator = normalize_math_atom(numerator);
                let denominator = normalize_math_atom(denominator);
                if !numerator.is_empty() && !denominator.is_empty() {
                    return format!("{} / ({})", numerator, denominator);
                }
            }
        }
    }

    normalize_math_atom(trimmed)
}

fn consume_braced_math(raw: &str) -> Option<(&str, &str)> {
    let trimmed = raw.trim_start();
    if !trimmed.starts_with('{') {
        return None;
    }

    let mut depth = 0usize;
    for (idx, ch) in trimmed.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some((&trimmed[1..idx], &trimmed[idx + 1..]));
                }
            }
            _ => {}
        }
    }

    None
}

fn normalize_math_atom(raw: &str) -> String {
    let normalized = raw
        .replace("\\%", "%")
        .replace("\\cdot", " × ")
        .replace("\\times", " × ")
        .replace('{', "")
        .replace('}', "")
        .replace('−', "-")
        .replace('–', "-")
        .replace('—', "-")
        .replace('+', " + ")
        .replace('-', " - ")
        .replace('=', " = ")
        .replace('\\', "");
    MULTISPACE_RE
        .replace_all(&normalized, " ")
        .trim()
        .to_string()
}

fn normalize_rendered_body(s: &str) -> String {
    let normalized = normalize_comments(s);
    let normalized = decode_template_entities(&normalized);
    let normalized = normalize_simple_templates(&normalized);
    let normalized = normalize_reader_artifacts(&normalized);
    let normalized = normalize_wikitext_list_lines(&normalized);
    let normalized = LMB_CLICKING_RE
        .replace_all(&normalized, "left-clicking")
        .to_string();
    let normalized = RMB_CLICKING_RE
        .replace_all(&normalized, "right-clicking")
        .to_string();
    let normalized = LMB_CLICK_RE
        .replace_all(&normalized, "left-click")
        .to_string();
    RMB_CLICK_RE
        .replace_all(&normalized, "right-click")
        .to_string()
}

fn normalize_wikitext_list_lines(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut changed = false;

    for segment in s.split_inclusive('\n') {
        let (line, has_newline) = if let Some(line) = segment.strip_suffix('\n') {
            (line, true)
        } else {
            (segment, false)
        };

        if let Some(normalized) = normalize_wikitext_list_line(line) {
            if normalized != line {
                changed = true;
            }
            out.push_str(&normalized);
        } else {
            out.push_str(line);
        }

        if has_newline {
            out.push('\n');
        }
    }

    if changed {
        out
    } else {
        s.to_string()
    }
}

fn normalize_wikitext_list_line(line: &str) -> Option<String> {
    let trimmed_start = line.trim_start();
    let leading_ws = &line[..line.len().saturating_sub(trimmed_start.len())];
    let marker_len = trimmed_start.chars().take_while(|ch| *ch == '*').count();
    if marker_len == 0 {
        return None;
    }
    let remainder = &trimmed_start[marker_len..];
    if !remainder
        .chars()
        .next()
        .map(|ch| ch.is_whitespace())
        .unwrap_or(false)
    {
        return None;
    }
    let content = remainder.trim();
    if content.is_empty() {
        return None;
    }

    let depth = marker_len;
    Some(format!(
        "{}{}{}{}",
        leading_ws,
        "  ".repeat(depth.saturating_sub(1)),
        "- ",
        content
    ))
}

fn normalize_comments(s: &str) -> String {
    COMMENT_RE
        .replace_all(s, |caps: &regex::Captures| {
            let body = caps
                .get(1)
                .map(|capture| capture.as_str().trim())
                .unwrap_or("");
            if body.is_empty() || body.eq_ignore_ascii_case("Blurb") {
                return String::new();
            }
            let normalized = normalize_comment_payload(body);
            if normalized.is_empty() {
                String::new()
            } else {
                format!("(Comment: {})", normalized)
            }
        })
        .to_string()
}

fn normalize_ref_body(body: &str) -> String {
    let trimmed = body.trim();
    if let Some(inner) = trimmed.strip_prefix("[[") {
        let inner = inner.trim_end_matches(']');
        if inner.starts_with("http://") || inner.starts_with("https://") {
            let mut parts = inner.splitn(2, char::is_whitespace);
            let url = parts.next().unwrap_or("");
            let label = parts
                .next()
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .unwrap_or(url);
            return format!("[{}]({})", label, url);
        }
    }
    let normalized_external = normalize_external_links(trimmed);
    if normalized_external != trimmed {
        return normalized_external;
    }
    trimmed.to_string()
}

fn normalize_refs(s: &str) -> String {
    let without_references = REFERENCES_RE.replace_all(s, "").to_string();
    let expanded = REF_RE
        .replace_all(&without_references, |caps: &regex::Captures| {
            let body = caps
                .get(2)
                .map(|capture| capture.as_str().trim())
                .unwrap_or("");
            if body.is_empty() {
                if let Some(name) = caps.get(1).map(|capture| capture.as_str().trim()) {
                    if !name.is_empty() {
                        return format!("[Source ref: {}]", name);
                    }
                }
                return String::new();
            }
            format!("[Source: {}]", normalize_ref_body(body))
        })
        .to_string();
    let self_closing = SELF_CLOSING_REF_RE
        .replace_all(&expanded, |caps: &regex::Captures| {
            let name = caps
                .get(1)
                .map(|capture| capture.as_str().trim())
                .unwrap_or("");
            if name.is_empty() {
                String::new()
            } else {
                format!("[Source ref: {}]", name)
            }
        })
        .to_string();
    MALFORMED_SELF_CLOSING_REF_RE
        .replace_all(&self_closing, |caps: &regex::Captures| {
            let name = caps
                .get(1)
                .map(|capture| capture.as_str().trim())
                .unwrap_or("");
            if name.is_empty() {
                String::new()
            } else {
                format!("[Source ref: {}]", name)
            }
        })
        .to_string()
}

fn strip_simple_html_tags(s: &str) -> String {
    SIMPLE_HTML_TAG_RE.replace_all(s, "").to_string()
}

fn normalize_external_links(s: &str) -> String {
    let mut out = String::new();
    let mut remaining = s;
    while let Some(open) = remaining.find('[') {
        out.push_str(&remaining[..open]);
        remaining = &remaining[open..];
        if remaining.starts_with("[[") {
            out.push_str("[[");
            remaining = &remaining[2..];
            continue;
        }
        let Some(close) = remaining.find(']') else {
            out.push_str(remaining);
            return out;
        };
        let inner = &remaining[1..close];
        if inner.starts_with("http://") || inner.starts_with("https://") {
            let mut parts = inner.splitn(2, char::is_whitespace);
            let url = parts.next().unwrap_or("");
            let label = parts
                .next()
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .unwrap_or(url);
            out.push_str(&format!("[{}]({})", label, url));
            remaining = &remaining[close + 1..];
            continue;
        }
        out.push('[');
        remaining = &remaining[1..];
    }
    out.push_str(remaining);
    out
}

fn normalize_table_value(value: &str, omit_falsey: bool) -> Option<String> {
    let normalized = normalize_all(value);
    let trimmed = normalized.trim();
    if trimmed.is_empty() {
        return None;
    }
    let lower = trimmed.to_ascii_lowercase();
    if matches!(lower.as_str(), "na" | "n/a" | "unknown") {
        return None;
    }
    if omit_falsey && matches!(lower.as_str(), "false" | "no" | "none") {
        return None;
    }
    let rendered = match lower.as_str() {
        "true" => "Yes".to_string(),
        "false" => "No".to_string(),
        _ => trimmed.to_string(),
    };
    Some(rendered)
}

fn render_starred_list(out: &mut String, notes: &[String]) {
    for note in notes {
        let trimmed = note.trim();
        if trimmed.is_empty() {
            continue;
        }
        let star_count = trimmed.chars().take_while(|c| *c == '*').count();
        let content = trimmed.trim_start_matches('*').trim();
        if content.is_empty() {
            continue;
        }
        // A leading `;` marks a wikitext definition-list term, which renders as
        // a bold sub-header (e.g. `;Zombie info`). Bold it so it reads as a
        // heading rather than literal `;` text.
        let content = match content.strip_prefix(';') {
            Some(term) if !term.trim().is_empty() => format!("'''{}'''", term.trim()),
            _ => content.to_string(),
        };
        let indent = "  ".repeat(star_count.saturating_sub(1));
        let normalized = normalize_all(&content);
        out.push_str(&indent);
        out.push_str("- ");
        out.push_str(&normalized);
        out.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        AdvancedStats, BasicInfo, ChampionSpecialMode, ChampionStatVariant, Change, PatchEntry,
        Stats,
    };
    use std::collections::HashMap;

    #[test]
    fn render_starred_list_bolds_definition_terms() {
        let mut out = String::new();
        render_starred_list(
            &mut out,
            &[
                ";Zombie info".to_string(),
                "* Zombie states trigger upon lethal damage.".to_string(),
            ],
        );
        assert_eq!(
            out,
            "- **Zombie info**\n- Zombie states trigger upon lethal damage.\n"
        );
    }

    #[test]
    fn apostrophes_normalization_basic() {
        assert_eq!(normalize_apostrophes("''italic''"), "_italic_");
        assert_eq!(normalize_apostrophes("'''bold'''"), "**bold**");
        assert_eq!(normalize_apostrophes("'''''both'''''"), "**_both_**");
    }

    #[test]
    fn internal_links_and_anchors() {
        assert_eq!(
            normalize_internal_links("See [[Aatrox]]"),
            "See [Aatrox](./Aatrox.md)"
        );
        assert_eq!(
            normalize_internal_links("See [[Akshan/Cosmetics|Akshan (Collection)]]"),
            "See [Akshan (Collection)](./Akshan_Cosmetics.md)"
        );
        let s = normalize_anchors("[[Page#Section Title]]");
        assert_eq!(s, "Page#Section-Title");
        let s2 = normalize_anchors("[[Page#(Section, Title)]]");
        assert_eq!(s2, "Page#Section-Title");
    }

    #[test]
    fn normalize_all_cleans_comments_refs_and_external_links() {
        let raw = "Seen on [https://example.com Example]<!-- Blurb --> <ref>[https://example.com/source Source]</ref> <!-- dev note -->";
        let normalized = normalize_all(raw);
        assert!(normalized.contains("[Example](https://example.com)"));
        assert!(normalized.contains("[Source: [Source](https://example.com/source)]"));
        assert!(normalized.contains("(Comment: dev note)"));
        assert!(!normalized.contains("<!--"));
        assert!(!normalized.contains("<ref"));
    }

    #[test]
    fn normalize_all_decodes_common_mojibake_segments() {
        let raw = format!("Damage {}\u{0080}\u{0093} bonus", '\u{00e2}');
        assert_eq!(normalize_all(&raw), "Damage – bonus");
    }

    #[test]
    fn normalize_all_unwraps_common_template_artifacts() {
        assert_eq!(
            normalize_all("{{tip|projectile|projectiles}} {{bug}} {{Pending for test}} {{ai|Severum|Aphelios}}"),
            "projectiles [Bug] [Pending test] _Severum_"
        );
    }

    #[test]
    fn normalize_all_handles_remaining_reader_template_families() {
        assert_eq!(
            normalize_all("{{LoR|Mistkeepers}} {{WRskin|Akali|Crystal Rose}} {{TFTc|Amumu|set=10|new splash art}} {{Gems|Gems}} {{uis|Tibbers}} {{cbis|Nagakabouros}}"),
            "Mistkeepers Crystal Rose new splash art Gems Tibbers Nagakabouros'"
        );
    }

    #[test]
    fn normalize_all_handles_more_leaked_reader_templates() {
        assert_eq!(
            normalize_all("{{fd|0.5}} {{ii|Runaan's Hurricane}} {{iis|Runaan's Hurricane}} {{nie|Wind's Fury}} {{cbi|Jayce|Jayce Talis}} {{ris|Conqueror}} {{TFTt|Cultist|Set=4}} {{Effect at cast time end}} {{ArcaneCiteEp|1x07}} {{degree}}"),
            "0.5 Runaan's Hurricane Runaan's Hurricane's Wind's Fury Jayce Conqueror Cultist (effect determined at cast time end) Arcane episode 1x07 °"
        );
    }

    #[test]
    fn normalize_all_handles_additional_trivia_templates() {
        assert_eq!(
            normalize_all("{{rd|12%|8%}} {{ig|Boots}} {{lll|Paddlemar|Jade}} {{Note|1}} heads up {{set|Event Horizon}} {{spoiler|Arcane}} {{sbc|Bug Fix:}} {{times}}"),
            "12% (melee) / 8% (ranged) Boots Paddlemar (Jade) Editor's note: heads up Set: Event Horizon (Spoilers: Arcane) Bug Fix: ×"
        );
    }

    #[test]
    fn normalize_all_cleans_tt_templates_inside_comments() {
        assert_eq!(
            normalize_all("<!-- / {{tt|X|Tibbers snap-back range}} -->"),
            "(Comment: X (Tibbers snap-back range))"
        );
    }

    #[test]
    fn normalize_all_handles_bracketed_external_links_and_malformed_refs() {
        assert_eq!(
            normalize_all("Seen [[https://x.com/test Example]] <ref name=\"Placeholder/>"),
            "Seen [Example](https://x.com/test) [Source ref: Placeholder]"
        );
    }

    #[test]
    fn normalize_all_handles_malformed_external_links_inside_refs() {
        assert_eq!(
            normalize_all("Voice source<ref>[[https://x.com/test Example Voice Actress]</ref>"),
            "Voice source[Source: [Example Voice Actress](https://x.com/test)]"
        );
    }

    #[test]
    fn normalize_all_handles_single_bracket_links_and_malformed_named_refs() {
        let riven = normalize_all(
            "Broken Wings<ref>Youtube — Phreak (22 July 2025) [https://youtu.be/OG7a43cu-LY?feature=shared&t=1237 Patch 25.15 Preview] ''(20:37)''</ref><ref>YouTube — Vandiril (19 July 2025) [https://www.youtube.com/watch?v=Qa-xqVypeoc Riot messed up AGAIN...]. Bug demonstration.</ref>",
        );
        assert!(!riven.contains("<ref"));
        assert!(riven
            .contains("[Patch 25.15 Preview](https://youtu.be/OG7a43cu-LY?feature=shared&t=1237)"));
        assert!(riven
            .contains("[Riot messed up AGAIN...](https://www.youtube.com/watch?v=Qa-xqVypeoc)"));

        assert_eq!(
            normalize_all(
                "Smolder<ref name=\"Placeholder>[https://twitter.com/riot_llama/status/1744423545328062930 Riot Llama on Smolder Placeholders]</ref>"
            ),
            "Smolder[Source: [Riot Llama on Smolder Placeholders](https://twitter.com/riot_llama/status/1744423545328062930)]"
        );
    }

    #[test]
    fn normalize_all_handles_equals_gold_and_named_champion_icon_templates() {
        assert_eq!(
            normalize_all(
                "Vi or {{Champion_icon|champion = Caitlyn}} gets +1{{Gold}} when N{{equals}}Number of times."
            ),
            "Vi or Caitlyn gets +1 gold when N = Number of times."
        );
    }

    #[test]
    fn normalize_all_handles_ability_icon_and_spell_thief_list_templates() {
        assert_eq!(
            normalize_all("{{Ability icon|Deadly Plumage|Xayah}} {{Zoe Spell Thief list}}"),
            "_Deadly Plumage_ (Spell Thief item-actives list omitted.)"
        );
    }

    #[test]
    fn normalize_all_collapses_residual_template_payloads_outside_comments() {
        assert_eq!(
            normalize_all(
                "Case 3: With both Rabadon's Deathcap {{tt|{{#expr:1+1}}|stacked additively}}"
            ),
            "Case 3: With both Rabadon's Deathcap (details omitted)"
        );
    }

    #[test]
    fn normalize_rendered_body_cleans_multiline_comments_with_templates() {
        assert_eq!(
            normalize_rendered_body("Value<!-- note {{fd|0.25}} {{LMB}} clicking -->"),
            "Value (Comment: note 0.25 left-clicking)"
        );
    }

    #[test]
    fn normalize_rendered_body_simplifies_parser_function_comments() {
        assert_eq!(
            normalize_rendered_body(
                "Value<!--\nOutdated\n{{#vardefine:x|1}}\n{{#expr:1+1}}\n|yvideo = abc\n-->"
            ),
            "Value (Comment: Outdated details omitted.)"
        );
    }

    #[test]
    fn normalize_rendered_body_converts_wikitext_nested_lists_to_markdown() {
        assert_eq!(
            normalize_rendered_body(
                "### Arena\n\n* _Obtained from the Combo Master augment._\n** Damage changed to 60 – 300 (based on level).\n** Bonus AD ratio changed to 55% **bonus** AD.\n**PASSIVE:** This is not a list item."
            ),
            "### Arena\n\n- _Obtained from the Combo Master augment._\n  - Damage changed to 60 – 300 (based on level).\n  - Bonus AD ratio changed to 55% **bonus** AD.\n**PASSIVE:** This is not a list item."
        );
    }

    #[test]
    fn normalize_all_cleans_icon_markup_and_param_artifacts() {
        assert_eq!(
            normalize_all("20px|border|link= primary abilities (move=false)"),
            "primary abilities"
        );
        assert_eq!(
            normalize_all("Value (Comment: Outdated details omitted.) |yvideo = abc123"),
            "Value (Comment: Outdated details omitted.)"
        );
    }

    #[test]
    fn normalize_all_fixes_bold_possessive_apostrophe_runs() {
        assert_eq!(
            normalize_all("The active effect of **Aphelios''''** varies."),
            "The active effect of **Aphelios'** varies."
        );
        assert_eq!(
            normalize_all("_Umbral Trespass' _ recast delay lasts longer."),
            "_Umbral Trespass'_ recast delay lasts longer."
        );
    }

    #[test]
    fn normalize_all_rewrites_crossed_bold_italic_spans() {
        assert_eq!(
            normalize_all(
                "'''''Aurelion Sol''' will be knocked down by any immobilizing crowd control during the dash.''"
            ),
            "***Aurelion Sol** will be knocked down by any immobilizing crowd control during the dash.*"
        );
        assert_eq!(
            normalize_all(
                "'''''Aurelion Sol''' will not dash if he is immobilized during the cast time. {{ai|Breath of Light|Aurelion Sol}} continues afterwards.''"
            ),
            "***Aurelion Sol** will not dash if he is immobilized during the cast time. _Breath of Light_ continues afterwards.*"
        );
    }

    #[test]
    fn normalize_all_handles_wild_rift_ward_and_mastery_templates() {
        assert_eq!(
            normalize_all("{{wi|Elementalist|size=32px}} {{wi|Underworld Poro|Ward Skin|size=20px}} {{WRcst|Master Yi|Zephyr Dragon}} {{mi3|Arcane Knowledge}} {{mi4|Juggernaut}}"),
            "Elementalist Ward Skin Zephyr Dragon Arcane Knowledge Juggernaut"
        );
    }

    #[test]
    fn normalize_all_handles_spell_card_and_recurring_templates() {
        assert_eq!(
            normalize_all("{{si|Recall}} {{sis|Recall}} {{cai|Certain Death|Briar}} {{cais|Certain Death|Briar}} {{TFTi|Neeko's Help}} {{LORskin|Pyke|Sand Wraith|Legends of Runeterra artworks}} 2.{{recurring|6}} {{citation needed}}"),
            "Recall Recall's _Certain Death_ _Certain Death's_ Neeko's Help Legends of Runeterra artworks 2.6̅ [Citation needed]"
        );
    }

    #[test]
    fn normalize_all_converts_definition_list_skin_note() {
        assert_eq!(
            normalize_all(":''This article section only contains champion skins. For all associated collection items, see [[Aatrox/Cosmetics|Aatrox (Collection)]].''"),
            "> _This article section only contains champion skins. For all associated collection items, see [Aatrox (Collection)](./Aatrox_Cosmetics.md)._"
        );
    }

    #[test]
    fn normalize_all_handles_adaptive_template_values() {
        assert_eq!(
            normalize_all("{{adaptive|15}}"),
            "9 **bonus** Attack Damage or 15 Ability Power (Adaptive)"
        );
        assert_eq!(
            normalize_all("{{adaptive|5 to 40}}"),
            "3 to 24 **bonus** Attack Damage or 5 to 40 Ability Power (Adaptive)"
        );
    }

    #[test]
    fn normalize_all_renders_math_fraction_tags() {
        assert_eq!(
            normalize_all("<math>\\frac{{100\\%}}{100\\%-12\\%}</math>"),
            "100% / (100% - 12%)"
        );
    }

    #[test]
    fn normalize_rendered_body_compacts_comments_with_residual_templates() {
        assert_eq!(
            normalize_rendered_body(
                "Value<!-- note feature is planned {{Ornn Masterwork items}} -->"
            ),
            "Value (Comment: note feature is planned (details omitted))"
        );
    }

    #[test]
    fn normalize_all_renders_nested_rd_and_percent_fd_templates() {
        assert_eq!(
            normalize_all("{{rd|{{fd|1.3%}}|{{fd|0.78%}}}}"),
            "1.3% (melee) / 0.78% (ranged)"
        );
    }

    #[test]
    fn detect_renderer_cleanup_template_names_reports_supported_leaks() {
        assert_eq!(
            detect_renderer_cleanup_template_names(
                "Value {{fd|0.5}} {{tip|League of Legends|LoL}} {{Unknown|x}}"
            ),
            vec!["fd".to_string(), "tip".to_string()]
        );
    }

    #[test]
    fn normalize_all_flattens_inline_wikitext_tables_and_tabbers() {
        assert_eq!(
            normalize_all(
                "Combinations include: {|align=center style=\"x\" |Elementalist |⇒ |Elementalist |- |⇓ |⇘ |}"
            ),
            "Combinations include: Elementalist | ⇒ | Elementalist / ⇓ | ⇘"
        );
        assert_eq!(
            normalize_all(
                "Lore: <tabber> Hide= |-| Show= {| class=\"wikitable\" |- ! Element ! Picking |- | Air | ''Winds'' |} </tabber>"
            ),
            "Lore: Hide= / Show= Element | Picking / Air | _Winds_"
        );
    }

    #[test]
    fn normalize_internal_links_preserves_unicode_text() {
        assert_eq!(
            normalize_internal_links("Őrség [[Page|Päge]] – kész"),
            "Őrség [Päge](./Page.md) – kész"
        );
    }

    #[test]
    fn normalize_internal_links_ignores_file_markup_without_caption() {
        assert_eq!(
            normalize_all("[[File:Shurima Crest icon.png|link=Shurima#Trivia|100px|center]]"),
            ""
        );
        assert_eq!(
            normalize_all("[[File:Example.png|thumb|Actual caption]]"),
            "Actual caption"
        );
    }

    #[test]
    fn rune_markdown_includes_trivia() {
        let rune = Rune {
            name: "Electrocute".to_string(),
            path: Some("Domination".to_string()),
            slot: Some("Keystone".to_string()),
            description: "Deal bonus damage.".to_string(),
            caption: None,
            map_changes: None,
            notes: vec!["* Triggered after three attacks".to_string()],
            trivia: vec!["* Popular in assassin matchups".to_string()],
            patch_history: Vec::new(),
            source_appendices: Vec::new(),
            warnings: Vec::new(),
        };
        let md = render_rune_markdown(&rune, "== Raw ==\nContent");
        assert!(md.contains("Electrocute is a rune in League of Legends."));
        assert!(md.contains("## Trivia"));
        assert!(md.contains("Popular in assassin matchups"));
        assert!(!md.contains("Raw excerpt"));
    }

    #[test]
    fn rune_markdown_renders_caption_and_map_changes() {
        let rune = Rune {
            name: "Electrocute".to_string(),
            path: Some("Domination".to_string()),
            slot: Some("Keystone".to_string()),
            description: "Deal bonus damage.".to_string(),
            caption: Some("> \"We called them the Thunderlords.\" — ''Rune caption''".to_string()),
            map_changes: Some(
                "### Howling Abyss\n\n* Cooldown changed to 10 seconds.\n\n### Arena\n\n* _Obtained from the Combo Master augment._\n** Damage changed to 60 – 300 (based on level).\n** Bonus AD ratio changed to 55% **bonus** AD."
                    .to_string(),
            ),
            notes: Vec::new(),
            trivia: Vec::new(),
            patch_history: Vec::new(),
            source_appendices: Vec::new(),
            warnings: Vec::new(),
        };

        let md = render_rune_markdown(&rune, "== Raw ==\nContent");
        assert!(md.contains("Thunderlords"));
        assert!(md.contains("## Map-Specific Differences"));
        assert!(md.contains("### Howling Abyss"));
        assert!(md.contains("Cooldown changed to 10 seconds."));
        assert!(md.contains("### Arena"));
        assert!(md.contains("- _Obtained from the Combo Master augment._"));
        assert!(md.contains("  - Damage changed to 60 – 300 (based on level)."));
        assert!(md.contains("  - Bonus AD ratio changed to 55% **bonus** AD."));
        assert!(!md.contains("Raw excerpt"));
    }

    #[test]
    fn champion_patch_history_rendering() {
        let champ = Champion {
            name: "Tester".to_string(),
            basic: BasicInfo::default(),
            summary: None,
            stats: Stats::default(),
            advanced: None,
            primary_stat_label: None,
            special_stats: Vec::new(),
            stat_variants: Vec::new(),
            abilities: Vec::new(),
            pets: Vec::new(),
            trivia: Vec::new(),
            patch_history: vec![PatchEntry {
                version: "V1.0".to_string(),
                changes: vec![
                    Change {
                        section: "Heroic Swing".to_string(),
                        text: "Base damage increased\nBug fix resolved".to_string(),
                    },
                    Change {
                        section: "General".to_string(),
                        text: "Minor tooltip update".to_string(),
                    },
                ],
            }],
            notes: Vec::new(),
            source_appendices: Vec::new(),
            warnings: Vec::new(),
        };
        let md = render_champion_markdown(&champ, "== Raw ==\nContent");
        assert!(md.contains("## Patch History"));
        assert!(md.contains("### V1.0"));
        assert!(md.contains("- **Heroic Swing**"));
        assert!(md.contains("  - Base damage increased"));
        assert!(md.contains("  - Bug fix resolved"));
        assert!(md.contains("- Minor tooltip update"));
    }

    #[test]
    fn champion_advanced_stats_rendering() {
        let mut metrics = HashMap::new();
        metrics.insert("Attack Speed Ratio".to_string(), "0.625".to_string());
        let champ = Champion {
            name: "StatsMaster".to_string(),
            basic: BasicInfo::default(),
            summary: None,
            stats: Stats::default(),
            advanced: Some(AdvancedStats { metrics }),
            primary_stat_label: None,
            special_stats: Vec::new(),
            stat_variants: Vec::new(),
            abilities: Vec::new(),
            pets: Vec::new(),
            trivia: Vec::new(),
            patch_history: Vec::new(),
            notes: Vec::new(),
            source_appendices: Vec::new(),
            warnings: Vec::new(),
        };
        let md = render_champion_markdown(&champ, "");
        assert!(md.contains("## Advanced Stats"));
        assert!(md.contains("| Attack Speed Ratio | 0.625 |"));
    }

    #[test]
    fn champion_variant_stats_rendering() {
        let mut mounted_stats = Stats::default();
        mounted_stats.base.insert(
            "Move Speed".to_string(),
            crate::model::StatLine {
                base: 345.0,
                growth: 0.0,
            },
        );
        let mut dismounted_stats = Stats::default();
        dismounted_stats.base.insert(
            "Move Speed".to_string(),
            crate::model::StatLine {
                base: 305.0,
                growth: 0.0,
            },
        );

        let champ = Champion {
            name: "Kled".to_string(),
            basic: BasicInfo::default(),
            summary: None,
            stats: mounted_stats,
            advanced: None,
            primary_stat_label: Some("Kled & Skaarl".to_string()),
            special_stats: vec![ChampionSpecialMode {
                mode: "ARAM".to_string(),
                metrics: HashMap::from([("Damage Taken".to_string(), "0.9".to_string())]),
            }],
            stat_variants: vec![ChampionStatVariant {
                label: "Kled (Dismounted)".to_string(),
                stats: dismounted_stats,
                advanced: None,
                special_stats: vec![ChampionSpecialMode {
                    mode: "Ultra Rapid Fire".to_string(),
                    metrics: HashMap::from([("Damage Taken".to_string(), "0.9".to_string())]),
                }],
            }],
            abilities: Vec::new(),
            pets: Vec::new(),
            trivia: Vec::new(),
            patch_history: Vec::new(),
            notes: Vec::new(),
            source_appendices: Vec::new(),
            warnings: Vec::new(),
        };

        let md = render_champion_markdown(&champ, "");
        assert!(md.contains("## Stats"));
        assert!(md.contains("### Kled & Skaarl"));
        assert!(md.contains("### Kled (Dismounted)"));
        assert!(md.contains("**Special Statistics**"));
        assert!(md.contains("#### ARAM"));
        assert!(md.contains("#### Ultra Rapid Fire"));
    }

    #[test]
    fn item_markdown_includes_rich_page_sections() {
        let item = Item {
            name: "Infinity Edge".to_string(),
            categories: vec!["Legendary".to_string()],
            background: Some("Forged in [[Shurima]].".to_string()),
            similar_items: vec!["Essence Reaver".to_string()],
            notes: vec!["* First note".to_string()],
            trivia: vec!["* Featured in The Call".to_string()],
            patch_history: vec![crate::model::PatchEntry {
                version: "V14.10".to_string(),
                changes: vec![crate::model::Change {
                    section: "General".to_string(),
                    text: "Attack damage increased.".to_string(),
                }],
            }],
            gold_efficiency: Some("110%".to_string()),
            ..Item::default()
        };
        let md = render_item_markdown(&item, "{{Item info}}...");
        assert!(md.contains("Infinity Edge is a legendary item in League of Legends."));
        assert!(md.contains("## Background"));
        assert!(md.contains("Forged in [Shurima](./Shurima.md)."));
        assert!(md.contains("## Similar Items"));
        assert!(md.contains("Essence Reaver"));
        assert!(md.contains("## Trivia"));
        assert!(md.contains("## Patch History"));
        assert!(md.contains("Attack damage increased."));
        assert!(md.contains("Gold efficiency: 110%"));
    }

    #[test]
    fn item_markdown_omits_source_appendix_when_present() {
        let item = Item {
            name: "Infinity Edge".to_string(),
            source_appendices: vec![crate::model::SourceAppendix {
                title: "Infinity Edge".to_string(),
                format: "wikitext".to_string(),
                content: "{{Item info|background=Forged in Shurima.}}".to_string(),
            }],
            ..Item::default()
        };
        let md = render_item_markdown(&item, "ignored excerpt");
        assert!(!md.contains("## Source Appendix"));
        assert!(!md.contains("<details><summary>Infinity Edge</summary>"));
        assert!(!md.contains("{{Item info|background=Forged in Shurima.}}"));
        assert!(!md.contains("Raw excerpt"));
    }

    #[test]
    fn item_markdown_renders_removed_historical_metadata() {
        let item = Item {
            name: "Ataraxia".to_string(),
            categories: vec!["Legendary".to_string()],
            caption: Some("**MASTER CRAFTSMAN:** All stats have been improved.".to_string()),
            ornn_forged: true,
            removed_patch: Some("V14.11".to_string()),
            ..Item::default()
        };

        let md = render_item_markdown(&item, "{{Item info}}...");
        assert!(md.contains("This article or section may contain obsolete information, but exists here for historical purposes."));
        assert!(md.contains("This item was removed on patch V14.11."));
        assert!(md.contains(
            "Ataraxia was a legendary item in League of Legends. Could only be forged by Ornn."
        ));
        assert!(md.contains("**MASTER CRAFTSMAN:** All stats have been improved."));
        assert!(md.contains("**Forged by:** Ornn"));
        assert!(md.contains("**Removed:** V14.11"));
    }
}
