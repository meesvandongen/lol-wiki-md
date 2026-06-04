use lol_wiki_md::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use lol_wiki_md::ConversionContext;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::tempdir;

fn expand(body: &str) -> String {
    let reg = TemplateRegistry::new();
    let inv = parse_invocation(body);
    let ctx = ExpanderCtx::new(2, &HashMap::new(), None);
    reg.expand(&inv, &ctx).unwrap().expanded
}

#[test]
fn quote_basic() {
    let out = expand("Quote|Victory awaits|Aatrox");
    assert!(out.starts_with("> Victory awaits — Aatrox"));
}

#[test]
fn sbc_uppercase() {
    let out = expand("sbc|small Caps");
    assert_eq!(out, "**SMALL CAPS**");
}

#[test]
fn ct_stub() {
    let out = expand("ct|Channeled");
    assert_eq!(out, "(Channeled)");
}

#[test]
fn ap_multi_sequence() {
    let out = expand("ap|40|50|60");
    assert_eq!(out, "40 / 50 / 60");
}

#[test]
fn pp_clean() {
    let out = expand("pp|10;20;30");
    assert_eq!(out, "10 / 20 / 30 (based on level)");
}

#[test]
fn ap_range_with_trailing_point_count() {
    // `60 to 310 6` is a 6-point range, not a literal string with a stray `6`.
    let out = expand("ap|60 to 310 6");
    assert_eq!(out, "60 / 110 / 160 / 210 / 260 / 310");
}

#[test]
fn ap_range_with_arithmetic_endpoints_and_count() {
    let out = expand("ap|80*1.4 to 310*1.4 6");
    assert_eq!(out, "112 / 176.4 / 240.8 / 305.2 / 369.6 / 434");
}

#[test]
fn pp_decreasing_by_step_renders_range() {
    // A `by` step is a magnitude; a decreasing series must still expand.
    let out = expand("pp|3.5 to 2 by 0.05");
    assert_eq!(out, "3.5 – 2 (based on level)");
}

#[test]
fn pp_cumulative_then_segments() {
    let out = expand("pp|16; then +4*x for 5; then +6*x for 5; then +8*x for 3; then +10*x for 6");
    assert_eq!(out, "16 – 130 (based on level)");
}

#[test]
fn pp_open_ended_then_after_counted_segments_fills_budget() {
    let out = expand("pp|2; then +2*x for 8; then +3*x for 4; then +4*x");
    assert_eq!(out, "2 – 50 (based on level)");
}

#[test]
fn pp_bare_then_without_count_is_left_raw() {
    // No preceding counted segment: the timing lives only in the prose formula,
    // so the increment must not be fabricated into a series.
    let out = expand("pp|0; then +5*x");
    assert_eq!(out, "0 / `then +5*x` (based on level)");
}

#[test]
fn ft_flip() {
    let out = expand("ft|a|b");
    assert_eq!(out.trim(), "a *(equivalently: b)*");
}

#[test]
fn ccd_and_cid_resolve_constants() {
    let td = tempdir().unwrap();
    let export_dir = td.path().join("export_out");
    std::fs::create_dir_all(&export_dir).unwrap();
    std::fs::write(
        export_dir.join("Module%3AItemData%2Fdata.txt"),
        r#"return {
    ["Infinity Edge"] = {
        ["stats"] = {
            ["critdamage"] = 40,
        }
    }
}"#,
    )
    .unwrap();

    let ctx = ConversionContext::new(&export_dir, 2).unwrap();
    let mut constants = HashMap::new();
    constants.insert("missile_speed".to_string(), "3800".to_string());
    ctx.insert_champion_constants("Graves", constants);

    let reg = TemplateRegistry::new();
    let expander_ctx = ExpanderCtx::new(2, &HashMap::new(), Some(Arc::new(ctx)));

    let ccd = parse_invocation("ccd|Graves|crit_base");
    assert_eq!(reg.expand(&ccd, &expander_ctx).unwrap().expanded, "175");

    let nested = parse_invocation("ccd|Graves|missile_speed");
    assert_eq!(reg.expand(&nested, &expander_ctx).unwrap().expanded, "3800");

    let cid = parse_invocation("cid|Infinity Edge|critdamage");
    assert_eq!(reg.expand(&cid, &expander_ctx).unwrap().expanded, "40");
}

fn expand_is_err(body: &str) -> bool {
    let reg = TemplateRegistry::new();
    let inv = parse_invocation(body);
    let ctx = ExpanderCtx::new(2, &HashMap::new(), None);
    reg.expand(&inv, &ctx).is_err()
}

