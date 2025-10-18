use crate::model::{AbilityKey, Champion, Item, Rune};
use regex::Regex;

pub fn render_champion_markdown(champ: &Champion, raw_excerpt: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", champ.name));
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
    // Stats section (basic subset)
    if !champ.stats.base.is_empty() {
        out.push_str("## Stats\n\n| Stat | Base | Growth |\n|------|------|--------|\n");
        let mut keys: Vec<_> = champ
            .stats
            .base
            .keys()
            .filter(|k| !k.starts_with("resource__"))
            .collect();
        keys.sort();
        for k in keys {
            if let Some(v) = champ.stats.base.get(k) {
                if k == "resource" {
                    continue;
                }
                out.push_str(&format!("| {} | {} | {} |\n", k, v.base, v.growth));
            }
        }
        out.push('\n');
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
            out.push_str(&format!("### {} – {}\n\n", key_label, a.name));
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
            // Filter out empty
            let mut any_info = false;
            for (_label, val_opt) in &info_rows {
                if let Some(val) = val_opt {
                    if !val.trim().is_empty() {
                        any_info = true;
                        break;
                    }
                }
            }
            if any_info {
                out.push_str("| Attribute | Value |\n|-----------|------:|\n");
                for (label, val_opt) in info_rows {
                    if let Some(val) = val_opt {
                        let v = normalize_internal_links(&normalize_anchors(
                            &normalize_apostrophes(val),
                        ));
                        if !v.trim().is_empty() {
                            out.push_str(&format!("| **{}** | {} |\n", label, v));
                        }
                    }
                }
                // Custom labels
                if let (Some(cl), Some(ci)) =
                    (a.extra.get("customlabel"), a.extra.get("custominfo"))
                {
                    let v =
                        normalize_internal_links(&normalize_anchors(&normalize_apostrophes(ci)));
                    if !v.trim().is_empty() && !cl.trim().is_empty() {
                        out.push_str(&format!("| **{}** | {} |\n", cl, v));
                    }
                }
                if let (Some(cl), Some(ci)) =
                    (a.extra.get("customlabel2"), a.extra.get("custominfo2"))
                {
                    let v =
                        normalize_internal_links(&normalize_anchors(&normalize_apostrophes(ci)));
                    if !v.trim().is_empty() && !cl.trim().is_empty() {
                        out.push_str(&format!("| **{}** | {} |\n", cl, v));
                    }
                }
                out.push('\n');
            }
            // Descriptions in order: description, description2..description6
            if !a.descriptions.is_empty() {
                for desc in &a.descriptions {
                    let d =
                        normalize_internal_links(&normalize_anchors(&normalize_apostrophes(desc)));
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
            let mut any_details = false;
            for (_l, v) in &details_rows {
                if let Some(s) = v {
                    if !s.trim().is_empty() {
                        any_details = true;
                        break;
                    }
                }
            }
            if any_details {
                out.push_str("| Detail | Value |\n|--------|------:|\n");
                for (label, val_opt) in details_rows {
                    if let Some(val) = val_opt {
                        let v = normalize_internal_links(&normalize_anchors(
                            &normalize_apostrophes(val),
                        ));
                        if !v.trim().is_empty() {
                            out.push_str(&format!("| **{}** | {} |\n", label, v));
                        }
                    }
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
            let desc = normalize_all(&pet.description);
            if desc.is_empty() {
                out.push_str(&format!("- {}\n", name));
            } else {
                out.push_str(&format!("- **{}** — {}\n", name, desc));
            }
        }
        out.push('\n');
    }
    if !champ.trivia.is_empty() {
        out.push_str("## Trivia\n\n");
        render_starred_list(&mut out, &champ.trivia);
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
                let text = normalize_all(&change.text);
                if text.trim().is_empty() {
                    continue;
                }
                out.push_str(&format!("- {}\n", text));
            }
            out.push('\n');
        }
    }
    out.push_str("<!-- Raw excerpt (first 20 lines) -->\n\n<details><summary>Raw excerpt</summary>\n\n```wikitext\n");
    let excerpt_norm =
        normalize_internal_links(&normalize_anchors(&normalize_apostrophes(raw_excerpt)));
    out.push_str(&excerpt_norm.lines().take(20).collect::<Vec<_>>().join("\n"));
    out.push_str("\n```\n</details>\n");
    ensure_trailing_newline(&collapse_blank_lines(&out))
}

pub fn render_item_markdown(item: &Item, raw_excerpt: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", item.name));

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
    if !item.modes.is_empty() {
        overview.push(format!("- **Modes:** {}", item.modes.join(", ")));
    }
    if !overview.is_empty() {
        out.push_str("## Overview\n\n");
        out.push_str(&overview.join("\n"));
        out.push_str("\n\n");
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

    out.push_str("<!-- Raw excerpt (first 20 lines) -->\n\n<details><summary>Raw excerpt</summary>\n\n```wikitext\n");
    let excerpt_norm = normalize_all(raw_excerpt);
    out.push_str(&excerpt_norm.lines().take(20).collect::<Vec<_>>().join("\n"));
    out.push_str("\n```\n</details>\n");
    ensure_trailing_newline(&collapse_blank_lines(&out))
}

pub fn render_rune_markdown(rune: &Rune, raw_excerpt: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("# {}\n\n", rune.name));

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

    out.push_str("<!-- Raw excerpt (first 20 lines) -->\n\n<details><summary>Raw excerpt</summary>\n\n```wikitext\n");
    let excerpt_norm = normalize_all(raw_excerpt);
    out.push_str(&excerpt_norm.lines().take(20).collect::<Vec<_>>().join("\n"));
    out.push_str("\n```\n</details>\n");
    ensure_trailing_newline(&collapse_blank_lines(&out))
}

pub fn normalize_internal_links(s: &str) -> String {
    // Simple pattern [[Page|Display]] or [[Page]] -> Display or Page (underscores)
    let mut out = String::new();
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'[' && bytes[i + 1] == b'[' {
            i += 2;
            let mut inner = String::new();
            while i + 1 < bytes.len() && !(bytes[i] == b']' && bytes[i + 1] == b']') {
                inner.push(bytes[i] as char);
                i += 1;
            }
            if i + 1 >= bytes.len() {
                out.push_str(&format!("[[{}", inner));
                break;
            }
            i += 2; // skip closing
            if inner.starts_with("File:") {
                continue;
            }
            let display = if let Some(bar) = inner.find('|') {
                let page = &inner[..bar];
                let disp = &inner[bar + 1..];
                if disp.is_empty() {
                    page
                } else {
                    disp
                }
            } else {
                &inner
            };
            out.push_str(display.replace(' ', "_").as_str());
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
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
            while j < bytes.len() && bytes[j] == b'\'' {
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

fn normalize_all(s: &str) -> String {
    normalize_internal_links(&normalize_anchors(&normalize_apostrophes(s)))
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
        let indent = "  ".repeat(star_count.saturating_sub(1));
        let normalized =
            normalize_internal_links(&normalize_anchors(&normalize_apostrophes(content)));
        out.push_str(&indent);
        out.push_str("- ");
        out.push_str(&normalized);
        out.push('\n');
    }
}

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
        assert_eq!(
            normalize_internal_links("See [[Akshan/Cosmetics|Akshan (Collection)]]"),
            "See Akshan_(Collection)"
        );
        let s = normalize_anchors("[[Page#Section Title]]");
        assert_eq!(s, "Page#Section-Title");
    }

    #[test]
    fn rune_markdown_includes_trivia() {
        let rune = Rune {
            name: "Electrocute".to_string(),
            path: Some("Domination".to_string()),
            slot: Some("Keystone".to_string()),
            description: "Deal bonus damage.".to_string(),
            notes: vec!["* Triggered after three attacks".to_string()],
            trivia: vec!["* Popular in assassin matchups".to_string()],
            patch_history: Vec::new(),
            warnings: Vec::new(),
        };
        let md = render_rune_markdown(&rune, "== Raw ==\nContent");
        assert!(md.contains("## Trivia"));
        assert!(md.contains("Popular in assassin matchups"));
    }
}
