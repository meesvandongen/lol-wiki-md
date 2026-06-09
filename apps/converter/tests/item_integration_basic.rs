use lol_wiki_md::{convert_item, Result};

#[test]
fn minimal_item_conversion_produces_markdown() -> Result<()> {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let export = root.join("export_out");
    std::fs::create_dir_all(&export)?;
    std::fs::write(
        export.join("Test%20Blade.txt"),
        "{{Item info}}Test blade page",
    )?;
    let module_path = export.join("Module%3AItemData%2Fdata.txt");
    if let Some(parent) = module_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let lua = r#"return {
  ["Test Blade"] = {
    tier = 3,
    type = {"Legendary"},
    buy = 3300,
    stats = { ad = 70 },
    effects = {
      pass = { name = "Sharpen", unique = true, description = "Deals {{ap|40}} bonus damage." }
    }
  }
}"#;
    std::fs::write(&module_path, lua)?;
    let out_dir = root.join("out");
    convert_item(&export, &out_dir, "Test Blade", 2)?;
    let md = std::fs::read_to_string(out_dir.join("Test_Blade.md"))?;
    let plain = std::fs::read_to_string(out_dir.join("Test_Blade.plain.txt"))?;
    assert!(md.contains("Test Blade"));
    assert!(md.contains("Deals 40 bonus damage."));
    assert!(md.contains("Sharpen"));
    assert!(plain.contains("Test Blade"));
    assert!(plain.contains("Deals 40 bonus damage."));
    assert!(!plain.contains("**"));
    Ok(())
}

#[test]
fn item_stats_use_labels_and_resolve_references() -> Result<()> {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let export = root.join("export_out");
    std::fs::create_dir_all(&export)?;
    std::fs::write(export.join("Muramana.txt"), "{{Item info}}Muramana page")?;
    let module_path = export.join("Module%3AItemData%2Fdata.txt");
    if let Some(parent) = module_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    // Muramana inherits its `ad` and `ah` from Manamune via the wiki's `=>` pointer
    // and carries its own flat `mana`.
    let lua = r#"return {
  ["Manamune"] = {
    tier = 3,
    type = {"Legendary"},
    buy = 2900,
    stats = { ad = 35, ah = 15, mana = 500 }
  },
  ["Muramana"] = {
    tier = 3,
    type = {"Legendary"},
    buy = 2900,
    stats = { ad = "=>Manamune", ah = "=>Manamune", mana = 1000 }
  }
}"#;
    std::fs::write(&module_path, lua)?;
    let out_dir = root.join("out");
    convert_item(&export, &out_dir, "Muramana", 2)?;
    let md = std::fs::read_to_string(out_dir.join("Muramana.md"))?;

    // Stat keys render with human-readable labels, not bare codes.
    assert!(md.contains("| Attack Damage | 35 |"), "got:\n{md}");
    assert!(md.contains("| Ability Haste | 15 |"), "got:\n{md}");
    assert!(md.contains("| Mana | 1000 |"), "got:\n{md}");
    // The `=>Manamune` pointer must be resolved, never printed verbatim.
    assert!(!md.contains("=>"), "unresolved reference leaked:\n{md}");
    assert!(!md.contains("Manamune"), "raw reference leaked:\n{md}");
    Ok(())
}
