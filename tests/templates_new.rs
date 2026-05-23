use lol_wiki_md::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use std::collections::HashMap;

fn expand(body: &str) -> String {
    let reg = TemplateRegistry::new();
    let inv = parse_invocation(body);
    let ctx = ExpanderCtx {
        precision: 2,
        vars: HashMap::new(),
        conversion_ctx: None,
    };
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
    assert_eq!(out, "(+40/50/60% AP)");
}

#[test]
fn pp_clean() {
    let out = expand("pp|10 | 20|30%| ");
    assert_eq!(out, "10 / 20 / 30");
}

#[test]
fn ft_flip() {
    let out = expand("ft|a|b");
    assert_eq!(out, "「 a ⟷ b 」");
}

#[test]
fn ccd_passthrough_key() {
    let out = expand("ccd|AD");
    assert_eq!(out, "AD");
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
