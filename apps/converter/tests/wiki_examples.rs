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
