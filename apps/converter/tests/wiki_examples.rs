//! AUTO-GENERATED — do not edit by hand.
//!
//! Self-contained differential tests built from real template invocations
//! mined from the wiki corpus. Each test is one real invocation whose
//! expected value was captured from the wiki's own rendered HTML
//! (`api.php?action=parse`) and confirmed to match the converter exactly.
//! Regenerate via the throwaway `_gen.rs` over
//! `validation_reports/test-mining/candidates.tsv` (see
//! `scripts/test-mining/harvest.py`).

#![allow(non_snake_case)]

use lol_wiki_md::convert::util::expand_inline_templates;
use lol_wiki_md::parse::templates::TemplateRegistry;
use std::collections::HashMap;

fn render(text: &str) -> String {
    let reg = TemplateRegistry::new();
    let vars = HashMap::new();
    expand_inline_templates(text, 2, &vars, &reg, None).expect("expansion should succeed")
}

#[test]
fn wiki_expr_001() {
    assert_eq!(render("{{#expr:(1/3)*100 round 2}}"), "33.33");
}

#[test]
fn wiki_expr_002() {
    assert_eq!(render("{{#expr:50*1.35}}"), "67.5");
}

#[test]
fn wiki_expr_003() {
    assert_eq!(render("{{#expr:50*1.25}}"), "62.5");
}

#[test]
fn wiki_expr_004() {
    assert_eq!(render("{{#expr:0.7025 + 0.0175 * (2 - 1)}}"), "0.72");
}

#[test]
fn wiki_expr_005() {
    assert_eq!(render("{{#expr:0.2*30}}"), "6");
}

#[test]
fn wiki_expr_006() {
    assert_eq!(render("{{#expr:0.2*45}}"), "9");
}

#[test]
fn wiki_expr_007() {
    assert_eq!(render("{{#expr:0.2*70}}"), "14");
}

#[test]
fn wiki_expr_008() {
    assert_eq!(render("{{#expr:0.2*275}}"), "55");
}

#[test]
fn wiki_expr_009() {
    assert_eq!(render("{{#expr:0.2*290}}"), "58");
}

#[test]
fn wiki_expr_010() {
    assert_eq!(render("{{#expr:100*(1.5)^1}}"), "150");
}

#[test]
fn wiki_expr_011() {
    assert_eq!(render("{{#expr:100*(1.5)^2}}"), "225");
}

#[test]
fn wiki_expr_012() {
    assert_eq!(render("{{#expr:100*(1.5)^3}}"), "337.5");
}

#[test]
fn wiki_expr_013() {
    assert_eq!(render("{{#expr:100*(1.8)^1}}"), "180");
}

#[test]
fn wiki_expr_014() {
    assert_eq!(render("{{#expr:100*(1.8)^2}}"), "324");
}

#[test]
fn wiki_expr_015() {
    assert_eq!(render("{{#expr:100*(1.8)^3}}"), "583.2");
}

#[test]
fn wiki_expr_016() {
    assert_eq!(render("{{#expr:1*3}}"), "3");
}

#[test]
fn wiki_expr_017() {
    assert_eq!(render("{{#expr:2*3-1}}"), "5");
}

#[test]
fn wiki_expr_018() {
    assert_eq!(render("{{#expr:3*3-1}}"), "8");
}

#[test]
fn wiki_expr_019() {
    assert_eq!(render("{{#expr:4*3-1}}"), "11");
}

#[test]
fn wiki_expr_020() {
    assert_eq!(render("{{#expr:5*3-1}}"), "14");
}

#[test]
fn wiki_expr_021() {
    assert_eq!(render("{{#expr:6*3-1}}"), "17");
}

#[test]
fn wiki_expr_022() {
    assert_eq!(render("{{#expr:1*3*3}}"), "9");
}

#[test]
fn wiki_expr_023() {
    assert_eq!(render("{{#expr:2*3*3-1}}"), "17");
}

#[test]
fn wiki_expr_024() {
    assert_eq!(render("{{#expr:3*3*3-1}}"), "26");
}

#[test]
fn wiki_expr_025() {
    assert_eq!(render("{{#expr:4*3*3-1}}"), "35");
}

#[test]
fn wiki_expr_026() {
    assert_eq!(render("{{#expr:5*3*3-1}}"), "44");
}

#[test]
fn wiki_expr_027() {
    assert_eq!(render("{{#expr:6*3*3-1}}"), "53");
}

#[test]
fn wiki_expr_028() {
    assert_eq!(render("{{#expr:1*3*3*3}}"), "27");
}

#[test]
fn wiki_expr_029() {
    assert_eq!(render("{{#expr:2*3*3*3-1}}"), "53");
}

#[test]
fn wiki_expr_030() {
    assert_eq!(render("{{#expr:3*3*3*3-1}}"), "80");
}

#[test]
fn wiki_expr_031() {
    assert_eq!(render("{{#expr:4*3*3*3-1}}"), "107");
}

#[test]
fn wiki_expr_032() {
    assert_eq!(render("{{#expr:5*3*3*3-1}}"), "134");
}

#[test]
fn wiki_expr_033() {
    assert_eq!(render("{{#expr:6*3*3*3-1}}"), "161");
}

#[test]
fn wiki_expr_034() {
    assert_eq!(render("{{#expr:290/1.7 round 2}}"), "170.59");
}

#[test]
fn wiki_expr_035() {
    assert_eq!(render("{{#expr:290/1.8 round 2}}"), "161.11");
}

#[test]
fn wiki_expr_036() {
    assert_eq!(render("{{#expr:290/1.88 round 2}}"), "154.26");
}

#[test]
fn wiki_expr_037() {
    assert_eq!(render("{{#expr:290/1.98 round 2}}"), "146.46");
}

#[test]
fn wiki_expr_038() {
    assert_eq!(render("{{#expr:290/2.24 round 2}}"), "129.46");
}

#[test]
fn wiki_expr_039() {
    assert_eq!(render("{{#expr:290/2.34 round 2}}"), "123.93");
}

#[test]
fn wiki_expr_040() {
    assert_eq!(render("{{#expr:290/2.42 round 2}}"), "119.83");
}

#[test]
fn wiki_expr_041() {
    assert_eq!(render("{{#expr:290/2.52 round 2}}"), "115.08");
}

#[test]
fn wiki_expr_042() {
    assert_eq!(render("{{#expr:(40/8)*5}}"), "25");
}

#[test]
fn wiki_expr_043() {
    assert_eq!(render("{{#expr:(30/8)*5}}"), "18.75");
}

#[test]
fn wiki_expr_044() {
    assert_eq!(render("{{#expr:30/75 round 3}}"), "0.4");
}

#[test]
fn wiki_expr_045() {
    assert_eq!(render("{{#expr:100*0.55}}"), "55");
}

#[test]
fn wiki_expr_046() {
    assert_eq!(render("{{#expr:100*0.40}}"), "40");
}

#[test]
fn wiki_expr_047() {
    assert_eq!(render("{{#expr:100*0.50}}"), "50");
}

#[test]
fn wiki_expr_048() {
    assert_eq!(render("{{#expr:100*(0.50)^2}}"), "25");
}

#[test]
fn wiki_expr_049() {
    assert_eq!(render("{{#expr:100*(0.55)^2}}"), "30.25");
}

#[test]
fn wiki_expr_050() {
    assert_eq!(render("{{#expr:6/2 round 2}}"), "3");
}

#[test]
fn wiki_expr_051() {
    assert_eq!(render("{{#expr:(6+5)/2 round 2}}"), "5.5");
}

#[test]
fn wiki_expr_052() {
    assert_eq!(render("{{#expr:2*6-3*2+5 round 2}}"), "11");
}

#[test]
fn wiki_expr_053() {
    assert_eq!(render("{{#expr:(2*6+5)/(3*2) round 2}}"), "2.83");
}

#[test]
fn wiki_expr_054() {
    assert_eq!(render("{{#expr:2/6 round 2}}"), "0.33");
}

#[test]
fn wiki_expr_055() {
    assert_eq!(render("{{#expr:(2+8)/6 round 2}}"), "1.67");
}

#[test]
fn wiki_expr_056() {
    assert_eq!(render("{{#expr:2*2-3*6+8 round 2}}"), "-6");
}

#[test]
fn wiki_expr_057() {
    assert_eq!(render("{{#expr:(2*2+8)/(3*6) round 2}}"), "0.67");
}

#[test]
fn wiki_expr_058() {
    assert_eq!(render("{{#expr:4/2 round 2}}"), "2");
}

#[test]
fn wiki_expr_059() {
    assert_eq!(render("{{#expr:(4+17)/2 round 2}}"), "10.5");
}

#[test]
fn wiki_expr_060() {
    assert_eq!(render("{{#expr:2*4-3*2+17 round 2}}"), "19");
}

#[test]
fn wiki_expr_061() {
    assert_eq!(render("{{#expr:(2*4+17)/(3*5) round 2}}"), "1.67");
}

#[test]
fn wiki_expr_062() {
    assert_eq!(render("{{#expr:175*(1-0.14)}}"), "150.5");
}

#[test]
fn wiki_expr_063() {
    assert_eq!(render("{{#expr:200*(1-0.25)}}"), "150");
}

#[test]
fn wiki_expr_064() {
    assert_eq!(render("{{#expr:75+(40*5)}}"), "275");
}

#[test]
fn wiki_expr_065() {
    assert_eq!(render("{{#expr:200*10/11 round 2}}"), "181.82");
}

#[test]
fn wiki_expr_066() {
    assert_eq!(render("{{#expr:400*10/11 round 2}}"), "363.64");
}

#[test]
fn wiki_expr_067() {
    assert_eq!(render("{{#expr:300*10/11 round 2}}"), "272.73");
}

#[test]
fn wiki_expr_068() {
    assert_eq!(render("{{#expr:70*(1+0.1*4)}}"), "98");
}

#[test]
fn wiki_expr_069() {
    assert_eq!(render("{{#expr:2*1}}"), "2");
}

#[test]
fn wiki_expr_070() {
    assert_eq!(render("{{#expr:2*2}}"), "4");
}

#[test]
fn wiki_expr_071() {
    assert_eq!(render("{{#expr:2*3}}"), "6");
}

#[test]
fn wiki_expr_072() {
    assert_eq!(render("{{#expr:2*4}}"), "8");
}

#[test]
fn wiki_expr_073() {
    assert_eq!(render("{{#expr:2*5}}"), "10");
}

#[test]
fn wiki_expr_074() {
    assert_eq!(render("{{#expr:200+30}}"), "230");
}

#[test]
fn wiki_expr_075() {
    assert_eq!(render("{{#expr:4*1}}"), "4");
}

#[test]
fn wiki_expr_076() {
    assert_eq!(render("{{#expr:4*2}}"), "8");
}

#[test]
fn wiki_expr_077() {
    assert_eq!(render("{{#expr:4*3}}"), "12");
}

#[test]
fn wiki_expr_078() {
    assert_eq!(render("{{#expr:4*4}}"), "16");
}

#[test]
fn wiki_expr_079() {
    assert_eq!(render("{{#expr:4*5}}"), "20");
}