#[test]
fn expr_failure_fails_fast() {
    assert!(expand_is_err("#expr:not actually math"));
}

#[test]
fn fd_invalid_number_fails_fast() {
    assert!(expand_is_err("fd|not-a-number"));
}

#[test]
fn critical_damage_invalid_params_fail_fast() {
    assert!(expand_is_err("critical damage|oops|100"));
}

#[test]
fn item_stat_table_without_context_fails_fast() {
    assert!(expand_is_err("Item stat table|pykehealth"));
}

#[test]
fn unknown_template_fails_fast() {
    assert!(expand_is_err("TotallyUnknownTemplate|x|y"));
}

#[test]
fn scroll_box_returns_content() {
    let out = expand("Scroll box|content=* Wrapped patch line");
    assert_eq!(out, "* Wrapped patch line");
}

#[test]
fn skill_tabs_positional_pairs_expand_to_marker() {
    // `{{st}}` (Template:Skill Tabs) is invoked with positional label/value
    // pairs and must surface the per-rank scaling, expanding nested templates.
    let out = expand("st|Physical Damage|{{ap|50 to 150}}");
    assert_eq!(
        out,
        "[SkillTab Physical Damage\u{1E}50 / 75 / 100 / 125 / 150]"
    );
}

#[test]
fn skill_tabs_multiple_columns_expand_to_marker() {
    let out = expand("st|Bonus Armor|{{ap|7 to 19}}|Maximum Bonus Armor|{{ap|7*8 to 19*8}}");
    assert_eq!(
        out,
        "[SkillTab Bonus Armor\u{1F}Maximum Bonus Armor\u{1E}7 / 10 / 13 / 16 / 19\u{1F}56 / 80 / 104 / 128 / 152]"
    );
}

#[test]
fn rune_data_reads_exported_template_fields() {
    let td = tempdir().unwrap();
    let export_dir = td.path().join("export_out");
    std::fs::create_dir_all(&export_dir).unwrap();
    std::fs::write(
        export_dir.join("Template%3ARune%20data%20Electrocute.txt"),
        concat!(
            "{{{{{1|Rune data}}}|Electrocute|{{{2|}}}|\n",
            "|path=Domination\n",
            "|slot=Keystone\n",
            "|caption={{quote|Storm quote|Rune caption}}\n",
            "}}"
        ),
    )
    .unwrap();

    let reg = TemplateRegistry::new();
    let ctx = ExpanderCtx::new(
        2,
        &HashMap::new(),
        Some(Arc::new(ConversionContext::new(td.path(), 2).unwrap())),
    );

    let path = parse_invocation("Rune data Electrocute|pst2|path");
    assert_eq!(reg.expand(&path, &ctx).unwrap().expanded, "Domination");

    let caption = parse_invocation("Rune data Electrocute|pst2|caption");
    assert!(reg
        .expand(&caption, &ctx)
        .unwrap()
        .expanded
        .contains("Storm quote"));
}

#[test]
fn map_changes_renders_mode_sections_from_exported_data() {
    let td = tempdir().unwrap();
    let export_dir = td.path().join("export_out");
    std::fs::create_dir_all(&export_dir).unwrap();
    std::fs::write(
        export_dir.join("Template%3AMap%20changes%2Fdata%2Faram.txt"),
        concat!(
            "{{{{{1}}}|aram|{{{2|}}}|\n",
            "|Electrocute =\n",
            "* Cooldown changed to {{pp|15 to 10}} seconds.\n",
            "}}"
        ),
    )
    .unwrap();

    let reg = TemplateRegistry::new();
    let ctx = ExpanderCtx::new(
        2,
        &HashMap::new(),
        Some(Arc::new(ConversionContext::new(td.path(), 2).unwrap())),
    );

    let inv = parse_invocation("Map changes|Electrocute");
    let out = reg.expand(&inv, &ctx).unwrap().expanded;
    assert!(out.contains("### Howling Abyss"));
    assert!(out.contains("Cooldown changed to 15 – 10 (based on level) seconds."));
}

#[test]
fn tip_prefers_display_label() {
    let out = expand("tip|League of Legends|LoL");
    assert_eq!(out, "LoL");
}

#[test]
fn tip_icononly_suppresses_output() {
    let out = expand("tip|icononly=true|Wild Rift");
    assert_eq!(out, "");
}

#[test]
fn icon_unwrap_handles_possessive() {
    let out = expand("ci|Akshan|'s");
    assert_eq!(out, "Akshan's");
}

#[test]
fn neutralize_template_returns_empty() {
    let out = expand("Section top");
    assert!(out.is_empty());
}
