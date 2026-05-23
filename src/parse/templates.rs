//! Template expansion framework (minimal subset).
use crate::convert::context::ConversionContext;
use crate::convert::util::expand_inline_templates;
use crate::error::{ConvertError, Result};
use crate::parse::default_wiki_configuration;
use crate::parse::expr::evaluate_expression;
use crate::parse::lua::LuaValue;
use std::collections::HashMap;
use std::sync::Arc;

use parse_wiki_text::{Node, Positioned};

pub trait ConversionContextTrait {
    fn champion_constants(&self, key: &str) -> Option<HashMap<String, String>>;
    fn item_module_map(&self) -> Result<&HashMap<String, HashMap<String, LuaValue>>>;
}

#[derive(Debug, Clone)]
pub struct TemplateInvocation {
    pub name: String,
    pub raw: String,
    pub params: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ExpansionResult {
    pub expanded: String,
}

pub trait TemplateExpander: Send + Sync {
    fn names(&self) -> &'static [&'static str];
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult>;
}

#[derive(Default)]
pub struct TemplateRegistry {
    expanders: Vec<Box<dyn TemplateExpander>>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut r = Self::default();
        r.register(Box::new(ExprExpander));
        r.register(Box::new(IfExpander));
        r.register(Box::new(IfEqExpander));
        r.register(Box::new(SwitchExpander));
        r.register(Box::new(TooltipExpander));
        r.register(Box::new(VarDefineExpander));
        r.register(Box::new(VarRefExpander));
        r.register(Box::new(ApExpander));
        r.register(Box::new(PpExpander));
        r.register(Box::new(PpTooltipExpander));
        r.register(Box::new(FdExpander));
        r.register(Box::new(IconUnwrapExpander));
        r.register(Box::new(AsExpander));
        r.register(Box::new(StiExpander));
        r.register(Box::new(TimesExpander));
        r.register(Box::new(GoldExpander));
        r.register(Box::new(CriticalDamageExpander));
        r.register(Box::new(QuoteExpander));
        r.register(Box::new(TipExpander));
        r.register(Box::new(WExpander));
        r.register(Box::new(SbcExpander));
        r.register(Box::new(ChannelTypeExpander));
        r.register(Box::new(SkillTabExpander));
        r.register(Box::new(FlipTextExpander));
        r.register(Box::new(ClearExpander));
        r.register(Box::new(ChampionWithInfiniteScalingExpander));
        r.register(Box::new(CcsExpander));
        r.register(Box::new(DegreeExpander));
        r.register(Box::new(DividedByExpander));
        r.register(Box::new(ColorExpander));
        r.register(Box::new(ConstantDataExpander));
        r.register(Box::new(IncludeInfoExpander));
        r.register(Box::new(NeutralizeExpander));
        r.register(Box::new(ChimeListExpander));
        r
    }
    pub fn register(&mut self, ex: Box<dyn TemplateExpander>) {
        self.expanders.push(ex);
    }
    pub fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        for e in &self.expanders {
            if e.names().iter().any(|n| n.eq_ignore_ascii_case(&inv.name)) {
                if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
                    conv_ctx.record_template_params(&inv.name, &inv.params);
                }
                return e.expand(inv, ctx);
            }
        }
        // Span-aware variant would require caller to attach; for now keep legacy
        Err(ConvertError::UnknownTemplate {
            name: inv.name.clone(),
        })
    }
    pub fn has_name(&self, name: &str) -> bool {
        self.expanders
            .iter()
            .any(|e| e.names().iter().any(|n| n.eq_ignore_ascii_case(name)))
    }
    pub fn list_names(&self) -> Vec<&'static str> {
        let mut out: Vec<&'static str> = Vec::new();
        for e in &self.expanders {
            for &n in e.names() {
                out.push(n);
            }
        }
        out.sort();
        out.dedup();
        out
    }
}

pub struct ExpanderCtx {
    pub precision: u8,
    pub vars: HashMap<String, String>,
    pub conversion_ctx: Option<Arc<ConversionContext>>,
}

// --- Simple expanders ---

struct ExprExpander;
impl TemplateExpander for ExprExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#expr"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let expr_body = inv
            .params
            .first()
            .map(|s| s.as_str())
            .unwrap_or_else(|| inv.raw.trim());
        // Pre-expand simple #var templates within the expression using current vars
        let pre = replace_var_templates(expr_body, &ctx.vars);
        match evaluate_expression(&pre, super::expr::ExprNumberFormat::Float(ctx.precision)) {
            Ok(v) => Ok(ExpansionResult { expanded: v }),
            Err(_e) => {
                // Fallback: strip any remaining template constructs and return the textual expr
                let cleaned = strip_templates(&pre);
                Ok(ExpansionResult { expanded: cleaned })
            }
        }
    }
}