#[test]
fn wiki_expr_080() {
    assert_eq!(render("{{#expr:4*6}}"), "24");
}

#[test]
fn wiki_expr_081() {
    assert_eq!(render("{{#expr:4*7}}"), "28");
}

#[test]
fn wiki_expr_082() {
    assert_eq!(render("{{#expr:4*8}}"), "32");
}

#[test]
fn wiki_expr_083() {
    assert_eq!(render("{{#expr:4*9}}"), "36");
}

#[test]
fn wiki_expr_084() {
    assert_eq!(render("{{#expr:4*10}}"), "40");
}

#[test]
fn wiki_expr_085() {
    assert_eq!(render("{{#expr:4*11}}"), "44");
}

#[test]
fn wiki_expr_086() {
    assert_eq!(render("{{#expr:4*12}}"), "48");
}

#[test]
fn wiki_expr_087() {
    assert_eq!(render("{{#expr:7*1}}"), "7");
}

#[test]
fn wiki_expr_088() {
    assert_eq!(render("{{#expr:7*2}}"), "14");
}

#[test]
fn wiki_expr_089() {
    assert_eq!(render("{{#expr:7*3}}"), "21");
}

#[test]
fn wiki_expr_090() {
    assert_eq!(render("{{#expr:7*4}}"), "28");
}

#[test]
fn wiki_expr_091() {
    assert_eq!(render("{{#expr:7*5}}"), "35");
}

#[test]
fn wiki_expr_092() {
    assert_eq!(render("{{#expr:7*6}}"), "42");
}

#[test]
fn wiki_expr_093() {
    assert_eq!(render("{{#expr:7*7}}"), "49");
}

#[test]
fn wiki_expr_094() {
    assert_eq!(render("{{#expr:7*8}}"), "56");
}

#[test]
fn wiki_expr_095() {
    assert_eq!(render("{{#expr:7*9}}"), "63");
}

#[test]
fn wiki_expr_096() {
    assert_eq!(render("{{#expr:7*10}}"), "70");
}

#[test]
fn wiki_expr_097() {
    assert_eq!(render("{{#expr:11*11}}"), "121");
}

#[test]
fn wiki_expr_098() {
    assert_eq!(render("{{#expr:7*12}}"), "84");
}

#[test]
fn wiki_expr_099() {
    assert_eq!(render("{{#expr:10*1}}"), "10");
}

#[test]
fn wiki_expr_100() {
    assert_eq!(render("{{#expr:10*2}}"), "20");
}

#[test]
fn wiki_expr_101() {
    assert_eq!(render("{{#expr:10*3}}"), "30");
}

#[test]
fn wiki_expr_102() {
    assert_eq!(render("{{#expr:10*4}}"), "40");
}

#[test]
fn wiki_expr_103() {
    assert_eq!(render("{{#expr:10*5}}"), "50");
}

#[test]
fn wiki_as_001() {
    assert_eq!(render("{{as|ability power}}"), "ability power");
}

#[test]
fn wiki_as_002() {
    assert_eq!(render("{{as|{{fd|2.5}} attack speed}}"), "2.5 attack speed");
}

#[test]
fn wiki_as_003() {
    assert_eq!(
        render("{{as|1% critical strike chance}}"),
        "1% critical strike chance"
    );
}

#[test]
fn wiki_as_004() {
    assert_eq!(
        render("{{as|100% critical strike chance}}"),
        "100% critical strike chance"
    );
}

#[test]
fn wiki_as_005() {
    assert_eq!(
        render("{{as|5% armor penetration}}"),
        "5% armor penetration"
    );
}

#[test]
fn wiki_as_006() {
    assert_eq!(
        render("{{as|6% magic penetration}}"),
        "6% magic penetration"
    );
}

#[test]
fn wiki_as_007() {
    assert_eq!(render("{{as|(+ 10% AP)}}"), "(+ 10% AP)");
}

#[test]
fn wiki_as_008() {
    assert_eq!(render("{{as|{{fd|2.5}}% AP}}"), "2.5% AP");
}

#[test]
fn wiki_as_009() {
    assert_eq!(render("{{as|15% AP}}"), "15% AP");
}

#[test]
fn wiki_as_010() {
    assert_eq!(render("{{as|4%|AD}}"), "4%");
}

#[test]
fn wiki_as_011() {
    assert_eq!(render("{{as|24%|AD}}"), "24%");
}

#[test]
fn wiki_as_012() {
    assert_eq!(render("{{as|4% AP}}"), "4% AP");
}

#[test]
fn wiki_as_013() {
    assert_eq!(render("{{as|24% AP}}"), "24% AP");
}

#[test]
fn wiki_as_014() {
    assert_eq!(render("{{as|(+ {{fd|7.5}}% AP)}}"), "(+ 7.5% AP)");
}

#[test]
fn wiki_as_015() {
    assert_eq!(render("{{as|80% AP}}"), "80% AP");
}

#[test]
fn wiki_as_016() {
    assert_eq!(render("{{as|70% AP}}"), "70% AP");
}

#[test]
fn wiki_as_017() {
    assert_eq!(render("{{as|75% AP}}"), "75% AP");
}

#[test]
fn wiki_as_018() {
    assert_eq!(render("{{as|85% AP}}"), "85% AP");
}

#[test]
fn wiki_as_019() {
    assert_eq!(render("{{as|40% AP}}"), "40% AP");
}

#[test]
fn wiki_as_020() {
    assert_eq!(render("{{as|45% AP}}"), "45% AP");
}

#[test]
fn wiki_as_021() {
    assert_eq!(render("{{as|{{ap|40*2}}% AP}}"), "80% AP");
}

#[test]
fn wiki_fd_001() {
    assert_eq!(render("{{fd|2.5}}"), "2.5");
}

#[test]
fn wiki_fd_002() {
    assert_eq!(render("{{fd|0.15}}"), "0.15");
}

#[test]
fn wiki_fd_003() {
    assert_eq!(render("{{fd|1.5}}"), "1.5");
}

#[test]
fn wiki_fd_004() {
    assert_eq!(render("{{fd|2.25}}"), "2.25");
}

#[test]
fn wiki_fd_005() {
    assert_eq!(render("{{fd|7.5}}"), "7.5");
}

#[test]
fn wiki_fd_006() {
    assert_eq!(render("{{fd|4.2}}"), "4.2");
}

#[test]
fn wiki_fd_007() {
    assert_eq!(render("{{fd|4.7}}"), "4.7");
}

#[test]
fn wiki_fd_008() {
    assert_eq!(render("{{fd|0.05}}"), "0.05");
}

#[test]
fn wiki_fd_009() {
    assert_eq!(render("{{fd|0.25}}"), "0.25");
}

#[test]
fn wiki_fd_010() {
    assert_eq!(render("{{fd|0.4}}"), "0.4");
}

#[test]
fn wiki_fd_011() {
    assert_eq!(render("{{fd|0.625}}"), "0.625");
}

#[test]
fn wiki_fd_012() {
    assert_eq!(render("{{fd|2.35}}"), "2.35");
}

#[test]
fn wiki_fd_013() {
    assert_eq!(render("{{fd|2.2}}"), "2.2");
}

#[test]
fn wiki_fd_014() {
    assert_eq!(render("{{fd|2.7}}"), "2.7");
}

#[test]
fn wiki_fd_015() {
    assert_eq!(render("{{fd|0.658}}"), "0.658");
}

#[test]
fn wiki_fd_016() {
    assert_eq!(render("{{fd|1.4}}"), "1.4");
}

#[test]
fn wiki_fd_017() {
    assert_eq!(render("{{fd|3.5}}"), "3.5");
}

#[test]
fn wiki_fd_018() {
    assert_eq!(render("{{fd|0.8}}"), "0.8");
}

#[test]
fn wiki_fd_019() {
    assert_eq!(render("{{fd|1.3}}"), "1.3");
}

#[test]
fn wiki_fd_020() {
    assert_eq!(render("{{fd|0.5}}"), "0.5");
}

#[test]
fn wiki_fd_021() {
    assert_eq!(render("{{fd|54.88}}"), "54.88");
}

#[test]
fn wiki_fd_022() {
    assert_eq!(render("{{fd|21.88}}"), "21.88");
}

#[test]
fn wiki_fd_023() {
    assert_eq!(render("{{fd|0.2}}"), "0.2");
}

#[test]
fn wiki_fd_024() {
    assert_eq!(render("{{fd|0.85}}"), "0.85");
}

#[test]
fn wiki_fd_025() {
    assert_eq!(render("{{fd|7.4}}"), "7.4");
}

#[test]
fn wiki_fd_026() {
    assert_eq!(render("{{fd|8.5}}"), "8.5");
}

#[test]
fn wiki_fd_027() {
    assert_eq!(render("{{fd|0.55}}"), "0.55");
}

#[test]
fn wiki_fd_028() {
    assert_eq!(render("{{fd|0.1}}"), "0.1");
}

#[test]
fn wiki_fd_029() {
    assert_eq!(render("{{fd|97.5}}"), "97.5");
}

#[test]
fn wiki_fd_030() {
    assert_eq!(render("{{fd|7.05}}"), "7.05");
}

#[test]
fn wiki_fd_031() {
    assert_eq!(render("{{fd|5.05}}"), "5.05");
}

#[test]
fn wiki_fd_032() {
    assert_eq!(render("{{fd|3.1}}"), "3.1");
}

#[test]
fn wiki_fd_033() {
    assert_eq!(render("{{fd|2.9}}"), "2.9");
}

#[test]
fn wiki_fd_034() {
    assert_eq!(render("{{fd|1.125}}"), "1.125");
}

#[test]
fn wiki_fd_035() {
    assert_eq!(render("{{fd|1.75}}"), "1.75");
}

#[test]
fn wiki_fd_036() {
    assert_eq!(render("{{fd|0.75}}"), "0.75");
}

#[test]
fn wiki_fd_037() {
    assert_eq!(render("{{fd|32.1}}"), "32.1");
}

#[test]
fn wiki_fd_038() {
    assert_eq!(render("{{fd|1.25}}"), "1.25");
}

#[test]
fn wiki_fd_039() {
    assert_eq!(render("{{fd|2.05}}"), "2.05");
}

#[test]
fn wiki_fd_040() {
    assert_eq!(render("{{fd|315.6}}"), "315.6");
}

#[test]
fn wiki_fd_041() {
    assert_eq!(render("{{fd|8.11}}"), "8.11");
}

#[test]
fn wiki_fd_042() {
    assert_eq!(render("{{fd|8.108}}"), "8.108");
}

#[test]
fn wiki_fd_043() {
    assert_eq!(render("{{fd|531.2}}"), "531.2");
}

#[test]
fn wiki_fd_044() {
    assert_eq!(render("{{fd|9.824}}"), "9.824");
}

#[test]
fn wiki_fd_045() {
    assert_eq!(render("{{fd|53.88}}"), "53.88");
}

#[test]
fn wiki_fd_046() {
    assert_eq!(render("{{fd|0.667}}"), "0.667");
}

#[test]
fn wiki_fd_047() {
    assert_eq!(render("{{fd|2.1}}"), "2.1");
}

