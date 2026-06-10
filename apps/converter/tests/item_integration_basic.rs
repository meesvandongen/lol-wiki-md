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
    // Stat labels come from the wiki's own Module:Gold value/data `name` field.
    std::fs::write(
        export.join("Module%3AGold value%2Fdata.txt"),
        r#"return {
  ["ad"]   = {["val"] = 35, ["name"] = "attack damage"},
  ["ah"]   = {["val"] = 50, ["name"] = "ability haste"},
  ["mana"] = {["val"] = 1,  ["name"] = "mana"},
}"#,
    )?;
    let out_dir = root.join("out");
    convert_item(&export, &out_dir, "Muramana", 2)?;
    let md = std::fs::read_to_string(out_dir.join("Muramana.md"))?;

    // Stat keys render with human-readable labels, not bare codes, and faithful
    // `+`-prefixed values resolved from the `=>Manamune` pointer.
    assert!(md.contains("| Attack Damage | +35 |"), "got:\n{md}");
    assert!(md.contains("| Ability Haste | +15 |"), "got:\n{md}");
    assert!(md.contains("| Mana | +1000 |"), "got:\n{md}");
    // The `=>Manamune` pointer must be resolved, never printed verbatim.
    assert!(!md.contains("=>"), "unresolved reference leaked:\n{md}");
    assert!(!md.contains("Manamune"), "raw reference leaked:\n{md}");
    Ok(())
}

/// Exercise every distinct stat-rendering shape (flat, percentage, per-5-second
/// regen, gold income, free-form `spec`, and a `*unique` variant) so the
/// fidelity rules transcribed from `Template:Infobox item/new/var` are pinned.
#[test]
fn item_stats_cover_all_rendering_shapes() -> Result<()> {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let export = root.join("export_out");
    std::fs::create_dir_all(&export)?;
    std::fs::write(export.join("Kitchen Sink.txt"), "{{Item info}}Kitchen Sink page")?;
    std::fs::write(
        export.join("Module%3AItemData%2Fdata.txt"),
        r#"return {
  ["Kitchen Sink"] = {
    tier = 3,
    type = {"Legendary"},
    buy = 3000,
    stats = {
      ad = 45,
      as = 30,
      crit = 25,
      cdr = 10,
      hp = 300,
      hp5 = 100,
      hp5flat = 36,
      mp5flat = 60,
      gp10 = 3,
      lethality = 15,
      ms = 5,
      msflat = 45,
      apunique = 40,
      spec = "Grants a [[shield]].",
    }
  }
}"#,
    )?;
    std::fs::write(
        export.join("Module%3AGold value%2Fdata.txt"),
        r#"return {
  ["ad"]      = {["val"] = 35, ["name"] = "attack damage"},
  ["as"]      = {["val"] = 25, ["name"] = "attack speed"},
  ["crit"]    = {["val"] = 40, ["name"] = "critical strike chance"},
  ["cdr"]     = {["val"] = 26, ["name"] = "cooldown reduction"},
  ["hp"]      = {["val"] = 2,  ["name"] = "health"},
  ["hp5"]     = {["val"] = 3,  ["name"] = "base health regeneration"},
  ["hp5flat"] = {["val"] = 36, ["name"] = "health regeneration"},
  ["mp5flat"] = {["val"] = 60, ["name"] = "mana regeneration"},
  ["lethality"] = {["val"] = 30, ["name"] = "lethality"},
  ["ms"]      = {["val"] = 65, ["name"] = "movement speed"},
  ["msflat"]  = {["val"] = 12, ["name"] = "movement speed"},
  ["ap"]      = {["val"] = 20, ["name"] = "ability power"},
}"#,
    )?;
    let out_dir = root.join("out");
    convert_item(&export, &out_dir, "Kitchen Sink", 2)?;
    let md = std::fs::read_to_string(out_dir.join("Kitchen_Sink.md"))?;

    let expected = [
        // Flat stats: `+value label`.
        "| Attack Damage | +45 |",
        "| Health | +300 |",
        "| Lethality | +15 |",
        "| Movement Speed | +45 |", // msflat is flat
        // `*unique` variants reuse the base stat's name and shape.
        "| Ability Power | +40 |",
        // Percentage stats: `+value% label`.
        "| Attack Speed | +30% |",
        "| Critical Strike Chance | +25% |",
        "| Cooldown Reduction | +10% |",
        "| Base Health Regeneration | +100% |",
        // ms (percent) shares the "movement speed" name but renders with %.
        // Per-5-second regen carries the trailing unit phrase.
        "| Health Regeneration | +36 per 5 seconds |",
        "| Mana Regeneration | +60 per 5 seconds |",
        // Gold income has no gold-value entry; labelled from ItemData's categoryTable.
        "| Gold Income | +3 per 10 seconds |",
        // Free-form spec text is printed verbatim (with wikilinks resolved) under
        // a "Special" label.
        "| Special | Grants a [shield](./shield.md). |",
    ];
    for row in expected {
        assert!(md.contains(row), "missing row `{row}` in:\n{md}");
    }
    // The percentage ms row coexists with the flat msflat row.
    assert!(md.contains("| Movement Speed | +5% |"), "got:\n{md}");
    Ok(())
}