fn replace_var_templates(input: &str, vars: &HashMap<String, String>) -> String {
    let mut out = String::new();
    let bytes = input.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if i + 7 < bytes.len() && &bytes[i..i + 7] == b"{{#var:" {
            let start = i + 7; // after "{{#var:"
                               // Read until '|' or '}'
            let mut j = start;
            while j < bytes.len() && bytes[j] != b'|' && bytes[j] != b'}' {
                j += 1;
            }
            let name = &input[start..j];
            // Advance to end of template '}}'
            let mut k = j;
            while k + 1 < bytes.len() && !(bytes[k] == b'}' && bytes[k + 1] == b'}') {
                k += 1;
            }
            let val = vars.get(name).cloned().unwrap_or_default();
            out.push_str(&val);
            i = if k + 2 <= bytes.len() {
                k + 2
            } else {
                bytes.len()
            };
        } else {
            out.push(bytes[i] as char);
            i += 1;
        }
    }
    out
}

fn strip_templates(input: &str) -> String {
    let mut out = String::new();
    let mut i = 0usize;
    let b = input.as_bytes();
    while i < b.len() {
        if i + 1 < b.len() && b[i] == b'{' && b[i + 1] == b'{' {
            // skip until matching '}}'
            i += 2;
            let mut depth = 1i32;
            while i + 1 < b.len() && depth > 0 {
                if b[i] == b'{' && b[i + 1] == b'{' {
                    depth += 1;
                    i += 2;
                    continue;
                }
                if b[i] == b'}' && b[i + 1] == b'}' {
                    depth -= 1;
                    i += 2;
                    continue;
                }
                i += 1;
            }
            continue;
        }
        out.push(b[i] as char);
        i += 1;
    }
    out
}

// Attribute style expander: {{as|text}} -> text (drop styling markers like ms parameter)
struct AsExpander;
impl TemplateExpander for AsExpander {
    fn names(&self) -> &'static [&'static str] {
        &["as"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Ok(ExpansionResult {
                expanded: String::new(),
            });
        }
        Ok(ExpansionResult {
            expanded: inv.params[0].trim().to_string(),
        })
    }
}

// Styled inline marker expander: {{sti|key|value}} -> value (or first non-empty)
struct StiExpander;
impl TemplateExpander for StiExpander {
    fn names(&self) -> &'static [&'static str] {
        &["sti"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        // Many usages are sti|ms|text or sti|text; pick the last param as display
        let val = inv.params.last().cloned().unwrap_or_default();
        Ok(ExpansionResult { expanded: val })
    }
}

// Times joiner: {{times}} -> × or concatenation marker; here return "×"
struct TimesExpander;
impl TemplateExpander for TimesExpander {
    fn names(&self) -> &'static [&'static str] {
        &["times"]
    }
    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: "x".into(),
        })
    }
}

struct ClearExpander;
impl TemplateExpander for ClearExpander {
    fn names(&self) -> &'static [&'static str] {
        &["clear"]
    }
    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

struct ChampionWithInfiniteScalingExpander;
impl TemplateExpander for ChampionWithInfiniteScalingExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Champion with infinite scaling"]
    }
    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

struct ChimeMilestone {
    time: &'static str,
    chimes: u32,
    effect: &'static str,
}

const CHIME_MILESTONES: &[ChimeMilestone] = &[
    ChimeMilestone {
        time: "3:20",
        chimes: 5,
        effect: "Meeps now slow by 25%.",
    },
    ChimeMilestone {
        time: "5:00",
        chimes: 10,
        effect: "Meep limit increased to 2.",
    },
    ChimeMilestone {
        time: "7:30",
        chimes: 15,
        effect: "Meeps now deal splash damage.",
    },
    ChimeMilestone {
        time: "9:10",
        chimes: 20,
        effect: "Recharge time reduced to 7 seconds.",
    },
    ChimeMilestone {
        time: "11:40",
        chimes: 25,
        effect: "Slow increased to 35%.",
    },
    ChimeMilestone {
        time: "13:20",
        chimes: 30,
        effect: "Meep limit increased to 3.",
    },
    ChimeMilestone {
        time: "15:50",
        chimes: 35,
        effect: "Splash damage area increased.",
    },
    ChimeMilestone {
        time: "17:30",
        chimes: 40,
        effect: "Recharge time reduced to 6 seconds.",
    },
    ChimeMilestone {
        time: "20:00",
        chimes: 45,
        effect: "Slow increased to 45%.",
    },
    ChimeMilestone {
        time: "21:40",
        chimes: 50,
        effect: "Meep limit increased to 4.",
    },
    ChimeMilestone {
        time: "24:10",
        chimes: 55,
        effect: "Recharge time reduced to 5 seconds.",
    },
    ChimeMilestone {
        time: "25:50",
        chimes: 60,
        effect: "Slow increased to 55%.",
    },
    ChimeMilestone {
        time: "28:20",
        chimes: 65,
        effect: "Meep limit increased to 5.",
    },
    ChimeMilestone {
        time: "30:00",
        chimes: 70,
        effect: "Recharge time reduced to 4 seconds.",
    },
    ChimeMilestone {
        time: "32:30",
        chimes: 75,
        effect: "Slow increased to 65%.",
    },
    ChimeMilestone {
        time: "34:10",
        chimes: 80,
        effect: "Meep limit increased to 6.",
    },
    ChimeMilestone {
        time: "36:40",
        chimes: 85,
        effect: "Slow increased to 75%.",
    },
    ChimeMilestone {
        time: "38:20",
        chimes: 90,
        effect: "Meep limit increased to 7.",
    },
    ChimeMilestone {
        time: "40:50",
        chimes: 95,
        effect: "Meep limit increased to 8.",
    },
    ChimeMilestone {
        time: "42:30",
        chimes: 100,
        effect: "Meep limit increased to 9.",
    },
];

