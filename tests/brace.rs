use lol_wiki_md::parse::extract_balanced_templates;

#[test]
fn nested_templates() {
    let src = "Before {{Outer|a=1|b={{Inner|x=2}}|c=3}} After";
    let spans = extract_balanced_templates(src).unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "Outer");
}

#[test]
fn unbalanced_error() {
    let err = extract_balanced_templates("{{Bad").unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("E_UNBALANCED_BRACES") || msg.contains("unbalanced"));
}
