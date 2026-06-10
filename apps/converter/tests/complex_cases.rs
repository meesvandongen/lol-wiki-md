//! Artificial adversarial tests — deep nesting, compound formulas, variable
//! threading, and whitespace edges that could plausibly break expansion.
//!
//! These are hand-crafted (not mined), but every expected value was still
//! verified against the wiki's own rendered HTML (`api.php?action=parse`) so
//! the assertions are ground truth, not enshrined converter behavior. Cases the
//! converter does NOT reproduce are recorded instead in
//! `template_combinations.rs::known_divergences` (see `scripts/test-mining/DIVERGENCES.md`).

use lol_wiki_md::convert::util::expand_inline_templates;
use lol_wiki_md::parse::templates::TemplateRegistry;
use std::collections::HashMap;

/// Expand through the full inline pipeline (handles nesting + the fixed-point
/// iteration), with no champion-data context.
fn render(text: &str) -> String {
    let reg = TemplateRegistry::new();
    let vars = HashMap::new();
    expand_inline_templates(text, 2, &vars, &reg, None).expect("expansion should succeed")
}

fn check(cases: &[(&str, &str)]) {
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
        "{} adversarial case(s) diverged:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

// ---------------------------------------------------------------------------
// Compound `#expr` formulas: precedence, parens, unary minus, magnitudes.
// ---------------------------------------------------------------------------

#[test]
fn expr_precedence_and_parentheses() {
    check(&[
        ("{{#expr:((1+2)*(3+4))^2}}", "441"),
        ("{{#expr:(2+3)*(4+5)/(1+2)}}", "15"),
        ("{{#expr: ( 5 + 3 ) * 2 - 1 }}", "15"),
        // Left-to-right for equal precedence.
        ("{{#expr:10 - 2 - 3}}", "5"),
    ]);
}

#[test]
fn expr_unary_minus_and_signs() {
    check(&[("{{#expr:2*-3}}", "-6"), ("{{#expr:-(-5)}}", "5")]);
}

#[test]
fn expr_magnitudes_and_chained_multiply() {
    check(&[
        ("{{#expr:100*0.5*0.5*0.5}}", "12.5"),
        ("{{#expr:1000000*1000000}}", "1000000000000"),
        ("{{#expr:0}}", "0"),
    ]);
}

#[test]
fn expr_functions_and_explicit_round() {
    check(&[
        ("{{#expr:abs(3-9)*ceil(1.1)}}", "12"),
        ("{{#expr:floor(9.9) + ceil(0.1)}}", "10"),
        ("{{#expr:3.14159 round 2}}", "3.14"),
        ("{{#expr:0.6222 * 10 round 2}}", "6.22"),
        // (0.6 + 0.4 * level/18) at level 18 — the lethality / scaling shape.
        ("{{#expr:(0.6 + 0.4 * 18 / 18)}}", "1"),
    ]);
}

// ---------------------------------------------------------------------------
// Deep nesting: templates inside templates inside templates.
// ---------------------------------------------------------------------------

#[test]
fn nesting_fd_wraps_expr_wraps_expr() {
    // fd( expr( expr * expr ) ) — three levels, integer result.
    // (The `fd|{{#expr:... round 4}}` form is a known divergence — the #expr
    // precision bug clobbers the explicit round; see known_divergences.)
    assert_eq!(
        render("{{fd|{{#expr:{{#expr:2+3}}*{{#expr:10/2}}}}}}"),
        "25"
    );
}

#[test]
fn nesting_value_template_inside_switch_branch() {
    // The selected #switch branch is itself a value template.
    assert_eq!(
        render("{{#switch:c|a=A|b=B|c={{ap|1|2|3}}|D}}"),
        "1 / 2 / 3"
    );
}

#[test]
fn nesting_value_template_inside_if_branch() {
    assert_eq!(render("{{#if:1|{{fd|{{#expr:7/2}}}}|nope}}"), "3.5");
}

#[test]
fn nesting_four_levels_if_ifeq_expr() {
    // {{#if:{{#ifeq:{{#expr:1+1}}|2|yes|no}}|MATCH|NO}} — four levels deep.
    assert_eq!(
        render("{{#if:{{#ifeq:{{#expr:1+1}}|2|yes|no}}|MATCH|NO}}"),
        "MATCH"
    );
}

// ---------------------------------------------------------------------------
// Variable threading: vardefine / vardefineecho / var across a string.
// ---------------------------------------------------------------------------

#[test]
fn vars_define_then_reference() {
    check(&[
        ("{{#vardefine:base|40}}{{#var:base}}", "40"),
        // vardefineecho both sets AND prints.
        ("{{#vardefineecho:y|9}}-{{#var:y}}", "9-9"),
    ]);
}

#[test]
fn vars_feed_arithmetic_in_ap() {
    // The var value is substituted, then ap evaluates the per-value arithmetic.
    assert_eq!(
        render("{{#vardefine:x|7}}{{ap|{{#var:x}}*2|{{#var:x}}*3}}"),
        "14 / 21"
    );
}

#[test]
fn vars_multiple_in_nested_formula() {
    // Pythagorean: sqrt(3^2 + 4^2) = 5, via two vars and ^0.5.
    assert_eq!(
        render(
            "{{#vardefine:a|3}}{{#vardefine:b|4}}{{fd|{{#expr:({{#var:a}}^2+{{#var:b}}^2)^0.5}}}}"
        ),
        "5"
    );
}

// ---------------------------------------------------------------------------
// Whitespace, empty args, and mixed prose.
// ---------------------------------------------------------------------------

#[test]
fn whitespace_around_args_is_trimmed() {
    check(&[
        ("{{ap| 40 | 50 | 60 }}", "40 / 50 / 60"),
        ("{{#expr: 1 + 2 }}", "3"),
    ]);
}

#[test]
fn empty_condition_is_falsey() {
    assert_eq!(render("{{#if:|empty-true|empty-false}}"), "empty-false");
}

#[test]
fn ap_constant_and_round_param() {
    check(&[
        ("{{ap|100|100|100}}", "100 / 100 / 100"),
        (
            "{{ap|10 to 30}} and {{ap|10 to 30|round=1}}",
            "10 / 15 / 20 / 25 / 30 and 10 / 15 / 20 / 25 / 30",
        ),
    ]);
}

#[test]
fn mixed_prose_with_several_templates() {
    assert_eq!(
        render("Deals {{ap|40|60|80}} (+50% AP) magic damage over {{pp|3 to 5}} seconds."),
        "Deals 40 / 60 / 80 (+50% AP) magic damage over 3 – 5 (based on level) seconds."
    );
}