struct ChimeListExpander;
impl TemplateExpander for ChimeListExpander {
    fn names(&self) -> &'static [&'static str] {
        &["chime list"]
    }
    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        const BASE_DAMAGE: i32 = 35;
        const DAMAGE_PER_STEP: i32 = 14;
        let mut out = String::new();
        out.push_str("| Minimum Time | Chimes | Effect | Base Damage |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for milestone in CHIME_MILESTONES {
            let steps = (milestone.chimes / 5) as i32;
            let damage = BASE_DAMAGE + DAMAGE_PER_STEP * steps;
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                milestone.time, milestone.chimes, milestone.effect, damage
            ));
        }
        out.push_str("\nEvery additional 5 chimes collected beyond 100 grant +14 bonus damage.\n");
        Ok(ExpansionResult { expanded: out })
    }
}

struct CcsExpander;
impl TemplateExpander for CcsExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ccs"]
    }
    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

struct DegreeExpander;
impl TemplateExpander for DegreeExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Degree", "degree", "degrees"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let positional: Vec<&str> = inv
            .params
            .iter()
            .filter_map(|p| {
                let trimmed = p.trim();
                if trimmed.is_empty() || trimmed.contains('=') {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .collect();
        if positional.is_empty() {
            return Ok(ExpansionResult {
                expanded: "°".to_string(),
            });
        }
        let mut out = String::new();
        out.push_str(positional[0]);
        out.push('°');
        if let Some(min) = positional.get(1) {
            if !min.is_empty() {
                out.push_str(min);
                out.push('′');
            }
        }
        if let Some(sec) = positional.get(2) {
            if !sec.is_empty() {
                out.push_str(sec);
                out.push('″');
            }
        }
        if let Some(dir) = positional.get(3) {
            if !dir.is_empty() {
                out.push_str(dir);
            }
        }
        Ok(ExpansionResult { expanded: out })
    }
}

struct DividedByExpander;
impl TemplateExpander for DividedByExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Divided by"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let left = inv.params.get(0).map(|s| s.trim()).unwrap_or("");
        let right = inv.params.get(1).map(|s| s.trim()).unwrap_or("");
        if left.is_empty() && right.is_empty() {
            return Ok(ExpansionResult {
                expanded: String::new(),
            });
        }
        let mut out = String::new();
        if !left.is_empty() {
            out.push_str(left);
        }
        out.push_str(" / ");
        if !right.is_empty() {
            out.push_str(right);
        }
        Ok(ExpansionResult { expanded: out })
    }
}

struct ColorExpander;
impl TemplateExpander for ColorExpander {
    fn names(&self) -> &'static [&'static str] {
        &["color", "Color"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Ok(ExpansionResult {
                expanded: String::new(),
            });
        }
        let text_param = if inv.params.len() >= 2 {
            inv.params[1].trim()
        } else {
            inv.params[0].trim()
        };
        Ok(ExpansionResult {
            expanded: text_param.to_string(),
        })
    }
}

const INFO_INCLUDE_TEMPLATES: &[&str] = &[
    "Spellblade info",
    "Energized info",
    "Diminishing gold info",
    "Manaflow info",
    "Quicksilver info",
    "Elixir info",
    "Ward timer info",
    "Zombie state info",
];

struct IncludeInfoExpander;
impl TemplateExpander for IncludeInfoExpander {
    fn names(&self) -> &'static [&'static str] {
        INFO_INCLUDE_TEMPLATES
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
            return Err(ConvertError::Internal(format!(
                "conversion context required for template '{}', but not provided",
                inv.name
            )));
        };
        let include = conv_ctx
            .template_includeonly(&inv.name)?
            .ok_or_else(|| ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "template page missing <includeonly> section".into(),
            })?;
        let registry = conv_ctx.registry();
        let expanded = expand_inline_templates(
            include.trim(),
            ctx.precision,
            &ctx.vars,
            registry,
            ctx.conversion_ctx.clone(),
        )?;
        Ok(ExpansionResult {
            expanded: expanded.trim_end().to_string(),
        })
    }
}

