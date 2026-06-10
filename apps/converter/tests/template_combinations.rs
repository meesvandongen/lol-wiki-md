//! Template combination tests mined from the live wiki corpus.
//!
//! Every case in this file is a real invocation (or a minimal reduction of one)
//! observed while mining `export_out/` — 597 champion/item/rune pages plus
//! 3,310 transcluded template/module pages. See `scripts/test-mining/` for the
//! mining scripts, the reproduction workflow, and `DIVERGENCES.md`.
//!
//! Two kinds of assertion live here:
//!
//! * **Differential** (`differential_*`): pure value / parser-function
//!   templates whose output is context-independent, so the wiki's own rendered
//!   HTML (`api.php?action=parse`) is authoritative ground truth, per AGENTS.md.
//!   The expected strings below were captured from that render and must match
//!   the converter exactly.
//! * **Contract** (`contract_*`): icon/label/tooltip templates where the
//!   converter renders a lossless *text* contract (see `STYLE.md`) that
//!   intentionally differs from the wiki's icons/colored spans. These pin the
//!   converter's documented behavior against real inputs.

use lol_wiki_md::convert::util::expand_inline_templates;
use lol_wiki_md::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use lol_wiki_md::ConversionContext;
use std::collections::HashMap;
use std::sync::Arc;
use tempfile::tempdir;

/// Expand a snippet through the full inline pipeline (handles nesting and the
/// fixed-point iteration exactly as real conversion does), with no champion
/// data context.
fn render(text: &str) -> String {
    let reg = TemplateRegistry::new();
    let vars = HashMap::new();
    expand_inline_templates(text, 2, &vars, &reg, None).expect("expansion should succeed")
}

/// Expand a single bare template body (no surrounding `{{ }}`) through the
/// registry, mirroring the helper used by `templates_new.rs`.
fn expand_one(body: &str) -> String {
    let reg = TemplateRegistry::new();
    let inv = parse_invocation(body);
    let ctx = ExpanderCtx::new(2, &HashMap::new(), None);
    reg.expand(&inv, &ctx).unwrap().expanded
}

