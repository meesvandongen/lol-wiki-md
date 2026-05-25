// (no unused imports)
use lol_wiki_md::{convert_champion, Result};

#[test]
fn minimal_champion_conversion_does_not_panic() -> Result<()> {
    // Create a temporary wiki root structure with minimal files (flat export_out format)
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Flat export_out folder
    std::fs::create_dir_all(root.join("export_out"))?;
    // Main page: TestChamp.txt
    std::fs::write(
        root.join("export_out").join("TestChamp.txt"),
        "{{#vardefine:x|10}}Test page with {{#var:x}} and {{ap|40}}\n{{Data TestChamp/I|Ability}}",
    )?;
    // ChampionData lua: Module%3AChampionData%2Fdata.txt
    let lua = r#"return { ["TestChamp"] = { hp = 600, hpGrowth = 100, ad = 60, adGrowth = 3 } }"#;
    std::fs::write(
        root.join("export_out")
            .join("Module%3AChampionData%2Fdata.txt"),
        lua,
    )?;
    // Ability data template
    std::fs::write(
        root.join("export_out")
            .join("Template%3AData%20TestChamp%2FI.txt"),
        "{{#vardefine:scale|0.5}}\n{{AbilityData|champion=TestChamp|skill=I|name=Heroic|description=Deals {{ap|{{#var:scale}}*100}} bonus damage.|cooldown=10 / 8|notes=* First note about the passive\n** Nested detail\nAdditional context}}",
    )?;
    println!(
        "[test] root={} export_out exists={} test file exists={}",
        root.display(),
        root.join("export_out").exists(),
        root.join("export_out/TestChamp.txt").exists()
    );
    if let Ok(rd) = std::fs::read_dir(root.join("export_out")) {
        for e in rd.flatten() {
            println!("[test] export_out has {}", e.path().display());
        }
    }
    let out_dir = root.join("out");
    // Use the export_out folder as wiki root
    convert_champion(&root.join("export_out"), &out_dir, "TestChamp", 2)?;
    let md = std::fs::read_to_string(out_dir.join("TestChamp.md"))?;
    let plain = std::fs::read_to_string(out_dir.join("TestChamp.plain.txt"))?;
    assert!(md.contains("TestChamp"));
    assert!(md.contains("Test page with 10 and 40"));
    assert!(md.contains("Passive – Heroic"));
    assert!(md.contains("Deals 50 bonus damage."));
    assert!(md.contains("**Notes:**"));
    assert!(md.contains("- First note"));
    assert!(md.contains("  - Nested detail Additional context"));
    assert!(plain.contains("TestChamp"));
    assert!(plain.contains("Passive – Heroic"));
    assert!(plain.contains("Deals 50 bonus damage."));
    assert!(plain.contains("Notes:"));
    assert!(!plain.contains("**"));
    Ok(())
}