// Gold amount: {{g|100}} -> 100 (drop currency glyph for now)
struct GoldExpander;
impl TemplateExpander for GoldExpander {
    fn names(&self) -> &'static [&'static str] {
        &["g"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let v = inv.params.first().cloned().unwrap_or_default();
        Ok(ExpansionResult { expanded: v })
    }
}

// Critical damage marker: {{critical damage|...|...|mod=0.9}} -> "90% total critical damage"
struct CriticalDamageExpander;
impl TemplateExpander for CriticalDamageExpander {
    fn names(&self) -> &'static [&'static str] {
        &["critical damage"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        // Look for a named param like mod=0.9 or a plain numeric param we can treat as percent
        let mut percent: Option<String> = None;
        for p in &inv.params {
            if let Some(eq) = p.find('=') {
                let (k, v) = p.split_at(eq);
                if k.trim().eq_ignore_ascii_case("mod") {
                    if let Ok(f) = v[1..].trim().parse::<f32>() {
                        percent = Some(format!("{}%", (f * 100.0).round() as i32));
                        break;
                    }
                }
            }
        }
        // Fallback: if first param looks numeric, use it as-is with %
        if percent.is_none() {
            if let Some(first) = inv.params.first() {
                if first.trim().chars().all(|c| c.is_ascii_digit()) {
                    percent = Some(format!("{}%", first.trim()));
                }
            }
        }
        let label = match percent {
            Some(p) => format!("{} total critical damage", p),
            None => "total critical damage".to_string(),
        };
        Ok(ExpansionResult { expanded: label })
    }
}

struct TooltipExpander; // {{tt|value|tooltip}}
impl TemplateExpander for TooltipExpander {
    fn names(&self) -> &'static [&'static str] {
        &["tt"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing value".into(),
            });
        }
        let value = &inv.params[0];
        let tooltip = inv
            .params
            .get(1)
            .map(|s| format!(" ({s})"))
            .unwrap_or_default();
        Ok(ExpansionResult {
            expanded: format!("{value}{tooltip}"),
        })
    }
}

// Variable definition: {{#vardefine:name|value}}
struct VarDefineExpander;
impl TemplateExpander for VarDefineExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#vardefine"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.len() < 2 {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "expected name|value".into(),
            });
        }
        // Definition removed from output (value substituted later via #var)
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

// Variable reference: {{#var:name}}
struct VarRefExpander;
impl TemplateExpander for VarRefExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#var"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing var name".into(),
            });
        }
        let key = &inv.params[0];
        match ctx.vars.get(key) {
            Some(v) => Ok(ExpansionResult {
                expanded: v.clone(),
            }),
            None => Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: format!("undefined var {key}"),
            }),
        }
    }
}

/// Depth-aware parser for a raw template body (without outer braces).
pub fn parse_invocation(raw_full: &str) -> TemplateInvocation {
    let wrapped = format!("{{{{{}}}}}", raw_full);
    let output = default_wiki_configuration().parse(&wrapped);

    let mut name = raw_full.trim().to_string();
    let mut params: Vec<String> = Vec::new();
    let mut parsed_with_parser = false;

    if let Some((template_name, template_params)) =
        output.nodes.iter().find_map(|node| match node {
            Node::Template {
                start,
                name,
                parameters,
                ..
            } if *start == 0 => Some((name, parameters)),
            _ => None,
        })
    {
        name = nodes_to_string(template_name, &wrapped);
        params = template_params
            .iter()
            .map(|parameter| {
                let mut entry = String::new();
                if let Some(param_name_nodes) = &parameter.name {
                    let key = nodes_to_string(param_name_nodes, &wrapped);
                    let key = key.trim();
                    if !key.is_empty() {
                        entry.push_str(key);
                        entry.push('=');
                    }
                }
                let value = nodes_to_string(&parameter.value, &wrapped);
                entry.push_str(value.trim());
                entry.trim().to_string()
            })
            .collect();
        parsed_with_parser = true;
    }

    if !parsed_with_parser {
        return legacy_parse_invocation(raw_full);
    }

    normalize_invocation(&mut name, &mut params);

    TemplateInvocation {
        name,
        raw: raw_full.to_string(),
        params,
    }
}