#[test]
fn wiki_fd_048() {
    assert_eq!(render("{{fd|3.6}}"), "3.6");
}

#[test]
fn wiki_fd_049() {
    assert_eq!(render("{{fd|26.5}}"), "26.5");
}

#[test]
fn wiki_fd_050() {
    assert_eq!(render("{{fd|3.8}}"), "3.8");
}

#[test]
fn wiki_fd_051() {
    assert_eq!(render("{{fd|265.6}}"), "265.6");
}

#[test]
fn wiki_fd_052() {
    assert_eq!(render("{{fd|30.25}}"), "30.25");
}

#[test]
fn wiki_fd_053() {
    assert_eq!(render("{{fd|26.72}}"), "26.72");
}

#[test]
fn wiki_fd_054() {
    assert_eq!(render("{{fd|3.75}}"), "3.75");
}

#[test]
fn wiki_fd_055() {
    assert_eq!(render("{{fd|3.3}}"), "3.3");
}

#[test]
fn wiki_fd_056() {
    assert_eq!(render("{{fd|12.5}}"), "12.5");
}

#[test]
fn wiki_fd_057() {
    assert_eq!(render("{{fd|14.75}}"), "14.75");
}

#[test]
fn wiki_fd_058() {
    assert_eq!(render("{{fd|3.85}}"), "3.85");
}

#[test]
fn wiki_fd_059() {
    assert_eq!(render("{{fd|0.7}}"), "0.7");
}

#[test]
fn wiki_fd_060() {
    assert_eq!(render("{{fd|1.04}}"), "1.04");
}

#[test]
fn wiki_fd_061() {
    assert_eq!(render("{{fd|1.352}}"), "1.352");
}

#[test]
fn wiki_fd_062() {
    assert_eq!(render("{{fd|35.2}}"), "35.2");
}

#[test]
fn wiki_fd_063() {
    assert_eq!(render("{{fd|5.2}}"), "5.2");
}

#[test]
fn wiki_fd_064() {
    assert_eq!(render("{{fd|2.55}}"), "2.55");
}

#[test]
fn wiki_fd_065() {
    assert_eq!(render("{{fd|0.9}}"), "0.9");
}

#[test]
fn wiki_fd_066() {
    assert_eq!(render("{{fd|36.1}}"), "36.1");
}

#[test]
fn wiki_fd_067() {
    assert_eq!(render("{{fd|0.35}}"), "0.35");
}

#[test]
fn wiki_fd_068() {
    assert_eq!(render("{{fd|0.6}}"), "0.6");
}

#[test]
fn wiki_fd_069() {
    assert_eq!(render("{{fd|9.5}}"), "9.5");
}

#[test]
fn wiki_fd_070() {
    assert_eq!(render("{{fd|55.04}}"), "55.04");
}

#[test]
fn wiki_fd_071() {
    assert_eq!(render("{{fd|75.92}}"), "75.92");
}

#[test]
fn wiki_fd_072() {
    assert_eq!(render("{{fd|49.5}}"), "49.5");
}

#[test]
fn wiki_fd_073() {
    assert_eq!(render("{{fd|8.8}}"), "8.8");
}

#[test]
fn wiki_fd_074() {
    assert_eq!(render("{{fd|5.5}}"), "5.5");
}

#[test]
fn wiki_fd_075() {
    assert_eq!(render("{{fd|23.1}}"), "23.1");
}

#[test]
fn wiki_fd_076() {
    assert_eq!(render("{{fd|38.5}}"), "38.5");
}

#[test]
fn wiki_fd_077() {
    assert_eq!(render("{{fd|2.75}}"), "2.75");
}

#[test]
fn wiki_fd_078() {
    assert_eq!(render("{{fd|19.8}}"), "19.8");
}

#[test]
fn wiki_fd_079() {
    assert_eq!(render("{{fd|6.6}}"), "6.6");
}

#[test]
fn wiki_fd_080() {
    assert_eq!(render("{{fd|7.7}}"), "7.7");
}

#[test]
fn wiki_fd_081() {
    assert_eq!(render("{{fd|1.65}}"), "1.65");
}

#[test]
fn wiki_fd_082() {
    assert_eq!(render("{{fd|1.1}}"), "1.1");
}

#[test]
fn wiki_fd_083() {
    assert_eq!(render("{{fd|4.4}}"), "4.4");
}

#[test]
fn wiki_fd_084() {
    assert_eq!(render("{{fd|3.95}}"), "3.95");
}

#[test]
fn wiki_fd_085() {
    assert_eq!(render("{{fd|25.33}}"), "25.33");
}

#[test]
fn wiki_fd_086() {
    assert_eq!(render("{{fd|102.9}}"), "102.9");
}

#[test]
fn wiki_fd_087() {
    assert_eq!(render("{{fd|27.5}}"), "27.5");
}

#[test]
fn wiki_fd_088() {
    assert_eq!(render("{{fd|6.4}}"), "6.4");
}

#[test]
fn wiki_fd_089() {
    assert_eq!(render("{{fd|1.6}}"), "1.6");
}

#[test]
fn wiki_fd_090() {
    assert_eq!(render("{{fd|1.8}}"), "1.8");
}

#[test]
fn wiki_fd_091() {
    assert_eq!(render("{{fd|1.2}}"), "1.2");
}

#[test]
fn wiki_fd_092() {
    assert_eq!(render("{{fd|0.3}}"), "0.3");
}

#[test]
fn wiki_fd_093() {
    assert_eq!(render("{{fd|15.5}}"), "15.5");
}

#[test]
fn wiki_sti_001() {
    assert_eq!(
        render("{{sti|15% magic penetration.|link=true}}"),
        "15% magic penetration."
    );
}

#[test]
fn wiki_sti_002() {
    assert_eq!(render("{{sti|shield}}"), "shield");
}

#[test]
fn wiki_sti_003() {
    assert_eq!(render("{{sti|ms|{{as|move speed}}}}"), "move speed");
}

#[test]
fn wiki_sti_004() {
    assert_eq!(render("{{sti|AP}}"), "AP");
}

#[test]
fn wiki_sti_005() {
    assert_eq!(render("{{sti|AD}}"), "AD");
}

#[test]
fn wiki_sti_006() {
    assert_eq!(render("{{sti|Range|31 range}}"), "31 range");
}

#[test]
fn wiki_sti_007() {
    assert_eq!(render("{{sti|Range|41 range}}"), "41 range");
}

#[test]
fn wiki_sti_008() {
    assert_eq!(render("{{sti|Range|181 range}}"), "181 range");
}

#[test]
fn wiki_sti_009() {
    assert_eq!(render("{{sti|Attack Speed}}"), "Attack Speed");
}

#[test]
fn wiki_sti_010() {
    assert_eq!(
        render("{{sti|Critical Strike Chance}}"),
        "Critical Strike Chance"
    );
}

#[test]
fn wiki_sti_011() {
    assert_eq!(render("{{sti|Life Steal}}"), "Life Steal");
}

#[test]
fn wiki_sti_012() {
    assert_eq!(render("{{sti|shields}}"), "shields");
}

#[test]
fn wiki_sti_013() {
    assert_eq!(render("{{sti|cdr|reduces}}"), "reduces");
}

#[test]
fn wiki_sti_014() {
    assert_eq!(render("{{sti|cdr|reducing}}"), "reducing");
}

#[test]
fn wiki_sti_015() {
    assert_eq!(
        render("{{sti|Heal and Shield Power}}"),
        "Heal and Shield Power"
    );
}

#[test]
fn wiki_sti_016() {
    assert_eq!(
        render("{{sti|5% Heal and shield power}}"),
        "5% Heal and shield power"
    );
}

#[test]
fn wiki_pp_001() {
    assert_eq!(
        render("{{pp|key=%|6.5 to 15}}"),
        "6.5% – 15% (based on level)"
    );
}

#[test]
fn wiki_pp_002() {
    assert_eq!(render("{{pp|10 to 146}}"), "10 – 146 (based on level)");
}

#[test]
fn wiki_pp_003() {
    assert_eq!(
        render("{{pp|65 to 310 for 15}}"),
        "65 – 310 (based on level)"
    );
}

#[test]
fn wiki_pp_004() {
    assert_eq!(render("{{pp|15 to 150}}"), "15 – 150 (based on level)");
}

#[test]
fn wiki_pp_005() {
    assert_eq!(render("{{pp|15 to 75}}"), "15 – 75 (based on level)");
}

#[test]
fn wiki_pp_006() {
    assert_eq!(
        render("{{pp|15;19;22;26;29;33;36;40;43;47;50;54;57;61;64;68;71;75}}"),
        "15 – 75 (based on level)"
    );
}

#[test]
fn wiki_pp_007() {
    assert_eq!(
        render("{{pp|6;5;4;3|1;6;11;16}}"),
        "6 / 5 / 4 / 3 (based on level)"
    );
}

#[test]
fn wiki_pp_008() {
    assert_eq!(render("{{pp|80+20*x}}"), "100 – 440 (based on level)");
}

#[test]
fn wiki_pp_009() {
    assert_eq!(
        render("{{pp|10;30;70;125|key=%|2;4;6;8|label1=Hellions|type=the number of Hellions}}"),
        "10% / 30% / 70% / 125% (based on the number of Hellions)"
    );
}

#[test]
fn wiki_pp_010() {
    assert_eq!(
        render("{{pp|10;30;80;140|key=%|2;4;6;8|label1=Hellions|showtype=false}}"),
        "10% / 30% / 80% / 140%"
    );
}

#[test]
fn wiki_pp_011() {
    assert_eq!(
        render("{{pp|10;30;80;140|key=%|2;4;6;8|label1=Hellions|type=the number of Hellions}}"),
        "10% / 30% / 80% / 140% (based on the number of Hellions)"
    );
}

#[test]
fn wiki_pp_012() {
    assert_eq!(
        render("{{pp|10;30;70;130|key=%|2;4;6;8|label1=Hellions|showtype=false}}"),
        "10% / 30% / 70% / 130%"
    );
}

#[test]
fn wiki_pp_013() {
    assert_eq!(
        render("{{pp|10;30;70;130|key=%|2;4;6;8|label1=Hellions|type=the number of Hellions}}"),
        "10% / 30% / 70% / 130% (based on the number of Hellions)"
    );
}

#[test]
fn wiki_pp_014() {
    assert_eq!(
        render("{{pp|5;30;75;150|key=%|2;4;6;8|label1=Hellions|showtype=false}}"),
        "5% / 30% / 75% / 150%"
    );
}

#[test]
fn wiki_pp_015() {
    assert_eq!(
        render("{{pp|5;30;75;150|key=%|2;4;6;8|label1=Hellions|type=the number of Hellions}}"),
        "5% / 30% / 75% / 150% (based on the number of Hellions)"
    );
}

#[test]
fn wiki_pp_016() {
    assert_eq!(
        render("{{pp|10;55;130|key=%|3;5;7|label1=Hellions|showtype=false}}"),
        "10% / 55% / 130%"
    );
}

