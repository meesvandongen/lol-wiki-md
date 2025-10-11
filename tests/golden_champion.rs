use lol_wiki_md::convert_champion;

#[test]
fn golden_minimal_champion_hash_stable() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    // Flat export_out folder
    std::fs::create_dir_all(root.join("export_out")).unwrap();
    // Minimal champion main page (flat layout)
    std::fs::write(
        root.join("export_out").join("HashTester.txt"),
        "{{#vardefine:X|5}}Ability text {{#var:X}} {{ap|40}}",
    )
    .unwrap();
    // Module ChampionData (flat encoded path)
    std::fs::write(
        root.join("export_out")
            .join("Module%3AChampionData%2Fdata.txt"),
        "return { ['HashTester'] = { hp = 600, hpGrowth = 100 } }",
    )
    .unwrap();
    let out_dir = root.join("out");
    convert_champion(root, &out_dir, "HashTester", 2).unwrap();
    let md = std::fs::read_to_string(out_dir.join("HashTester.md")).unwrap();
    assert!(md.contains("HashTester"));
    assert!(md.contains("(+40% AP)"));
}