fn legacy_parse_invocation(raw_full: &str) -> TemplateInvocation {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut brace = 0i32;
    let mut bracket = 0i32;
    let mut paren = 0i32;
    for c in raw_full.chars() {
        match c {
            '{' => {
                brace += 1;
                current.push(c);
            }
            '}' => {
                brace -= 1;
                current.push(c);
            }
            '[' => {
                bracket += 1;
                current.push(c);
            }
            ']' => {
                bracket -= 1;
                current.push(c);
            }
            '(' => {
                paren += 1;
                current.push(c);
            }
            ')' => {
                paren -= 1;
                current.push(c);
            }
            '|' if brace == 0 && bracket == 0 && paren == 0 => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }
    let mut name = parts.first().cloned().unwrap_or_default();
    let mut params = if parts.len() > 1 {
        parts[1..].to_vec()
    } else {
        vec![]
    };
    normalize_invocation(&mut name, &mut params);
    TemplateInvocation {
        name,
        raw: raw_full.to_string(),
        params,
    }
}

fn nodes_to_string(nodes: &[Node<'_>], source: &str) -> String {
    if nodes.is_empty() {
        return String::new();
    }
    let start = nodes.first().map(|node| node.start()).unwrap_or(0);
    let end = nodes.last().map(|node| node.end()).unwrap_or(start);
    source[start..end].trim().to_string()
}

fn normalize_invocation(name: &mut String, params: &mut Vec<String>) {
    *name = name.trim().to_string();
    for param in params.iter_mut() {
        *param = param.trim().to_string();
    }

    let lower = name.to_ascii_lowercase();
    if lower.starts_with("#vardefine:") {
        if let Some(idx) = name.find(':') {
            let var_name = name[idx + 1..].trim().to_string();
            *name = "#vardefine".into();
            params.insert(0, var_name);
        }
    } else if lower.starts_with("#var:") {
        if let Some(idx) = name.find(':') {
            let var_name = name[idx + 1..].trim().to_string();
            *name = "#var".into();
            params.insert(0, var_name);
        }
    } else if lower.starts_with("#expr:") {
        if let Some(idx) = name.find(':') {
            let expr = name[idx + 1..].trim().to_string();
            *name = "#expr".into();
            params.insert(0, expr);
        }
    } else if lower.starts_with("#if:") {
        if let Some(idx) = name.find(':') {
            let test = name[idx + 1..].trim().to_string();
            *name = "#if".into();
            params.insert(0, test);
        }
    } else if lower.starts_with("#ifeq:") {
        if let Some(idx) = name.find(':') {
            let left = name[idx + 1..].trim().to_string();
            *name = "#ifeq".into();
            params.insert(0, left);
        }
    } else if lower.starts_with("#switch:") {
        if let Some(idx) = name.find(':') {
            let value = name[idx + 1..].trim().to_string();
            *name = "#switch".into();
            params.insert(0, value);
        }
    } else if lower.starts_with("#invoke:") {
        if let Some(idx) = name.find(':') {
            let module = name[idx + 1..].trim().to_string();
            *name = "#invoke".into();
            params.insert(0, module);
        }
    } else if lower.starts_with("data ") || lower.starts_with("data_") {
        *name = "Data".into();
    }
}

// --- Domain stub expanders (initial minimal formatting) ---

// {{#if: test | then | else}}
struct IfExpander;
impl TemplateExpander for IfExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#if"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing test".into(),
            });
        }
        let test = inv.params[0].trim();
        let truthy = !(test.is_empty() || test == "0");
        let then_v = inv.params.get(1).cloned().unwrap_or_default();
        let else_v = inv.params.get(2).cloned().unwrap_or_default();
        Ok(ExpansionResult {
            expanded: if truthy { then_v } else { else_v },
        })
    }
}

// {{#ifeq: a | b | then | else}}
struct IfEqExpander;
impl TemplateExpander for IfEqExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#ifeq"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.len() < 2 {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "expected a|b|then|else".into(),
            });
        }
        let a = inv.params[0].trim();
        let b = inv.params[1].trim();
        let then_v = inv.params.get(2).cloned().unwrap_or_default();
        let else_v = inv.params.get(3).cloned().unwrap_or_default();
        Ok(ExpansionResult {
            expanded: if a == b { then_v } else { else_v },
        })
    }
}

// {{#switch: val | case1=result1 | case2=result2 | #default=def }}
struct SwitchExpander;
impl TemplateExpander for SwitchExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#switch"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing switch value".into(),
            });
        }
        let val = inv.params[0].trim();
        let mut default: Option<String> = None;
        // scan for first key=value pair matching val
        for p in inv.params.iter().skip(1) {
            if let Some(eq) = p.find('=') {
                let (k, v) = p.split_at(eq);
                let key = k.trim();
                let rhs = v[1..].to_string();
                if key.eq_ignore_ascii_case("#default") {
                    default = Some(rhs);
                    continue;
                }
                if key == val {
                    return Ok(ExpansionResult { expanded: rhs });
                }
            }
        }
        Ok(ExpansionResult {
            expanded: default.unwrap_or_default(),
        })
    }
}

struct ApExpander; // {{ap|value}} -> +<value>% AP or preserves scaling sequence
impl TemplateExpander for ApExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ap"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing ap value".into(),
            });
        }
        // Allow multiple values: {{ap|40|50|60}} -> (+40/50/60% AP)
        let mut vals: Vec<String> = inv
            .params
            .iter()
            .map(|s| s.trim().trim_end_matches('%').to_string())
            .collect();
        vals.retain(|v| !v.is_empty());
        if vals.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "empty ap param".into(),
            });
        }
        let joined = if vals.len() == 1 {
            format!("{}%", vals[0])
        } else {
            format!("{}%", vals.join("/"))
        };
        Ok(ExpansionResult {
            expanded: format!("(+{joined} AP)"),
        })
    }
}

