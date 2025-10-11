use crate::error::Result;
use crate::parse::extract_balanced_templates;
use crate::parse::templates::{parse_invocation, ExpanderCtx, TemplateRegistry};
use std::collections::HashMap;

/// Collect `#vardefine` assignments from the provided raw wikitext into a map.
pub fn collect_page_vars(raw: &str) -> Result<HashMap<String, String>> {
    let mut vars = HashMap::new();
    if let Ok(spans) = extract_balanced_templates(raw) {
        for span in spans {
            let body = &span.raw[2..span.raw.len() - 2];
            let inv = parse_invocation(body);
            if inv.name.eq_ignore_ascii_case("#vardefine") && inv.params.len() >= 2 {
                vars.insert(inv.params[0].clone(), inv.params[1].clone());
            }
        }
    }
    Ok(vars)
}

/// Expand all top-level templates in `raw` using the provided registry and variable map.
pub fn expand_with_vars(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<String> {
    if let Ok(spans) = extract_balanced_templates(raw) {
        let mut output = String::new();
        let mut last = 0usize;
        let ctx = ExpanderCtx {
            precision,
            vars: vars.clone(),
        };
        for span in spans {
            output.push_str(&raw[last..span.start]);
            let body = &span.raw[2..span.raw.len() - 2];
            let inv = parse_invocation(body);
            let exp = registry.expand(&inv, &ctx)?;
            output.push_str(&exp.expanded);
            last = span.end;
        }
        output.push_str(&raw[last..]);
        Ok(output)
    } else {
        Ok(raw.to_string())
    }
}

/// Iteratively expand inline templates inside a value until a fixed point or iteration cap is reached.
pub fn expand_inline_templates(
    raw: &str,
    precision: u8,
    vars: &HashMap<String, String>,
    registry: &TemplateRegistry,
) -> Result<String> {
    let mut curr = raw.to_string();
    let ctx = ExpanderCtx {
        precision,
        vars: vars.clone(),
    };
    for _ in 0..6 {
        let Ok(spans) = extract_balanced_templates(&curr) else {
            break;
        };
        if spans.is_empty() {
            break;
        }
        let mut out = String::new();
        let mut last = 0usize;
        for sp in spans {
            out.push_str(&curr[last..sp.start]);
            let body = &sp.raw[2..sp.raw.len() - 2];
            let inv = parse_invocation(body);
            let exp = registry.expand(&inv, &ctx)?;
            out.push_str(&exp.expanded);
            last = sp.end;
        }
        out.push_str(&curr[last..]);
        if out == curr {
            break;
        }
        curr = out;
        if !curr.contains("{{") {
            break;
        }
    }
    Ok(curr)
}
