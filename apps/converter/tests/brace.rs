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
    // These exercise real, messy champion dumps from the local `export_out/`
    // cache, which is gitignored and absent in fresh clones / CI. Read them at
    // runtime (not via compile-time `include_str!`, which would break the whole
    // test binary when the cache is missing) and skip when unavailable.
    let cache = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../export_out");
    let pages = [
        "Azir.txt",
        "Malzahar.txt",
        "Orianna.txt",
        "Bel%27Veth.txt",
        "Shaco.txt",
        "Zed.txt",
    ];
    for page in pages {
        let Ok(raw) = std::fs::read_to_string(cache.join(page)) else {
            eprintln!("skipping {page}: local export_out cache not present");
            continue;
        };
        let spans = extract_balanced_templates(&raw).unwrap();
        assert!(spans.iter().any(|span| span.name == "Champion info"));
        assert!(spans.iter().any(|span| span.name.contains("Patch box")));
    }
}