struct PpExpander; // {{pp|10|20|30}} per-level sequence -> join with /
impl TemplateExpander for PpExpander {
    fn names(&self) -> &'static [&'static str] {
        &["pp"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "no values".into(),
            });
        }
        // Trim & collapse blanks, keep ordering
        let cleaned: Vec<String> = inv
            .params
            .iter()
            .map(|p| p.trim().trim_matches(|c: char| c == '%').to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if cleaned.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "no effective values".into(),
            });
        }
        Ok(ExpansionResult {
            expanded: cleaned.join(" / "),
        })
    }
}

struct PpTooltipExpander; // {{pptooltip|...}} treat same as pp for now
impl TemplateExpander for PpTooltipExpander {
    fn names(&self) -> &'static [&'static str] {
        &["pptooltip"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        PpExpander.expand(inv, ctx)
    }
}

struct FdExpander; // {{fd|number|2}} fixed decimals
impl TemplateExpander for FdExpander {
    fn names(&self) -> &'static [&'static str] {
        &["fd"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing number".into(),
            });
        }
        let num: f64 = inv.params[0].parse().unwrap_or(0.0);
        let prec: usize = inv.params.get(1).and_then(|p| p.parse().ok()).unwrap_or(2);
        Ok(ExpansionResult {
            expanded: format!("{:.*}", prec, num),
        })
    }
}

struct IconUnwrapExpander; // minimal icon unwrap: {{ci|Aatrox}} -> Aatrox; if second param is non-possessive display, prefer it
impl TemplateExpander for IconUnwrapExpander {
    fn names(&self) -> &'static [&'static str] {
        &[
            "ci", "cis", "ai", "ais", "ii", "iis", "ri", "bi", "ui", "cai", "cais", "cci", "ccis",
            "fi", "nie", "nies", "si",
        ]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing label".into(),
            });
        }
        // Prefer second param as display label if present and not possessive marker
        let mut label = if let Some(sec) = inv.params.get(1) {
            let s = sec.trim();
            if s == "'s" || s == "’s" || s.is_empty() {
                inv.params[0].trim().to_string()
            } else {
                s.to_string()
            }
        } else {
            inv.params[0].trim().to_string()
        };
        // If third (or second when first used) is possessive marker, append it
        if let Some(sec) = inv.params.get(1) {
            let s = sec.trim();
            if s == "'s" || s == "’s" {
                label.push_str("'s");
            }
        }
        Ok(ExpansionResult { expanded: label })
    }
}

// Quote expander: {{Quote|text|author}} -> blockquote; no styling beyond plain text.
struct QuoteExpander;
impl TemplateExpander for QuoteExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Quote", "quote"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing quote body".into(),
            });
        }
        let body = inv.params[0].trim();
        let author = inv
            .params
            .get(1)
            .map(|a| a.trim())
            .filter(|a| !a.is_empty());
        let mut out = String::new();
        out.push_str("> ");
        out.push_str(body);
        if let Some(a) = author {
            out.push_str(" — ");
            out.push_str(a);
        }
        Ok(ExpansionResult { expanded: out })
    }
}

// Tip wrapper (game badge / italic on wiki). Keep plain text of the first arg.
struct TipExpander;
impl TemplateExpander for TipExpander {
    fn names(&self) -> &'static [&'static str] {
        &["tip"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing label".into(),
            });
        }
        // If any param mentions icononly=true, suppress output (icon-only markers)
        if inv
            .params
            .iter()
            .any(|p| p.to_lowercase().contains("icononly"))
        {
            return Ok(ExpansionResult {
                expanded: String::new(),
            });
        }
        // Prefer the last non-empty param (often the display label), else the first
        let chosen = inv
            .params
            .iter()
            .rev()
            .find(|s| !s.trim().is_empty())
            .cloned()
            .unwrap_or_else(|| inv.params[0].clone());
        Ok(ExpansionResult {
            expanded: chosen.trim().to_string(),
        })
    }
}

// Wikipedia link shortcut: {{w|Page|Label}} -> Label (fallback to Page)
struct WExpander;
impl TemplateExpander for WExpander {
    fn names(&self) -> &'static [&'static str] {
        &["w", "W"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing page".into(),
            });
        }
        let label = inv
            .params
            .get(1)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| inv.params[0].trim());
        Ok(ExpansionResult {
            expanded: label.to_string(),
        })
    }
}

// Small Bold Caps: {{sbc|text}} -> **TEXT** (uppercase)
struct SbcExpander;
impl TemplateExpander for SbcExpander {
    fn names(&self) -> &'static [&'static str] {
        &["sbc"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing text".into(),
            });
        }
        let txt = inv.params[0].to_uppercase();
        Ok(ExpansionResult {
            expanded: format!("**{}**", txt),
        })
    }
}

