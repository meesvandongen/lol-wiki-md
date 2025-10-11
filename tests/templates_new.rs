use lol_wiki_md::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use std::collections::HashMap;

fn expand(body: &str) -> String {
    let reg = TemplateRegistry::new();
    let inv = parse_invocation(body);
    let ctx = ExpanderCtx {
        precision: 2,
        vars: HashMap::new(),
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
    let out = expand("ft|abc");
    assert_eq!(out, "cba");
}

#[test]
fn ccd_passthrough_key() {
    let out = expand("ccd|AD");
    assert_eq!(out, "AD");
}
