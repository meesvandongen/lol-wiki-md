//! End-to-end conversion of real wiki pages, from committed fixtures.
//!
//! Unlike a live-export integration test, this is fully self-contained: the
//! fixtures under `tests/fixtures/mini_export/` are the exact file-dependency
//! closure (captured from a real wiki download) needed to convert one champion,
//! one item, and one rune — the champion/item data modules are trimmed to just
//! the referenced entries (and an item's recipe components) so the tree stays
//! small. No network, no skip.
//!
//! The guarantee under test: real wikitext — multi-form ability `Data`
//! templates, an item with a computed combine cost, a rune with structured
//! sections — flows through the whole pipeline and the fail-fast template
//! registry recognizes every template it transcludes. A clean full run over the
//! complete wiki download was 172/172 champions, 594/594 items, 77/77 runes
//! (see `scripts/test-mining/DIVERGENCES.md`).

use lol_wiki_md::convert::context::ConversionContext;
use lol_wiki_md::convert::util::expand_inline_templates;
use lol_wiki_md::parse::templates::TemplateRegistry;
use lol_wiki_md::{convert_champion, convert_item, convert_rune};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini_export")
}

#[test]
fn real_champion_page_converts_with_full_kit() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path();
    convert_champion(&fixture_root(), out, "Aatrox", 2).expect("Aatrox should convert");
    let md = std::fs::read_to_string(out.join("Aatrox.md")).unwrap();

    // Stats come from the trimmed Module:ChampionData entry.
    assert!(md.contains("# Aatrox"));
    assert!(md.contains("## Stats"));
    assert!(md.contains("| HP | 650 |"), "base HP from module missing:\n{md}");

    // All five real ability slots resolve from the Data Aatrox/* templates.
    assert!(md.contains("## Abilities"));
    for ability in [
        "Passive – Deathbringer Stance",
        "Q – The Darkin Blade",
        "W – Infernal Chains",
        "E – Umbral Dash",
        "R – World Ender",
    ] {
        assert!(md.contains(ability), "missing ability `{ability}`");
    }
}

#[test]
fn real_item_page_computes_combine_cost() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path();
    convert_item(&fixture_root(), out, "Infinity Edge", 2).expect("Infinity Edge should convert");
    let md = std::fs::read_to_string(out.join("Infinity_Edge.md")).unwrap();

    assert!(md.contains("# Infinity Edge"));
    // Combine cost is derived from the recipe components present in the fixture
    // (B. F. Sword + Cloak of Agility + Pickaxe), not read directly.
    assert!(
        md.contains("Combine: 725"),
        "combine cost not computed from components:\n{md}"
    );
    assert!(
        !md.contains("not found in Module:ItemData"),
        "unexpected missing-component warning:\n{md}"
    );
}

#[test]
fn real_rune_page_converts_with_sections() {
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path();
    convert_rune(&fixture_root(), out, "Electrocute", 2).expect("Electrocute should convert");
    let md = std::fs::read_to_string(out.join("Electrocute.md")).unwrap();

    assert!(md.contains("# Electrocute"));
    assert!(md.contains("## Patch History"));
}

/// Data-lookup templates resolve against the (trimmed) real modules, including a
/// `ccd` lookup nested inside `ap` arithmetic — the shape attack-windup ratios
/// use on the wiki. `as_base` is read directly from the module; `crit_base` is
/// absent and must fall back to the getter's `or 200`.
#[test]
fn real_data_lookups_and_nesting_resolve() {
    let ctx = Arc::new(ConversionContext::new(&fixture_root(), 2).unwrap());
    ctx.champion_constants_or_load("Aatrox");
    let reg = TemplateRegistry::new();
    let vars = HashMap::new();
    let expand =
        |text: &str| expand_inline_templates(text, 2, &vars, &reg, Some(ctx.clone())).unwrap();

    assert_eq!(expand("{{ccd|Aatrox|as_base}}"), "0.651");
    assert_eq!(expand("{{ccd|Aatrox|crit_base}}"), "200");
    assert_eq!(expand("{{cid|Infinity Edge|critdamage}}"), "30");
    // (1 / 0.651) * 1.6 = 2.458 (rounded to 3 places).
    assert_eq!(
        expand("{{ap|(1/{{ccd|Aatrox|as_base}})*1.6|round=3}}"),
        "2.458"
    );
}