// Channel Type (ct) stub: {{ct|channeled}} -> (Channeled)
struct ChannelTypeExpander;
impl TemplateExpander for ChannelTypeExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ct"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing channel type".into(),
            });
        }
        let t = inv.params[0].trim();
        Ok(ExpansionResult {
            expanded: format!("({})", t),
        })
    }
}

// Skill Tab (st) expander: convert to a simple inline marker; actual structured capture handled separately.
struct SkillTabExpander;
impl TemplateExpander for SkillTabExpander {
    fn names(&self) -> &'static [&'static str] {
        &["st"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        // Parse headers (h:) and rows (r:, r2:, r3:, etc.)
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        let mut row_index = 1;
        for p in &inv.params {
            if let Some(eq) = p.find('=') {
                let (k, v) = p.split_at(eq);
                let k = k.trim();
                let v = v[1..].trim();
                if k.eq_ignore_ascii_case("h") {
                    headers.push(v.to_string());
                } else if k.eq_ignore_ascii_case("r")
                    || k.eq_ignore_ascii_case(&format!("r{}", row_index))
                {
                    current_row.push(v.to_string());
                    if k.eq_ignore_ascii_case(&format!("r{}", row_index)) {
                        rows.push(current_row);
                        current_row = Vec::new();
                        row_index += 1;
                    }
                }
            }
        }
        // If there's an unfinished row
        if !current_row.is_empty() {
            rows.push(current_row);
        }
        // Fallback if no structured data
        if headers.is_empty() && rows.is_empty() {
            let pairs = inv.params.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            return Ok(ExpansionResult {
                expanded: format!("[SkillTab {}]", pairs.join(" | ")),
            });
        }
        // Generate marker with structured data
        let mut parts = Vec::new();
        for h in &headers {
            parts.push(format!("h:{}", h));
        }
        for (i, row) in rows.iter().enumerate() {
            let row_key = if i == 0 { "r" } else { &format!("r{}", i + 1) };
            for val in row {
                parts.push(format!("{}:{}", row_key, val));
            }
        }
        Ok(ExpansionResult {
            expanded: format!("[SkillTab {}]", parts.join(" | ")),
        })
    }
}

// Flip Text (ft) stylistic wrapper -> 「 a ⟷ b 」
struct FlipTextExpander;
impl TemplateExpander for FlipTextExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ft"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.len() < 2 {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "requires two parameters: a and b".into(),
            });
        }
        let a = inv.params[0].trim();
        let b = inv.params[1].trim();
        let result = format!("「 {} ⟷ {} 」", a, b);
        Ok(ExpansionResult { expanded: result })
    }
}

// Champion / item constant data substitution (ccd / cid).
struct ConstantDataExpander;
impl TemplateExpander for ConstantDataExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ccd", "cid"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing key".into(),
            });
        }
        let entity = inv.params[0].trim();
        let field = inv.params.get(1).map(|s| s.trim()).unwrap_or("");
        if inv.name.eq_ignore_ascii_case("ccd") {
            // Champion constant data
            if let Some(ref conv_ctx) = ctx.conversion_ctx {
                if let Some(constants) = conv_ctx.champion_constants(entity) {
                    if let Some(value) = constants.get(field) {
                        return Ok(ExpansionResult {
                            expanded: value.clone(),
                        });
                    }
                }
            }
        } else if inv.name.eq_ignore_ascii_case("cid") {
            // Item constant data
            if let Some(ref conv_ctx) = ctx.conversion_ctx {
                match conv_ctx.item_module_map() {
                    Ok(map) => {
                        if let Some(item_data) = map.get(entity) {
                            if let Some(LuaValue::String(s)) = item_data.get(field) {
                                return Ok(ExpansionResult {
                                    expanded: s.clone(),
                                });
                            } else if let Some(LuaValue::Number(n)) = item_data.get(field) {
                                return Ok(ExpansionResult {
                                    expanded: n.to_string(),
                                });
                            }
                        }
                    }
                    Err(_) => {} // Ignore error for now
                }
            }
        }
        // Fallback to vars or key
        if let Some(v) = ctx.vars.get(entity) {
            return Ok(ExpansionResult {
                expanded: v.clone(),
            });
        }
        Ok(ExpansionResult {
            expanded: entity.to_string(),
        })
    }
}