/// Run a table of `(input, expected)` cases through `render`, collecting every
/// mismatch so a single failure reports the full diff instead of stopping at
/// the first.
fn assert_table(cases: &[(&str, &str)]) {
    let mut failures = Vec::new();
    for (input, expected) in cases {
        let got = render(input);
        if got != *expected {
            failures.push(format!(
                "  {input}\n    expected: {expected:?}\n    got:      {got:?}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} case(s) diverged from wiki ground truth:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

// ---------------------------------------------------------------------------
// Differential: `ap` ability/scaling progressions
// ---------------------------------------------------------------------------

#[test]
fn differential_ap_explicit_sequences() {
    // Real corpus invocations (Aatrox, Ahri, et al.). Explicit per-rank lists
    // of 3–6 values render verbatim, slash-separated.
    assert_table(&[
        ("{{ap|40|50|60}}", "40 / 50 / 60"),
        ("{{ap|60|45|30}}", "60 / 45 / 30"),
        ("{{ap|18|16|14|12|10}}", "18 / 16 / 14 / 12 / 10"),
        ("{{ap|1|2|3|5|7|9}}", "1 / 2 / 3 / 5 / 7 / 9"),
        ("{{ap|20|30|40|50}}", "20 / 30 / 40 / 50"),
    ]);
}

#[test]
fn differential_ap_ranges_and_arithmetic() {
    assert_table(&[
        // `start to end count`: inclusive linear interpolation.
        ("{{ap|45 to 25 3}}", "45 / 35 / 25"),
        ("{{ap|60 to 310 6}}", "60 / 110 / 160 / 210 / 260 / 310"),
        // Arithmetic endpoints are evaluated before interpolation.
        (
            "{{ap|80*1.4 to 310*1.4 6}}",
            "112 / 176.4 / 240.8 / 305.2 / 369.6 / 434",
        ),
        ("{{ap|40*0.4 to 120*0.4}}", "16 / 24 / 32 / 40 / 48"),
        // A single arithmetic scalar collapses to one value.
        ("{{ap|85*0.75}}", "63.75"),
    ]);
}

// ---------------------------------------------------------------------------
// Differential: `pp` per-level progressions
// ---------------------------------------------------------------------------

#[test]
fn differential_pp_ranges_and_sequences() {
    // Only the context-independent `pp` forms are differential-tested; the
    // `then +k*x` cumulative forms depend on the champion's level count and are
    // covered by the existing in-context unit tests instead (see DIVERGENCES.md).
    assert_table(&[
        ("{{pp|10;20;30}}", "10 / 20 / 30 (based on level)"),
        ("{{pp|15 to 10}}", "15 – 10 (based on level)"),
        ("{{pp|10 to 146}}", "10 – 146 (based on level)"),
        // `key=%` appends a percent unit to each endpoint.
        ("{{pp|key=%|6.5 to 15}}", "6.5% – 15% (based on level)"),
        // A `by` step is a magnitude; the range still collapses to endpoints.
        ("{{pp|3.5 to 2 by 0.05}}", "3.5 – 2 (based on level)"),
    ]);
}

// ---------------------------------------------------------------------------
// Differential: `fd` fixed-decimal formatting
// ---------------------------------------------------------------------------

#[test]
fn differential_fd_passthrough() {
    assert_table(&[
        ("{{fd|0.625}}", "0.625"),
        ("{{fd|1.5}}", "1.5"),
        ("{{fd|802.75}}", "802.75"),
    ]);
}

// ---------------------------------------------------------------------------
// Differential: parser functions (`#expr`, `#if`, `#ifeq`, `#switch`)
// ---------------------------------------------------------------------------

#[test]
fn differential_expr_arithmetic() {
    assert_table(&[
        ("{{#expr:1+2*3}}", "7"),
        ("{{#expr:(5+3)*2-1}}", "15"),
        ("{{#expr:100/8}}", "12.5"),
        ("{{#expr:2^10}}", "1024"),
        ("{{#expr:(1/3)*100 round 2}}", "33.33"),
        ("{{#expr:7 round 0}}", "7"),
        ("{{#expr:2.5 round 0}}", "3"),
        ("{{#expr:abs(-4.2)}}", "4.2"),
        ("{{#expr:ceil(2.1)}}", "3"),
        ("{{#expr:floor(2.9)}}", "2"),
    ]);
}

#[test]
fn differential_conditional_parser_functions() {
    assert_table(&[
        ("{{#if:x|yes|no}}", "yes"),
        ("{{#if:|yes|no}}", "no"),
        ("{{#ifeq:5|5|equal|nope}}", "equal"),
        ("{{#ifeq:abc|abc|equal|nope}}", "equal"),
        ("{{#ifeq:5|6|equal|nope}}", "nope"),
        ("{{#switch:b|a=Apple|b=Banana|c=Cherry|Default}}", "Banana"),
        // An explicit matching key wins even when a `#default=` is present.
        ("{{#switch:c|a=Apple|#default=Fallback|c=Cherry}}", "Cherry"),
    ]);
}

// ---------------------------------------------------------------------------
// Nesting: templates inside other templates' parameters
// ---------------------------------------------------------------------------

#[test]
fn nesting_fd_wraps_expr() {
    // {{fd|{{#expr:(1/3)*100 round 2}}}} — formatter wrapping a computed value.
    assert_eq!(render("{{fd|{{#expr:(1/3)*100 round 2}}}}"), "33.33");
}

#[test]
fn nesting_as_and_sti_wrap_inner_templates() {
    // `{{sti|{{as|...}}}}` and `{{as|{{fd|..}} ...|label}}` are common ability
    // scaling annotations; the inner value template is expanded and the wrapper
    // unwraps to its reader-facing text.
    assert_eq!(
        render("{{sti|{{as|45% '''bonus''' movement speed}}}}"),
        "45% '''bonus''' movement speed"
    );
    assert_eq!(
        render("{{as|{{fd|1.5}} '''additional bonus''' magic damage|magic damage}}"),
        "1.5 '''additional bonus''' magic damage"
    );
}

#[test]
fn nesting_skill_tabs_wrap_ap_progressions() {
    // {{st|Label|{{ap|range}}%}} — Skill Tabs expands the nested progression and
    // emits the per-column marker (record-separator \u{1E} between label/value).
    assert_eq!(
        render("{{st|Damage Reduction|{{ap|55 to 75}}%}}"),
        "[SkillTab Damage Reduction\u{1E}55 / 60 / 65 / 70 / 75%]"
    );
}

#[test]
fn nesting_dv_wraps_tooltips() {
    // {{dv|{{tt|..|..}}|{{tt|..|..}}}} — Delimit values joins tooltip entries
    // with the bullet separator.
    assert_eq!(
        render("{{dv|{{tt|70|Orb radius}}|{{tt|230|Expanded field radius}}}}"),
        "70 (Orb radius) • 230 (Expanded field radius)"
    );
}

#[test]
fn nesting_flip_text_wraps_value_templates() {
    // {{ft|{{ap|..}} {{as|..}} magic|120%}} — flip-text expands both sides and
    // renders them with the documented `(equivalently: ...)` normalization.
    let out = render("{{ft|{{ap|66|96|126|156|186}} {{as|(+ 60% AP)}} magic|120%}}");
    assert_eq!(
        out.trim(),
        "66 / 96 / 126 / 156 / 186 (+ 60% AP) magic *(equivalently: 120%)*"
    );
}

// ---------------------------------------------------------------------------
// Contract: icon / label / tooltip templates render lossless text
// ---------------------------------------------------------------------------

#[test]
fn contract_inline_icon_labels() {
    // {{ci|Champion}} unwraps to the champion name; a trailing display label is
    // preferred when present.
    assert_eq!(expand_one("ci|Corki"), "Corki");
    assert_eq!(
        expand_one("ci|Morgana|size=32|Morgana, the Fallen"),
        "Morgana, the Fallen"
    );
    // {{ai|Ability|Champion}} surfaces the ability name.
    assert_eq!(expand_one("ai|Wind Wall|Yasuo"), "Wind Wall");
    // An explicit possessive marker is appended to the name.
    assert_eq!(expand_one("ci|Akshan|'s"), "Akshan's");
}

#[test]
fn contract_tooltip_and_tip() {
    // {{tt|value|hover}} keeps the hover text in parentheses; a bare value is
    // passed through.
    assert_eq!(
        expand_one("tt|865|Maximum total range (Estimated)"),
        "865 (Maximum total range (Estimated))"
    );
    assert_eq!(expand_one("tt|50%"), "50%");
    // {{tip|key}} / {{tip|key|display}} render the reader-facing label.
    assert_eq!(expand_one("tip|proc damage"), "proc damage");
    assert_eq!(
        expand_one("tip|homing projectile destruction|popped"),
        "popped"
    );
}

#[test]
fn contract_numeric_and_symbol_helpers() {
    // Melee/ranged split is made explicit (wiki shows `( 85 / 65)`).
    assert_eq!(expand_one("rd|85|65"), "85 (melee) / 65 (ranged)");
    // The gold icon becomes the word "gold"; the bare {{g}} numeric helper does not.
    assert_eq!(expand_one("gold|500"), "500 gold");
    assert_eq!(expand_one("g|500"), "500");
    // Small-caps becomes bold uppercase.
    assert_eq!(expand_one("sbc|Seething Strike"), "**SEETHING STRIKE**");
}

// ---------------------------------------------------------------------------
// Differential / contract: a wider mined sweep (round 2)
// ---------------------------------------------------------------------------

#[test]
fn differential_ap_more_sequences_and_ranges() {
    assert_table(&[
        // Two-value and six-value explicit lists (Ahri, Jhin, et al.).
        ("{{ap|25|50}}", "25 / 50"),
        (
            "{{ap|70|110|150|190|230|270}}",
            "70 / 110 / 150 / 190 / 230 / 270",
        ),
        ("{{ap|80|140|200|360}}", "80 / 140 / 200 / 360"),
        // Implicit 5-point range (no count) defaults to five steps.
        ("{{ap|0.5 to 2.5}}", "0.5 / 1 / 1.5 / 2 / 2.5"),
        // Explicit `round=` named parameter.
        ("{{ap|10 to 30|round=1}}", "10 / 15 / 20 / 25 / 30"),
        // Arithmetic on both endpoints (Maximum Bonus Armor pattern).
        ("{{ap|7*8 to 19*8}}", "56 / 80 / 104 / 128 / 152"),
    ]);
}

#[test]
fn differential_pp_more_forms() {
    assert_table(&[
        // `;`-list with `key=%` per-value unit.
        (
            "{{pp|10;30;70;125|key=%}}",
            "10% / 30% / 70% / 125% (based on level)",
        ),
        ("{{pp|3 to 4.5|key=%}}", "3% – 4.5% (based on level)"),
        // `start to end for N` counted range.
        (
            "{{pp|12 to 24 for 4}}",
            "12 / 16 / 20 / 24 (based on level)",
        ),
    ]);
}

#[test]
fn differential_expr_and_conditionals_more() {
    assert_table(&[
        ("{{#expr:10 / 4 round 0}}", "3"),
        ("{{#expr:-5 + 3}}", "-2"),
        // `#switch` on a numeric key, and a present `#default=` fallback.
        ("{{#switch:5|4=four|5=five|other}}", "five"),
        ("{{#switch:B|a=1|b=2|#default=def}}", "def"),
        // `#ifeq` trims whitespace before comparing.
        ("{{#ifeq: 5 |5|eq|ne}}", "eq"),
        // Whitespace-only condition is falsey.
        ("{{#if: |t|f}}", "f"),
    ]);
}

#[test]
fn nesting_tt_wraps_ap_progression() {
    // {{tt|{{ap|180|160|140}}}} — a tooltip wrapping a progression passes the
    // expanded value straight through.
    assert_eq!(render("{{tt|{{ap|180|160|140}}}}"), "180 / 160 / 140");
}

#[test]
fn contract_more_inline_helpers() {
    // {{NumberSup|N}} is an ordinal, not a superscript digit.
    assert_eq!(expand_one("NumberSup|14"), "14th");
    // {{degrees}} is the degree sign (alias of {{degree}}).
    assert_eq!(expand_one("degrees"), "°");
    // {{color|name|text}} drops the color and keeps the (bold) content markup,
    // which the markdown layer renders downstream.
    assert_eq!(expand_one("color|yellow|'''Steal'''"), "'''Steal'''");
    // {{sbc}} uppercases the first positional; extra args are ignored.
    assert_eq!(expand_one("sbc|Singed|Surfer"), "**SINGED**");
    // {{rd}} also handles percent-suffixed melee/ranged values.
    assert_eq!(expand_one("rd|40%|20%"), "40% (melee) / 20% (ranged)");
}

// ---------------------------------------------------------------------------
// Data-lookup combinations with inline module fixtures
// ---------------------------------------------------------------------------

/// Build a ConversionContext over a temp export dir carrying just enough module
/// data for the `ccd`/`cid` cases below.
fn fixture_ctx() -> (tempfile::TempDir, Arc<ConversionContext>) {
    let td = tempdir().unwrap();
    let export = td.path().join("export_out");
    std::fs::create_dir_all(&export).unwrap();
    // Talon carries as_base directly; crit_base is absent so the getter's
    // `or 200` fallback (mirrored from the dumped module) must apply.
    std::fs::write(
        export.join("Module%3AChampionData%2Fdata.txt"),
        r#"return {
  ["Talon"] = { stats = { as_base = 0.625 } },
  ["Hecarim"] = { stats = { } }
}"#,
    )
    .unwrap();
    std::fs::write(
        export.join("Module%3AChampionData%2Fgetter.txt"),
        "function p.crit_base(champname)\n\treturn getData(champname, true).crit_base or 200\nend\n",
    )
    .unwrap();
    std::fs::write(
        export.join("Module%3AItemData%2Fdata.txt"),
        r#"return {
  ["Seraph's Embrace"] = { ["stats"] = { ["mana"] = 1000 } },
  ["Infinity Edge"] = { ["stats"] = { ["critdamage"] = 30 } }
}"#,
    )
    .unwrap();
    let ctx = Arc::new(ConversionContext::new(td.path(), 2).unwrap());
    (td, ctx)
}

fn render_ctx(text: &str, ctx: Arc<ConversionContext>) -> String {
    let reg = TemplateRegistry::new();
    let vars = HashMap::new();
    expand_inline_templates(text, 2, &vars, &reg, Some(ctx)).unwrap()
}

#[test]
fn ccd_and_cid_resolve_from_modules() {
    let (_td, ctx) = fixture_ctx();
    // Direct stat lookups.
    assert_eq!(render_ctx("{{ccd|Talon|as_base}}", ctx.clone()), "0.625");
    // Missing field falls back to the getter's `or 200` default (wiki-faithful).
    assert_eq!(render_ctx("{{ccd|Hecarim|crit_base}}", ctx.clone()), "200");
    // Item-data lookups.
    assert_eq!(
        render_ctx("{{cid|Seraph's Embrace|mana}}", ctx.clone()),
        "1000"
    );
    assert_eq!(
        render_ctx("{{cid|Infinity Edge|critdamage}}", ctx.clone()),
        "30"
    );
}

#[test]
fn ccd_nested_inside_ap_arithmetic() {
    // Real Talon invocation: {{ap|(1/{{ccd|Talon|as_base}})*{{ccd|...|windup}}|round=3}}
    // Here we substitute an explicit second factor to keep the fixture minimal
    // while still exercising data-lookup-inside-arithmetic-inside-ap nesting.
    // (1 / 0.625) * 0.5 = 0.8.
    let (_td, ctx) = fixture_ctx();
    assert_eq!(
        render_ctx("{{ap|(1/{{ccd|Talon|as_base}})*0.5|round=3}}", ctx),
        "0.8"
    );
}

// ---------------------------------------------------------------------------
// Regression tests for divergences that were mined as #[ignore]d failures and
// have since been FIXED. Each asserts the wiki-correct value (verified against
// rendered HTML) and now passes. Issue references are in
// scripts/test-mining/DIVERGENCES.md.
// ---------------------------------------------------------------------------

mod fixed_divergences {
    use super::render;

    /// #13 — `#switch` honors a bare trailing parameter as the default.
    #[test]
    fn switch_bare_default() {
        assert_eq!(render("{{#switch:z|a=Apple|b=Banana|Default}}"), "Default");
    }

    /// #14 — `#expr` supports `mod`, comparison, boolean, and `sqrt`/`trunc`.
    #[test]
    fn expr_extended_operators() {
        assert_eq!(render("{{#expr:10 mod 3}}"), "1");
        assert_eq!(render("{{#expr:5 > 3}}"), "1");
        assert_eq!(render("{{#expr:5 < 3}}"), "0");
        assert_eq!(render("{{#expr:1 and 0}}"), "0");
        assert_eq!(render("{{#expr:1 or 0}}"), "1");
        assert_eq!(render("{{#expr:not 0}}"), "1");
        assert_eq!(render("{{#expr:sqrt 16}}"), "4");
        assert_eq!(render("{{#expr:trunc 7.9}}"), "7");
    }

    /// #14 — `^` is left-associative (`2^3^2 = (2^3)^2 = 64`).
    #[test]
    fn exponent_is_left_associative() {
        assert_eq!(render("{{#expr:2^3^2}}"), "64");
    }

    /// #15 — string-casing magic words.
    #[test]
    fn case_magic_words() {
        assert_eq!(render("{{lc:Hello}}"), "hello");
        assert_eq!(render("{{uc:hello}}"), "HELLO");
        assert_eq!(render("{{ucfirst:hello}}"), "Hello");
        assert_eq!(render("{{lcfirst:Hello}}"), "hello");
        // The pipe form is a different (label) template and is unaffected.
        assert_eq!(render("{{lc|Calibrum}}"), "**Calibrum:**");
    }

    /// #23 — `#titleparts:` and `formatnum:` magic words.
    #[test]
    fn other_magic_words() {
        assert_eq!(render("{{#titleparts:A/B/C|1}}"), "A");
        assert_eq!(render("{{#titleparts:A/B/C|1|2}}"), "B");
        assert_eq!(render("{{formatnum:12345.678}}"), "12,345.678");
    }

    /// #16 — `#replace:`.
    #[test]
    fn replace_parser_function() {
        assert_eq!(
            render("{{#replace:hello world|world|there}}"),
            "hello there"
        );
    }

    /// #19 — `{{Divided by}}` renders the division sign inline (it ignores its
    /// args), so `(24{{divided by}}n)` is `(24 ÷ n)`, not the garbled `(24n)`.
    #[test]
    fn divided_by_is_the_division_sign() {
        assert_eq!(render("(24{{divided by}}n)"), "(24 ÷ n)");
    }

    /// #20 — `fd` uses only arg 1, ignoring extras and preserving trailing zeros.
    #[test]
    fn fd_uses_only_first_arg() {
        assert_eq!(render("{{fd|500|750|1000}}"), "500");
        assert_eq!(render("{{fd|0.6666666|2}}"), "0.6666666");
        assert_eq!(render("{{fd|1.30}}"), "1.30");
    }

    /// #21 — `{{times}}` is the multiplication sign `×`.
    #[test]
    fn times_is_the_multiplication_sign() {
        assert_eq!(render("{{times}}"), "×");
    }

    /// #24 — bare `#expr` keeps full precision; an explicit `round N` is honored.
    #[test]
    fn expr_keeps_full_precision() {
        assert_eq!(render("{{#expr:0.02*0.6}}"), "0.012");
        assert_eq!(render("{{#expr:(345*0.24)*0.12}}"), "9.936");
        assert_eq!(render("{{#expr:700/2200}}"), "0.31818181818182");
        assert_eq!(render("{{#expr:1/3 round 10}}"), "0.3333333333");
        assert_eq!(render("{{fd|{{#expr:(100/3) round 4}}}}"), "33.3333");
    }

    /// #27 — `#if`/`#ifeq`/`#switch` expand nested templates in their
    /// condition/compared value/key before evaluating.
    #[test]
    fn parser_functions_expand_their_condition() {
        assert_eq!(render("{{#if:{{#var:undefined}}|set|unset}}"), "unset");
        assert_eq!(render("{{#ifeq:{{#expr:2+2}}|4|correct|wrong}}"), "correct");
        assert_eq!(
            render("{{#switch:{{#expr:2+3}}|5=five|6=six|other}}"),
            "five"
        );
    }

    /// #25 — `{{ccs|text|type}}` renders its text (it no longer drops content).
    #[test]
    fn ccs_keeps_its_text() {
        assert_eq!(
            render("{{ccs|60% increased damage|physical}}"),
            "60% increased damage"
        );
    }

    /// #22 — `{{lethality|N}}` renders the full armor-penetration clause.
    #[test]
    fn lethality_includes_armor_penetration_scaling() {
        assert_eq!(
            render("{{lethality|10}}"),
            "10 Lethality (6.22 – 10 (based on level) armor penetration)"
        );
    }

    /// #17 — possessive icon variants render the possessive form.
    #[test]
    fn possessive_icon_variants() {
        assert_eq!(render("{{cis|Aatrox}}"), "Aatrox's");
        assert_eq!(render("{{uis|Tibbers}}"), "Tibbers'");
    }
}

// ---------------------------------------------------------------------------
// Still-open divergences: assert the WIKI-CORRECT value, #[ignore]d so the suite
// stays green. Run with `cargo test -- --ignored`; un-ignore once fixed.
// ---------------------------------------------------------------------------

mod known_divergences {
    use super::render;

    /// When an `ap`/`pp` range's endpoints are themselves templates, the range
    /// is not re-parsed after expansion: the converter code-fences the literal
    /// (`` `112 to 434 6` ``) instead of interpolating. Wiki expands first, then
    /// interpolates. (Single-value nested arithmetic like
    /// `{{ap|(1/{{ccd|..}})*..}}` DOES work — see corpus_coverage; only the
    /// `X to Y` range form with nested-template endpoints breaks.)
    #[test]
    #[ignore = "ap/pp don't reparse a range with nested-template endpoints; see DIVERGENCES.md"]
    fn ap_range_with_nested_template_endpoints() {
        assert_eq!(
            render("{{ap|{{#expr:80*1.4}} to {{#expr:310*1.4}} 6}}"),
            "112 / 176.4 / 240.8 / 305.2 / 369.6 / 434"
        );
    }
}