#[test]
fn wiki_pp_017() {
    assert_eq!(
        render("{{pp|10;55;130|key=%|3;5;7|label1=Hellions|type=the number of Hellions}}"),
        "10% / 55% / 130% (based on the number of Hellions)"
    );
}

#[test]
fn wiki_pp_018() {
    assert_eq!(
        render("{{pp|5;50;125|key=%|3;5;7|label1=Hellions|showtype=false}}"),
        "5% / 50% / 125%"
    );
}

#[test]
fn wiki_pp_019() {
    assert_eq!(
        render("{{pp|key=%|label1=Shapeshifters|type=the number of Shapeshifters|60;100|3;6}}"),
        "60% / 100% (based on the number of Shapeshifters)"
    );
}

#[test]
fn wiki_pp_020() {
    assert_eq!(
        render("{{pp|key=%|label1=Shapeshifters|showtype=false|60;120|3;6}}"),
        "60% / 120%"
    );
}

#[test]
fn wiki_pp_021() {
    assert_eq!(
        render("{{pp|key=%|label1=Shapeshifters|type=the number of Shapeshifters|60;120|3;6}}"),
        "60% / 120% (based on the number of Shapeshifters)"
    );
}

#[test]
fn wiki_pp_022() {
    assert_eq!(render("{{pp|9 to 60}}"), "9 – 60 (based on level)");
}

#[test]
fn wiki_pp_023() {
    assert_eq!(render("{{pp|5 to 47.5}}"), "5 – 47.5 (based on level)");
}

#[test]
fn wiki_pp_024() {
    assert_eq!(
        render("{{pp|5;8;10;13;15;18;20;23;25;28;30;33;35;38;40;43;45;48}}"),
        "5 – 48 (based on level)"
    );
}

#[test]
fn wiki_pp_025() {
    assert_eq!(
        render("{{pp|key=%|12 to 24 for 4|1;5 to 15;}}"),
        "12% / 16% / 20% / 24% (based on level)"
    );
}

#[test]
fn wiki_pp_026() {
    assert_eq!(render("{{pp|75 to 330}}"), "75 – 330 (based on level)");
}

#[test]
fn wiki_pp_027() {
    assert_eq!(
        render("{{pp|55 to 265 for 15|color=magic damage}}"),
        "55 – 265 (based on level)"
    );
}

#[test]
fn wiki_pp_028() {
    assert_eq!(
        render("{{pp|key=%|34.5 to 97.5 for 15}}"),
        "34.5% – 97.5% (based on level)"
    );
}

#[test]
fn wiki_pp_029() {
    assert_eq!(
        render("{{pp|key=%|78 to 96 for 4|1 to 13}}"),
        "78% / 84% / 90% / 96% (based on level)"
    );
}

#[test]
fn wiki_pp_030() {
    assert_eq!(
        render("{{pp|25.5 to 130.5 for 15}}"),
        "25.5 – 130.5 (based on level)"
    );
}

#[test]
fn wiki_pp_031() {
    assert_eq!(render("{{pp|1;2;3|1;7;13}}"), "1 / 2 / 3 (based on level)");
}

#[test]
fn wiki_pp_032() {
    assert_eq!(
        render("{{pp|.2;.35;.5|1;7;13}}"),
        "0.2 / 0.35 / 0.5 (based on level)"
    );
}

#[test]
fn wiki_pp_033() {
    assert_eq!(
        render("{{pp|10;25;45;70|2;3;4;5|key=%|label1=Revenants|type=the number of Revenants}}"),
        "10% / 25% / 45% / 70% (based on the number of Revenants)"
    );
}

#[test]
fn wiki_pp_034() {
    assert_eq!(
        render("{{pp|10;40;75|2;3;4|key=%|label1=Revenants|showtype=false}}"),
        "10% / 40% / 75%"
    );
}

#[test]
fn wiki_pp_035() {
    assert_eq!(
        render("{{pp|10;40;75|2;3;4|key=%|label1=Revenants|type=the number of Revenants}}"),
        "10% / 40% / 75% (based on the number of Revenants)"
    );
}

#[test]
fn wiki_pp_036() {
    assert_eq!(
        render("{{pp|10;40|2;3|key=%|label1=Revenants|showtype=false}}"),
        "10% / 40%"
    );
}

#[test]
fn wiki_pp_037() {
    assert_eq!(
        render("{{pp|key=%|label1=Revenants|type=the number of Revenants|10;40|2;3}}"),
        "10% / 40% (based on the number of Revenants)"
    );
}

#[test]
fn wiki_pp_038() {
    assert_eq!(
        render(
            "{{pp|key=%|label1=Revenants|type=the number of Revenants|30;75|2;3|showtype=false}}"
        ),
        "30% / 75%"
    );
}

#[test]
fn wiki_pp_039() {
    assert_eq!(
        render("{{pp|50 to 260 for 15}}"),
        "50 – 260 (based on level)"
    );
}

#[test]
fn wiki_pp_040() {
    assert_eq!(
        render("{{pp|40 to 250 for 15}}"),
        "40 – 250 (based on level)"
    );
}

#[test]
fn wiki_pp_041() {
    assert_eq!(
        render("{{pp|12 to 110 for 15}}"),
        "12 – 110 (based on level)"
    );
}

#[test]
fn wiki_pp_042() {
    assert_eq!(
        render("{{pp|25 to 125 for 15}}"),
        "25 – 125 (based on level)"
    );
}

#[test]
fn wiki_pp_043() {
    assert_eq!(render("{{pp|12 to 46 for 15}}"), "12 – 46 (based on level)");
}

#[test]
fn wiki_pp_044() {
    assert_eq!(render("{{pp|12 to 61 for 15}}"), "12 – 61 (based on level)");
}

#[test]
fn wiki_pp_045() {
    assert_eq!(render("{{pp|5 to 20 for 4|0 to 3|key=%|type= nearby enemy champions|label1= Enemy champions}}"), "5% / 10% / 15% / 20% (based on nearby enemy champions)");
}

#[test]
fn wiki_pp_046() {
    assert_eq!(
        render("{{pp|100 to 250|round=floor}}"),
        "100 – 250 (based on level)"
    );
}

#[test]
fn wiki_pp_047() {
    assert_eq!(
        render("{{pp|90 to 120|type=average of all champion levels}}"),
        "90 – 120 (based on average of all champion levels)"
    );
}

#[test]
fn wiki_pp_048() {
    assert_eq!(
        render("{{pp|4;6;8;10|1;6;11;15}}"),
        "4 / 6 / 8 / 10 (based on level)"
    );
}

#[test]
fn wiki_pp_049() {
    assert_eq!(render("{{pp|5 to 30}}"), "5 – 30 (based on level)");
}

#[test]
fn wiki_pp_050() {
    assert_eq!(render("{{pp|100 to 300}}"), "100 – 300 (based on level)");
}

#[test]
fn wiki_pp_051() {
    assert_eq!(render("{{pp|85 to 350}}"), "85 – 350 (based on level)");
}

#[test]
fn wiki_pp_052() {
    assert_eq!(render("{{pp|39/6 to 54/6}}"), "6.5 – 9 (based on level)");
}

#[test]
fn wiki_pp_053() {
    assert_eq!(
        render("{{pp|400 to 460|1 to 13 by 1}}"),
        "400 – 460 (based on level)"
    );
}

#[test]
fn wiki_pp_054() {
    assert_eq!(render("{{pp|4 to 8}}"), "4 – 8 (based on level)");
}

#[test]
fn wiki_pp_055() {
    assert_eq!(render("{{pp|4 to 7.5}}"), "4 – 7.5 (based on level)");
}

#[test]
fn wiki_pp_056() {
    assert_eq!(
        render("{{pp|8/8 to 14/8|key=%}}"),
        "1% – 1.75% (based on level)"
    );
}

#[test]
fn wiki_pp_057() {
    assert_eq!(
        render("{{pp|8/8 to 16/8|key=%}}"),
        "1% – 2% (based on level)"
    );
}

#[test]
fn wiki_pp_058() {
    assert_eq!(render("{{pp|10 to 50}}"), "10 – 50 (based on level)");
}

#[test]
fn wiki_pp_059() {
    assert_eq!(render("{{pp|5 to 35}}"), "5 – 35 (based on level)");
}

#[test]
fn wiki_pp_060() {
    assert_eq!(render("{{pp|10*8 to 50*8}}"), "80 – 400 (based on level)");
}

#[test]
fn wiki_pp_061() {
    assert_eq!(render("{{pp|5*8 to 35*8}}"), "40 – 280 (based on level)");
}

#[test]
fn wiki_pp_062() {
    assert_eq!(
        render("{{pp|25 to 10 for 6|1;8 to 12}}"),
        "25 – 10 (based on level)"
    );
}

#[test]
fn wiki_pp_063() {
    assert_eq!(render("{{pp|100 to 460}}"), "100 – 460 (based on level)");
}

#[test]
fn wiki_pp_064() {
    assert_eq!(render("{{pp|120 to 480}}"), "120 – 480 (based on level)");
}

#[test]
fn wiki_pp_065() {
    assert_eq!(render("{{pp|400 to 750}}"), "400 – 750 (based on level)");
}

#[test]
fn wiki_pp_066() {
    assert_eq!(render("{{pp|100 to 250}}"), "100 – 250 (based on level)");
}

#[test]
fn wiki_pp_067() {
    assert_eq!(
        render("{{pp|15 to 25|key=%}}"),
        "15% – 25% (based on level)"
    );
}

#[test]
fn wiki_pp_068() {
    assert_eq!(
        render("{{pp|20;15;10|1;7;13}}"),
        "20 / 15 / 10 (based on level)"
    );
}

#[test]
fn wiki_pp_069() {
    assert_eq!(
        render("{{pp|75;80;87;94;102;111;120;131;143;155;168;183;198;214;231;248;267;287}}"),
        "75 – 287 (based on level)"
    );
}

#[test]
fn wiki_pp_070() {
    assert_eq!(
        render("{{pp|75;78;83;88;95;103;112;122;133;145;159;173;189;206;224;243;264;285}}"),
        "75 – 285 (based on level)"
    );
}

#[test]
fn wiki_pp_071() {
    assert_eq!(
        render("{{pp|1;2;3;4;5|1;5;9;13;17}}"),
        "1 / 2 / 3 / 4 / 5 (based on level)"
    );
}

#[test]
fn wiki_pp_072() {
    assert_eq!(
        render("{{pp|2;3;4;5;6|1;5;9;13;17}}"),
        "2 / 3 / 4 / 5 / 6 (based on level)"
    );
}

#[test]
fn wiki_pp_073() {
    assert_eq!(
        render("{{pp|0.28 to 1 for 17|key=%}}"),
        "0.28% – 1% (based on level)"
    );
}

#[test]
fn wiki_pp_074() {
    assert_eq!(
        render("{{pp|0.28 to 1 for 13|key=%}}"),
        "0.28% – 1% (based on level)"
    );
}

#[test]
fn wiki_pp_075() {
    assert_eq!(
        render("{{pp|20+(20/17)*(x-1)*(0.7025+0.0175*(x-1))|key=%}}"),
        "20% – 40% (based on level)"
    );
}

