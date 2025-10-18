use blake3::Hash;
use lol_wiki_md::{convert_champion, convert_item, convert_rune};

const EXPECTED_CHAMPION_HASH: &str =
    "c6bb938b2e98c40c7726d670c2ceea49a7a1118aee1fc480c6de6dc98909d030";
const EXPECTED_ITEM_HASH: &str = "3266d40aeb239892f34229e06fda7199d9362519daf64f08dc1f59986e58202b";
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
        "return { ['TestChamp'] = { hp = 600, hpGrowth = 100, ad = 60, adGrowth = 3 } }",
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
