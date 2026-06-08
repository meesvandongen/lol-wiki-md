//! Corpus-driven coverage tests.
//!
//! These exercise real, full champion/item/rune pages from a downloaded wiki
//! export. Like the `real_pages_with_html_warnings_*` test in `brace.rs`, they
//! read the export at runtime and **skip gracefully when it is absent** (fresh
//! clones / CI without a download). Populate the cache with either:
//!
//! ```text
//! cargo run -p lol-wiki-export -- \
//!   --page-list /tmp/page_list.txt \
//!   --out-dir ./export_out --meta-dir ./export_out_meta
//! ```
//!
//! or a full `npm run download` (which writes `generated/wiki-export/out`).
//!
//! The guarantee under test: the fail-fast template registry recognizes every
//! template actually reached on the convertible champion/item/rune surface — a
//! conversion that hit an unknown/unexpandable template would return an error
//! rather than completing. A clean full run over the mined corpus was 170/170
//! champions, 305/305 items, 76/76 runes (see
//! `scripts/test-mining/DIVERGENCES.md`).

use lol_wiki_md::convert::context::ConversionContext;
use lol_wiki_md::convert::util::expand_inline_templates;
use lol_wiki_md::parse::templates::TemplateRegistry;
use lol_wiki_md::{convert_champion, convert_item, convert_rune};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Locate a usable export root, preferring the canonical pipeline output and
/// falling back to the local quick-validation cache. Returns `None` when
/// neither exists so callers can skip.
fn locate_export() -> Option<PathBuf> {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    for candidate in [
        manifest.join("../../generated/wiki-export/out"),
        manifest.join("../../export_out"),
    ] {
        // Require at least one champion-marker page to consider it populated.
        if candidate.join("Aatrox.txt").exists() {
            return Some(candidate);
        }
    }
    None
}

#[test]
fn diverse_real_pages_convert_without_error() {
    let Some(export) = locate_export() else {
        eprintln!("skipping: no wiki export present (export_out / generated/wiki-export/out)");
        return;
    };
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path();

    // A deliberately diverse slice exercising distinct template machinery:
    // standard kits, multi-form transforms, pets, energy/no-resource, and the
    // GnarVar-style `#var`/`ccd` interplay.
    let champions = [
        "Aatrox",  // standard melee fighter
        "Ahri",    // mana mage, multi-cast
        "Gnar",    // transform forms + GnarVar #var lookups
        "Kayn",    // form transform
        "Yorick",  // pets / summons
        "Senna",   // unusual scaling
        "Viego",   // possession mechanics
        "Sona",    // aura / power chords
        "Ornn",    // masterwork item interactions
        "Kindred", // stacking / rd ranges
    ];
    for champ in champions {
        convert_champion(&export, out, champ, 2)
            .unwrap_or_else(|e| panic!("champion {champ} failed to convert: {e}"));
        let md = std::fs::read_to_string(out.join(format!("{champ}.md")))
            .unwrap_or_else(|_| panic!("no markdown emitted for {champ}"));
        assert!(md.contains(champ), "{champ} markdown missing its own name");
        assert!(
            md.contains("## Abilities") && md.contains("## Stats"),
            "{champ} markdown missing core sections"
        );
    }

    let items = [
        "Infinity Edge", // crit + passive
        "Trinity Force", // multi-stat recipe
        "Sunfire Aegis", // burn aura
        "Bloodthirster", // lifesteal + shield
        "Kraken Slayer", // on-hit stacks
    ];
    for item in items {
        convert_item(&export, out, item, 2)
            .unwrap_or_else(|e| panic!("item {item} failed to convert: {e}"));
        // Output filename mirrors the title with spaces -> underscores.
        let path = out.join(format!("{}.md", item.replace(' ', "_")));
        assert!(path.exists(), "no markdown emitted for item {item}");
    }

    let runes = ["Electrocute", "Conqueror", "Grasp of the Undying"];
    for rune in runes {
        convert_rune(&export, out, rune, 2)
            .unwrap_or_else(|e| panic!("rune {rune} failed to convert: {e}"));
    }
}

/// Data-lookup templates resolve against the real exported modules, including
/// a `ccd` lookup nested inside `ap` arithmetic — the shape Talon's attack
/// windup uses on the wiki. `as_base` is read directly from
/// `Module:ChampionData/data`; `(1 / 0.625) * 1.6 = 2.56`.
#[test]
fn real_data_lookups_resolve() {
    let Some(export) = locate_export() else {
        eprintln!("skipping: no wiki export present");
        return;
    };
    let ctx = Arc::new(ConversionContext::new(&export, 2).unwrap());
    ctx.champion_constants_or_load("Talon");
    let reg = TemplateRegistry::new();
    let vars = HashMap::new();
    let expand =
        |text: &str| expand_inline_templates(text, 2, &vars, &reg, Some(ctx.clone())).unwrap();
    assert_eq!(expand("{{ccd|Talon|as_base}}"), "0.625");
    assert_eq!(expand("{{cid|Infinity Edge|critdamage}}"), "30");
    assert_eq!(
        expand("{{ap|(1/{{ccd|Talon|as_base}})*1.6|round=3}}"),
        "2.56"
    );
}