#[test]
fn wiki_pp_076() {
    assert_eq!(
        render("{{pp|25 to 50|key=%}}"),
        "25% – 50% (based on level)"
    );
}

#[test]
fn wiki_pp_077() {
    assert_eq!(
        render("{{pp|0.25 to 1.25|key=%}}"),
        "0.25% – 1.25% (based on level)"
    );
}

#[test]
fn wiki_pp_078() {
    assert_eq!(
        render("{{pp|0.3;0.5;0.7;0.9;1.1;1.3|1;4;7;10;13;16}}"),
        "0.3 – 1.3 (based on level)"
    );
}

#[test]
fn wiki_pp_079() {
    assert_eq!(render("{{pp|15 to 200}}"), "15 – 200 (based on level)");
}

#[test]
fn wiki_pp_080() {
    assert_eq!(render("{{pp|15 to 180}}"), "15 – 180 (based on level)");
}

#[test]
fn wiki_pp_081() {
    assert_eq!(
        render("{{pp|1.5;2;2.5;3;3.5|1;5;10;14;18}}"),
        "1.5 / 2 / 2.5 / 3 / 3.5 (based on level)"
    );
}

#[test]
fn wiki_pp_082() {
    assert_eq!(
        render("{{pp|60;50;40|1;7;13}}"),
        "60 / 50 / 40 (based on level)"
    );
}

#[test]
fn wiki_pp_083() {
    assert_eq!(
        render("{{pp|75;70;65|1;7;13}}"),
        "75 / 70 / 65 (based on level)"
    );
}

#[test]
fn wiki_pp_084() {
    assert_eq!(
        render("{{pp|50;55;65|1;7;13}}"),
        "50 / 55 / 65 (based on level)"
    );
}

#[test]
fn wiki_pp_085() {
    assert_eq!(
        render("{{pp|15;17.5;20;22.5|1;6;11;16}}"),
        "15 / 17.5 / 20 / 22.5 (based on level)"
    );
}

#[test]
fn wiki_pp_086() {
    assert_eq!(
        render("{{pp|90;900;9000|1;7;13}}"),
        "90 / 900 / 9000 (based on level)"
    );
}

#[test]
fn wiki_pp_087() {
    assert_eq!(
        render("{{pp|90000;900000;9000000|1;7;13}}"),
        "90000 / 900000 / 9000000 (based on level)"
    );
}

#[test]
fn wiki_pp_088() {
    assert_eq!(
        render("{{pp|90;95;100|1;7;13}}"),
        "90 / 95 / 100 (based on level)"
    );
}

#[test]
fn wiki_pp_089() {
    assert_eq!(render("{{pp|0;0;0|1;7;13}}"), "0 / 0 / 0 (based on level)");
}

#[test]
fn wiki_pp_090() {
    assert_eq!(
        render("{{pp|75;150;225|1;7;13}}"),
        "75 / 150 / 225 (based on level)"
    );
}

#[test]
fn wiki_pp_091() {
    assert_eq!(render("{{pp|3;5;7|1;7;13}}"), "3 / 5 / 7 (based on level)");
}

#[test]
fn wiki_pp_092() {
    assert_eq!(
        render("{{pp|35;30;25;20|1;6;11;16}}"),
        "35 / 30 / 25 / 20 (based on level)"
    );
}

#[test]
fn wiki_pp_093() {
    assert_eq!(render("{{pp|5;6;7|1;7;13}}"), "5 / 6 / 7 (based on level)");
}

#[test]
fn wiki_pp_094() {
    assert_eq!(
        render("{{pp|20 to 125 for 15}}"),
        "20 – 125 (based on level)"
    );
}

#[test]
fn wiki_pp_095() {
    assert_eq!(
        render("{{pp|27 to 160 for 15}}"),
        "27 – 160 (based on level)"
    );
}

#[test]
fn wiki_pp_096() {
    assert_eq!(
        render("{{pp|54 to 320 for 15}}"),
        "54 – 320 (based on level)"
    );
}

#[test]
fn wiki_pp_097() {
    assert_eq!(
        render("{{pp|key=%|40;60;80;100|1;5;9;13}}"),
        "40% / 60% / 80% / 100% (based on level)"
    );
}

#[test]
fn wiki_pp_098() {
    assert_eq!(
        render("{{pp|key=%|5 to 25 for 15}}"),
        "5% – 25% (based on level)"
    );
}

#[test]
fn wiki_pp_099() {
    assert_eq!(
        render("{{pp|key=%|5 to 35 for 15}}"),
        "5% – 35% (based on level)"
    );
}

#[test]
fn wiki_ap_001() {
    assert_eq!(render("{{ap|45 to 25 3}}"), "45 / 35 / 25");
}

#[test]
fn wiki_ap_002() {
    assert_eq!(render("{{ap|50 to 30 3}}"), "50 / 40 / 30");
}

#[test]
fn wiki_ap_003() {
    assert_eq!(render("{{ap|65 to 165}}"), "65 / 90 / 115 / 140 / 165");
}

#[test]
fn wiki_ap_004() {
    assert_eq!(render("{{ap|70 to 170}}"), "70 / 95 / 120 / 145 / 170");
}

#[test]
fn wiki_ap_005() {
    assert_eq!(render("{{ap|40*2}}"), "80");
}

#[test]
fn wiki_ap_006() {
    assert_eq!(render("{{ap|45*2}}"), "90");
}

#[test]
fn wiki_ap_007() {
    assert_eq!(render("{{ap|15 to 10}}"), "15 / 13.75 / 12.5 / 11.25 / 10");
}

#[test]
fn wiki_ap_008() {
    assert_eq!(render("{{ap|18 to 10}}"), "18 / 16 / 14 / 12 / 10");
}

#[test]
fn wiki_ap_009() {
    assert_eq!(render("{{ap|50 to 70}}"), "50 / 55 / 60 / 65 / 70");
}

#[test]
fn wiki_ap_010() {
    assert_eq!(render("{{ap|60 to 30 3}}"), "60 / 45 / 30");
}

#[test]
fn wiki_ap_011() {
    assert_eq!(render("{{ap|75 to 235}}"), "75 / 115 / 155 / 195 / 235");
}

#[test]
fn wiki_ap_012() {
    assert_eq!(render("{{ap|75 to 215}}"), "75 / 110 / 145 / 180 / 215");
}

#[test]
fn wiki_ap_013() {
    assert_eq!(render("{{ap|55 to 155}}"), "55 / 80 / 105 / 130 / 155");
}

#[test]
fn wiki_ap_014() {
    assert_eq!(render("{{ap|130 to 330}}"), "130 / 180 / 230 / 280 / 330");
}

#[test]
fn wiki_ap_015() {
    assert_eq!(render("{{ap|110 to 310}}"), "110 / 160 / 210 / 260 / 310");
}

#[test]
fn wiki_ap_016() {
    assert_eq!(render("{{ap|60 to 100}}"), "60 / 70 / 80 / 90 / 100");
}

#[test]
fn wiki_ap_017() {
    assert_eq!(render("{{ap|60 to 120}}"), "60 / 75 / 90 / 105 / 120");
}

#[test]
fn wiki_ap_018() {
    assert_eq!(render("{{ap|50 to 90}}"), "50 / 60 / 70 / 80 / 90");
}

#[test]
fn wiki_ap_019() {
    assert_eq!(render("{{ap|50 to 130}}"), "50 / 70 / 90 / 110 / 130");
}

#[test]
fn wiki_ap_020() {
    assert_eq!(render("{{ap|40 to 120}}"), "40 / 60 / 80 / 100 / 120");
}

#[test]
fn wiki_ap_021() {
    assert_eq!(render("{{ap|80 to 240}}"), "80 / 120 / 160 / 200 / 240");
}

#[test]
fn wiki_ap_022() {
    assert_eq!(render("{{ap|70 to 230}}"), "70 / 110 / 150 / 190 / 230");
}

#[test]
fn wiki_ap_023() {
    assert_eq!(render("{{ap|60 to 180}}"), "60 / 90 / 120 / 150 / 180");
}

#[test]
fn wiki_ap_024() {
    assert_eq!(render("{{ap|18|16|14|12|10}}"), "18 / 16 / 14 / 12 / 10");
}

#[test]
fn wiki_ap_025() {
    assert_eq!(render("{{ap|14|13|12|11|10}}"), "14 / 13 / 12 / 11 / 10");
}

#[test]
fn wiki_ap_026() {
    assert_eq!(
        render("{{ap|75|115|155|195|235}}"),
        "75 / 115 / 155 / 195 / 235"
    );
}

#[test]
fn wiki_ap_027() {
    assert_eq!(
        render("{{ap|85|125|165|205|245}}"),
        "85 / 125 / 165 / 205 / 245"
    );
}

#[test]
fn wiki_ap_028() {
    assert_eq!(
        render("{{ap|55|80|105|130|155}}"),
        "55 / 80 / 105 / 130 / 155"
    );
}

#[test]
fn wiki_ap_029() {
    assert_eq!(render("{{ap|50|60|70|80|90}}"), "50 / 60 / 70 / 80 / 90");
}

#[test]
fn wiki_ap_030() {
    assert_eq!(render("{{ap|40|55|70|85|100}}"), "40 / 55 / 70 / 85 / 100");
}

#[test]
fn wiki_ap_031() {
    assert_eq!(
        render("{{ap|60|75|90|105|120}}"),
        "60 / 75 / 90 / 105 / 120"
    );
}

#[test]
fn wiki_ap_032() {
    assert_eq!(
        render("{{ap|70|80|90|100|110}}"),
        "70 / 80 / 90 / 100 / 110"
    );
}

#[test]
fn wiki_ap_033() {
    assert_eq!(
        render("{{ap|60|90|120|150|180}}"),
        "60 / 90 / 120 / 150 / 180"
    );
}

#[test]
fn wiki_ap_034() {
    assert_eq!(
        render("{{ap|40|60|80|100|120}}"),
        "40 / 60 / 80 / 100 / 120"
    );
}

#[test]
fn wiki_ap_035() {
    assert_eq!(render("{{ap|60|45|30}}"), "60 / 45 / 30");
}

#[test]
fn wiki_ap_036() {
    assert_eq!(render("{{ap|70|140|210}}"), "70 / 140 / 210");
}

#[test]
fn wiki_ap_037() {
    assert_eq!(render("{{ap|140|280|420}}"), "140 / 280 / 420");
}

#[test]
fn wiki_ap_038() {
    assert_eq!(render("{{ap|150|300|450}}"), "150 / 300 / 450");
}

#[test]
fn wiki_ap_039() {
    assert_eq!(
        render("{{ap|44|72|100|128|156}}"),
        "44 / 72 / 100 / 128 / 156"
    );
}

#[test]
fn wiki_ap_040() {
    assert_eq!(render("{{ap|22|36|50|64|78}}"), "22 / 36 / 50 / 64 / 78");
}

#[test]
fn wiki_ap_041() {
    assert_eq!(
        render("{{ap|85|120|155|190|225}}"),
        "85 / 120 / 155 / 190 / 225"
    );
}

#[test]
fn wiki_ap_042() {
    assert_eq!(render("{{ap|60|120|180}}"), "60 / 120 / 180");
}

