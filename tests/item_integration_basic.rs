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
    assert!(md.contains("Test Blade"));
    assert!(md.contains("(+40% AP)"));
    assert!(md.contains("Sharpen"));
    Ok(())
}
