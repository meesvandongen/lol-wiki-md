use blake3::Hash;
use lol_wiki_md::{convert_champion, convert_item, convert_rune};

const EXPECTED_CHAMPION_HASH: &str =
    "49e93a8b5b2aafdf91c35a7bb66039bb4961f448324ff94f24ff739d2d69d965";
const EXPECTED_ITEM_HASH: &str = "7461b800e829839c1654e9b5213ebf9e840149906939d3868154c4168b29c10b";
const EXPECTED_RUNE_HASH: &str = "58d3a3086a5d583a60dec1aef59ff3838211e1bb8a3a1b5aabe748316d231fc9";

#[test]
fn golden_champion_output_hash() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let export = root.join("export_out");
    std::fs::create_dir_all(&export).unwrap();

    std::fs::write(
        export.join("TestChamp.txt"),
        "{{#vardefine:x|10}}Test body {{#var:x}} {{ap|40}}\n{{Data TestChamp/I|Ability}}",
    )
    .unwrap();

    std::fs::write(
        export.join("Module%3AChampionData%2Fdata.txt"),
        "return { ['TestChamp'] = { stats = { hp_base = 600, hp_lvl = 100, dam_base = 60, dam_lvl = 3, as_ratio = 0.625, attack_cast_time = 0.3, attack_total_time = 1.0, acquisition_radius = 550 } } }",
    )
    .unwrap();

    std::fs::write(
        export.join("Template%3AData%20TestChamp%2FI.txt"),
        "{{#vardefine:scale|0.5}}\n{{AbilityData|champion=TestChamp|skill=I|name=Heroic Swing|description=Deals {{ap|{{#var:scale}}*100}} bonus damage.|cooldown=10 / 8|notes=* First note about the passive\n** Nested detail\nAdditional context}}",
    )
    .unwrap();

    let out_dir = root.join("out");
    convert_champion(&export, &out_dir, "TestChamp", 2).unwrap();
    let md = std::fs::read_to_string(out_dir.join("TestChamp.md")).unwrap();
    assert_eq!(hash_hex(&md), EXPECTED_CHAMPION_HASH);
}

#[test]
fn ability_with_only_falsey_details_emits_no_empty_table() {
    // An ability whose only detail value normalizes away (e.g. `spellshield=No`)
    // must not leave an empty `| Detail | Value |` header behind.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let export = root.join("export_out");
    std::fs::create_dir_all(&export).unwrap();

    std::fs::write(
        export.join("TestChamp.txt"),
        "Body\n{{Data TestChamp/Q|Ability}}",
    )
    .unwrap();
    std::fs::write(
        export.join("Module%3AChampionData%2Fdata.txt"),
        "return { ['TestChamp'] = { stats = { hp_base = 600, hp_lvl = 100, dam_base = 60, dam_lvl = 3, as_ratio = 0.625, attack_cast_time = 0.3, attack_total_time = 1.0, acquisition_radius = 550 } } }",
    )
    .unwrap();
    std::fs::write(
        export.join("Template%3AData%20TestChamp%2FQ.txt"),
        "{{AbilityData|champion=TestChamp|skill=Q|name=Plain Strike|description=Deals damage.|spellshield=No|projectile=No}}",
    )
    .unwrap();

    let out_dir = root.join("out");
    convert_champion(&export, &out_dir, "TestChamp", 2).unwrap();
    let md = std::fs::read_to_string(out_dir.join("TestChamp.md")).unwrap();

    assert!(md.contains("Plain Strike"));
    assert!(
        !has_empty_table(&md),
        "rendered markdown contains an empty table:\n{md}"
    );
}

/// True if any table header line is immediately followed by a separator row
/// and then a line that is not a table row (i.e. the table has no body).
fn has_empty_table(md: &str) -> bool {
    let lines: Vec<&str> = md.lines().collect();
    for window in lines.windows(3) {
        let [header, separator, after] = window else {
            continue;
        };
        let is_header = header.starts_with("| ") && header.ends_with(" |");
        let is_separator = separator.starts_with('|') && separator.contains("---");
        if is_header && is_separator && !after.trim_start().starts_with('|') {
            return true;
        }
    }
    false
}

#[test]
fn golden_item_output_hash() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let export = root.join("export_out");
    std::fs::create_dir_all(&export).unwrap();

    std::fs::write(
        export.join("Test%20Blade.txt"),
        "{{Item info}}Test blade page",
    )
    .unwrap();
    let module_path = export.join("Module%3AItemData%2Fdata.txt");
    std::fs::write(
        &module_path,
        r#"return {
  ["Test Blade"] = {
    tier = 3,
    type = {"Legendary"},
    buy = 3300,
    recipe = {"Long Sword", "Sheen"},
    stats = { ad = 70, haste = 20 },
    effects = {
      pass = { name = "Sharpen", unique = true, description = "Deals {{ap|40}} bonus damage." }
    }
  }
}"#,
    )
    .unwrap();

    let out_dir = root.join("out");
    convert_item(&export, &out_dir, "Test Blade", 2).unwrap();
    let md = std::fs::read_to_string(out_dir.join("Test_Blade.md")).unwrap();
    assert_eq!(hash_hex(&md), EXPECTED_ITEM_HASH);
}

#[test]
fn golden_rune_output_hash() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let export = root.join("export_out");
    std::fs::create_dir_all(&export).unwrap();

    std::fs::write(
        export.join("Trailblazer.txt"),
        "Trailblazer rune overview.\n\n== Notes ==\n* Gains bonus movement speed after eliminations.\n\n== Trivia ==\n* Named after early explorers.\n\n== Patch History ==\n* '''V14.3:''' Introduced.",
    )
    .unwrap();

    let out_dir = root.join("out");
    convert_rune(&export, &out_dir, "Trailblazer", 2).unwrap();
    let md = std::fs::read_to_string(out_dir.join("Trailblazer.md")).unwrap();
    assert_eq!(hash_hex(&md), EXPECTED_RUNE_HASH);
}

fn hash_hex(contents: &str) -> String {
    let hash: Hash = blake3::hash(contents.as_bytes());
    hash.to_hex().to_string()
}
