use lol_wiki_md::parse::brace::extract_balanced_templates;
use lol_wiki_md::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use std::collections::HashMap;

#[test]
fn var_define_and_reference() {
    let input = "Before {{#vardefine:Foo|42}} mid {{#var:Foo}} after";
    let spans = extract_balanced_templates(input).unwrap();
    // Collect vars
    let mut vars = HashMap::new();
    for s in &spans {
        let body = &s.raw[2..s.raw.len() - 2];
        let inv = parse_invocation(body);
        if inv.name.eq_ignore_ascii_case("#vardefine") {
            vars.insert(inv.params[0].clone(), inv.params[1].clone());
        }
    }
    let ctx = ExpanderCtx::new(2, &vars, None);
    let reg = TemplateRegistry::new();
    let mut out = String::new();
    let mut last = 0usize;
    for s in spans {
        out.push_str(&input[last..s.start]);
        let body = &s.raw[2..s.raw.len() - 2];
        let inv = parse_invocation(body);
        let exp = reg.expand(&inv, &ctx).unwrap();
        out.push_str(&exp.expanded);
        last = s.end;
    }
    out.push_str(&input[last..]);
    assert_eq!(out, "Before  mid 42 after");
}