#[test]
fn wiki_ap_043() {
    assert_eq!(render("{{ap|125|225|325}}"), "125 / 225 / 325");
}

#[test]
fn wiki_ap_044() {
    assert_eq!(
        render("{{ap|55|90|125|160|195}}"),
        "55 / 90 / 125 / 160 / 195"
    );
}

#[test]
fn wiki_ap_045() {
    assert_eq!(
        render("{{ap|40|65|90|115|140}}"),
        "40 / 65 / 90 / 115 / 140"
    );
}

#[test]
fn wiki_ap_046() {
    assert_eq!(render("{{ap|40|45|50|55|60}}"), "40 / 45 / 50 / 55 / 60");
}

#[test]
fn wiki_ap_047() {
    assert_eq!(
        render("{{ap|66|96|126|156|186}}"),
        "66 / 96 / 126 / 156 / 186"
    );
}

#[test]
fn wiki_ap_048() {
    assert_eq!(render("{{ap|80|85|90|95|100}}"), "80 / 85 / 90 / 95 / 100");
}

#[test]
fn wiki_ap_049() {
    assert_eq!(render("{{ap|54|42|30}}"), "54 / 42 / 30");
}

#[test]
fn wiki_ap_050() {
    assert_eq!(render("{{ap|40|32|24}}"), "40 / 32 / 24");
}

#[test]
fn wiki_ap_051() {
    assert_eq!(render("{{ap|160|140|120}}"), "160 / 140 / 120");
}

#[test]
fn wiki_ap_052() {
    assert_eq!(render("{{ap|150|275|400}}"), "150 / 275 / 400");
}

#[test]
fn wiki_ap_053() {
    assert_eq!(render("{{ap|100|160|220}}"), "100 / 160 / 220");
}

#[test]
fn wiki_ap_054() {
    assert_eq!(render("{{ap|100|200|300}}"), "100 / 200 / 300");
}

#[test]
fn wiki_ap_055() {
    assert_eq!(
        render("{{ap|80|90|100|110|120}}"),
        "80 / 90 / 100 / 110 / 120"
    );
}

#[test]
fn wiki_ap_056() {
    assert_eq!(render("{{ap|70|75|80|85|90}}"), "70 / 75 / 80 / 85 / 90");
}

#[test]
fn wiki_ap_057() {
    assert_eq!(
        render("{{ap|70|110|150|190|230}}"),
        "70 / 110 / 150 / 190 / 230"
    );
}

#[test]
fn wiki_ap_058() {
    assert_eq!(render("{{ap|20|40|60|80|100}}"), "20 / 40 / 60 / 80 / 100");
}

#[test]
fn wiki_ap_059() {
    assert_eq!(
        render("{{ap|110|160|210|260|310}}"),
        "110 / 160 / 210 / 260 / 310"
    );
}

#[test]
fn wiki_ap_060() {
    assert_eq!(
        render("{{ap|90|150|210|270|330}}"),
        "90 / 150 / 210 / 270 / 330"
    );
}

#[test]
fn wiki_ap_061() {
    assert_eq!(
        render("{{ap|1|1.3|1.6|1.9|2.2}}"),
        "1 / 1.3 / 1.6 / 1.9 / 2.2"
    );
}

#[test]
fn wiki_ap_062() {
    assert_eq!(render("{{ap|100|50|0}}"), "100 / 50 / 0");
}

#[test]
fn wiki_ap_063() {
    assert_eq!(render("{{ap|20|18|16|14|12}}"), "20 / 18 / 16 / 14 / 12");
}

#[test]
fn wiki_ap_064() {
    assert_eq!(
        render("{{ap|1|1.25|1.5|1.75|2}}"),
        "1 / 1.25 / 1.5 / 1.75 / 2"
    );
}

#[test]
fn wiki_ap_065() {
    assert_eq!(render("{{ap|40|35|30}}"), "40 / 35 / 30");
}

#[test]
fn wiki_ap_066() {
    assert_eq!(
        render("{{ap|80|125|170|215|260}}"),
        "80 / 125 / 170 / 215 / 260"
    );
}

#[test]
fn wiki_ap_067() {
    assert_eq!(render("{{ap|60|65|70|75|80}}"), "60 / 65 / 70 / 75 / 80");
}

#[test]
fn wiki_ap_068() {
    assert_eq!(
        render("{{ap|90|130|170|210|250}}"),
        "90 / 130 / 170 / 210 / 250"
    );
}

#[test]
fn wiki_ap_069() {
    assert_eq!(render("{{ap|60|70|80|90|100}}"), "60 / 70 / 80 / 90 / 100");
}

#[test]
fn wiki_ap_070() {
    assert_eq!(
        render("{{ap|45|70|95|120|145}}"),
        "45 / 70 / 95 / 120 / 145"
    );
}

#[test]
fn wiki_ap_071() {
    assert_eq!(render("{{ap|10|25|40}}"), "10 / 25 / 40");
}

#[test]
fn wiki_ap_072() {
    assert_eq!(render("{{ap|20|30|40}}"), "20 / 30 / 40");
}

#[test]
fn wiki_ap_073() {
    assert_eq!(render("{{ap|20|35|50}}"), "20 / 35 / 50");
}

#[test]
fn wiki_ap_074() {
    assert_eq!(render("{{ap|70 to 190}}"), "70 / 100 / 130 / 160 / 190");
}

#[test]
fn wiki_ap_075() {
    assert_eq!(
        render("{{ap|90*0.75 to 190*0.75}}"),
        "67.5 / 86.25 / 105 / 123.75 / 142.5"
    );
}

#[test]
fn wiki_ap_076() {
    assert_eq!(render("{{ap|85*0.75}}"), "63.75");
}

#[test]
fn wiki_ap_077() {
    assert_eq!(
        render("{{ap|70*1.6 to 190*1.6}}"),
        "112 / 160 / 208 / 256 / 304"
    );
}

#[test]
fn wiki_ap_078() {
    assert_eq!(render("{{ap|85*1.6}}"), "136");
}

#[test]
fn wiki_ap_079() {
    assert_eq!(
        render("{{ap|70*(0.75+0.6) to 190*(0.75+0.6)}}"),
        "94.5 / 135 / 175.5 / 216 / 256.5"
    );
}

#[test]
fn wiki_ap_080() {
    assert_eq!(render("{{ap|85*(0.75+0.6)}}"), "114.75");
}

#[test]
fn wiki_ap_081() {
    assert_eq!(render("{{ap|25 to 130 4}}"), "25 / 60 / 95 / 130");
}

#[test]
fn wiki_ap_082() {
    assert_eq!(render("{{ap|9 to 7.5 4}}"), "9 / 8.5 / 8 / 7.5");
}

#[test]
fn wiki_ap_083() {
    assert_eq!(render("{{ap|55 to 75}}"), "55 / 60 / 65 / 70 / 75");
}

#[test]
fn wiki_ap_084() {
    assert_eq!(render("{{ap|120 to 80}}"), "120 / 110 / 100 / 90 / 80");
}

#[test]
fn wiki_ap_085() {
    assert_eq!(render("{{ap|60 to 240 4}}"), "60 / 120 / 180 / 240");
}

#[test]
fn wiki_ap_086() {
    assert_eq!(render("{{ap|30 to 90 4}}"), "30 / 50 / 70 / 90");
}

#[test]
fn wiki_ap_087() {
    assert_eq!(render("{{ap|250 to 550}}"), "250 / 325 / 400 / 475 / 550");
}

#[test]
fn wiki_ap_088() {
    assert_eq!(render("{{ap|110 to 80}}"), "110 / 102.5 / 95 / 87.5 / 80");
}

#[test]
fn wiki_ap_089() {
    assert_eq!(render("{{ap|300 to 500}}"), "300 / 350 / 400 / 450 / 500");
}

#[test]
fn wiki_ap_090() {
    assert_eq!(render("{{ap|60 to 40}}"), "60 / 55 / 50 / 45 / 40");
}

#[test]
fn wiki_ap_091() {
    assert_eq!(render("{{ap|55 to 115}}"), "55 / 70 / 85 / 100 / 115");
}

#[test]
fn wiki_ap_092() {
    assert_eq!(render("{{ap|75 to 175}}"), "75 / 100 / 125 / 150 / 175");
}

#[test]
fn wiki_ap_093() {
    assert_eq!(render("{{ap|30 to 50}}"), "30 / 35 / 40 / 45 / 50");
}

#[test]
fn wiki_ap_094() {
    assert_eq!(render("{{ap|75*2.4}}"), "180");
}

#[test]
fn wiki_ap_095() {
    assert_eq!(render("{{ap|65*2.4}}"), "156");
}

#[test]
fn wiki_ap_096() {
    assert_eq!(render("{{ap|15 to 55}}"), "15 / 25 / 35 / 45 / 55");
}

#[test]
fn wiki_ap_097() {
    assert_eq!(render("{{ap|10 to 50}}"), "10 / 20 / 30 / 40 / 50");
}

#[test]
fn wiki_ap_098() {
    assert_eq!(
        render("{{ap|1.25 to 2.25}}"),
        "1.25 / 1.5 / 1.75 / 2 / 2.25"
    );
}

#[test]
fn wiki_ap_099() {
    assert_eq!(render("{{ap|1 to 2}}"), "1 / 1.25 / 1.5 / 1.75 / 2");
}

#[test]
fn wiki_ap_100() {
    assert_eq!(render("{{ap|35 to 45}}"), "35 / 37.5 / 40 / 42.5 / 45");
}

#[test]
fn wiki_ap_101() {
    assert_eq!(render("{{ap|25 to 35}}"), "25 / 27.5 / 30 / 32.5 / 35");
}

#[test]
fn wiki_ap_102() {
    assert_eq!(render("{{ap|120 to 80 3}}"), "120 / 100 / 80");
}

#[test]
fn wiki_ap_103() {
    assert_eq!(render("{{ap|140 to 80 3}}"), "140 / 110 / 80");
}

#[test]
fn wiki_ap_104() {
    assert_eq!(render("{{ap|25 to 45}}"), "25 / 30 / 35 / 40 / 45");
}

#[test]
fn wiki_ap_105() {
    assert_eq!(render("{{ap|95 to 175}}"), "95 / 115 / 135 / 155 / 175");
}

#[test]
fn wiki_ap_106() {
    assert_eq!(render("{{ap|125 to 375 3}}"), "125 / 250 / 375");
}

#[test]
fn wiki_ap_107() {
    assert_eq!(render("{{ap|150 to 400 3}}"), "150 / 275 / 400");
}

#[test]
fn wiki_ap_108() {
    assert_eq!(
        render("{{ap|55|70|85|100|115}}"),
        "55 / 70 / 85 / 100 / 115"
    );
}

#[test]
fn wiki_ap_109() {
    assert_eq!(
        render("{{ap|95|115|135|155|175}}"),
        "95 / 115 / 135 / 155 / 175"
    );
}

#[test]
fn wiki_ap_110() {
    assert_eq!(
        render("{{ap|70|90|110|130|150}}"),
        "70 / 90 / 110 / 130 / 150"
    );
}

