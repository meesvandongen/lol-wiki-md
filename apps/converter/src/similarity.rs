use crate::error::{ConvertError, Result};
use crate::parse::ability::parse_ability_template;
use crate::wiki_export::{url_decode, WikiExport};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AbilityMatchMode {
    Heading,
    BodyMention,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExpectedAbilitySpec {
    name: String,
    match_mode: AbilityMatchMode,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RenderedAbilityCoverage {
    heading_names: Vec<String>,
    normalized_body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChampionSimilarityReport {
    pub name: String,
    pub basis: String,
    pub score: f32,
    pub threshold: f32,
    pub expected_abilities: Vec<String>,
    pub rendered_abilities: Vec<String>,
    pub missing_abilities: Vec<String>,
    pub expected_sections: Vec<String>,
    pub missing_sections: Vec<String>,
    pub needs_followup: bool,
}

pub fn audit_champion_output(
    export: &WikiExport,
    output_dir: &Path,
    name: &str,
    threshold: f32,
) -> Result<ChampionSimilarityReport> {
    let source_main = export.read_champion_main(name)?;
    let output_path = output_dir.join(format!("{}.md", name.replace(' ', "_")));
    let rendered = match fs::read_to_string(&output_path) {
        Ok(text) => text,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(err) => return Err(ConvertError::Io(err)),
    };

    let expected_ability_specs = expected_ability_specs(export, name)?;
    let expected_abilities = expected_ability_specs
        .iter()
        .map(|spec| spec.name.clone())
        .collect::<Vec<_>>();
    let rendered_coverage = extract_rendered_ability_coverage(&rendered);
    let rendered_abilities = rendered_coverage.heading_names.clone();
    let missing_abilities = missing_expected_abilities(
        &expected_ability_specs,
        &rendered_coverage.heading_names,
        &rendered_coverage.normalized_body,
    );

    let expected_sections = expected_sections(&source_main, !expected_abilities.is_empty());
    let rendered_sections = extract_rendered_sections(&rendered);
    let missing_sections = expected_sections
        .iter()
        .filter(|section| !rendered_sections.contains(&section.to_ascii_lowercase()))
        .cloned()
        .collect::<Vec<_>>();

    let ability_score = if expected_abilities.is_empty() {
        1.0
    } else {
        let heading_expected_count = expected_ability_specs
            .iter()
            .filter(|spec| spec.match_mode == AbilityMatchMode::Heading)
            .count();
        let matched_heading_count = expected_ability_specs
            .iter()
            .filter(|spec| {
                spec.match_mode == AbilityMatchMode::Heading
                    && expected_ability_is_matched(
                        spec,
                        &rendered_coverage.heading_names,
                        &rendered_coverage.normalized_body,
                    )
            })
            .count();
        let count_score = if heading_expected_count == 0 {
            1.0
        } else {
            matched_heading_count as f32 / heading_expected_count as f32
        };
        let matched_name_count = expected_ability_specs
            .iter()
            .filter(|spec| {
                expected_ability_is_matched(
                    spec,
                    &rendered_coverage.heading_names,
                    &rendered_coverage.normalized_body,
                )
            })
            .count();
        let name_score = matched_name_count as f32 / expected_abilities.len() as f32;
        (count_score + name_score) / 2.0
    };

    let section_score = if expected_sections.is_empty() {
        1.0
    } else {
        (expected_sections
            .len()
            .saturating_sub(missing_sections.len())) as f32
            / expected_sections.len() as f32
    };

    let score = ((ability_score * 0.7) + (section_score * 0.3) * 1.0).clamp(0.0, 1.0);
    let score = (score * 100.0).round() / 100.0;
    let needs_followup =
        score < threshold || !missing_abilities.is_empty() || !missing_sections.is_empty();

    Ok(ChampionSimilarityReport {
        name: name.to_string(),
        basis: "local-export-structural-v1".to_string(),
        score,
        threshold,
        expected_abilities,
        rendered_abilities,
        missing_abilities,
        expected_sections,
        missing_sections,
        needs_followup,
    })
}

pub fn audit_all_champion_outputs(
    export: &WikiExport,
    output_dir: &Path,
    threshold: f32,
) -> Result<Vec<ChampionSimilarityReport>> {
    let mut reports = Vec::new();
    for name in export.list_champion_names()? {
        reports.push(audit_champion_output(export, output_dir, &name, threshold)?);
    }
    Ok(reports)
}

fn expected_ability_specs(export: &WikiExport, champ: &str) -> Result<Vec<ExpectedAbilitySpec>> {
    let mut out: Vec<ExpectedAbilitySpec> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    for title in export.list_champion_ability_templates(champ)? {
        let Some(raw) = export.read_template_page(&title)? else {
            continue;
        };
        let param_map = parse_ability_template(&raw).ok();
        let Some(name) = canonical_ability_name(&title, &raw, param_map.as_ref()) else {
            continue;
        };
        let Some(match_mode) = expected_ability_match_mode(param_map.as_ref()) else {
            continue;
        };
        let dedupe_key = normalize_match_text(&name);
        if dedupe_key.is_empty() {
            continue;
        }
        if let Some(existing_idx) = seen.get(&dedupe_key).copied() {
            if match_mode == AbilityMatchMode::Heading {
                out[existing_idx].match_mode = AbilityMatchMode::Heading;
            }
            continue;
        }
        seen.insert(dedupe_key, out.len());
        out.push(ExpectedAbilitySpec { name, match_mode });
    }
    Ok(out)
}

#[cfg(test)]
fn extract_rendered_ability_names(rendered: &str) -> Vec<String> {
    extract_rendered_ability_coverage(rendered).heading_names
}

fn extract_rendered_ability_coverage(rendered: &str) -> RenderedAbilityCoverage {
    let mut in_abilities = false;
    let mut names = Vec::new();
    let mut body_lines = Vec::new();
    for line in rendered.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            if in_abilities {
                break;
            }
            in_abilities = trimmed.eq_ignore_ascii_case("## Abilities");
            continue;
        }
        if !in_abilities {
            continue;
        }
        body_lines.push(line);
        if let Some(rest) = trimmed.strip_prefix("### ") {
            if let Some((_, name)) = rest.split_once(" – ") {
                let clean = name.trim();
                if !clean.is_empty() {
                    names.push(clean.to_string());
                }
            }
        }
    }
    RenderedAbilityCoverage {
        heading_names: names,
        normalized_body: normalize_match_text(&body_lines.join("\n")),
    }
}

fn extract_rendered_sections(rendered: &str) -> HashSet<String> {
    rendered
        .lines()
        .filter_map(|line| line.trim().strip_prefix("## "))
        .map(|heading| heading.trim().to_ascii_lowercase())
        .collect()
}

fn expected_sections(source_main: &str, has_abilities: bool) -> Vec<String> {
    let lower = source_main.to_ascii_lowercase();
    let level_two_headings = extract_source_level_two_headings(source_main);
    let mut sections = Vec::new();
    if has_abilities {
        sections.push("Abilities".to_string());
    }
    if level_two_headings.contains("pets") || lower.contains("{{infobox/pet") {
        sections.push("Pets".to_string());
    }
    if level_two_headings.contains("trivia") {
        sections.push("Trivia".to_string());
    }
    if level_two_headings.contains("notes") {
        sections.push("Notes".to_string());
    }
    if lower.contains("patch box") || level_two_headings.contains("patch history") {
        sections.push("Patch History".to_string());
    }
    sections
}

fn missing_expected_abilities(
    expected: &[ExpectedAbilitySpec],
    rendered_headings: &[String],
    rendered_body: &str,
) -> Vec<String> {
    expected
        .iter()
        .filter(|expected_spec| {
            !expected_ability_is_matched(expected_spec, rendered_headings, rendered_body)
        })
        .map(|expected_spec| expected_spec.name.clone())
        .collect()
}

fn expected_ability_is_matched(
    expected_spec: &ExpectedAbilitySpec,
    rendered_headings: &[String],
    rendered_body: &str,
) -> bool {
    match expected_spec.match_mode {
        AbilityMatchMode::Heading => rendered_headings.iter().any(|rendered_name| {
            normalize_match_text(rendered_name) == normalize_match_text(&expected_spec.name)
        }),
        AbilityMatchMode::BodyMention => expected_ability_aliases(&expected_spec.name)
            .iter()
            .any(|alias| contains_normalized_phrase(rendered_body, alias)),
    }
}

fn expected_ability_match_mode(
    param_map: Option<&HashMap<String, String>>,
) -> Option<AbilityMatchMode> {
    let Some(skill_slot) = param_map.and_then(|map| map.get("skill")) else {
        return Some(AbilityMatchMode::BodyMention);
    };
    let normalized = skill_slot.trim();
    if normalized.is_empty() {
        return Some(AbilityMatchMode::BodyMention);
    }
    match normalized.to_ascii_uppercase().as_str() {
        "I" | "P" | "PASSIVE" | "Q" | "W" | "E" | "R" | "ULTIMATE" => {
            Some(AbilityMatchMode::Heading)
        }
        _ => None,
    }
}

fn expected_ability_aliases(name: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    let mut seen = HashSet::new();
    for candidate in [
        Some(name.to_string()),
        strip_trailing_numeric_suffix(name),
        strip_trailing_letter_suffix(name),
    ] {
        let Some(candidate) = candidate else {
            continue;
        };
        let normalized = normalize_match_text(&candidate);
        if normalized.is_empty() || !seen.insert(normalized) {
            continue;
        }
        aliases.push(candidate);
    }
    aliases
}

fn contains_normalized_phrase(rendered_body: &str, phrase: &str) -> bool {
    let normalized_phrase = normalize_match_text(phrase);
    if rendered_body.is_empty() || normalized_phrase.is_empty() {
        return false;
    }
    let padded_haystack = format!(" {rendered_body} ");
    let padded_phrase = format!(" {normalized_phrase} ");
    padded_haystack.contains(&padded_phrase)
}

fn normalize_match_text(input: &str) -> String {
    let mut normalized = String::new();
    let mut last_was_space = true;
    for ch in input.trim().chars() {
        if ch == '_' {
            if !last_was_space {
                normalized.push(' ');
                last_was_space = true;
            }
            continue;
        }
        let lowered = ch.to_ascii_lowercase();
        if lowered.is_ascii_alphanumeric() {
            normalized.push(lowered);
            last_was_space = false;
        } else if !last_was_space {
            normalized.push(' ');
            last_was_space = true;
        }
    }
    normalized.trim().to_string()
}

fn extract_source_level_two_headings(source_main: &str) -> HashSet<String> {
    source_main
        .lines()
        .filter_map(|line| parse_heading_line(line.trim()))
        .filter_map(|(level, heading)| (level == 2).then_some(heading))
        .collect()
}

fn parse_heading_line(line: &str) -> Option<(usize, String)> {
    let trimmed = line.trim();
    if !trimmed.starts_with('=') {
        return None;
    }

    let leading = trimmed.chars().take_while(|&ch| ch == '=').count();
    let trailing = trimmed.chars().rev().take_while(|&ch| ch == '=').count();
    if leading < 2 || trailing < 2 {
        return None;
    }

    let content = trimmed[leading..trimmed.len() - trailing].trim();
    if content.is_empty() {
        return None;
    }

    Some((leading.min(trailing), content.to_ascii_lowercase()))
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

fn canonical_ability_name(
    template_title: &str,
    raw_template: &str,
    param_map: Option<&HashMap<String, String>>,
) -> Option<String> {
    if let Some(name) = param_map
        .and_then(|map| map.get("name"))
        .map(|value| value.trim())
        .filter(|value| !value.is_empty() && !is_placeholder_ability_name(value))
    {
        return Some(name.to_string());
    }

    let template_name = ability_name_from_template_source(template_title, raw_template);
    if let Some(disp_name) = param_map
        .and_then(|map| map.get("disp_name"))
        .map(|value| value.trim())
        .filter(|value| !value.is_empty() && !is_placeholder_ability_name(value))
    {
        if template_name
            .as_deref()
            .map_or(true, |name| name.len() <= 1 || has_numeric_suffix(name))
        {
            return Some(disp_name.to_string());
        }
    }

    template_name
        .map(|name| strip_trailing_numeric_suffix(&name).unwrap_or(name))
        .or_else(|| {
            param_map
                .and_then(|map| map.get("disp_name"))
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty() && !is_placeholder_ability_name(value))
        })
}

fn ability_name_from_template_path(path: &str) -> Option<String> {
    let name = url_decode(path.rsplit('/').next()?.trim()).replace('_', " ");
    if name.len() <= 1 || is_placeholder_ability_name(&name) {
        None
    } else {
        Some(name)
    }
}

fn has_numeric_suffix(name: &str) -> bool {
    name.trim()
        .chars()
        .last()
        .is_some_and(|ch| ch.is_ascii_digit())
}

fn strip_trailing_numeric_suffix(name: &str) -> Option<String> {
    let trimmed = name.trim();
    let suffix = trimmed.rsplit_once(' ')?;
    if suffix.1.chars().all(|ch| ch.is_ascii_digit()) {
        let base = suffix.0.trim();
        if !base.is_empty() {
            return Some(base.to_string());
        }
    }
    None
}

fn strip_trailing_letter_suffix(name: &str) -> Option<String> {
    let trimmed = name.trim();
    let suffix = trimmed.rsplit_once(' ')?;
    if suffix.1.len() == 1 && suffix.1.chars().all(|ch| ch.is_ascii_uppercase()) {
        let base = suffix.0.trim();
        if !base.is_empty() {
            return Some(base.to_string());
        }
    }
    None
}

fn ability_name_from_template_wrapper(raw_template: &str) -> Option<String> {
    let trimmed = raw_template.trim_start();
    let start = trimmed.find("}}}|")? + 4;
    let rest = &trimmed[start..];
    let end = rest.find('|')?;
    ability_name_from_template_path(rest[..end].trim())
}

fn is_placeholder_ability_name(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "" | "false" | "n/a" | "na" | "none" | "unknown"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wiki_export::url_encode;
    use tempfile::tempdir;

    #[test]
    fn extract_rendered_ability_names_ignores_patch_history_headings() {
        let rendered = concat!(
            "## Abilities\n\n",
            "### Q – Neurotoxin\n\n",
            "### Q – Venomous Bite\n\n",
            "## Patch History\n\n",
            "### V25.15\n"
        );
        assert_eq!(
            extract_rendered_ability_names(rendered),
            vec!["Neurotoxin".to_string(), "Venomous Bite".to_string()]
        );
    }

    #[test]
    fn audit_champion_output_flags_missing_transform_abilities() {
        let td = tempdir().unwrap();
        let export_out = td.path().join("export_out");
        let rendered_out = td.path().join("rendered");
        std::fs::create_dir_all(&export_out).unwrap();
        std::fs::create_dir_all(&rendered_out).unwrap();

        std::fs::write(
            export_out.join("Elise.txt"),
            concat!(
                "{{Champion info|Elise}}\n",
                "== Abilities ==\n",
                "{{Data Elise/I|Ability}}\n",
                "{{Image tabber|title1=Human|content1=\n",
                "{{Data Elise/Q|Ability}}\n",
                "|title2=Spider|content2=\n",
                "{{Data Elise/Venomous Bite|Ability}}\n",
                "}}\n",
                "{{Data Elise/R|Ability}}\n",
                "== Trivia ==\n* Fact\n",
                "== Patch history ==\n{{Patch box|Elise}}\n"
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
            std::fs::write(
                export_out.join(format!("{}.txt", url_encode(title))),
                content,
            )
            .unwrap();
        }

        std::fs::write(
            rendered_out.join("Elise.md"),
            concat!(
                "# Elise\n\n",
                "## Abilities\n\n",
                "### Passive – Spider Queen\n\n",
                "### R – Spider Form\n\n",
                "## Trivia\n\n",
                "## Patch History\n"
            ),
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let report = audit_champion_output(&export, &rendered_out, "Elise", 0.85).unwrap();

        assert!(report.needs_followup);
        assert!(report.missing_abilities.contains(&"Neurotoxin".to_string()));
        assert!(report
            .missing_abilities
            .contains(&"Venomous Bite".to_string()));
        assert!(report.score < 0.85);
    }

    #[test]
    fn audit_champion_output_ignores_unsupported_non_core_slots() {
        let td = tempdir().unwrap();
        let export_out = td.path().join("export_out");
        let rendered_out = td.path().join("rendered");
        std::fs::create_dir_all(&export_out).unwrap();
        std::fs::create_dir_all(&rendered_out).unwrap();

        std::fs::write(
            export_out.join("Tester.txt"),
            concat!(
                "{{Champion info|Tester}}\n",
                "== Abilities ==\n",
                "{{Data Tester/Q|Ability}}\n",
                "{{Data Tester/Basic Attack|Ability}}\n",
            ),
        )
        .unwrap();

        for (title, content) in [
            (
                "Template:Data Tester/Q",
                "{{{{{1|Ability data}}}|Heroic Swing|skill=Q|description=Core ability}}",
            ),
            (
                "Template:Data Tester/Basic Attack",
                "{{{{{1|Ability data}}}|Basic Attack|skill=A|description=Helper attack}}",
            ),
        ] {
            std::fs::write(
                export_out.join(format!("{}.txt", url_encode(title))),
                content,
            )
            .unwrap();
        }

        std::fs::write(
            rendered_out.join("Tester.md"),
            concat!(
                "# Tester\n\n",
                "## Abilities\n\n",
                "### Q – Heroic Swing\n\n"
            ),
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let report = audit_champion_output(&export, &rendered_out, "Tester", 0.85).unwrap();

        assert!(!report
            .expected_abilities
            .contains(&"Basic Attack".to_string()));
        assert!(report.missing_abilities.is_empty());
        assert!(!report.needs_followup);
    }

    #[test]
    fn audit_champion_output_treats_auxiliary_body_mentions_as_covered() {
        let td = tempdir().unwrap();
        let export_out = td.path().join("export_out");
        let rendered_out = td.path().join("rendered");
        std::fs::create_dir_all(&export_out).unwrap();
        std::fs::create_dir_all(&rendered_out).unwrap();

        std::fs::write(
            export_out.join("Tester.txt"),
            concat!(
                "{{Champion info|Tester}}\n",
                "== Abilities ==\n",
                "{{Data Tester/W|Ability}}\n",
                "{{Data Tester/Blood Thirst|Ability}}\n",
            ),
        )
        .unwrap();

        std::fs::write(
            export_out.join(format!("{}.txt", url_encode("Template:Data Tester/W"))),
            "{{{{{1|Ability data}}}|name=Blood Thirst / Blood Price|skill=W|description=Toggle ability}}",
        )
        .unwrap();
        std::fs::write(
            export_out.join(format!(
                "{}.txt",
                url_encode("Template:Data Tester/Blood Thirst")
            )),
            "{{Data Tester/W|Ability}}",
        )
        .unwrap();

        std::fs::write(
            rendered_out.join("Tester.md"),
            concat!(
                "# Tester\n\n",
                "## Abilities\n\n",
                "### W – Blood Thirst / Blood Price\n\n",
                "**TOGGLE OFF – BLOOD THIRST:** Tester heals on-hit.\n\n"
            ),
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let report = audit_champion_output(&export, &rendered_out, "Tester", 0.85).unwrap();

        assert!(
            report
                .expected_abilities
                .contains(&"Blood Thirst".to_string()),
            "{report:?}"
        );
        assert!(report.missing_abilities.is_empty(), "{report:?}");
        assert_eq!(report.score, 1.0);
        assert!(!report.needs_followup);
    }

    #[test]
    fn expected_sections_ignore_nested_notes_headings() {
        let source = concat!(
            "== Abilities ==\n",
            "{{Infobox/Pet\n",
            "=== Notes ===\n",
            "* Pet note\n",
            "}}\n",
            "== Patch history ==\n",
        );

        let sections = expected_sections(source, true);

        assert!(sections.contains(&"Abilities".to_string()));
        assert!(sections.contains(&"Patch History".to_string()));
        assert!(!sections.contains(&"Notes".to_string()));
    }

    #[test]
    fn body_mentions_match_single_letter_suffix_variants() {
        let coverage = extract_rendered_ability_coverage(concat!(
            "# Tester\n\n",
            "## Abilities\n\n",
            "### E – Shadow Step\n\n",
            "**Shadow Assassin Bonus:** Shadow Step gains extra effects.\n"
        ));
        let expected = ExpectedAbilitySpec {
            name: "Shadow Step A".to_string(),
            match_mode: AbilityMatchMode::BodyMention,
        };

        assert!(expected_ability_is_matched(
            &expected,
            &coverage.heading_names,
            &coverage.normalized_body,
        ));
    }

    #[test]
    fn redirect_aliases_canonicalize_to_target_heading() {
        let td = tempdir().unwrap();
        let export_out = td.path().join("export_out");
        let rendered_out = td.path().join("rendered");
        std::fs::create_dir_all(&export_out).unwrap();
        std::fs::create_dir_all(&rendered_out).unwrap();

        std::fs::write(
            export_out.join("Tester.txt"),
            concat!(
                "{{Champion info|Tester}}\n",
                "== Abilities ==\n",
                "{{Data Tester/Q|Ability}}\n",
            ),
        )
        .unwrap();

        std::fs::write(
            export_out.join(format!("{}.txt", url_encode("Template:Data Tester/Q"))),
            "{{{{{1|Ability data}}}|Heroic Swing|skill=Q|description=Core ability}}",
        )
        .unwrap();
        std::fs::write(
            export_out.join(format!(
                "{}.txt",
                url_encode("Template:Data Tester/Heroic Swing")
            )),
            "{{Data Tester/Q|Ability}}",
        )
        .unwrap();
        std::fs::write(
            export_out.join(format!(
                "{}.txt",
                url_encode("Template:Data Tester/Big Swing")
            )),
            "#REDIRECT [[Template:Data_Tester/Heroic Swing]]",
        )
        .unwrap();

        std::fs::write(
            rendered_out.join("Tester.md"),
            concat!(
                "# Tester\n\n",
                "## Abilities\n\n",
                "### Q – Heroic Swing\n\n",
            ),
        )
        .unwrap();

        let export = WikiExport::new(td.path());
        let report = audit_champion_output(&export, &rendered_out, "Tester", 0.85).unwrap();

        assert_eq!(report.expected_abilities, vec!["Heroic Swing".to_string()]);
        assert!(report.missing_abilities.is_empty(), "{report:?}");
        assert_eq!(report.score, 1.0);
        assert!(!report.needs_followup);
    }

    #[test]
    fn ability_name_from_template_path_decodes_percent_escapes() {
        assert_eq!(
            ability_name_from_template_path("Template:Data Taliyah/Weaver%27s Wall"),
            Some("Weaver's Wall".to_string())
        );
    }
}