// Neutralize structural templates that carry no information payload in markdown output.
struct NeutralizeExpander;
impl TemplateExpander for NeutralizeExpander {
    fn names(&self) -> &'static [&'static str] {
        // Add more known structural/no-op templates here as they are observed.
        &[
            "Section top",
            "Section end",
            "section top",
            "section end",
            "Game banner",
            "LoL navigation",
            "Navbox",
            "Hatnote",
            // Infoboxes / page scaffolding
            "Infobox champion",
            "Infobox item",
            "Infobox rune",
            "Infobox spell",
            "Infobox stats",
            "Infobox buff",
            "Infobox unit",
            "Infobox video",
            "Item info",
            "Champion info",
            "Champions",
            "Champion categories",
            // Grouped ability wrapper used on some pages
            "Grouped ability",
            // Champion Data templates (e.g., Data Akshan/I)
            "Data",
            // Lua module invocations (e.g., #invoke:SkinData|...)
            "#invoke",
            // Patch history inclusion box
            "Patch box",
            // Casting helper/marker templates
            "Effect at cast time end",
            "Effect at cast time start",
            // Audio sample / minor markers
            "sm2",
            // Bug marker becomes empty in text
            "bug",
            // Range/time formatting helpers that don't affect plain text content here
            "rutngt",
            "pending for test",
            // Anchor for sections
            "Anchor",
            // Pet infobox
            "Infobox/Pet",
            // Champion AP ratio category
            "Champion without ability power ratio",
            // TFT item marker
            "TFT Item",
            // References block wrappers
            "References",
            // Misc attribution banners
            "wikia",
        ]
    }
    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tempfile::tempdir;

    fn ctx() -> ExpanderCtx {
        ExpanderCtx {
            precision: 2,
            vars: Default::default(),
            conversion_ctx: None,
        }
    }

    #[test]
    fn parse_invocation_colon_forms() {
        let inv = parse_invocation("#expr: 1+2");
        assert_eq!(inv.name, "#expr");
        assert_eq!(inv.params[0], "1+2");
        let inv = parse_invocation("#vardefine:x|5");
        assert_eq!(inv.name, "#vardefine");
        assert_eq!(inv.params[0], "x");
        let inv = parse_invocation("#var:x");
        assert_eq!(inv.name, "#var");
        assert_eq!(inv.params[0], "x");
    }

    #[test]
    fn expander_ap_pp_fd_icon() {
        let reg = TemplateRegistry::new();
        let ap = parse_invocation("ap|40");
        assert_eq!(reg.expand(&ap, &ctx()).unwrap().expanded, "(+40% AP)");
        let ap2 = parse_invocation("ap|40|50|60");
        assert_eq!(
            reg.expand(&ap2, &ctx()).unwrap().expanded,
            "(+40/50/60% AP)"
        );
        let pp = parse_invocation("pp|10|20|30");
        assert_eq!(reg.expand(&pp, &ctx()).unwrap().expanded, "10 / 20 / 30");
        let ppt = parse_invocation("pptooltip|5|15");
        assert_eq!(reg.expand(&ppt, &ctx()).unwrap().expanded, "5 / 15");
        let fd = parse_invocation("fd|3.14159|3");
        assert_eq!(reg.expand(&fd, &ctx()).unwrap().expanded, "3.142");
        let ci = parse_invocation("ci|Aatrox|'s");
        assert_eq!(reg.expand(&ci, &ctx()).unwrap().expanded, "Aatrox's");
    }

    #[test]
    fn expander_tt_sbc_ct_if_switch() {
        let reg = TemplateRegistry::new();
        let tt = parse_invocation("tt|Value|Tooltip");
        assert_eq!(reg.expand(&tt, &ctx()).unwrap().expanded, "Value (Tooltip)");
        let sbc = parse_invocation("sbc|hello");
        assert_eq!(reg.expand(&sbc, &ctx()).unwrap().expanded, "**HELLO**");
        let ct = parse_invocation("ct|Channeled");
        assert_eq!(reg.expand(&ct, &ctx()).unwrap().expanded, "(Channeled)");
        let iff = parse_invocation("#if:1|yes|no");
        assert_eq!(reg.expand(&iff, &ctx()).unwrap().expanded, "yes");
        let ifeq = parse_invocation("#ifeq:a|a|Y|N");
        assert_eq!(reg.expand(&ifeq, &ctx()).unwrap().expanded, "Y");
        let sw = parse_invocation("#switch: b | a=1 | b=2 | #default=0");
        assert_eq!(reg.expand(&sw, &ctx()).unwrap().expanded, "2");
    }

    #[test]
    fn include_info_templates_inline_includeonly_content() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        let file_path = export_dir.join("Template%3ASpellblade%20info.txt");
        std::fs::write(
            &file_path,
            "<includeonly>* {{sbc|Spellblade}} deals {{tip|proc damage}}.</includeonly>",
        )
        .unwrap();

        let ctx = ConversionContext::new(tmp.path(), 2).unwrap();
        let ctx_arc = Arc::new(ctx);
        let registry = TemplateRegistry::new();
        let inv = parse_invocation("Spellblade info");
        let result = registry
            .expand(
                &inv,
                &ExpanderCtx {
                    precision: 2,
                    vars: HashMap::new(),
                    conversion_ctx: Some(ctx_arc.clone()),
                },
            )
            .unwrap();
        assert!(result
            .expanded
            .contains("* **SPELLBLADE** deals proc damage."));
    }
}