#[test]
fn wiki_ap_111() {
    assert_eq!(render("{{ap|30|45|60|75|90}}"), "30 / 45 / 60 / 75 / 90");
}

#[test]
fn wiki_ap_112() {
    assert_eq!(
        render("{{ap|60|80|100|120|140}}"),
        "60 / 80 / 100 / 120 / 140"
    );
}

#[test]
fn wiki_ap_113() {
    assert_eq!(render("{{ap|25|30|35|40|45}}"), "25 / 30 / 35 / 40 / 45");
}

#[test]
fn wiki_ap_114() {
    assert_eq!(render("{{ap|10|20|30|40|50}}"), "10 / 20 / 30 / 40 / 50");
}

#[test]
fn wiki_ap_115() {
    assert_eq!(
        render("{{ap|25|27.5|30|32.5|35}}"),
        "25 / 27.5 / 30 / 32.5 / 35"
    );
}

#[test]
fn wiki_ap_116() {
    assert_eq!(
        render("{{ap|250|300|350|400|450}}"),
        "250 / 300 / 350 / 400 / 450"
    );
}

#[test]
fn wiki_ap_117() {
    assert_eq!(render("{{ap|140|110|80}}"), "140 / 110 / 80");
}

#[test]
fn wiki_ap_118() {
    assert_eq!(render("{{ap|120|100|80}}"), "120 / 100 / 80");
}

#[test]
fn wiki_ap_119() {
    assert_eq!(render("{{ap|150|120|90}}"), "150 / 120 / 90");
}

#[test]
fn wiki_ap_120() {
    assert_eq!(render("{{ap|40|50|60|70|80}}"), "40 / 50 / 60 / 70 / 80");
}

#[test]
fn wiki_ap_121() {
    assert_eq!(render("{{ap|40|60|80}}"), "40 / 60 / 80");
}

#[test]
fn wiki_ap_122() {
    assert_eq!(render("{{ap|30|50|70}}"), "30 / 50 / 70");
}

#[test]
fn wiki_ap_123() {
    assert_eq!(render("{{ap|35|40|45|50|55}}"), "35 / 40 / 45 / 50 / 55");
}

#[test]
fn wiki_ap_124() {
    assert_eq!(render("{{ap|50|55|60|65|70}}"), "50 / 55 / 60 / 65 / 70");
}

#[test]
fn wiki_ap_125() {
    assert_eq!(render("{{ap|12|18|24|30|36}}"), "12 / 18 / 24 / 30 / 36");
}

#[test]
fn wiki_ap_126() {
    assert_eq!(render("{{ap|16|22|28|34|40}}"), "16 / 22 / 28 / 34 / 40");
}

#[test]
fn wiki_ap_127() {
    assert_eq!(render("{{ap|14|18|22|26|30}}"), "14 / 18 / 22 / 26 / 30");
}

#[test]
fn wiki_ap_128() {
    assert_eq!(render("{{ap|35|55|75|95|115}}"), "35 / 55 / 75 / 95 / 115");
}

#[test]
fn wiki_ap_129() {
    assert_eq!(
        render("{{ap|35|60|85|110|135}}"),
        "35 / 60 / 85 / 110 / 135"
    );
}

#[test]
fn wiki_ap_130() {
    assert_eq!(render("{{ap|15|20|25}}"), "15 / 20 / 25");
}

#[test]
fn wiki_ap_131() {
    assert_eq!(render("{{ap|120|90|60}}"), "120 / 90 / 60");
}

#[test]
fn wiki_ap_132() {
    assert_eq!(render("{{ap|19|23|27|31|35}}"), "19 / 23 / 27 / 31 / 35");
}

#[test]
fn wiki_ap_133() {
    assert_eq!(render("{{ap|25|40|55|70|85}}"), "25 / 40 / 55 / 70 / 85");
}

#[test]
fn wiki_ap_134() {
    assert_eq!(render("{{ap|12|15|18|21|24}}"), "12 / 15 / 18 / 21 / 24");
}

#[test]
fn wiki_ap_135() {
    assert_eq!(render("{{ap|4|8|12|16|20}}"), "4 / 8 / 12 / 16 / 20");
}

#[test]
fn wiki_ap_136() {
    assert_eq!(render("{{ap|30|40|50|60|70}}"), "30 / 40 / 50 / 60 / 70");
}

#[test]
fn wiki_ap_137() {
    assert_eq!(
        render("{{ap|80|135|190|255|320}}"),
        "80 / 135 / 190 / 255 / 320"
    );
}

#[test]
fn wiki_ap_138() {
    assert_eq!(
        render("{{ap|90|105|120|135|150}}"),
        "90 / 105 / 120 / 135 / 150"
    );
}

#[test]
fn wiki_ap_139() {
    assert_eq!(render("{{ap|150|225|300}}"), "150 / 225 / 300");
}

#[test]
fn wiki_ap_140() {
    assert_eq!(render("{{ap|30|35|40|45|50}}"), "30 / 35 / 40 / 45 / 50");
}

#[test]
fn wiki_ap_141() {
    assert_eq!(render("{{ap|350|500|650}}"), "350 / 500 / 650");
}

#[test]
fn wiki_ap_142() {
    assert_eq!(render("{{ap|25|50|75}}"), "25 / 50 / 75");
}

#[test]
fn wiki_ap_143() {
    assert_eq!(render("{{ap|50|75|100}}"), "50 / 75 / 100");
}

#[test]
fn wiki_ap_144() {
    assert_eq!(render("{{ap|8|10|12|14|16}}"), "8 / 10 / 12 / 14 / 16");
}

#[test]
fn wiki_ap_145() {
    assert_eq!(render("{{ap|20|30|40|50|60}}"), "20 / 30 / 40 / 50 / 60");
}

#[test]
fn wiki_ap_146() {
    assert_eq!(render("{{ap|12|11|10|9|8}}"), "12 / 11 / 10 / 9 / 8");
}

#[test]
fn wiki_ap_147() {
    assert_eq!(render("{{ap|8|12|16|20|24}}"), "8 / 12 / 16 / 20 / 24");
}

#[test]
fn wiki_ap_148() {
    assert_eq!(render("{{ap|6|12|18|24|30}}"), "6 / 12 / 18 / 24 / 30");
}

#[test]
fn wiki_ap_149() {
    assert_eq!(
        render("{{ap|60|125|190|255|320}}"),
        "60 / 125 / 190 / 255 / 320"
    );
}

#[test]
fn wiki_ap_150() {
    assert_eq!(render("{{ap|30|50|70|90|110}}"), "30 / 50 / 70 / 90 / 110");
}

#[test]
fn wiki_ap_151() {
    assert_eq!(
        render("{{ap|25|50|75|100|125}}"),
        "25 / 50 / 75 / 100 / 125"
    );
}

#[test]
fn wiki_ap_152() {
    assert_eq!(
        render("{{ap|30|55|80|105|130}}"),
        "30 / 55 / 80 / 105 / 130"
    );
}

#[test]
fn wiki_ap_153() {
    assert_eq!(render("{{ap|10|14|18|22|26}}"), "10 / 14 / 18 / 22 / 26");
}

#[test]
fn wiki_ap_154() {
    assert_eq!(render("{{ap|10|15|20|25|30}}"), "10 / 15 / 20 / 25 / 30");
}

#[test]
fn wiki_ap_155() {
    assert_eq!(render("{{ap|10|16|22|28|34}}"), "10 / 16 / 22 / 28 / 34");
}

#[test]
fn wiki_ap_156() {
    assert_eq!(render("{{ap|7|14|21|28|35}}"), "7 / 14 / 21 / 28 / 35");
}

#[test]
fn wiki_ap_157() {
    assert_eq!(render("{{ap|1.33|2.66|4}}"), "1.33 / 2.66 / 4");
}

#[test]
fn wiki_ap_158() {
    assert_eq!(render("{{ap|45 to 90 4}}"), "45 / 60 / 75 / 90");
}

#[test]
fn wiki_ap_159() {
    assert_eq!(render("{{ap|30 to 105 4}}"), "30 / 55 / 80 / 105");
}

#[test]
fn wiki_ap_160() {
    assert_eq!(render("{{ap|20 to 80 4}}"), "20 / 40 / 60 / 80");
}

#[test]
fn wiki_ap_161() {
    assert_eq!(render("{{ap|45+20 to 90+80 4}}"), "65 / 100 / 135 / 170");
}

#[test]
fn wiki_ap_162() {
    assert_eq!(render("{{ap|30+40}}"), "70");
}

#[test]
fn wiki_ap_163() {
    assert_eq!(render("{{ap|50 to 170 4}}"), "50 / 90 / 130 / 170");
}

#[test]
fn wiki_ap_164() {
    assert_eq!(render("{{ap|8 to 5 4}}"), "8 / 7 / 6 / 5");
}

#[test]
fn wiki_ap_165() {
    assert_eq!(render("{{ap|50 to 65 4}}"), "50 / 55 / 60 / 65");
}

#[test]
fn wiki_ap_166() {
    assert_eq!(render("{{ap|70 to 270}}"), "70 / 120 / 170 / 220 / 270");
}

#[test]
fn wiki_ap_167() {
    assert_eq!(render("{{ap|8 to 24}}"), "8 / 12 / 16 / 20 / 24");
}

#[test]
fn wiki_ap_168() {
    assert_eq!(render("{{ap|50 to 150}}"), "50 / 75 / 100 / 125 / 150");
}

#[test]
fn wiki_ap_169() {
    assert_eq!(render("{{ap|14 to 30}}"), "14 / 18 / 22 / 26 / 30");
}

#[test]
fn wiki_ap_170() {
    assert_eq!(render("{{ap|150 to 350 3}}"), "150 / 250 / 350");
}

#[test]
fn wiki_dv_001() {
    assert_eq!(render("{{dv|1200|1544}}"), "1200 • 1544");
}

#[test]
fn wiki_numbersup_001() {
    assert_eq!(render("{{NumberSup|14}}"), "14th");
}

#[test]
fn wiki_numbersup_002() {
    assert_eq!(render("{{NumberSup|2}}"), "2nd");
}

#[test]
fn wiki_numbersup_003() {
    assert_eq!(render("{{NumberSup|6}}"), "6th");
}

#[test]
fn wiki_numbersup_004() {
    assert_eq!(render("{{NumberSup|20}}"), "20th");
}

#[test]
fn wiki_numbersup_005() {
    assert_eq!(render("{{NumberSup|31}}"), "31st");
}

#[test]
fn wiki_numbersup_006() {
    assert_eq!(render("{{NumberSup|24}}"), "24th");
}

#[test]
fn wiki_numbersup_007() {
    assert_eq!(render("{{NumberSup|15}}"), "15th");
}

#[test]
fn wiki_numbersup_008() {
    assert_eq!(render("{{NumberSup|13}}"), "13th");
}

#[test]
fn wiki_numbersup_009() {
    assert_eq!(render("{{NumberSup|10}}"), "10th");
}

#[test]
fn wiki_numbersup_010() {
    assert_eq!(render("{{NumberSup|28}}"), "28th");
}

