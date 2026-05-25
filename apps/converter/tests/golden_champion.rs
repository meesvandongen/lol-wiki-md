use blake3::Hash;
use lol_wiki_md::{convert_champion, convert_item, convert_rune};

const EXPECTED_CHAMPION_HASH: &str =
    "d402ebdcbdd97ee73d17a49a9f0c04e3474fab0c65ad1d7445c532f5b0974d97";
const EXPECTED_ITEM_HASH: &str = "a78c0416b55399d46b2873ffa8c7fdf2fd45d02f3daac808be041a3aad661be8";
const EXPECTED_RUNE_HASH: &str = "e93f7b7b1345cc98f8ccd7e0e19de51a3c9953c72f8896875dbb9c78434d5f35";

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
