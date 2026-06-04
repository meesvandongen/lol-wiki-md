use lol_wiki_md::parse::extract_balanced_templates;

#[test]
fn nested_templates() {
    let src = "Before {{Outer|a=1|b={{Inner|x=2}}|c=3}} After";
    let spans = extract_balanced_templates(src).unwrap();
    assert_eq!(spans.len(), 1);
    assert_eq!(spans[0].name, "Outer");
}

#[test]
fn parameter_after_sibling_template_does_not_overlap() {
    // Regression: a trailing `{{{param}}}` used to make parameter recovery
    // latch onto the `{{` of the earlier `{{#if:}}` sibling, yielding an
    // overlapping/out-of-order span that panicked downstream slicing.
    let src = "({{{1|0}}}){{#if:{{{mod|}}}|*({{{mod}}})}}-({{{base|0}}})";
    let spans = extract_balanced_templates(src).unwrap();
    // Spans must be sorted and non-overlapping.
    let mut last_end = 0usize;
    for span in &spans {
        assert!(
            span.start >= last_end,
            "spans must be sorted and disjoint, got {span:?} after end {last_end}"
        );
        assert!(span.start < span.end);
        last_end = span.end;
    }
    // The only real template here is the `{{#if:}}`.
    assert!(spans.iter().any(|s| s.name.starts_with("#if")));
}

#[test]
fn unbalanced_error() {
    let err = extract_balanced_templates("{{Bad").unwrap_err();
    let msg = format!("{}", err);
    assert!(msg.contains("E_UNBALANCED_BRACES") || msg.contains("unbalanced"));
}

#[test]
fn real_pages_with_html_warnings_still_extract_templates() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../export_out/Azir.txt"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../export_out/Malzahar.txt"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../export_out/Orianna.txt"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../export_out/Bel%27Veth.txt"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../export_out/Shaco.txt"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../export_out/Zed.txt"
        )),
    ] {
        let spans = extract_balanced_templates(raw).unwrap();
        assert!(spans.iter().any(|span| span.name == "Champion info"));
        assert!(spans.iter().any(|span| span.name.contains("Patch box")));
    }
}