#[test]
fn wiki_numbersup_011() {
    assert_eq!(render("{{NumberSup|21}}"), "21st");
}

#[test]
fn wiki_numbersup_012() {
    assert_eq!(render("{{NumberSup|16}}"), "16th");
}

#[test]
fn wiki_numbersup_013() {
    assert_eq!(render("{{NumberSup|1}}"), "1st");
}

#[test]
fn wiki_numbersup_014() {
    assert_eq!(render("{{NumberSup|17}}"), "17th");
}

#[test]
fn wiki_numbersup_015() {
    assert_eq!(render("{{NumberSup|5}}"), "5th");
}

#[test]
fn wiki_numbersup_016() {
    assert_eq!(render("{{NumberSup|11}}"), "11th");
}

#[test]
fn wiki_numbersup_017() {
    assert_eq!(render("{{NumberSup|19}}"), "19th");
}

#[test]
fn wiki_numbersup_018() {
    assert_eq!(render("{{NumberSup|9}}"), "9th");
}

#[test]
fn wiki_numbersup_019() {
    assert_eq!(render("{{NumberSup|12}}"), "12th");
}

#[test]
fn wiki_numbersup_020() {
    assert_eq!(render("{{NumberSup|18}}"), "18th");
}

#[test]
fn wiki_if_001() {
    assert_eq!(render("{{#if: yes | sbc | 1x }}"), "sbc");
}

#[test]
fn wiki_if_002() {
    assert_eq!(render("{{#if:  | sbc | 1x }}"), "1x");
}

#[test]
fn wiki_if_003() {
    assert_eq!(render("{{#if:{{#pos:{{lc:Epic}}|epic}}|c}}"), "c");
}

#[test]
fn wiki_if_004() {
    assert_eq!(render("{{#if:{{#pos:3|4}}{{#pos:3|3}}|2}}"), "2");
}

#[test]
fn wiki_color_001() {
    assert_eq!(render("{{color|red|Disabled}}"), "Disabled");
}

#[test]
fn wiki_color_002() {
    assert_eq!(render("{{color|red|Yes}}"), "Yes");
}

#[test]
fn wiki_color_003() {
    assert_eq!(render("{{color|red|No}}"), "No");
}

#[test]
fn wiki_color_004() {
    assert_eq!(render("{{color|green|Allowed}}"), "Allowed");
}

#[test]
fn wiki_color_005() {
    assert_eq!(render("{{color|yellow|Parried}}"), "Parried");
}

#[test]
fn wiki_color_006() {
    assert_eq!(render("{{color|green|No}}"), "No");
}

#[test]
fn wiki_color_007() {
    assert_eq!(render("{{color|green|None}}"), "None");
}

#[test]
fn wiki_color_008() {
    assert_eq!(render("{{color|green|Yes}}"), "Yes");
}

#[test]
fn wiki_color_009() {
    assert_eq!(render("{{color|yellow|Reduced}}"), "Reduced");
}

#[test]
fn wiki_color_010() {
    assert_eq!(render("{{color|red|Uncontrollable}}"), "Uncontrollable");
}

#[test]
fn wiki_color_011() {
    assert_eq!(render("{{color|yellow|Partial}}"), "Partial");
}

#[test]
fn wiki_color_012() {
    assert_eq!(render("{{color|Green|No}}"), "No");
}

#[test]
fn wiki_color_013() {
    assert_eq!(render("{{color|red|None}}"), "None");
}

#[test]
fn wiki_color_014() {
    assert_eq!(render("{{color|red|All}}"), "All");
}

#[test]
fn wiki_color_015() {
    assert_eq!(
        render("{{color|#66a3ff|Radiating Hammer Shock}}"),
        "Radiating Hammer Shock"
    );
}

#[test]
fn wiki_color_016() {
    assert_eq!(render("{{color| #66a3ff|Vulnerable}}"), "Vulnerable");
}

#[test]
fn wiki_color_017() {
    assert_eq!(render("{{color|#66a3ff|Aura Effect}}"), "Aura Effect");
}

#[test]
fn wiki_color_018() {
    assert_eq!(
        render("{{color| #66a3ff|Hammer of Grimoire}}"),
        "Hammer of Grimoire"
    );
}

#[test]
fn wiki_color_019() {
    assert_eq!(render("{{color| #66a3ff|Vulnerability}}"), "Vulnerability");
}

#[test]
fn wiki_color_020() {
    assert_eq!(render("{{color| #66a3ff|Haste Motion}}"), "Haste Motion");
}

#[test]
fn wiki_color_021() {
    assert_eq!(render("{{color| #66a3ff|Frenzy}}"), "Frenzy");
}

#[test]
fn wiki_color_022() {
    assert_eq!(render("{{color|#66a3ff|Degrade}}"), "Degrade");
}

#[test]
fn wiki_aug_001() {
    assert_eq!(render("{{aug|ar|Light 'em Up!}}"), "Light 'em Up!");
}

#[test]
fn wiki_aug_002() {
    assert_eq!(
        render("{{aug|mayhem|Quest: Sneakerhead}}"),
        "Quest: Sneakerhead"
    );
}

#[test]
fn wiki_aug_003() {
    assert_eq!(render("{{aug|ar|Die Another Day}}"), "Die Another Day");
}

#[test]
fn wiki_aug_004() {
    assert_eq!(render("{{aug|ar|Blade Waltz}}"), "Blade Waltz");
}

#[test]
fn wiki_aug_005() {
    assert_eq!(
        render("{{aug|ar|We'll Be Right Back}}"),
        "We'll Be Right Back"
    );
}

#[test]
fn wiki_aug_006() {
    assert_eq!(render("{{aug|ar|Orbital Laser}}"), "Orbital Laser");
}

#[test]
fn wiki_aug_007() {
    assert_eq!(render("{{aug|mayhem|Buff Buddies}}"), "Buff Buddies");
}

#[test]
fn wiki_aug_008() {
    assert_eq!(render("{{aug|mayhem|Juiced}}"), "Juiced");
}

#[test]
fn wiki_aug_009() {
    assert_eq!(render("{{aug|mayhem|Mind to Matter}}"), "Mind to Matter");
}

#[test]
fn wiki_aug_010() {
    assert_eq!(render("{{aug|mayhem|Ocean Soul}}"), "Ocean Soul");
}

#[test]
fn wiki_aug_011() {
    assert_eq!(render("{{aug|mayhem|Overflow}}"), "Overflow");
}

#[test]
fn wiki_aug_012() {
    assert_eq!(render("{{aug|mayhem|Clown College}}"), "Clown College");
}

#[test]
fn wiki_aug_013() {
    assert_eq!(render("{{aug|mayhem|Dive Bomber}}"), "Dive Bomber");
}

#[test]
fn wiki_aug_014() {
    assert_eq!(
        render("{{aug|mayhem|Final City Transit}}"),
        "Final City Transit"
    );
}

#[test]
fn wiki_aug_015() {
    assert_eq!(render("{{aug|mayhem|Self Destruct}}"), "Self Destruct");
}

#[test]
fn wiki_aug_016() {
    assert_eq!(
        render("{{aug|mayhem|Critical Missile}}"),
        "Critical Missile"
    );
}

#[test]
fn wiki_aug_017() {
    assert_eq!(render("{{aug|mayhem|Fan the Hammer}}"), "Fan the Hammer");
}

#[test]
fn wiki_aug_018() {
    assert_eq!(render("{{aug|mayhem|Light 'em Up!}}"), "Light 'em Up!");
}

#[test]
fn wiki_aug_019() {
    assert_eq!(render("{{aug|mayhem|Magic Missile}}"), "Magic Missile");
}

#[test]
fn wiki_lll_001() {
    assert_eq!(render("{{lll|Choncc}}"), "Choncc");
}

#[test]
fn wiki_lll_002() {
    assert_eq!(render("{{lll|Tocker|Base|Tocker}}"), "Tocker");
}

#[test]
fn wiki_lll_003() {
    assert_eq!(
        render("{{lll|Featherknight|Pengu|image=Pengu_Featherknight_profileicon.png|Pengu}}"),
        "Pengu"
    );
}

#[test]
fn wiki_lll_004() {
    assert_eq!(
        render("{{lll|Chibi Briar|Shork Cosplay|tier=1|circle=true|Chibi Shork Cosplay Briar}}"),
        "Chibi Shork Cosplay Briar"
    );
}

#[test]
fn wiki_lll_005() {
    assert_eq!(
        render("{{lll|Chibi Jinx|Base|tier=1|text=Chibi Jinx}}"),
        "Chibi Jinx"
    );
}

#[test]
fn wiki_lll_006() {
    assert_eq!(
        render("{{lll|Chibi Irelia|Base|tier=1|text=Chibi Irelia}}"),
        "Chibi Irelia"
    );
}

#[test]
fn wiki_lll_007() {
    assert_eq!(
        render("{{lll|Chibi Lee Sin|Base|tier=1|text=Chibi Lee Sin}}"),
        "Chibi Lee Sin"
    );
}

#[test]
fn wiki_lll_008() {
    assert_eq!(
        render("{{lll|Chibi Katarina|Base|tier=1|text=Chibi Katarina}}"),
        "Chibi Katarina"
    );
}

#[test]
fn wiki_lll_009() {
    assert_eq!(render("{{lll|Duckbill|Base|Duckbill}}"), "Duckbill");
}

#[test]
fn wiki_lll_010() {
    assert_eq!(render("{{lll|Choncc|Base|Choncc}}"), "Choncc");
}

#[test]
fn wiki_lll_011() {
    assert_eq!(render("{{lll|Gloop|Base|Gloop}}"), "Gloop");
}

#[test]
fn wiki_lll_012() {
    assert_eq!(render("{{lll|Featherknight|Pengu|Pengu}}"), "Pengu");
}

#[test]
fn wiki_ifeq_001() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Bayen|1|2}} ||background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_002() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Bayen|1|2}} |Background|background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_003() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Bayen|1|2}} |Strategy|background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_004() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Bayen|1|2}} |SkinsTrivia|background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_005() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Arthur|1|2}} ||background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_006() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Arthur|1|2}} |Background|background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_007() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Arthur|1|2}} |Strategy|background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_008() {
    assert_eq!(render("{{#ifeq: {{#titleparts:User:MetalBiM/Arthur|1|2}} |SkinsTrivia|background-color:#082848|background-color:#041424}}"), "background-color:#041424");
}

#[test]
fn wiki_ifeq_009() {
    assert_eq!(render("{{#ifeq:{{NAMESPACE}}|Template||not}}"), "not");
}

#[test]
fn wiki_ifeq_010() {
    assert_eq!(render("{{#ifeq:2||yes|no}}"), "no");
}

#[test]
fn wiki_switch_001() {
    assert_eq!(render("{{#switch:%TITLE%|%TITLE%=yes|#default=no}}"), "yes");
}

#[test]
fn wiki_minutedisplay_001() {
    assert_eq!(render("{{MinuteDisplay|60|-50|10}}"), "0:20");
}

#[test]
fn wiki_minutedisplay_002() {
    assert_eq!(render("{{MinuteDisplay|240|-30|35}}"), "4:05");
}
