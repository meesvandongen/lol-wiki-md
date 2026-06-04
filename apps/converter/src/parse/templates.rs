//! Template expansion framework (minimal subset).
use crate::convert::context::ConversionContext;
use crate::convert::util::expand_inline_templates_with_store;
use crate::error::{ConvertError, Result};
use crate::parse::default_wiki_configuration;
use crate::parse::expr::{evaluate_expression, ExprNumberFormat};
use crate::parse::lua::{lua_value_to_string, parse_champion_module, LuaValue};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

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

/// Normalize a template name into the canonical lookup key used by the
/// registry. MediaWiki treats underscores and spaces as equivalent in page and
/// template names and collapses runs of whitespace, so `{{Zombie_state_info}}`
/// and `{{Zombie state info}}` resolve to the same template. The key is also
/// lowercased to keep the registry case-insensitive on the first character
/// (and, in practice here, the whole name).
fn normalize_template_name(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut prev_was_space = false;
    for ch in name.trim().chars() {
        if ch == '_' || ch.is_whitespace() {
            if !prev_was_space {
                out.push(' ');
                prev_was_space = true;
            }
        } else {
            out.extend(ch.to_lowercase());
            prev_was_space = false;
        }
    }
    out
}

#[derive(Default)]
pub struct TemplateRegistry {
    expanders: Vec<Box<dyn TemplateExpander>>,
    /// Lookup of normalized (lowercase) template name to the index in
    /// `expanders` that first claims it. Keeps "first registered wins"
    /// semantics for shared names (e.g. the catch-all `SimpleInlineExpander`
    /// is registered last so dedicated handlers win) while eliminating the
    /// O(N_expanders) scan that previously ran for every template invocation.
    name_index: HashMap<String, usize>,
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
        r.register(Box::new(VarDefineEchoExpander));
        r.register(Box::new(VarRefExpander));
        r.register(Box::new(ApExpander));
        r.register(Box::new(PpExpander));
        r.register(Box::new(PpTooltipExpander));
        r.register(Box::new(FdExpander));
        r.register(Box::new(RdExpander));
        r.register(Box::new(IconUnwrapExpander));
        r.register(Box::new(ChampionSkinLinkExpander));
        r.register(Box::new(ChampionSkinTriviaExpander));
        r.register(Box::new(CharacterBackgroundIconExpander));
        r.register(Box::new(CustomContentIconExpander));
        r.register(Box::new(LegendsOfRuneterraExpander));
        r.register(Box::new(FandomExpander));
        r.register(Box::new(GemsExpander));
        r.register(Box::new(UniverseExpander));
        r.register(Box::new(LabelColonExpander));
        r.register(Box::new(CurrencyExpander));
        r.register(Box::new(SymbolExpander));
        r.register(Box::new(AsExpander));
        r.register(Box::new(StiExpander));
        r.register(Box::new(RecurringExpander));
        r.register(Box::new(MasteryIconExpander));
        r.register(Box::new(TimesExpander));
        r.register(Box::new(PipeEscapeExpander));
        r.register(Box::new(SimpleLabelExpander));
        r.register(Box::new(LethalityExpander));
        r.register(Box::new(RoundUpToGameTickExpander));
        r.register(Box::new(WildRiftItemExpander));
        r.register(Box::new(GoldExpander));
        r.register(Box::new(GoldValueExpander));
        r.register(Box::new(GoldEfficiencyCalculationExpander));
        r.register(Box::new(CriticalDamageExpander));
        r.register(Box::new(QuoteExpander));
        r.register(Box::new(TipExpander));
        r.register(Box::new(WExpander));
        r.register(Box::new(SbcExpander));
        r.register(Box::new(ChannelTypeExpander));
        r.register(Box::new(SkillTabExpander));
        r.register(Box::new(FlipTextExpander));
        r.register(Box::new(ColumnExpander));
        r.register(Box::new(ClearExpander));
        r.register(Box::new(ChampionWithInfiniteScalingExpander));
        r.register(Box::new(CcsExpander));
        r.register(Box::new(DegreeExpander));
        r.register(Box::new(DividedByExpander));
        r.register(Box::new(ColorExpander));
        r.register(Box::new(ConstantDataExpander));
        r.register(Box::new(HighestLowestStatsExpander));
        r.register(Box::new(IncludeInfoExpander));
        r.register(Box::new(ScrollBoxExpander));
        r.register(Box::new(RuneDataExpander));
        r.register(Box::new(MapChangesExpander));
        r.register(Box::new(ItemInfoVarExpander));
        r.register(Box::new(ItemStatTableExpander));
        r.register(Box::new(SuperimposeExpander));
        r.register(Box::new(InvokeExpander));
        r.register(Box::new(RecipeItemExpander));
        r.register(Box::new(RuneInlineStyleExpander));
        r.register(Box::new(SimpleStatExpander));
        r.register(Box::new(NeutralizeExpander));
        r.register(Box::new(ChimeListExpander));
        r.register(Box::new(DelimitValuesExpander));
        r.register(Box::new(CategoryChampionListExpander));
        r.register(Box::new(DecorativeEmptyExpander));
        r.register(Box::new(NamedItemEffectExpander));
        r.register(Box::new(UniqueExpander));
        r.register(Box::new(UnexpectedExpander));
        r.register(Box::new(TipDataExpander));
        r.register(Box::new(SupNoteExpander));
        r.register(Box::new(AugExpander));
        r.register(Box::new(NumberSupExpander));
        r.register(Box::new(MinuteDisplayExpander));
        r.register(Box::new(AdaptiveExpander));
        // Registered last: a catch-all for inline icon/label helper templates that
        // render to plain reader-facing text. Dedicated expanders above take
        // precedence for any shared names.
        r.register(Box::new(SimpleInlineExpander));
        r
    }
    pub fn register(&mut self, ex: Box<dyn TemplateExpander>) {
        let idx = self.expanders.len();
        for &raw_name in ex.names() {
            let key = normalize_template_name(raw_name);
            // First registration wins; later expanders sharing a name (notably
            // the catch-all `SimpleInlineExpander`) defer to the dedicated one.
            self.name_index.entry(key).or_insert(idx);
        }
        self.expanders.push(ex);
    }
    pub fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let lookup_key = normalize_template_name(&inv.name);
        if let Some(&idx) = self.name_index.get(&lookup_key) {
            if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
                conv_ctx.record_template_params(&inv.name, &inv.params);
            }
            return self.expanders[idx].expand(inv, ctx);
        }
        if let Some(recovered) = recover_malformed_wrapper_invocation(inv) {
            return self.expand(&recovered, ctx);
        }
        // MediaWiki substitution prefixes (`subst:` / `safesubst:`) are dispatch
        // hints, not part of the template name. Strip them and re-dispatch.
        let lower_name = inv.name.trim().to_ascii_lowercase();
        for prefix in ["safesubst:", "subst:"] {
            if lower_name.starts_with(prefix) {
                let stripped = inv.name.trim()[prefix.len()..].trim().to_string();
                if !stripped.is_empty() {
                    let recovered = TemplateInvocation {
                        name: stripped.clone(),
                        raw: format!(
                            "{}{}",
                            stripped,
                            inv.raw.trim().strip_prefix(inv.name.trim()).unwrap_or("")
                        ),
                        params: inv.params.clone(),
                    };
                    return self.expand(&recovered, ctx);
                }
            }
        }
        // `Tip data/<topic>` is a buzzword data family resolved from the topic's
        // template page.
        if lower_name.starts_with("tip data/") {
            if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
                conv_ctx.record_template_params(&inv.name, &inv.params);
            }
            return TipDataExpander.expand(inv, ctx);
        }
        if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
            conv_ctx.record_template_params(&inv.name, &inv.params);
        }
        Err(ConvertError::UnknownTemplate {
            name: inv.name.clone(),
        })
    }
    pub fn has_name(&self, name: &str) -> bool {
        self.name_index.contains_key(&normalize_template_name(name))
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

/// Error produced when a recognized template cannot be expanded for the given
/// invocation (missing data, malformed arguments, unsupported variant). The
/// pipeline fails fast rather than emitting a placeholder marker.
fn unhandled_template_error(inv: &TemplateInvocation) -> ConvertError {
    ConvertError::MalformedTemplate {
        name: inv.name.clone(),
        detail: format!("could not expand invocation: {}", inv.raw),
    }
}

/// Catch-all expander for inline icon/label/helper templates that render to
/// plain reader-facing text (e.g. `csl`, `cai`, `lor`, `tip`). Shares its
/// rendering logic with the markdown renderer. Returns
/// [`ConvertError::UnknownTemplate`] for any name it does not handle so the
/// pipeline fails fast on genuinely unsupported templates.
struct SimpleInlineExpander;
impl TemplateExpander for SimpleInlineExpander {
    fn names(&self) -> &'static [&'static str] {
        &[
            "as",
            "ap",
            "sti",
            "ci",
            "ui",
            "uis",
            "ii",
            "nie",
            "ris",
            "cbi",
            "lor",
            "gems",
            "skin tier",
            "si",
            "tfti",
            "adaptive",
            "fd",
            "tftc",
            "tftt",
            "wrskin",
            "cis",
            "cbis",
            "iis",
            "nies",
            "sis",
            "csl",
            "fi",
            "tip",
            "wrtip",
            "lorskin",
            "w",
            "univ",
            "citation needed",
            "equals",
            "gold",
            "champion_icon",
            "champion icon",
            "ability icon",
            "zoe spell thief list",
            "bug",
            "pending for test",
            "pft",
            "effect at cast time start",
            "effect at cast time end",
            "degree",
            "minus",
            "plus",
            "lmb",
            "rmb",
            "times",
            "arcaneciteep",
            "ai",
            "cai",
            "ais",
            "cais",
            "sbc",
            "spoiler",
            "note",
            "rd",
            "ig",
            "wi",
            "recurring",
            "wrcst",
            "mi1",
            "mi2",
            "mi3",
            "mi4",
            "mi6",
            "mi7",
            "lll",
            "set",
            "item stat table",
            "tt",
        ]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let args: Vec<&str> = inv.params.iter().map(|p| p.as_str()).collect();
        match crate::render::markdown::render_simple_inline_template(&inv.name, &args) {
            Some(expanded) => Ok(ExpansionResult { expanded }),
            None => Err(ConvertError::UnknownTemplate {
                name: inv.name.clone(),
            }),
        }
    }
}

fn recover_malformed_wrapper_invocation(inv: &TemplateInvocation) -> Option<TemplateInvocation> {
    if !inv.params.is_empty() {
        return None;
    }

    let trimmed = inv.name.trim();
    let lower = trimmed.to_ascii_lowercase();
    if lower.starts_with("as") {
        let remainder = trimmed[2..].trim();
        if !remainder.is_empty() && remainder.chars().any(char::is_whitespace) {
            return Some(TemplateInvocation {
                name: "as".to_string(),
                raw: inv.raw.clone(),
                params: vec![remainder.to_string()],
            });
        }
    }

    None
}

pub struct ExpanderCtx {
    pub precision: u8,
    pub vars: Arc<Mutex<HashMap<String, String>>>,
    pub conversion_ctx: Option<Arc<ConversionContext>>,
}

impl ExpanderCtx {
    pub fn new(
        precision: u8,
        vars: &HashMap<String, String>,
        conversion_ctx: Option<Arc<ConversionContext>>,
    ) -> Self {
        Self {
            precision,
            vars: Arc::new(Mutex::new(vars.clone())),
            conversion_ctx,
        }
    }

    fn vars_snapshot(&self) -> HashMap<String, String> {
        self.vars
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_default()
    }

    fn get_var(&self, key: &str) -> Option<String> {
        self.vars.lock().ok()?.get(key).cloned()
    }

    fn set_var(&self, key: impl Into<String>, value: impl Into<String>) {
        if let Ok(mut guard) = self.vars.lock() {
            guard.insert(key.into(), value.into());
        }
    }
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
        let pre_local = replace_var_templates(expr_body, &ctx.vars_snapshot(), true);
        let nested = expand_nested_template_text(&pre_local, ctx)
            .unwrap_or_else(|_| pre_local.trim().to_string());
        let pre = replace_var_templates(&nested, &ctx.vars_snapshot(), true);
        match evaluate_expression_display(&pre, ctx.precision) {
            Some(v) => Ok(ExpansionResult { expanded: v }),
            None => Err(unhandled_template_error(inv)),
        }
    }
}

fn replace_var_templates(
    input: &str,
    vars: &HashMap<String, String>,
    missing_as_empty: bool,
) -> String {
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
            let val = vars.get(name).cloned().unwrap_or_else(|| {
                if missing_as_empty {
                    String::new()
                } else {
                    format!("{{{{#var:{}}}}}", name.trim())
                }
            });
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

fn expand_nested_template_text(raw: &str, ctx: &ExpanderCtx) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    let fallback_registry;
    let registry = if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
        conv_ctx.registry()
    } else {
        fallback_registry = TemplateRegistry::new();
        &fallback_registry
    };

    let expanded = expand_inline_templates_with_store(
        trimmed,
        ctx.precision,
        ctx.vars.clone(),
        registry,
        ctx.conversion_ctx.clone(),
    )?;
    Ok(expanded.trim().to_string())
}

fn looks_like_textual_fd_value(raw: &str) -> bool {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return false;
    }
    if matches!(trimmed.to_ascii_lowercase().as_str(), "none" | "instant") {
        return true;
    }
    if trimmed.contains('\u{0305}') {
        return true;
    }

    let mut end = 0usize;
    let mut saw_numeric = false;
    for (idx, ch) in trimmed.char_indices() {
        if ch.is_ascii_digit() || ch == '.' || (idx == 0 && (ch == '+' || ch == '-')) {
            saw_numeric = true;
            end = idx + ch.len_utf8();
            continue;
        }
        break;
    }
    if !saw_numeric {
        return false;
    }

    let remainder = trimmed[end..].trim_start();
    if remainder.is_empty() {
        return false;
    }
    remainder.starts_with('-')
        || remainder.starts_with('–')
        || remainder.starts_with('—')
        || remainder.starts_with('(')
        || remainder.starts_with('%')
        || remainder
            .chars()
            .next()
            .map(|ch| ch.is_ascii_alphabetic())
            .unwrap_or(false)
}

// Attribute style expander: {{as|text}} -> text (drop styling markers like ms parameter)
struct AsExpander;
impl TemplateExpander for AsExpander {
    fn names(&self) -> &'static [&'static str] {
        &["as", "skin tier"]
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
        &["sti", "stil"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        // Many usages are sti|ms|text or sti|text. Ignore named params such as link=true.
        let (positional, _named) = split_named_and_positional(inv);
        let val = positional.last().cloned().unwrap_or_default();
        Ok(ExpansionResult { expanded: val })
    }
}

struct RecurringExpander;
impl TemplateExpander for RecurringExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Recurring"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let digits = inv.params.first().map(|value| value.trim()).unwrap_or("");
        let mut expanded = String::new();
        for ch in digits.chars() {
            expanded.push(ch);
            if !ch.is_whitespace() {
                expanded.push('\u{0305}');
            }
        }
        Ok(ExpansionResult { expanded })
    }
}

struct MasteryIconExpander;
impl TemplateExpander for MasteryIconExpander {
    fn names(&self) -> &'static [&'static str] {
        &["mi1", "mi2", "mi3", "mi4", "mi6", "mi7"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .first()
                .map(|value| value.trim().to_string())
                .unwrap_or_default(),
        })
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

struct PipeEscapeExpander;
impl TemplateExpander for PipeEscapeExpander {
    fn names(&self) -> &'static [&'static str] {
        &["!"]
    }

    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: "|".to_string(),
        })
    }
}

struct SimpleLabelExpander;
impl TemplateExpander for SimpleLabelExpander {
    fn names(&self) -> &'static [&'static str] {
        &["a"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional.first().cloned().unwrap_or_default(),
        })
    }
}

struct LethalityExpander;
impl TemplateExpander for LethalityExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Lethality"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let value = positional
            .first()
            .map(|value| value.trim())
            .unwrap_or_default();
        Ok(ExpansionResult {
            expanded: if value.is_empty() {
                "lethality".to_string()
            } else {
                format!("{} lethality", value)
            },
        })
    }
}

/// `{{rutngt|x}}` (a.k.a. `{{Rounded up to next game tick|x}}`) rounds a
/// duration in seconds UP to the next server game tick and renders it as
/// `"<value> seconds"`. Mirrors the wiki template
/// `Template:Rounded up to next game tick`, whose tick length is `0.033`s; the
/// computation is parameter-driven so it applies to any duration, not a single
/// instance. The wiki evaluates the rounding at full precision, so this is a
/// dedicated handler rather than a body transclusion (which would inherit the
/// converter's 2-decimal display precision and drop the third decimal).
struct RoundUpToGameTickExpander;
impl TemplateExpander for RoundUpToGameTickExpander {
    fn names(&self) -> &'static [&'static str] {
        &["rutngt", "Rounded up to next game tick"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        // Tick length defined by the wiki template; one tick is 0.033 seconds.
        const TICK_LENGTH: f64 = 0.033;
        let (positional, _named) = split_named_and_positional(inv);
        let raw = positional.first().map(|s| s.trim()).unwrap_or_default();
        // The duration is frequently a nested expression — `{{#expr:700/2200}}`,
        // possibly with `{{ccd|...}}`/`{{#var:...}}` lookups inside — so resolve
        // nested templates to a bare numeric value before evaluating.
        let resolved = expand_nested_template_text(raw, ctx).unwrap_or_else(|_| raw.to_string());
        let seconds =
            evaluate_numeric(resolved.trim()).ok_or_else(|| ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: format!("expected a numeric duration, got {raw:?}"),
            })?;
        let rounded = (seconds / TICK_LENGTH).ceil() * TICK_LENGTH;
        // TICK_LENGTH has three decimals and the tick count is an integer, so the
        // result is exact to three decimals; round there to drop binary float
        // noise (e.g. 0.264000000000000012 -> 0.264).
        let rounded = (rounded * 1000.0).round() / 1000.0;
        let mut rendered = format!("{rounded:.3}");
        while rendered.contains('.') && rendered.ends_with('0') {
            rendered.pop();
        }
        if rendered.ends_with('.') {
            rendered.pop();
        }
        Ok(ExpansionResult {
            expanded: format!("{rendered} seconds"),
        })
    }
}

struct WildRiftItemExpander;
impl TemplateExpander for WildRiftItemExpander {
    fn names(&self) -> &'static [&'static str] {
        &["WRi", "WR item"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .get(1)
                .cloned()
                .or_else(|| positional.first().cloned())
                .unwrap_or_default(),
        })
    }
}

struct ClearExpander;
impl TemplateExpander for ClearExpander {
    fn names(&self) -> &'static [&'static str] {
        &["clear", "Clr"]
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
    "Unit-targeted cancel conditions",
    "Summoner spell cooldown table",
    "Healing modifiers",
    "Experimental Hexplate ultimate interactions",
    "Always quickcast",
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
        let include = conv_ctx.template_includeonly(&inv.name)?.ok_or_else(|| {
            ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "template page missing <includeonly> section".into(),
            }
        })?;
        let registry = conv_ctx.registry();
        let expanded = expand_inline_templates_with_store(
            include.trim(),
            ctx.precision,
            ctx.vars.clone(),
            registry,
            ctx.conversion_ctx.clone(),
        )?;
        Ok(ExpansionResult {
            expanded: expanded.trim_end().to_string(),
        })
    }
}

/// `{{dv|a|b|...}}` (a.k.a. `Delimit values`) joins its positional values with
/// the bullet delimiter used by `Module:Ability progression`'s `dv` function.
struct DelimitValuesExpander;
impl TemplateExpander for DelimitValuesExpander {
    fn names(&self) -> &'static [&'static str] {
        &["dv", "Delimit values"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional.join(" • "),
        })
    }
}

/// Category-driven champion-list templates (`Shapeshifter Champion`,
/// `Self Crowd Control champion`, `Champion without ability power ratio`)
/// render a DPL roster that cannot be resolved offline. Emit the reader-facing
/// lead sentence and omit the dynamic roster.
struct CategoryChampionListExpander;
impl TemplateExpander for CategoryChampionListExpander {
    fn names(&self) -> &'static [&'static str] {
        &[
            "Shapeshifter Champion",
            "Self Crowd Control champion",
            "Champion without ability power ratio",
        ]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let trait_clause = if inv.name.eq_ignore_ascii_case("Self Crowd Control champion") {
            "can apply a form of crowd control to themselves by using an ability"
        } else if inv
            .name
            .eq_ignore_ascii_case("Champion without ability power ratio")
        {
            "do not have a single ability power ratio on any of their abilities"
        } else {
            "can change shape, altering some or all of their abilities"
        };
        let expanded = match positional.first().map(|value| value.trim()) {
            Some(name) if !name.is_empty() => {
                format!("'''{name}''' is one of the champions that {trait_clause}.")
            }
            _ => format!("The following champions {trait_clause}."),
        };
        Ok(ExpansionResult { expanded })
    }
}

/// Navigation, gallery, and hatnote templates that carry no reader-facing
/// prose in the Markdown output. Rendered as empty rather than dropped silently
/// so unknown templates still fail fast.
struct DecorativeEmptyExpander;
impl TemplateExpander for DecorativeEmptyExpander {
    fn names(&self) -> &'static [&'static str] {
        &[
            "game navigation",
            "Article game navigation",
            "GalleryHelper",
            "Image tabber",
            "SeeOther",
            "ToDo",
            "cr",
            "Item haste table",
            "Ward table",
        ]
    }

    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

/// `{{Named item effect|name|anchor|wr=}}` links to a named item effect; the
/// display text is the first positional value.
struct NamedItemEffectExpander;
impl TemplateExpander for NamedItemEffectExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Named item effect"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .first()
                .map(|value| value.trim().to_string())
                .unwrap_or_default(),
        })
    }
}

/// `{{Unique|effect}}` / `{{Unique|effect|text}}` render an item's unique
/// passive label.
struct UniqueExpander;
impl TemplateExpander for UniqueExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Unique"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let first = positional.first().map(|v| v.trim()).unwrap_or("");
        let expanded = match positional.get(1).map(|v| v.trim()).filter(|v| !v.is_empty()) {
            Some(second) => format!("Unique – {first}: {second}"),
            None => format!("Unique: {first}"),
        };
        Ok(ExpansionResult { expanded })
    }
}

/// `{{unexpected|desc=...}}` flags unexpected in-game behaviour; the reader
/// content is the description text.
struct UnexpectedExpander;
impl TemplateExpander for UnexpectedExpander {
    fn names(&self) -> &'static [&'static str] {
        &["unexpected"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (_positional, named) = split_named_and_positional(inv);
        let desc = named
            .get("desc")
            .or_else(|| named.get("1"))
            .cloned()
            .unwrap_or_default();
        let expanded = expand_nested_template_text(&desc, ctx).unwrap_or(desc);
        Ok(ExpansionResult {
            expanded: expanded.trim().to_string(),
        })
    }
}

/// `{{Tip data/<topic>|selector|field}}` exposes a single field (e.g.
/// `description`) of a buzzword tooltip stored on the topic's template page.
struct TipDataExpander;
impl TemplateExpander for TipDataExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Tip data"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
            return Err(unhandled_template_error(inv));
        };
        let (positional, _named) = split_named_and_positional(inv);
        let field = positional
            .get(1)
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "description".to_string());
        let title = format!("Template:{}", inv.name.trim());
        let Some(raw) = conv_ctx.export().read_template_page(&title)? else {
            return Err(unhandled_template_error(inv));
        };
        let Some(named_params) = read_named_template_params_from_page(&raw) else {
            return Err(unhandled_template_error(inv));
        };
        let Some(value_raw) = named_params.get(&field) else {
            return Err(unhandled_template_error(inv));
        };
        let expanded = expand_inline_templates_with_store(
            value_raw,
            ctx.precision,
            ctx.vars.clone(),
            conv_ctx.registry(),
            ctx.conversion_ctx.clone(),
        )?;
        Ok(ExpansionResult {
            expanded: expanded.trim().to_string(),
        })
    }
}

/// `{{adaptive|value|levels|wr=}}` renders an adaptive bonus as
/// `<AD> bonus Attack Damage or <AP> Ability Power (Adaptive)`. Per-level
/// formulas (values containing `x`) delegate to the `{{pp}}` progression
/// engine; plain numbers and `a to b` ranges use the inline renderer.
struct AdaptiveExpander;
impl TemplateExpander for AdaptiveExpander {
    fn names(&self) -> &'static [&'static str] {
        &["adaptive"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let value = positional.first().map(|v| v.trim()).unwrap_or("");
        if !value.contains('x') {
            // Plain number or `a to b` range: reuse the shared inline renderer.
            let args: Vec<&str> = inv.params.iter().map(|p| p.as_str()).collect();
            return match crate::render::markdown::render_simple_inline_template("adaptive", &args) {
                Some(expanded) => Ok(ExpansionResult { expanded }),
                None => Err(unhandled_template_error(inv)),
            };
        }

        let wild_rift = named
            .get("wr")
            .map(|v| v.trim().eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let af = if wild_rift { "0.5" } else { "0.6" };
        let for_clause = match positional.get(1).map(|v| v.trim()).filter(|v| !v.is_empty()) {
            Some(levels) => format!(" for {levels}"),
            None if wild_rift => " for 15".to_string(),
            None => String::new(),
        };

        let render_pp = |expr: String| -> Result<String> {
            render_pp_progression_from_parts(vec![expr], HashMap::new(), ctx)
        };
        let attack_damage = render_pp(format!("({value})*{af}{for_clause}"))?;
        let ability_power = render_pp(format!("({value}){for_clause}"))?;
        Ok(ExpansionResult {
            expanded: format!(
                "{attack_damage} **bonus** Attack Damage or {ability_power} Ability Power (Adaptive)"
            ),
        })
    }
}

/// `{{aug|type|name|...}}` renders an Arena augment icon and label; the reader
/// content is the augment name (second positional value).
struct AugExpander;
impl TemplateExpander for AugExpander {
    fn names(&self) -> &'static [&'static str] {
        &["aug"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .get(1)
                .or_else(|| positional.first())
                .map(|value| value.trim().to_string())
                .unwrap_or_default(),
        })
    }
}

/// `{{NumberSup|n}}` renders an ordinal (e.g. `1st`, `12th`).
struct NumberSupExpander;
impl TemplateExpander for NumberSupExpander {
    fn names(&self) -> &'static [&'static str] {
        &["NumberSup"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let raw = positional.first().map(|v| v.trim()).unwrap_or("");
        let value = expand_nested_template_text(raw, ctx).unwrap_or_else(|_| raw.to_string());
        let Some(number) = evaluate_numeric(value.trim()) else {
            return Err(unhandled_template_error(inv));
        };
        let n = number as i64;
        let suffix = match (n.rem_euclid(100), n.rem_euclid(10)) {
            (11..=13, _) => "th",
            (_, 1) => "st",
            (_, 2) => "nd",
            (_, 3) => "rd",
            _ => "th",
        };
        Ok(ExpansionResult {
            expanded: format!("{}{}", format_progression_number(number, None), suffix),
        })
    }
}

/// `{{MinuteDisplay|s1|s2|...}}` sums the (second) values and renders `M:SS`.
struct MinuteDisplayExpander;
impl TemplateExpander for MinuteDisplayExpander {
    fn names(&self) -> &'static [&'static str] {
        &["MinuteDisplay"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let mut total = 0.0;
        for value in &positional {
            let resolved =
                expand_nested_template_text(value, ctx).unwrap_or_else(|_| value.trim().to_string());
            let resolved = resolved.trim();
            if resolved.is_empty() {
                continue;
            }
            let Some(seconds) = evaluate_numeric(resolved) else {
                return Err(unhandled_template_error(inv));
            };
            total += seconds;
        }
        let total = total as i64;
        Ok(ExpansionResult {
            expanded: format!("{}:{:02}", total / 60, total.rem_euclid(60)),
        })
    }
}

/// `{{supNote|text}}` is a superscript footnote marker pointing at a note.
struct SupNoteExpander;
impl TemplateExpander for SupNoteExpander {
    fn names(&self) -> &'static [&'static str] {
        &["supNote"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let expanded = match positional.first().map(|v| v.trim()).filter(|v| !v.is_empty()) {
            Some(note) => format!("(note: {note})"),
            None => "(note)".to_string(),
        };
        Ok(ExpansionResult { expanded })
    }
}

struct ScrollBoxExpander;
impl TemplateExpander for ScrollBoxExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Scroll box"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: named
                .get("content")
                .cloned()
                .or_else(|| positional.first().cloned())
                .unwrap_or_default(),
        })
    }
}

struct RuneDataExpander;
impl TemplateExpander for RuneDataExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Rune data"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
            return Err(unhandled_template_error(inv));
        };

        let (positional, _named) = split_named_and_positional(inv);
        let Some(rune_name) = positional.first().map(|value| value.trim()) else {
            return Err(unhandled_template_error(inv));
        };
        let field = if positional.len() >= 3 {
            positional[2].trim()
        } else {
            positional.get(1).map(|value| value.trim()).unwrap_or("")
        };
        if rune_name.is_empty() || field.is_empty() {
            return Err(unhandled_template_error(inv));
        }

        let title = format!("Template:Rune data {}", rune_name);
        let Some(raw) = conv_ctx.export().read_template_page(&title)? else {
            return Err(unhandled_template_error(inv));
        };
        let Some(named_params) = read_named_template_params_from_page(&raw) else {
            return Err(unhandled_template_error(inv));
        };
        let Some(value_raw) = named_params.get(&field.to_ascii_lowercase()) else {
            return Err(unhandled_template_error(inv));
        };
        let expanded = expand_inline_templates_with_store(
            value_raw,
            ctx.precision,
            ctx.vars.clone(),
            conv_ctx.registry(),
            ctx.conversion_ctx.clone(),
        )?;
        Ok(ExpansionResult {
            expanded: expanded.trim().to_string(),
        })
    }
}

struct MapChangesExpander;
impl TemplateExpander for MapChangesExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Map changes"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
            return Err(unhandled_template_error(inv));
        };

        let (positional, _named) = split_named_and_positional(inv);
        let Some(entity) = positional.first().map(|value| value.trim()) else {
            return Err(unhandled_template_error(inv));
        };
        if entity.is_empty() || entity.eq_ignore_ascii_case("table") {
            return Err(unhandled_template_error(inv));
        }

        const MODE_TEMPLATES: &[(&str, &str)] = &[
            ("swift", "Swiftplay"),
            ("aram", "Howling Abyss"),
            ("ofa", "One for All"),
            ("nb", "Nexus Blitz"),
            ("urf", "Ultra Rapid Fire"),
            ("usb", "Ultimate Spellbook"),
            ("ar", "Arena"),
        ];

        let mut sections = Vec::new();
        for (mode_code, mode_label) in MODE_TEMPLATES {
            let title = format!("Template:Map changes/data/{}", mode_code);
            let Some(raw) = conv_ctx.export().read_template_page(&title)? else {
                continue;
            };
            let Some(named_params) = read_named_template_params_from_page(&raw) else {
                continue;
            };
            let Some(value_raw) = named_params.get(&entity.to_ascii_lowercase()) else {
                continue;
            };
            let expanded = expand_inline_templates_with_store(
                value_raw,
                ctx.precision,
                ctx.vars.clone(),
                conv_ctx.registry(),
                ctx.conversion_ctx.clone(),
            )?;
            let trimmed = expanded.trim();
            if trimmed.is_empty() {
                continue;
            }
            sections.push(format!("### {}\n\n{}", mode_label, trimmed));
        }

        Ok(ExpansionResult {
            expanded: sections.join("\n\n"),
        })
    }
}

struct ItemInfoVarExpander;
impl TemplateExpander for ItemInfoVarExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Item info/var", "Item info var"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let label = positional
            .first()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .or_else(|| {
                named
                    .get("1")
                    .map(|value| value.trim())
                    .filter(|value| !value.is_empty())
            })
            .unwrap_or_default();
        Ok(ExpansionResult {
            expanded: label.to_string(),
        })
    }
}

struct InvokeExpander;
impl TemplateExpander for InvokeExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#invoke"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let module = positional
            .first()
            .map(|value| value.trim())
            .unwrap_or_default();
        let function = positional
            .get(1)
            .map(|value| value.trim())
            .unwrap_or_default();

        if module.eq_ignore_ascii_case("Gold value")
            && function.eq_ignore_ascii_case("wikivaluedefine")
        {
            if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
                for (key, data) in conv_ctx.gold_value_data_map()? {
                    if let Some(value) = data.get("val").and_then(lua_value_to_string) {
                        ctx.set_var(key.clone(), value);
                    }
                }
            }
        }

        // {{#invoke:ItemData|get|item=X|datatype=Y}} resolves an item constant,
        // the same data source backing the {{cid}} template.
        if module.eq_ignore_ascii_case("ItemData") && function.eq_ignore_ascii_case("get") {
            if let (Some(item), Some(datatype)) = (named.get("item"), named.get("datatype")) {
                if let Ok(value) = resolve_item_constant(ctx, item.trim(), datatype.trim()) {
                    return Ok(ExpansionResult { expanded: value });
                }
            }
        }

        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

struct RecipeItemExpander;
impl TemplateExpander for RecipeItemExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Recipe/item", "Recipe item"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let label = positional
            .first()
            .map(|value| {
                expand_nested_template_text(value, ctx).unwrap_or_else(|_| value.trim().to_string())
            })
            .unwrap_or_default();
        Ok(ExpansionResult {
            expanded: if label.trim().is_empty() {
                String::new()
            } else {
                format!("* {}", label.trim())
            },
        })
    }
}

struct RuneInlineStyleExpander;
impl TemplateExpander for RuneInlineStyleExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ris"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .first()
                .map(|value| {
                    expand_nested_template_text(value, ctx)
                        .unwrap_or_else(|_| value.trim().to_string())
                })
                .unwrap_or_default(),
        })
    }
}

struct SimpleStatExpander;
impl TemplateExpander for SimpleStatExpander {
    fn names(&self) -> &'static [&'static str] {
        &["sse"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .first()
                .map(|value| {
                    expand_nested_template_text(value, ctx)
                        .unwrap_or_else(|_| value.trim().to_string())
                })
                .unwrap_or_default(),
        })
    }
}

fn read_named_template_params_from_page(raw: &str) -> Option<HashMap<String, String>> {
    let span = crate::parse::extract_balanced_templates(raw)
        .ok()?
        .into_iter()
        .next()?;
    let body = &span.raw[2..span.raw.len() - 2];
    let inv = parse_invocation(body);
    let mut named = HashMap::new();
    for param in inv.params {
        let Some(eq) = param.find('=') else {
            continue;
        };
        let key = param[..eq].trim().to_ascii_lowercase();
        let value = param[eq + 1..].trim();
        if key.is_empty() || value.is_empty() {
            continue;
        }
        named.insert(key, value.to_string());
    }
    Some(named)
}

// Gold amount: {{g|100}} -> 100 (drop currency glyph for now)
struct GoldExpander;
impl TemplateExpander for GoldExpander {
    fn names(&self) -> &'static [&'static str] {
        &["g"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let v = inv.params.first().cloned().unwrap_or_default();
        let expanded = expand_nested_template_text(&v, ctx).unwrap_or(v);
        Ok(ExpansionResult { expanded })
    }
}

struct GoldValueExpander;
impl TemplateExpander for GoldValueExpander {
    fn names(&self) -> &'static [&'static str] {
        &["gold value"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let label = positional
            .first()
            .map(|value| value.trim().trim_matches('\''))
            .unwrap_or("");
        if label.is_empty() || label.eq_ignore_ascii_case("gold value") {
            return Ok(ExpansionResult {
                expanded: String::new(),
            });
        }
        Ok(ExpansionResult {
            expanded: format!("**{}**", label),
        })
    }
}

struct GoldEfficiencyCalculationExpander;
impl TemplateExpander for GoldEfficiencyCalculationExpander {
    fn names(&self) -> &'static [&'static str] {
        &["gec", "gold efficiency calculation"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let item_name = positional
            .first()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .or_else(|| {
                ctx.get_var("__item_name")
                    .map(|value| value.trim().to_string())
                    .filter(|value| !value.is_empty())
            });
        let Some(item_name) = item_name else {
            return Err(unhandled_template_error(inv));
        };
        let item = item_name.as_str();

        let buy_raw = named
            .get("cost")
            .cloned()
            .or_else(|| resolve_item_constant(ctx, item, "buy").ok())
            .unwrap_or_default();
        let buy_resolved = expand_nested_template_text(&buy_raw, ctx).unwrap_or(buy_raw);
        let Some(buy_value) = evaluate_numeric(buy_resolved.trim()) else {
            return Err(unhandled_template_error(inv));
        };
        if buy_value == 0.0 {
            return Err(unhandled_template_error(inv));
        }

        let tgv_source = named
            .get("tgv")
            .cloned()
            .or_else(|| positional.get(1).cloned())
            .or_else(|| ctx.get_var("total"));
        let Some(tgv_source) = tgv_source else {
            return Err(unhandled_template_error(inv));
        };
        let tgv_resolved = expand_nested_template_text(&tgv_source, ctx).unwrap_or(tgv_source);
        let trimmed = tgv_resolved.trim();
        if trimmed.is_empty() {
            return Err(unhandled_template_error(inv));
        }

        let (plus_mode, numeric_expr) = if let Some(rest) = trimmed.strip_prefix('+') {
            (true, rest.trim())
        } else {
            (false, trimmed)
        };
        let Some(value) = evaluate_numeric(numeric_expr) else {
            return Err(unhandled_template_error(inv));
        };

        let percent = format!(
            "{}%",
            format_progression_number((value / buy_value) * 100.0, None)
        );
        let expanded = if plus_mode {
            format!("{} (+{}g)", percent, format_progression_number(value, None))
        } else {
            let delta = value - buy_value;
            let delta_text = if delta < 0.0 {
                format!("{}g", format_progression_number(delta, None))
            } else {
                format!("+{}g", format_progression_number(delta, None))
            };
            format!("{} ({})", percent, delta_text)
        };

        Ok(ExpansionResult { expanded })
    }
}

// Critical damage marker: {{critical damage|...|...|mod=0.9}} -> "90%"
struct CriticalDamageExpander;
impl TemplateExpander for CriticalDamageExpander {
    fn names(&self) -> &'static [&'static str] {
        &["critical damage"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Ok(ExpansionResult {
                expanded: "critical damage".to_string(),
            });
        }

        let (positional, named) = split_named_and_positional(inv);
        let suffix = if named
            .get("flat")
            .map(|value| value.trim().eq_ignore_ascii_case("true"))
            .unwrap_or(false)
        {
            ""
        } else {
            "%"
        };

        // Prefer the modifier percentage when present.
        if let Some(factor) = named.get("mod").and_then(|value| {
            let resolved =
                expand_nested_template_text(value, ctx).unwrap_or_else(|_| value.trim().to_string());
            evaluate_numeric(resolved.trim())
        }) {
            return Ok(ExpansionResult {
                expanded: format!("{}%", format_progression_number(factor * 100.0, None)),
            });
        }

        let first_raw = positional.first().map(|value| value.trim()).unwrap_or("");
        let first = expand_nested_template_text(first_raw, ctx)
            .unwrap_or_else(|_| first_raw.to_string());
        let first = first.trim();

        // Chance-scaling and other range forms render `a to b` as a percentage range.
        if let Some((start, end)) = first.split_once(" to ") {
            if let (Some(start_value), Some(end_value)) =
                (evaluate_numeric(start.trim()), evaluate_numeric(end.trim()))
            {
                return Ok(ExpansionResult {
                    expanded: format!(
                        "{}{suffix} to {}{suffix}",
                        format_progression_number(start_value, None),
                        format_progression_number(end_value, None),
                    ),
                });
            }
        }

        if let Some(percent) = render_percentage_literal(first) {
            return Ok(ExpansionResult { expanded: percent });
        }

        Err(unhandled_template_error(inv))
    }
}

fn render_percentage_literal(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if let Some(stripped) = trimmed.strip_suffix('%') {
        let value = stripped.trim();
        if !value.is_empty() {
            return Some(format!("{}%", value));
        }
    }
    if let Ok(value) = trimmed.parse::<f64>() {
        return Some(format!("{}%", format_progression_number(value, None)));
    }
    if let Ok(value) = evaluate_expression(trimmed, ExprNumberFormat::Float(6)) {
        if let Ok(number) = value.trim().parse::<f64>() {
            return Some(format!("{}%", format_progression_number(number, None)));
        }
    }
    None
}

struct TooltipExpander; // {{tt|value|tooltip}}
impl TemplateExpander for TooltipExpander {
    fn names(&self) -> &'static [&'static str] {
        &["tt"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing value".into(),
            });
        }
        let value = expand_nested_template_text(&inv.params[0], ctx)
            .unwrap_or_else(|_| inv.params[0].trim().to_string());
        let tooltip = inv
            .params
            .get(1)
            .map(|s| {
                let expanded =
                    expand_nested_template_text(s, ctx).unwrap_or_else(|_| s.trim().to_string());
                format!(" ({expanded})")
            })
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
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.len() < 2 {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "expected name|value".into(),
            });
        }
        let expanded_value = expand_assignment_value(inv, ctx)?;
        ctx.set_var(inv.params[0].trim(), expanded_value);
        // Definition removed from output (value substituted later via #var)
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

struct VarDefineEchoExpander;
impl TemplateExpander for VarDefineEchoExpander {
    fn names(&self) -> &'static [&'static str] {
        &["#vardefineecho"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.len() < 2 {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "expected name|value".into(),
            });
        }
        let expanded_value = expand_assignment_value(inv, ctx)?;
        ctx.set_var(inv.params[0].trim(), expanded_value.clone());
        Ok(ExpansionResult {
            expanded: expanded_value,
        })
    }
}

fn expand_assignment_value(inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<String> {
    let value_raw = inv.params.get(1).map(|value| value.trim()).unwrap_or("");
    if value_raw.is_empty() {
        return Ok(String::new());
    }
    let fallback_registry;
    let registry = if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
        conv_ctx.registry()
    } else {
        fallback_registry = TemplateRegistry::new();
        &fallback_registry
    };
    let expanded = expand_inline_templates_with_store(
        value_raw,
        ctx.precision,
        ctx.vars.clone(),
        registry,
        ctx.conversion_ctx.clone(),
    )?;
    Ok(expanded.trim().to_string())
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
        match ctx.get_var(key) {
            Some(v) => Ok(ExpansionResult {
                expanded: v.clone(),
            }),
            None => Ok(ExpansionResult {
                expanded: format!("<!-- UNDEFINED VAR: {} -->", key.trim()),
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
    if lower.starts_with("#vardefineecho:") {
        if let Some(idx) = name.find(':') {
            let var_name = name[idx + 1..].trim().to_string();
            *name = "#vardefineecho".into();
            params.insert(0, var_name);
        }
    } else if lower.starts_with("#vardefine:") {
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
    } else if lower.starts_with("rune data ") {
        let rune_name = name[10..].trim().to_string();
        *name = "Rune data".into();
        params.insert(0, rune_name);
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

struct ApExpander;
impl TemplateExpander for ApExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ap"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: render_ap_progression(inv, ctx)?,
        })
    }
}

struct PpExpander;
impl TemplateExpander for PpExpander {
    fn names(&self) -> &'static [&'static str] {
        &["pp"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: render_pp_progression(inv, ctx)?,
        })
    }
}

struct PpTooltipExpander;
impl TemplateExpander for PpTooltipExpander {
    fn names(&self) -> &'static [&'static str] {
        &["pptooltip"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (mut positional, named) = split_named_and_positional(inv);
        if positional.is_empty() {
            if let Some(bot_values) = named.get("bot_values").cloned() {
                positional.push(bot_values);
            }
            if let Some(top_values) = named.get("top_values").cloned() {
                positional.push(top_values);
            }
        }
        Ok(ExpansionResult {
            expanded: render_pp_progression_from_parts(positional, named, ctx)?,
        })
    }
}

struct FdExpander; // {{fd|number|2}} fixed decimals
impl TemplateExpander for FdExpander {
    fn names(&self) -> &'static [&'static str] {
        &["fd"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing number".into(),
            });
        }
        let raw = expand_nested_template_text(inv.params[0].trim(), ctx)
            .unwrap_or_else(|_| inv.params[0].trim().to_string());
        let aux = inv
            .params
            .get(1)
            .map(|value| {
                expand_nested_template_text(value.trim(), ctx)
                    .unwrap_or_else(|_| value.trim().to_string())
            })
            .unwrap_or_default();
        let aux = aux.trim();
        let (expr, suffix) = match raw.strip_suffix('%') {
            Some(value) => (value.trim(), "%"),
            None => (raw.as_str(), ""),
        };
        if let Some(num) = evaluate_numeric(expr) {
            if aux.is_empty() {
                return Ok(ExpansionResult {
                    expanded: format!("{:.2}{}", num, suffix),
                });
            }
            if let Ok(prec) = aux.parse::<usize>() {
                return Ok(ExpansionResult {
                    expanded: format!("{:.*}{}", prec, num, suffix),
                });
            }

            return Ok(ExpansionResult {
                expanded: format!("{} ({})", format!("{:.2}{}", num, suffix), aux),
            });
        }

        if raw.contains("<!--") || raw.contains("{{") {
            return Err(unhandled_template_error(inv));
        }
        if !looks_like_textual_fd_value(&raw) && (aux.is_empty() || aux.parse::<usize>().is_ok()) {
            return Err(unhandled_template_error(inv));
        }

        Ok(ExpansionResult {
            expanded: if aux.is_empty() || aux.parse::<usize>().is_ok() {
                raw.trim().to_string()
            } else {
                format!("{} ({})", raw.trim(), aux)
            },
        })
    }
}

struct ColumnExpander;
impl TemplateExpander for ColumnExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Column"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let content = if positional.len() > 1 {
            positional[1..].join("\n")
        } else {
            positional.first().cloned().unwrap_or_default()
        };
        let trimmed = content.trim();
        Ok(ExpansionResult {
            expanded: if trimmed.is_empty() {
                String::new()
            } else {
                format!("\n{}\n", trimmed)
            },
        })
    }
}

struct RdExpander; // {{rd|melee|ranged}} melee/ranged split display
impl TemplateExpander for RdExpander {
    fn names(&self) -> &'static [&'static str] {
        &["rd"]
    }

    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let positional: Vec<&str> = inv
            .params
            .iter()
            .map(|param| param.trim())
            .filter(|param| !param.is_empty() && !looks_like_named_param(param))
            .collect();
        let melee = positional.first().copied().unwrap_or("").trim();
        let ranged = positional.get(1).copied().unwrap_or(melee).trim();
        let expanded = if melee.is_empty() && ranged.is_empty() {
            String::new()
        } else if ranged.is_empty() || ranged == melee {
            melee.to_string()
        } else {
            format!("{} (melee) / {} (ranged)", melee, ranged)
        };
        Ok(ExpansionResult { expanded })
    }
}

struct IconUnwrapExpander; // minimal icon unwrap: {{ci|Aatrox}} -> Aatrox; if second param is non-possessive display, prefer it
impl TemplateExpander for IconUnwrapExpander {
    fn names(&self) -> &'static [&'static str] {
        &[
            "ci", "cis", "ai", "ais", "ii", "iis", "ri", "bi", "ui", "cai", "cais", "cci", "ccis",
            "fi", "nie", "nies", "si", "sis", "uis",
        ]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "missing label".into(),
            });
        }
        let positional: Vec<&str> = inv
            .params
            .iter()
            .map(|param| param.trim())
            .filter(|param| !param.is_empty() && !looks_like_named_param(param))
            .collect();
        let primary = positional
            .first()
            .copied()
            .unwrap_or_else(|| inv.params[0].trim());
        let is_ability_icon =
            inv.name.eq_ignore_ascii_case("ai") || inv.name.eq_ignore_ascii_case("ais");
        let display = if is_ability_icon {
            positional.get(2).copied()
        } else {
            positional.get(1).copied()
        };
        // Prefer template-specific display label if present and not possessive marker.
        let mut label = if let Some(display) = display {
            let s = display.trim();
            if s == "'s" || s == "’s" || s.is_empty() {
                primary.to_string()
            } else {
                s.to_string()
            }
        } else {
            primary.to_string()
        };
        if let Some(display) = display {
            let s = display.trim();
            if s == "'s" || s == "’s" {
                append_possessive(&mut label);
            }
        }
        Ok(ExpansionResult { expanded: label })
    }
}

fn append_possessive(label: &mut String) {
    let trimmed = label.trim_end();
    if trimmed.is_empty()
        || trimmed.ends_with("'s")
        || trimmed.ends_with("’s")
        || trimmed.ends_with('\'')
        || trimmed.ends_with('’')
    {
        return;
    }
    if trimmed.ends_with('s') || trimmed.ends_with('S') {
        label.push('\'');
    } else {
        label.push_str("'s");
    }
}

fn looks_like_named_param(param: &str) -> bool {
    split_named_param(param).is_some()
}

fn split_named_param(param: &str) -> Option<(&str, &str)> {
    let (name, value) = param.split_once('=')?;
    let key = name.trim();
    let value = value.trim();
    if !looks_like_named_param_key(key) || value.is_empty() {
        return None;
    }
    Some((key, value))
}

fn looks_like_named_param_key(key: &str) -> bool {
    let trimmed = key.trim();
    let mut chars = trimmed.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '#') {
        return false;
    }
    chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-'))
}

fn split_named_and_positional(inv: &TemplateInvocation) -> (Vec<String>, HashMap<String, String>) {
    let mut positional = Vec::new();
    let mut named = HashMap::new();
    for param in &inv.params {
        let trimmed = param.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((name, value)) = split_named_param(trimmed) {
            let key = name.to_ascii_lowercase();
            named.entry(key).or_insert_with(|| value.to_string());
            continue;
        }
        positional.push(trimmed.to_string());
    }
    (positional, named)
}

#[derive(Clone, Debug)]
struct SeriesValue {
    text: String,
    numeric: bool,
}

fn render_ap_progression(inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<String> {
    let (positional, named) = split_named_and_positional(inv);
    render_ap_progression_from_parts(positional, named, ctx)
}

fn render_ap_progression_from_parts(
    positional: Vec<String>,
    named: HashMap<String, String>,
    ctx: &ExpanderCtx,
) -> Result<String> {
    if positional.is_empty() {
        return Err(ConvertError::MalformedTemplate {
            name: "ap".into(),
            detail: "missing ability progression values".into(),
        });
    }
    let positional: Vec<String> = positional
        .into_iter()
        .map(|value| resolve_progression_fragment(&value, ctx))
        .collect();
    let named: HashMap<String, String> = named
        .into_iter()
        .map(|(key, value)| (key, resolve_progression_fragment(&value, ctx)))
        .collect();
    let round = named.get("round").map(|value| value.as_str());
    let mut values = Vec::new();
    for value in positional {
        values.extend(expand_progression_values(&value, 5, round));
    }
    if values.is_empty() {
        return Err(ConvertError::MalformedTemplate {
            name: "ap".into(),
            detail: "empty ability progression".into(),
        });
    }
    normalize_non_numeric_progression_values(&mut values);
    Ok(values
        .into_iter()
        .map(|value| value.text)
        .collect::<Vec<_>>()
        .join(" / "))
}

fn render_pp_progression(inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<String> {
    let (positional, named) = split_named_and_positional(inv);
    render_pp_progression_from_parts(positional, named, ctx)
}

fn render_pp_progression_from_parts(
    mut positional: Vec<String>,
    mut named: HashMap<String, String>,
    ctx: &ExpanderCtx,
) -> Result<String> {
    const POSITIONAL_ALIASES: &[&str] = &[
        "changedisplay",
        "showtype",
        "label1",
        "type",
        "label",
        "formula",
        "key",
        "key1",
        "round",
        "round1",
        "color",
    ];

    if positional.len() > 2 {
        for (index, alias) in POSITIONAL_ALIASES.iter().enumerate() {
            if let Some(value) = positional.get(index + 2).cloned() {
                named.entry((*alias).to_string()).or_insert(value);
            }
        }
        positional.truncate(2);
    }

    positional = positional
        .into_iter()
        .map(|value| resolve_progression_fragment(&value, ctx))
        .collect();
    named = named
        .into_iter()
        .map(|(key, value)| (key, resolve_progression_fragment(&value, ctx)))
        .collect();

    let Some(values_raw) = positional
        .first()
        .cloned()
        .filter(|value| !value.trim().is_empty())
    else {
        return Err(ConvertError::MalformedTemplate {
            name: "pp".into(),
            detail: "missing passive progression values".into(),
        });
    };

    let round = named.get("round").map(|value| value.as_str());
    let key = named.get("key").map(|value| value.as_str());
    let mut values = expand_progression_values(&values_raw, 18, round);
    append_numeric_key(&mut values, key);
    normalize_non_numeric_progression_values(&mut values);

    let mut display = if values.is_empty() {
        values_raw.trim().to_string()
    } else {
        render_progression_display(
            &values,
            named
                .get("changedisplay")
                .map(|value| is_truthy_flag(value))
                .unwrap_or(false),
        )
    };

    let show_type = !named
        .get("showtype")
        .map(|value| value.eq_ignore_ascii_case("false"))
        .unwrap_or(false);
    let mut annotations = Vec::new();
    if show_type {
        let inline_type = named
            .get("type")
            .or_else(|| named.get("label1"))
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .unwrap_or("level");
        annotations.push(format!("based on {}", inline_type));
    }
    if let Some(formula) = named.get("formula").map(|value| value.trim()) {
        if !formula.is_empty() {
            annotations.push(format!("formula: {}", prettify_formula(formula)));
        }
    }
    if !annotations.is_empty() {
        display.push_str(" (");
        display.push_str(&annotations.join("; "));
        display.push(')');
    }

    Ok(display)
}

fn resolve_progression_fragment(fragment: &str, ctx: &ExpanderCtx) -> String {
    let replaced = replace_var_templates(fragment, &ctx.vars_snapshot(), false);
    if let Some(conv_ctx) = ctx.conversion_ctx.as_ref() {
        if let Ok(expanded) = expand_inline_templates_with_store(
            replaced.trim(),
            ctx.precision,
            ctx.vars.clone(),
            conv_ctx.registry(),
            ctx.conversion_ctx.clone(),
        ) {
            return expanded.trim().to_string();
        }
    }
    replaced.trim().to_string()
}

fn expand_progression_values(
    raw: &str,
    default_count: usize,
    round: Option<&str>,
) -> Vec<SeriesValue> {
    let mut values = Vec::new();
    let mut saw_counted_then = false;
    for segment in raw
        .split(';')
        .map(|segment| segment.trim())
        .filter(|segment| !segment.is_empty())
    {
        // `then …` segments are cumulative increments relative to the running
        // value produced by the preceding segments (e.g. piecewise per-level
        // growth: `16; then +4*x for 5; then +6*x for 5`).
        if let Some(rest) = strip_ascii_prefix(segment, "then ") {
            let has_count = split_suffix(rest, " for ").is_some();
            // An open-ended trailing increment (no `for N`) continues to the
            // level/rank cap, but only when preceding counted segments have
            // positioned the series. A bare `base; then +N*x` (no counted
            // segment) encodes a delayed start whose timing lives only in the
            // prose formula, so expanding it would fabricate values.
            if let Some(expanded) =
                expand_then_segment(rest, &values, default_count, round, saw_counted_then)
            {
                saw_counted_then |= has_count;
                values.extend(expanded);
                continue;
            }
            values.push(SeriesValue {
                text: segment.to_string(),
                numeric: false,
            });
            continue;
        }
        if let Some(expanded) = expand_progression_segment(segment, default_count, round) {
            values.extend(expanded);
        } else {
            values.push(SeriesValue {
                text: segment.to_string(),
                numeric: false,
            });
        }
    }
    values
}

fn strip_ascii_prefix<'a>(value: &'a str, prefix: &str) -> Option<&'a str> {
    if value.len() >= prefix.len() && value[..prefix.len()].eq_ignore_ascii_case(prefix) {
        Some(&value[prefix.len()..])
    } else {
        None
    }
}

/// Expand a cumulative `+EXPR*x for N` increment segment, offset from the last
/// numeric value produced so far. Only segments with an explicit `for N` count
/// are expanded; open-ended increments are left to the caller (rendered raw)
/// rather than fabricating an unbounded series. The produced count is clamped
/// to the remaining level/rank budget so a series never exceeds `default_count`
/// points.
fn expand_then_segment(
    incr_raw: &str,
    prior: &[SeriesValue],
    default_count: usize,
    round: Option<&str>,
    allow_open: bool,
) -> Option<Vec<SeriesValue>> {
    let base = prior
        .iter()
        .rev()
        .find(|value| value.numeric)
        .and_then(|value| evaluate_numeric(&value.text))?;
    let count = match split_suffix(incr_raw, " for ") {
        Some((expr_part, count_part)) => {
            let count = count_part.trim().parse::<usize>().ok()?;
            // Reassign `incr_raw` to the expression portion below.
            return finish_then_segment(expr_part, base, count, prior, default_count, round);
        }
        None => {
            if !allow_open {
                return None;
            }
            // Open-ended: fill the remaining level/rank budget.
            default_count.saturating_sub(prior.len())
        }
    };
    finish_then_segment(incr_raw, base, count, prior, default_count, round)
}

fn finish_then_segment(
    expr: &str,
    base: f64,
    count: usize,
    prior: &[SeriesValue],
    default_count: usize,
    round: Option<&str>,
) -> Option<Vec<SeriesValue>> {
    let remaining = default_count.saturating_sub(prior.len());
    let count = count.min(remaining);
    if count == 0 {
        return Some(Vec::new());
    }
    let expr = expr.trim().trim_start_matches('+').trim();
    if !expr.contains('x') {
        return None;
    }
    let mut values = Vec::with_capacity(count);
    for index in 1..=count {
        let replaced = expr.replace('x', &index.to_string());
        let delta = evaluate_numeric(&replaced)?;
        values.push(SeriesValue {
            text: format_progression_number(base + delta, round),
            numeric: true,
        });
    }
    Some(values)
}

fn expand_progression_segment(
    raw: &str,
    default_count: usize,
    round: Option<&str>,
) -> Option<Vec<SeriesValue>> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Some(Vec::new());
    }
    if trimmed.to_ascii_lowercase().starts_with("then ") {
        return None;
    }

    if let Some((expr, count)) = split_suffix(trimmed, " for ") {
        if expr.contains('x') {
            let count = count.trim().parse::<usize>().ok()?;
            return evaluate_x_series(expr, count, round);
        }
        if let Some((start, end)) = expr.split_once(" to ") {
            let count = count.trim().parse::<usize>().ok()?;
            return interpolate_series(start.trim(), end.trim(), count, round);
        }
    }

    if let Some((expr, step)) = split_suffix(trimmed, " by ") {
        if let Some((start, end)) = expr.split_once(" to ") {
            return step_series(start.trim(), end.trim(), step.trim(), round);
        }
    }

    if trimmed.contains('x') {
        let (expr, count) = split_trailing_count(trimmed);
        return evaluate_x_series(expr.trim(), count.unwrap_or(default_count), round);
    }

    if let Some((start, end)) = trimmed.split_once(" to ") {
        // The end value may carry an explicit point count, e.g. `60 to 310 6`.
        let (end_expr, trailing_count) = split_trailing_count(end.trim());
        let count = trailing_count.unwrap_or(default_count);
        return interpolate_series(start.trim(), end_expr.trim(), count, round);
    }

    evaluate_numeric(trimmed).map(|value| {
        vec![SeriesValue {
            text: format_progression_number(value, round),
            numeric: true,
        }]
    })
}

fn split_suffix<'a>(value: &'a str, marker: &str) -> Option<(&'a str, &'a str)> {
    let index = value.to_ascii_lowercase().rfind(marker)?;
    Some((&value[..index], &value[index + marker.len()..]))
}

fn split_trailing_count(value: &str) -> (&str, Option<usize>) {
    // A trailing count must be separated by whitespace (e.g. `60 to 310 6`).
    // Without this guard a bare single token like `310` would be misread as a
    // count of 310 with an empty expression.
    let trimmed = value.trim_end();
    let Some(pos) = trimmed.rfind(char::is_whitespace) else {
        return (value, None);
    };
    let tail = trimmed[pos..].trim();
    if !tail.is_empty() && tail.chars().all(|ch| ch.is_ascii_digit()) {
        if let Ok(count) = tail.parse::<usize>() {
            return (trimmed[..pos].trim_end(), Some(count));
        }
    }
    (value, None)
}

fn interpolate_series(
    start_expr: &str,
    end_expr: &str,
    count: usize,
    round: Option<&str>,
) -> Option<Vec<SeriesValue>> {
    if count == 0 {
        return Some(Vec::new());
    }
    let start = evaluate_numeric(start_expr)?;
    let end = evaluate_numeric(end_expr)?;
    if count == 1 {
        return Some(vec![SeriesValue {
            text: format_progression_number(start, round),
            numeric: true,
        }]);
    }
    let step = (end - start) / (count as f64 - 1.0);
    Some(
        (0..count)
            .map(|index| SeriesValue {
                text: format_progression_number(start + step * index as f64, round),
                numeric: true,
            })
            .collect(),
    )
}

fn step_series(
    start_expr: &str,
    end_expr: &str,
    step_expr: &str,
    round: Option<&str>,
) -> Option<Vec<SeriesValue>> {
    let start = evaluate_numeric(start_expr)?;
    let end = evaluate_numeric(end_expr)?;
    let step = evaluate_numeric(step_expr)?;
    if step == 0.0 {
        return None;
    }
    // The `by` step is a magnitude; the direction of travel is implied by the
    // start/end endpoints (a `3.5 to 2 by 0.05` series decreases).
    let forward = end >= start;
    let step = if forward { step.abs() } else { -step.abs() };
    let mut current = start;
    let mut values = Vec::new();
    let mut iterations = 0usize;
    while iterations < 64 {
        values.push(SeriesValue {
            text: format_progression_number(current, round),
            numeric: true,
        });
        if (forward && current >= end) || (!forward && current <= end) {
            break;
        }
        current += step;
        if (forward && current > end) || (!forward && current < end) {
            current = end;
        }
        iterations += 1;
    }
    Some(values)
}

fn evaluate_x_series(expr: &str, count: usize, round: Option<&str>) -> Option<Vec<SeriesValue>> {
    if count == 0 {
        return Some(Vec::new());
    }
    let mut values = Vec::new();
    for index in 1..=count {
        let replaced = expr.replace('x', &index.to_string());
        let value = evaluate_numeric(&replaced)?;
        values.push(SeriesValue {
            text: format_progression_number(value, round),
            numeric: true,
        });
    }
    Some(values)
}

fn split_expr_rounding(expr: &str) -> (String, Option<String>) {
    let trimmed = expr.trim();
    let lower = trimmed.to_ascii_lowercase();

    if let Some(idx) = lower.rfind(" round") {
        let digits = lower[idx + " round".len()..].trim();
        if !digits.is_empty() && digits.chars().all(|ch| ch.is_ascii_digit()) {
            return (
                trimmed[..idx].trim_end().to_string(),
                Some(digits.to_string()),
            );
        }
    }

    if let Some(idx) = lower.rfind("round") {
        let digits = lower[idx + "round".len()..].trim();
        let prev = lower[..idx].chars().last().unwrap_or(' ');
        if !digits.is_empty()
            && digits.chars().all(|ch| ch.is_ascii_digit())
            && !prev.is_ascii_alphabetic()
        {
            return (
                trimmed[..idx].trim_end().to_string(),
                Some(digits.to_string()),
            );
        }
    }

    (trimmed.to_string(), None)
}

fn evaluate_expression_display(expr: &str, precision: u8) -> Option<String> {
    let (core, round_digits) = split_expr_rounding(expr);
    let evaluated = evaluate_expression(core.trim(), ExprNumberFormat::Float(precision)).ok()?;
    if let Some(digits) = round_digits {
        let numeric = evaluated.trim().parse::<f64>().ok()?;
        return Some(format_progression_number(numeric, Some(digits.as_str())));
    }
    Some(evaluated)
}

fn evaluate_numeric(expr: &str) -> Option<f64> {
    let (core, round_digits) = split_expr_rounding(expr);
    let evaluated = evaluate_expression(core.trim(), ExprNumberFormat::Float(6)).ok()?;
    let mut value = evaluated.trim().parse::<f64>().ok()?;
    if let Some(digits) = round_digits {
        let precision = digits.parse::<i32>().ok()?;
        let factor = 10f64.powi(precision);
        value = (value * factor).round() / factor;
    }
    Some(value)
}

fn format_progression_number(value: f64, round: Option<&str>) -> String {
    let round = round.map(|value| value.trim().to_ascii_lowercase());
    let rounded = match round.as_deref() {
        Some("abs") => value.abs(),
        Some("ceil") => value.ceil(),
        Some("floor") => value.floor(),
        Some("trunc") => value.trunc(),
        Some("false") => value,
        Some(digits) => {
            if let Ok(precision) = digits.parse::<u32>() {
                let factor = 10f64.powi(precision as i32);
                (value * factor).round() / factor
            } else {
                (value * 100.0).round() / 100.0
            }
        }
        None => (value * 100.0).round() / 100.0,
    };

    let mut rendered = match round.as_deref() {
        Some("false") => format!("{rounded:.6}"),
        Some(digits) if digits.chars().all(|ch| ch.is_ascii_digit()) => {
            let precision = digits.parse::<usize>().unwrap_or(2);
            format!("{rounded:.precision$}")
        }
        _ => format!("{rounded:.2}"),
    };
    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }
    if rendered == "-0" {
        "0".to_string()
    } else {
        rendered
    }
}

fn render_progression_display(values: &[SeriesValue], changed_display: bool) -> String {
    if values.is_empty() {
        return String::new();
    }
    let show_range = if values.len() <= 5 {
        changed_display
    } else {
        !changed_display
    };
    if show_range && values.len() > 1 {
        format!(
            "{} – {}",
            values.first().unwrap().text,
            values.last().unwrap().text
        )
    } else {
        values
            .iter()
            .map(|value| value.text.as_str())
            .collect::<Vec<_>>()
            .join(" / ")
    }
}

fn normalize_non_numeric_progression_values(values: &mut [SeriesValue]) {
    for value in values {
        if !value.numeric {
            value.text = wrap_formula_code_if_needed(&value.text);
        }
    }
}

fn wrap_formula_code_if_needed(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.starts_with('`') && trimmed.ends_with('`') {
        return trimmed.to_string();
    }
    if looks_formula_like(trimmed) {
        format!("`{}`", trimmed)
    } else {
        trimmed.to_string()
    }
}

fn looks_formula_like(raw: &str) -> bool {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return false;
    }
    let has_digit = trimmed.chars().any(|ch| ch.is_ascii_digit());
    let has_symbolic = trimmed.contains('%')
        || trimmed.chars().any(|ch| ch.is_ascii_alphabetic())
        || trimmed.contains('x');
    let has_operator = trimmed.contains('*')
        || trimmed.contains('/')
        || trimmed.contains('(')
        || trimmed.contains(')');
    has_digit && has_symbolic && has_operator
}

fn append_numeric_key(values: &mut [SeriesValue], key: Option<&str>) {
    let Some(key) = key
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    else {
        return;
    };
    for value in values {
        if value.numeric {
            value.text.push_str(key);
        }
    }
}

fn is_truthy_flag(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes"
    )
}

fn prettify_formula(formula: &str) -> String {
    formula
        .replace('*', " × ")
        .replace('/', " ÷ ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

struct ChampionSkinLinkExpander;
impl TemplateExpander for ChampionSkinLinkExpander {
    fn names(&self) -> &'static [&'static str] {
        &["csl", "Champion skin link", "WRskin", "TFTc"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let mut label = positional
            .get(2)
            .cloned()
            .or_else(|| positional.get(1).cloned())
            .or_else(|| positional.first().cloned())
            .unwrap_or_default();
        if named
            .get("possessive")
            .map(|value| is_truthy_flag(value))
            .unwrap_or(false)
        {
            append_possessive(&mut label);
        }
        Ok(ExpansionResult { expanded: label })
    }
}

struct ChampionSkinTriviaExpander;
impl TemplateExpander for ChampionSkinTriviaExpander {
    fn names(&self) -> &'static [&'static str] {
        &["cst", "Champion skin trivia"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let label = positional
            .get(2)
            .cloned()
            .or_else(|| positional.get(1).cloned())
            .unwrap_or_default();
        Ok(ExpansionResult {
            expanded: if label.trim().is_empty() {
                String::new()
            } else {
                format!("[Skin: {}]", label.trim())
            },
        })
    }
}

struct CharacterBackgroundIconExpander;
impl TemplateExpander for CharacterBackgroundIconExpander {
    fn names(&self) -> &'static [&'static str] {
        &["cbi", "cbis", "Character background icon"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let mut label = positional
            .get(1)
            .cloned()
            .or_else(|| positional.first().cloned())
            .unwrap_or_default();
        let force_possessive = inv.name.eq_ignore_ascii_case("cbis");
        if force_possessive
            || named
                .get("possessive")
                .map(|value| is_truthy_flag(value))
                .unwrap_or(false)
        {
            append_possessive(&mut label);
        }
        Ok(ExpansionResult { expanded: label })
    }
}

struct CustomContentIconExpander;
impl TemplateExpander for CustomContentIconExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ccib", "Custom content icon"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .get(2)
                .cloned()
                .or_else(|| positional.get(1).cloned())
                .unwrap_or_default(),
        })
    }
}

struct LegendsOfRuneterraExpander;
impl TemplateExpander for LegendsOfRuneterraExpander {
    fn names(&self) -> &'static [&'static str] {
        &["LoR"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional.first().cloned().unwrap_or_default(),
        })
    }
}

struct FandomExpander;
impl TemplateExpander for FandomExpander {
    fn names(&self) -> &'static [&'static str] {
        &["f", "Fandom"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .get(2)
                .cloned()
                .or_else(|| positional.get(1).cloned())
                .unwrap_or_default(),
        })
    }
}

struct GemsExpander;
impl TemplateExpander for GemsExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Gems"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        Ok(ExpansionResult {
            expanded: positional
                .first()
                .cloned()
                .unwrap_or_else(|| "Gems".to_string()),
        })
    }
}

struct UniverseExpander;
impl TemplateExpander for UniverseExpander {
    fn names(&self) -> &'static [&'static str] {
        &["univ", "Universe icon"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let mut label = positional
            .get(1)
            .cloned()
            .or_else(|| positional.first().cloned())
            .unwrap_or_default();
        if named
            .get("possessive")
            .map(|value| is_truthy_flag(value))
            .unwrap_or(false)
        {
            append_possessive(&mut label);
        }
        Ok(ExpansionResult { expanded: label })
    }
}

struct LabelColonExpander;
impl TemplateExpander for LabelColonExpander {
    fn names(&self) -> &'static [&'static str] {
        &["lc"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, _named) = split_named_and_positional(inv);
        let label = positional
            .first()
            .map(|value| value.trim().trim_end_matches(':'))
            .unwrap_or("");
        Ok(ExpansionResult {
            expanded: if label.is_empty() {
                String::new()
            } else {
                format!("**{}:**", label)
            },
        })
    }
}

struct CurrencyExpander;
impl TemplateExpander for CurrencyExpander {
    fn names(&self) -> &'static [&'static str] {
        &["RP", "IP", "BE", "XP", "ME"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let code = inv.name.to_ascii_uppercase();
        let value = named
            .get("text")
            .cloned()
            .or_else(|| positional.first().cloned())
            .unwrap_or_default();
        let trimmed = value.trim();
        let expanded = if trimmed.is_empty() {
            code.clone()
        } else if trimmed.eq_ignore_ascii_case(&code)
            || trimmed
                .to_ascii_uppercase()
                .ends_with(&format!(" {}", code))
            || !trimmed.chars().any(|ch| ch.is_ascii_digit())
        {
            trimmed.to_string()
        } else {
            format!("{} {}", trimmed, code)
        };
        Ok(ExpansionResult { expanded })
    }
}

struct SymbolExpander;
impl TemplateExpander for SymbolExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Infinity", "plus"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let expanded = if inv.name.eq_ignore_ascii_case("Infinity") {
            "∞"
        } else {
            " + "
        };
        Ok(ExpansionResult {
            expanded: expanded.to_string(),
        })
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

// Flip Text (ft) stylistic wrapper -> "a *(equivalently: b)*"
struct FlipTextExpander;
impl TemplateExpander for FlipTextExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ft"]
    }
    fn expand(&self, inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.is_empty() {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "requires at least one parameter".into(),
            });
        }
        let primary = inv.params[0].trim();
        let alt = inv
            .params
            .get(1)
            .map(|value| value.trim().trim_end_matches(|ch| matches!(ch, ',' | ';')))
            .unwrap_or("");
        let result = if alt.is_empty() {
            primary.to_string()
        } else {
            format!("{} *(equivalently: {})*", primary, alt)
        };
        Ok(ExpansionResult {
            expanded: format!(" {} ", result.trim()),
        })
    }
}

// Champion / item constant data substitution (ccd / cid).
struct ConstantDataExpander;
impl TemplateExpander for ConstantDataExpander {
    fn names(&self) -> &'static [&'static str] {
        &["ccd", "cid"]
    }
    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        if inv.params.len() < 2 {
            return Err(ConvertError::MalformedTemplate {
                name: inv.name.clone(),
                detail: "expected entity|field".into(),
            });
        }
        let entity = inv.params[0].trim();
        let field = inv.params[1].trim();
        if inv.name.eq_ignore_ascii_case("ccd") {
            return Ok(ExpansionResult {
                expanded: resolve_champion_constant(ctx, entity, field)?,
            });
        } else if inv.name.eq_ignore_ascii_case("cid") {
            return Ok(ExpansionResult {
                expanded: resolve_item_constant(ctx, entity, field)?,
            });
        }
        Err(ConvertError::Internal(format!(
            "unsupported constant resolver `{}`",
            inv.name
        )))
    }
}

fn resolve_champion_constant(ctx: &ExpanderCtx, entity: &str, field: &str) -> Result<String> {
    let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
        return Err(ConvertError::Internal(
            "conversion context required for champion constant lookup".to_string(),
        ));
    };

    if let Some(constants) = conv_ctx.champion_constants_or_load(entity) {
        if let Some(value) = constants.get(field) {
            return Ok(value.clone());
        }
    }

    if let Some(default) = default_champion_constant(field) {
        return Ok(default.to_string());
    }

    Err(ConvertError::Internal(format!(
        "unresolved champion constant `{entity}.{field}`"
    )))
}

fn default_champion_constant(field: &str) -> Option<&'static str> {
    match field.trim().to_ascii_lowercase().as_str() {
        "crit_base" => Some("175"),
        _ => None,
    }
}

fn resolve_item_constant(ctx: &ExpanderCtx, entity: &str, field: &str) -> Result<String> {
    let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
        return Err(ConvertError::Internal(
            "conversion context required for item constant lookup".to_string(),
        ));
    };

    let map = conv_ctx.item_module_map()?;
    if let Some(value) = resolve_item_constant_recursive(map, entity, field, &mut Vec::new()) {
        return Ok(value);
    }

    Err(ConvertError::Internal(format!(
        "unresolved item constant `{entity}.{field}`"
    )))
}

fn resolve_item_constant_recursive(
    map: &HashMap<String, HashMap<String, LuaValue>>,
    entity: &str,
    field: &str,
    visited: &mut Vec<(String, String)>,
) -> Option<String> {
    let entity_key = entity.trim().to_ascii_lowercase();
    let field_key = field.trim().to_ascii_lowercase();
    if visited.iter().any(|(existing_entity, existing_field)| {
        *existing_entity == entity_key && *existing_field == field_key
    }) {
        return None;
    }
    visited.push((entity_key, field_key));

    let item_data = map.get(entity).or_else(|| {
        map.iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(entity))
            .map(|(_, value)| value)
    })?;

    if field.eq_ignore_ascii_case("sell") {
        if let (Some(buy), Some(ratio)) = (
            resolve_item_constant_recursive(map, entity, "buy", visited),
            resolve_item_constant_recursive(map, entity, "sellratio", visited),
        ) {
            if let (Ok(buy), Ok(ratio)) = (buy.trim().parse::<f64>(), ratio.trim().parse::<f64>()) {
                return Some(format_progression_number((buy * ratio).round(), Some("0")));
            }
        }
    }

    for candidate in item_field_candidates(field) {
        if let Some(value) = get_case_insensitive(item_data, &candidate) {
            if let Some(resolved) = resolve_item_constant_value(map, field, value, visited) {
                return Some(resolved);
            }
        }
    }

    if let Some(LuaValue::Table(stats)) = get_case_insensitive(item_data, "stats") {
        for candidate in item_field_candidates(field) {
            if let Some(value) = get_case_insensitive(stats, &candidate) {
                if let Some(resolved) = resolve_item_constant_value(map, field, value, visited) {
                    return Some(resolved);
                }
            }
        }
    }

    None
}

fn resolve_item_constant_value(
    map: &HashMap<String, HashMap<String, LuaValue>>,
    field: &str,
    value: &LuaValue,
    visited: &mut Vec<(String, String)>,
) -> Option<String> {
    match value {
        LuaValue::String(raw) => {
            let trimmed = raw.trim();
            if let Some(target) = trimmed.strip_prefix("=>") {
                return resolve_item_constant_recursive(map, target.trim(), field, visited);
            }
            Some(trimmed.to_string())
        }
        LuaValue::Number(value) => Some(value.to_string()),
        LuaValue::Bool(true) => Some("1".to_string()),
        LuaValue::Bool(false) => Some("0".to_string()),
        _ => None,
    }
}

fn item_field_candidates(field: &str) -> Vec<String> {
    let normalized = field
        .trim()
        .to_ascii_lowercase()
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    match normalized.as_str() {
        "health" => vec!["health".to_string(), "hp".to_string()],
        "mana" => vec!["mana".to_string(), "mp".to_string()],
        "ability power" => vec!["ability power".to_string(), "ap".to_string()],
        "attack damage" => vec![
            "attack damage".to_string(),
            "ad".to_string(),
            "dam".to_string(),
        ],
        "armor" => vec!["armor".to_string(), "arm".to_string()],
        "magic resistance" => vec!["magic resistance".to_string(), "mr".to_string()],
        "movement speed" => vec![
            "movement speed".to_string(),
            "ms".to_string(),
            "msflat".to_string(),
            "msunique".to_string(),
        ],
        "critical strike chance" => vec!["critical strike chance".to_string(), "crit".to_string()],
        "ability haste" => vec!["ability haste".to_string(), "ah".to_string()],
        "cooldown reduction" => vec!["cooldown reduction".to_string(), "cdr".to_string()],
        "health regeneration" => vec![
            "health regeneration".to_string(),
            "hp5flat".to_string(),
            "hp5".to_string(),
        ],
        "mana regeneration" => vec![
            "mana regeneration".to_string(),
            "mp5flat".to_string(),
            "mp5".to_string(),
        ],
        "life steal" => vec!["life steal".to_string(), "lifesteal".to_string()],
        "heal and shield power" => vec!["heal and shield power".to_string(), "hsp".to_string()],
        other => vec![other.to_string()],
    }
}

fn get_case_insensitive<'a, T>(map: &'a HashMap<String, T>, key: &str) -> Option<&'a T> {
    map.get(key).or_else(|| {
        map.iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(key))
            .map(|(_, value)| value)
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HighestLowestStatField {
    Health,
    Mana,
    Armor,
    MagicResist,
    HealthRegen,
    ManaRegen,
    AttackDamage,
    AttackSpeed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HighestLowestSort {
    Top,
    Bottom,
}

#[derive(Clone, Debug)]
struct HighestLowestRow {
    champion: String,
    value: f32,
}

struct HighestLowestStatsExpander;
impl TemplateExpander for HighestLowestStatsExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Highest lowest stats"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
            return Err(unhandled_template_error(inv));
        };
        let Some(module_raw) = conv_ctx.champion_module_raw()? else {
            return Err(unhandled_template_error(inv));
        };
        let module = match parse_champion_module(module_raw) {
            Ok(module) => module,
            Err(_) => {
                return Err(unhandled_template_error(inv))
            }
        };

        let (positional, named) = split_named_and_positional(inv);
        let Some(stat_field) = positional
            .first()
            .and_then(|value| parse_highest_lowest_stat_field(value))
        else {
            return Err(unhandled_template_error(inv));
        };

        let sort = named
            .get("sortby")
            .and_then(|value| parse_highest_lowest_sort(value));
        let rangetype = named
            .get("rangetype")
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty());
        let level = named
            .get("lvl")
            .and_then(|value| value.trim().parse::<usize>().ok())
            .filter(|value| *value >= 1)
            .unwrap_or(1);
        let size = named
            .get("size")
            .and_then(|value| value.trim().parse::<usize>().ok())
            .filter(|value| *value >= 1)
            .unwrap_or(5);
        let offset = named
            .get("show")
            .and_then(|value| value.trim().parse::<usize>().ok())
            .unwrap_or(1)
            .saturating_sub(1);
        let get = named
            .get("get")
            .map(|value| value.trim().to_ascii_lowercase())
            .filter(|value| !value.is_empty());

        let mut rows = module
            .iter()
            .filter_map(|(fallback_name, entry)| {
                if let Some(ref desired_range) = rangetype {
                    let actual_range = champion_rangetype(entry)?;
                    if !actual_range.eq_ignore_ascii_case(desired_range) {
                        return None;
                    }
                }
                let value = compute_highest_lowest_stat_value(entry, stat_field, level)?;
                Some(HighestLowestRow {
                    champion: champion_display_name(entry, fallback_name),
                    value,
                })
            })
            .collect::<Vec<_>>();
        if rows.is_empty() {
            return Err(unhandled_template_error(inv));
        }

        let expanded = if let Some(sort) = sort {
            sort_highest_lowest_rows(&mut rows, sort);
            let selected = rows.into_iter().skip(offset).take(size).collect::<Vec<_>>();
            if selected.is_empty() {
                return Err(unhandled_template_error(inv));
            }
            render_highest_lowest_rows(&selected, get.as_deref())
        } else {
            let mut bottom = rows.clone();
            sort_highest_lowest_rows(&mut bottom, HighestLowestSort::Bottom);
            let bottom = bottom.into_iter().take(size).collect::<Vec<_>>();
            let mut top = rows;
            sort_highest_lowest_rows(&mut top, HighestLowestSort::Top);
            let top = top.into_iter().take(size).collect::<Vec<_>>();
            format!(
                "Lowest: {}; Highest: {}",
                render_highest_lowest_rows(&bottom, get.as_deref()),
                render_highest_lowest_rows(&top, get.as_deref())
            )
        };

        Ok(ExpansionResult { expanded })
    }
}

fn parse_highest_lowest_stat_field(raw: &str) -> Option<HighestLowestStatField> {
    let normalized = raw
        .trim()
        .to_ascii_lowercase()
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    match normalized.as_str() {
        "hp" | "health" => Some(HighestLowestStatField::Health),
        "mp" | "mana" => Some(HighestLowestStatField::Mana),
        "armor" | "arm" => Some(HighestLowestStatField::Armor),
        "magic resistance" | "magic resist" | "mr" => Some(HighestLowestStatField::MagicResist),
        "hp5" | "health regen" | "health regeneration" => Some(HighestLowestStatField::HealthRegen),
        "mp5" | "mana regen" | "mana regeneration" => Some(HighestLowestStatField::ManaRegen),
        "ad" | "attack damage" | "damage" | "dam" => Some(HighestLowestStatField::AttackDamage),
        "as" | "attack speed" => Some(HighestLowestStatField::AttackSpeed),
        _ => None,
    }
}

fn parse_highest_lowest_sort(raw: &str) -> Option<HighestLowestSort> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "top" | "highest" => Some(HighestLowestSort::Top),
        "bot" | "bottom" | "lowest" => Some(HighestLowestSort::Bottom),
        _ => None,
    }
}

fn champion_display_name(entry: &HashMap<String, LuaValue>, fallback: &str) -> String {
    for key in ["disp_name", "name", "fullname", "apiname"] {
        if let Some(label) = entry.get(key).and_then(lua_value_to_string) {
            let trimmed = label.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
    }
    fallback.to_string()
}

fn champion_rangetype(entry: &HashMap<String, LuaValue>) -> Option<String> {
    entry
        .get("rangetype")
        .and_then(lua_value_to_string)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn compute_highest_lowest_stat_value(
    entry: &HashMap<String, LuaValue>,
    field: HighestLowestStatField,
    level: usize,
) -> Option<f32> {
    let LuaValue::Table(stats) = entry.get("stats")? else {
        return None;
    };
    match field {
        HighestLowestStatField::Health => compute_scaled_stat(stats, "hp_base", "hp_lvl", level),
        HighestLowestStatField::Mana => compute_scaled_stat(stats, "mp_base", "mp_lvl", level),
        HighestLowestStatField::Armor => compute_scaled_stat(stats, "arm_base", "arm_lvl", level),
        HighestLowestStatField::MagicResist => {
            compute_scaled_stat(stats, "mr_base", "mr_lvl", level)
        }
        HighestLowestStatField::HealthRegen => {
            compute_scaled_stat(stats, "hp5_base", "hp5_lvl", level)
        }
        HighestLowestStatField::ManaRegen => {
            compute_scaled_stat(stats, "mp5_base", "mp5_lvl", level)
        }
        HighestLowestStatField::AttackDamage => {
            compute_scaled_stat(stats, "dam_base", "dam_lvl", level)
        }
        HighestLowestStatField::AttackSpeed => {
            let base = stat_number(stats, "as_base")?;
            let growth_percent = stat_number(stats, "as_lvl").unwrap_or(0.0);
            let ratio = stat_number(stats, "as_ratio").unwrap_or(base);
            let bonus = scaled_stat_growth(growth_percent / 100.0, level);
            Some(base + ratio * bonus)
        }
    }
}

fn compute_scaled_stat(
    stats: &HashMap<String, LuaValue>,
    base_key: &str,
    growth_key: &str,
    level: usize,
) -> Option<f32> {
    let base = stat_number(stats, base_key)?;
    let growth = stat_number(stats, growth_key).unwrap_or(0.0);
    Some(base + scaled_stat_growth(growth, level))
}

fn stat_number(stats: &HashMap<String, LuaValue>, key: &str) -> Option<f32> {
    match stats.get(key)? {
        LuaValue::Number(raw) | LuaValue::String(raw) => raw.trim().parse::<f32>().ok(),
        LuaValue::Bool(true) => Some(1.0),
        LuaValue::Bool(false) => Some(0.0),
        _ => None,
    }
}

fn scaled_stat_growth(growth: f32, level: usize) -> f32 {
    if level <= 1 {
        return 0.0;
    }
    let n = level.saturating_sub(1) as f32;
    growth * n * (0.7025 + 0.0175 * n)
}

fn sort_highest_lowest_rows(rows: &mut [HighestLowestRow], sort: HighestLowestSort) {
    rows.sort_by(|left, right| {
        let value_cmp = left
            .value
            .partial_cmp(&right.value)
            .unwrap_or(Ordering::Equal);
        let value_cmp = match sort {
            HighestLowestSort::Top => value_cmp.reverse(),
            HighestLowestSort::Bottom => value_cmp,
        };
        value_cmp.then_with(|| {
            left.champion
                .to_ascii_lowercase()
                .cmp(&right.champion.to_ascii_lowercase())
        })
    });
}

fn render_highest_lowest_rows(rows: &[HighestLowestRow], get: Option<&str>) -> String {
    if rows.is_empty() {
        return String::new();
    }
    if rows.len() == 1 {
        return match get {
            Some("champ") | Some("champion") => rows[0].champion.clone(),
            Some("stat") | Some("value") | None => format_highest_lowest_stat_value(rows[0].value),
            _ => format_highest_lowest_stat_value(rows[0].value),
        };
    }
    match get {
        Some("champ") | Some("champion") => rows
            .iter()
            .map(|row| row.champion.as_str())
            .collect::<Vec<_>>()
            .join(", "),
        Some("stat") | Some("value") => rows
            .iter()
            .map(|row| format_highest_lowest_stat_value(row.value))
            .collect::<Vec<_>>()
            .join(", "),
        _ => rows
            .iter()
            .map(|row| {
                format!(
                    "{} ({})",
                    row.champion,
                    format_highest_lowest_stat_value(row.value)
                )
            })
            .collect::<Vec<_>>()
            .join(", "),
    }
}

fn format_highest_lowest_stat_value(value: f32) -> String {
    let mut rendered = if value.abs() >= 1000.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.4}", value)
    };
    while rendered.contains('.') && rendered.ends_with('0') {
        rendered.pop();
    }
    if rendered.ends_with('.') {
        rendered.pop();
    }
    if rendered == "-0" {
        "0".to_string()
    } else {
        rendered
    }
}

struct ItemStatTableExpander;
impl TemplateExpander for ItemStatTableExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Item stat table"]
    }

    fn expand(&self, inv: &TemplateInvocation, ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        let (positional, named) = split_named_and_positional(inv);
        let key = positional
            .first()
            .map(|value| value.trim().to_ascii_lowercase())
            .unwrap_or_default();
        if key.is_empty() {
            return Err(unhandled_template_error(inv));
        }
        if named
            .get("wr")
            .map(|value| value.trim().eq_ignore_ascii_case("true"))
            .unwrap_or(false)
        {
            return Err(unhandled_template_error(inv));
        }
        let Some(conv_ctx) = ctx.conversion_ctx.as_ref() else {
            return Err(unhandled_template_error(inv));
        };
        let Ok(module) = conv_ctx.item_module_map() else {
            return Err(unhandled_template_error(inv));
        };
        let Some(expanded) = render_item_stat_table(module, &key) else {
            return Err(unhandled_template_error(inv));
        };
        Ok(ExpansionResult { expanded })
    }
}

struct SuperimposeExpander;
impl TemplateExpander for SuperimposeExpander {
    fn names(&self) -> &'static [&'static str] {
        &["Superimpose"]
    }

    fn expand(&self, _inv: &TemplateInvocation, _ctx: &ExpanderCtx) -> Result<ExpansionResult> {
        Ok(ExpansionResult {
            expanded: String::new(),
        })
    }
}

#[derive(Debug, Clone)]
struct ItemStatTableRow {
    item: String,
    cost: String,
    amount: String,
}

fn render_item_stat_table(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    key: &str,
) -> Option<String> {
    let rows = collect_item_stat_table_rows(module, key);
    if rows.is_empty() {
        return None;
    }
    Some(
        rows.into_iter()
            .map(|row| format!("{} ({} → {})", row.item, row.cost, row.amount))
            .collect::<Vec<_>>()
            .join("; "),
    )
}

fn collect_item_stat_table_rows(
    module: &HashMap<String, HashMap<String, LuaValue>>,
    key: &str,
) -> Vec<ItemStatTableRow> {
    if matches!(
        key,
        "offensive" | "magical" | "defensive" | "misc" | "baseefficiency"
    ) {
        return Vec::new();
    }

    let mut rows = module
        .iter()
        .filter_map(|(item_name, entry)| {
            let cost_value = item_entry_number(entry, "buy")?;
            if cost_value <= 0.0 {
                return None;
            }
            let cost = format_highest_lowest_stat_value(cost_value);
            let LuaValue::Table(stats) = entry.get("stats")? else {
                return None;
            };
            let amount = if key == "pykehealth" {
                let hp = stat_number(stats, "hp")?;
                let ad = stat_number(stats, "ad").unwrap_or(0.0);
                let converted = ((hp / 14.0 + ad) * 10.0).round() / 10.0;
                format_highest_lowest_stat_value(converted)
            } else {
                let mut parts: Vec<String> = Vec::new();
                if let Some(value) = stat_number(stats, key) {
                    parts.push(format!(
                        "{}{}",
                        format_highest_lowest_stat_value(value),
                        item_stat_table_suffix(key)
                    ));
                }
                let unique_key = format!("{}unique", key);
                if let Some(value) = stat_number(stats, &unique_key) {
                    parts.push(format!(
                        "{}{} (Unique)",
                        format_highest_lowest_stat_value(value),
                        item_stat_table_suffix(key)
                    ));
                }
                if parts.is_empty() {
                    return None;
                }
                parts.join(", ")
            };
            Some(ItemStatTableRow {
                item: item_name.clone(),
                cost,
                amount,
            })
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.item
            .to_ascii_lowercase()
            .cmp(&right.item.to_ascii_lowercase())
    });
    rows
}

fn item_entry_number(entry: &HashMap<String, LuaValue>, key: &str) -> Option<f32> {
    match entry.get(key)? {
        LuaValue::Number(raw) | LuaValue::String(raw) => raw.trim().parse::<f32>().ok(),
        LuaValue::Bool(true) => Some(1.0),
        LuaValue::Bool(false) => Some(0.0),
        _ => None,
    }
}

fn item_stat_table_suffix(key: &str) -> &'static str {
    match key {
        "as" | "crit" | "lifesteal" | "armpen" | "mpen" | "hp5" | "mp5" | "hsp" | "omnivamp"
        | "tenacity" | "ms" | "critdamage" => "%",
        "gp10" => " per 10s",
        _ => "",
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
            // Audio sample / minor markers
            "sm2",
            "#ev:youtube",
            // Anchor for sections
            "Anchor",
            // Pet infobox
            "Infobox/Pet",
            // TFT item marker
            "TFT Item",
            // References block wrappers
            "References",
            "Champion eternals",
            "Infobox/Credits",
            "Champion bio",
            "Rune header",
            "Rune footer",
            "Buff header",
            "Jungle pet info",
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
    use crate::convert::util::expand_inline_templates_mut;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tempfile::tempdir;

    fn ctx() -> ExpanderCtx {
        ExpanderCtx::new(2, &HashMap::new(), None)
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
        assert_eq!(reg.expand(&ap, &ctx()).unwrap().expanded, "40");
        let ap2 = parse_invocation("ap|60 to 140 by 20");
        assert_eq!(
            reg.expand(&ap2, &ctx()).unwrap().expanded,
            "60 / 80 / 100 / 120 / 140"
        );
        let pp = parse_invocation("pp|10;20;30");
        assert_eq!(
            reg.expand(&pp, &ctx()).unwrap().expanded,
            "10 / 20 / 30 (based on level)"
        );
        let ppt = parse_invocation("pptooltip|bot_values=5;15");
        assert_eq!(
            reg.expand(&ppt, &ctx()).unwrap().expanded,
            "5 / 15 (based on level)"
        );
        let fd = parse_invocation("fd|3.14159|3");
        assert_eq!(reg.expand(&fd, &ctx()).unwrap().expanded, "3.142");
        let ci = parse_invocation("ci|Aatrox|'s");
        assert_eq!(reg.expand(&ci, &ctx()).unwrap().expanded, "Aatrox's");
        let ii = parse_invocation("ii|Infinity Edge|image=Infinity Edge item old.png");
        assert_eq!(reg.expand(&ii, &ctx()).unwrap().expanded, "Infinity Edge");
        let ap_equation = parse_invocation("ap|75 + 90% of 45 = 115.5 magic damage");
        assert_eq!(
            reg.expand(&ap_equation, &ctx()).unwrap().expanded,
            "75 + 90% of 45=115.5 magic damage"
        );
        let cis = parse_invocation("cis|Kayle|variant=old3");
        assert_eq!(reg.expand(&cis, &ctx()).unwrap().expanded, "Kayle");
        let ai = parse_invocation("ai|World Ender|Aatrox");
        assert_eq!(reg.expand(&ai, &ctx()).unwrap().expanded, "World Ender");
        let ai_display = parse_invocation("ai|Living Forge|Ornn|Masterwork");
        assert_eq!(
            reg.expand(&ai_display, &ctx()).unwrap().expanded,
            "Masterwork"
        );
        let crit = parse_invocation("critical damage|175|100|mod=0.9");
        assert_eq!(reg.expand(&crit, &ctx()).unwrap().expanded, "90%");
    }

    #[test]
    fn expander_wraps_unresolved_formulae_in_code_spans() {
        let reg = TemplateRegistry::new();
        let ap = parse_invocation("ap|((1+(Graves-100)*0.45/100)*(1+5*0.33302)/(1+3*0.33302)*100)");
        assert_eq!(
            reg.expand(&ap, &ctx()).unwrap().expanded,
            "`((1+(Graves-100)*0.45/100)*(1+5*0.33302)/(1+3*0.33302)*100)`"
        );
    }

    #[test]
    fn constant_data_expander_resolves_defaults_and_nested_stats() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        std::fs::write(
            export_dir.join("Module%3AItemData%2Fdata.txt"),
            r#"return {
    ["Infinity Edge"] = {
        ["stats"] = {
            ["critdamage"] = 40,
        }
    },
    ["Control Ward"] = {
        ["buy"] = 75,
        ["sellratio"] = 0.4,
    }
}"#,
        )
        .unwrap();

        let conversion_ctx = Arc::new(ConversionContext::new(&export_dir, 2).unwrap());
        let mut constants = HashMap::new();
        constants.insert("missile_speed".to_string(), "3800".to_string());
        conversion_ctx.insert_champion_constants("Graves", constants);
        let ctx = ExpanderCtx::new(2, &HashMap::new(), Some(conversion_ctx));
        let reg = TemplateRegistry::new();

        let ccd = parse_invocation("ccd|Graves|crit_base");
        assert_eq!(reg.expand(&ccd, &ctx).unwrap().expanded, "175");

        let nested = parse_invocation("ccd|Graves|missile_speed");
        assert_eq!(reg.expand(&nested, &ctx).unwrap().expanded, "3800");

        let cid = parse_invocation("cid|Infinity Edge|critdamage");
        assert_eq!(reg.expand(&cid, &ctx).unwrap().expanded, "40");

        let sell = parse_invocation("cid|Control Ward|sell");
        assert_eq!(reg.expand(&sell, &ctx).unwrap().expanded, "30");
    }

    #[test]
    fn constant_data_expander_errors_on_unresolved_lookup() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        let conversion_ctx = Arc::new(ConversionContext::new(&export_dir, 2).unwrap());
        conversion_ctx.insert_champion_constants("Graves", HashMap::new());
        let ctx = ExpanderCtx::new(2, &HashMap::new(), Some(conversion_ctx));
        let reg = TemplateRegistry::new();

        let missing = parse_invocation("ccd|Graves|not_a_real_field");
        let err = reg.expand(&missing, &ctx).unwrap_err();
        assert!(err
            .to_string()
            .contains("unresolved champion constant `Graves.not_a_real_field`"));
    }

    #[test]
    fn vardefineecho_updates_state_and_allows_blank_var_in_expr() {
        let reg = TemplateRegistry::new();
        let mut vars = HashMap::new();
        let expanded = expand_inline_templates_mut(
            "{{#vardefineecho:total|{{#expr:{{#var:total}}+5}}}} then {{#var:total}}",
            2,
            &mut vars,
            &reg,
            None,
        )
        .unwrap();

        assert_eq!(expanded, "5 then 5");
        assert_eq!(vars.get("total"), Some(&"5".to_string()));
    }

    #[test]
    fn gold_value_expander_promotes_custom_section_labels() {
        let reg = TemplateRegistry::new();
        let inv = parse_invocation("gold value|First Light Gold Value (from base stats)|nolink=");
        assert_eq!(
            reg.expand(&inv, &ctx()).unwrap().expanded,
            "**First Light Gold Value (from base stats)**"
        );

        let default = parse_invocation("gold value|Gold Value");
        assert!(reg.expand(&default, &ctx()).unwrap().expanded.is_empty());
    }

    #[test]
    fn expander_progression_flags_and_template_helpers() {
        let reg = TemplateRegistry::new();

        let pp = parse_invocation("pp|key=k|2*x for 6");
        assert_eq!(
            reg.expand(&pp, &ctx()).unwrap().expanded,
            "2k – 12k (based on level)"
        );

        let pp_formula = parse_invocation(
            "pp|type=target's missing health|key=%|0 to 200 for 11|0 to 100|formula=2% per 1% of target's missing health",
        );
        assert_eq!(
            reg.expand(&pp_formula, &ctx()).unwrap().expanded,
            "0% – 200% (based on target's missing health; formula: 2% per 1% of target's missing health)"
        );

        let lc = parse_invocation("lc|Calibrum");
        assert_eq!(reg.expand(&lc, &ctx()).unwrap().expanded, "**Calibrum:**");

        let rp = parse_invocation("RP|390");
        assert_eq!(reg.expand(&rp, &ctx()).unwrap().expanded, "390 RP");

        let csl = parse_invocation("csl|Aatrox|Original|Aatrox, the Darkin Blade");
        assert_eq!(
            reg.expand(&csl, &ctx()).unwrap().expanded,
            "Aatrox, the Darkin Blade"
        );

        let cst = parse_invocation("cst|Akshan|Cyber Pop");
        assert_eq!(
            reg.expand(&cst, &ctx()).unwrap().expanded,
            "[Skin: Cyber Pop]"
        );

        let infinity = parse_invocation("Infinity");
        assert_eq!(reg.expand(&infinity, &ctx()).unwrap().expanded, "∞");
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
    fn expander_additional_reader_facing_templates() {
        let reg = TemplateRegistry::new();

        let lor = parse_invocation("LoR|Mistkeepers");
        assert_eq!(reg.expand(&lor, &ctx()).unwrap().expanded, "Mistkeepers");

        let wrskin = parse_invocation("WRskin|Akali|Crystal Rose");
        assert_eq!(
            reg.expand(&wrskin, &ctx()).unwrap().expanded,
            "Crystal Rose"
        );

        let skin_tier = parse_invocation("skin tier|legendary");
        assert_eq!(
            reg.expand(&skin_tier, &ctx()).unwrap().expanded,
            "legendary"
        );

        let tftc_display = parse_invocation("TFTc|Amumu|set=10|new splash art");
        assert_eq!(
            reg.expand(&tftc_display, &ctx()).unwrap().expanded,
            "new splash art"
        );

        let tftc_subject = parse_invocation("TFTc|Alune|set=11");
        assert_eq!(reg.expand(&tftc_subject, &ctx()).unwrap().expanded, "Alune");

        let gems = parse_invocation("Gems|Gems");
        assert_eq!(reg.expand(&gems, &ctx()).unwrap().expanded, "Gems");

        let uis = parse_invocation("uis|Tibbers");
        assert_eq!(reg.expand(&uis, &ctx()).unwrap().expanded, "Tibbers");

        let cbis = parse_invocation("cbis|Nagakabouros");
        assert_eq!(reg.expand(&cbis, &ctx()).unwrap().expanded, "Nagakabouros'");
    }

    #[test]
    fn item_stat_table_expander_renders_basic_and_pykehealth_tables() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        std::fs::write(
            export_dir.join("Module%3AItemData%2Fdata.txt"),
            r#"return {
    ["Ruby Crystal"] = { buy = 400, stats = { hp = 150 } },
    ["Black Cleaver"] = { buy = 3000, stats = { hp = 350, ad = 40 } },
    ["Long Sword"] = { buy = 350, stats = { ad = 10 } }
}"#,
        )
        .unwrap();
        let reg = TemplateRegistry::new();
        let ctx = ExpanderCtx::new(
            2,
            &HashMap::new(),
            Some(Arc::new(ConversionContext::new(&export_dir, 2).unwrap())),
        );

        let hp = parse_invocation("Item stat table|hp");
        let hp_expanded = reg.expand(&hp, &ctx).unwrap().expanded;
        assert!(hp_expanded.contains("Ruby Crystal (400 → 150)"));
        assert!(hp_expanded.contains("Black Cleaver (3000 → 350)"));
        assert!(!hp_expanded.contains("Long Sword"));

        let pykehealth = parse_invocation("Item stat table|pykehealth");
        let pyke_expanded = reg.expand(&pykehealth, &ctx).unwrap().expanded;
        assert!(pyke_expanded.contains("Ruby Crystal (400 → 10.7)"));
        assert!(pyke_expanded.contains("Black Cleaver (3000 → 65)"));
        assert!(!pyke_expanded.contains("Long Sword"));
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
                &ExpanderCtx::new(2, &HashMap::new(), Some(ctx_arc.clone())),
            )
            .unwrap();
        assert!(result
            .expanded
            .contains("* **SPELLBLADE** deals proc damage."));
    }

    #[test]
    fn include_info_templates_expand_plain_template_bodies() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        let file_path = export_dir.join("Template%3AHealing%20modifiers.txt");
        std::fs::write(&file_path, "* {{a|Yes}}\n* {{Lethality|8}}\n").unwrap();

        let ctx = ConversionContext::new(tmp.path(), 2).unwrap();
        let ctx_arc = Arc::new(ctx);
        let registry = TemplateRegistry::new();
        let inv = parse_invocation("Healing modifiers");
        let result = registry
            .expand(
                &inv,
                &ExpanderCtx::new(2, &HashMap::new(), Some(ctx_arc.clone())),
            )
            .unwrap();
        assert!(result.expanded.contains("* Yes"));
        assert!(result.expanded.contains("* 8 lethality"));
    }

    #[test]
    fn template_names_match_with_underscores_or_spaces() {
        // MediaWiki treats underscores and spaces as equivalent in template
        // names, so `{{Zombie_state_info}}` must resolve to the same expander as
        // `{{Zombie state info}}`. Regression test for E_UNKNOWN_TEMPLATE on
        // Sion's "Glory in Death" passive, which invokes `{{Zombie_state_info}}`.
        let reg = TemplateRegistry::new();
        assert!(reg.has_name("Zombie_state_info"));
        assert!(reg.has_name("Zombie state info"));
        assert!(reg.has_name("zombie__state  info"));
        assert!(reg.has_name("Spellblade_info"));
    }

    #[test]
    fn include_info_template_resolves_via_underscore_invocation() {
        // End-to-end: the underscore-form invocation `{{Zombie_state_info}}`
        // used on Sion's "Glory in Death" passive must dispatch to the
        // IncludeInfoExpander and expand the template body, not fail with
        // E_UNKNOWN_TEMPLATE.
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        std::fs::write(
            export_dir.join("Template%3AZombie%20state%20info.txt"),
            "* Zombie states trigger upon taking {{tip|death|lethal damage}}.\n<noinclude>[[Category:Data templates]]</noinclude>",
        )
        .unwrap();

        let ctx_arc = Arc::new(ConversionContext::new(tmp.path(), 2).unwrap());
        let registry = TemplateRegistry::new();
        let result = registry
            .expand(
                &parse_invocation("Zombie_state_info"),
                &ExpanderCtx::new(2, &HashMap::new(), Some(ctx_arc)),
            )
            .unwrap();
        assert!(result
            .expanded
            .contains("Zombie states trigger upon taking lethal damage."));
        assert!(!result.expanded.contains("Category:Data templates"));
    }

    #[test]
    fn fd_template_preserves_percent_suffix() {
        let reg = TemplateRegistry::new();
        let result = reg.expand(&parse_invocation("fd|1.3%"), &ctx()).unwrap();
        assert_eq!(result.expanded, "1.30%");
    }

    #[test]
    fn rutngt_rounds_duration_up_to_next_game_tick() {
        let reg = TemplateRegistry::new();
        // 0.25 / 0.033 = 7.57..., rounds up to 8 ticks * 0.033 = 0.264 seconds.
        // Regression for Sion's "Glory in Death" passive, which used to drop the
        // interval entirely ("...health every , increasing...").
        assert_eq!(
            reg.expand(&parse_invocation("rutngt|0.25"), &ctx())
                .unwrap()
                .expanded,
            "0.264 seconds"
        );
        // Canonical (non-redirect) template name behaves identically.
        assert_eq!(
            reg.expand(&parse_invocation("Rounded up to next game tick|1.5"), &ctx())
                .unwrap()
                .expanded,
            "1.518 seconds"
        );
        // A value already on a tick boundary keeps its exact duration.
        assert_eq!(
            reg.expand(&parse_invocation("rutngt|0.066"), &ctx())
                .unwrap()
                .expanded,
            "0.066 seconds"
        );
        // The duration is often a nested expression rather than a bare number
        // (Lissandra, Nami, Tahm Kench, ...); it must be resolved before
        // evaluation. Values verified against the live wiki.
        assert_eq!(
            reg.expand(&parse_invocation("rutngt|{{#expr:700/2200}}"), &ctx())
                .unwrap()
                .expanded,
            "0.33 seconds"
        );
        assert_eq!(
            reg.expand(
                &parse_invocation("rutngt|{{#expr:2750 / 850 round 4}}"),
                &ctx()
            )
            .unwrap()
            .expanded,
            "3.267 seconds"
        );
    }

    #[test]
    fn expander_handles_fd_sti_mastery_and_column_variants() {
        let reg = TemplateRegistry::new();

        let textual_fd = reg
            .expand(&parse_invocation("fd|0.5-second"), &ctx())
            .unwrap();
        assert_eq!(textual_fd.expanded, "0.5-second");

        let textual_percent_fd = reg.expand(&parse_invocation("fd|1.5% AP"), &ctx()).unwrap();
        assert_eq!(textual_percent_fd.expanded, "1.5% AP");

        let ranged_fd = reg
            .expand(&parse_invocation("fd|1.75 – 2"), &ctx())
            .unwrap();
        assert_eq!(ranged_fd.expanded, "1.75 – 2");

        let labeled_text_fd = reg
            .expand(&parse_invocation("fd|None|Recast"), &ctx())
            .unwrap();
        assert_eq!(labeled_text_fd.expanded, "None (Recast)");

        let labeled_numeric_fd = reg
            .expand(&parse_invocation("fd|802.75|Forwards edge range"), &ctx())
            .unwrap();
        assert_eq!(labeled_numeric_fd.expanded, "802.75 (Forwards edge range)");

        let recurring_fd = reg
            .expand(&parse_invocation("fd|11.1{{Recurring|1}}"), &ctx())
            .unwrap();
        assert_eq!(recurring_fd.expanded, format!("11.11{}", '\u{0305}'));

        let sti = reg
            .expand(&parse_invocation("sti|cooldown|link=true"), &ctx())
            .unwrap();
        assert_eq!(sti.expanded, "cooldown");

        let mastery = reg
            .expand(&parse_invocation("mi3|Explorer"), &ctx())
            .unwrap();
        assert_eq!(mastery.expanded, "Explorer");

        let pipe = reg.expand(&parse_invocation("!"), &ctx()).unwrap();
        assert_eq!(pipe.expanded, "|");

        let affirmative = reg.expand(&parse_invocation("a|Yes"), &ctx()).unwrap();
        assert_eq!(affirmative.expanded, "Yes");

        let lethality = reg
            .expand(&parse_invocation("Lethality|8"), &ctx())
            .unwrap();
        assert_eq!(lethality.expanded, "8 lethality");

        let wr_item = reg
            .expand(&parse_invocation("WRi|Stinger"), &ctx())
            .unwrap();
        assert_eq!(wr_item.expanded, "Stinger");

        let malformed_wrapper = expand_inline_templates_mut(
            "{{as{{sti|ability haste}}}}",
            2,
            &mut HashMap::new(),
            &reg,
            None,
        )
        .unwrap();
        assert_eq!(malformed_wrapper, "ability haste");

        let column = reg
            .expand(
                &parse_invocation("Column|2|** Pengu\n** Pool Party"),
                &ctx(),
            )
            .unwrap();
        assert_eq!(column.expanded, "\n** Pengu\n** Pool Party\n");
    }

    #[test]
    fn expander_resolves_nested_constants_and_gold_efficiency_calculations() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        std::fs::write(
            export_dir.join("Module%3AItemData%2Fdata.txt"),
            r#"return {
    ["Dark Seal"] = {
        ["buy"] = 350,
        ["sellratio"] = 0.4,
        ["stats"] = {
            ["hp"] = 50,
            ["ap"] = 15,
        },
    },
    ["Winter's Approach"] = {
        ["stats"] = {
            ["hp"] = 550,
            ["ah"] = 15,
        },
    },
    ["Fimbulwinter"] = {
        ["buy"] = 2400,
        ["stats"] = {
            ["hp"] = "=>Winter's Approach",
            ["mana"] = 860,
        },
    },
}"#,
        )
        .unwrap();

        let conversion_ctx = Arc::new(ConversionContext::new(&export_dir, 2).unwrap());
        let mut constants = HashMap::new();
        constants.insert("crit_base".to_string(), "175".to_string());
        conversion_ctx.insert_champion_constants("Yasuo", constants);
        let ctx = ExpanderCtx::new(2, &HashMap::new(), Some(conversion_ctx));
        let reg = TemplateRegistry::new();

        let crit = parse_invocation("critical damage|{{ccd|Yasuo|crit_base}}|100");
        assert_eq!(reg.expand(&crit, &ctx).unwrap().expanded, "175%");

        let expr = parse_invocation("#expr:{{cid|Dark Seal|buy}}*2");
        assert_eq!(reg.expand(&expr, &ctx).unwrap().expanded, "700");

        let aliased = parse_invocation("cid|Fimbulwinter|health");
        assert_eq!(reg.expand(&aliased, &ctx).unwrap().expanded, "550");

        let gec = parse_invocation("gec|Dark Seal|+80");
        assert_eq!(reg.expand(&gec, &ctx).unwrap().expanded, "22.86% (+80g)");
    }

    #[test]
    fn invoke_recipe_ris_and_sse_templates_expand_reader_values() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        std::fs::write(
            export_dir.join("Module%3AGold%20value%2Fdata.txt"),
            r#"return {
    ["hp"] = {
        ["val"] = 2.666667,
    },
}"#,
        )
        .unwrap();

        let conversion_ctx = Arc::new(ConversionContext::new(&export_dir, 2).unwrap());
        let reg = TemplateRegistry::new();
        let ctx = ExpanderCtx::new(2, &HashMap::new(), Some(conversion_ctx.clone()));

        let mut vars = HashMap::new();
        let invoked = expand_inline_templates_mut(
            "{{#invoke:Gold value|wikivaluedefine}}{{g|{{#expr:{{#var:hp}}*300}}}}",
            2,
            &mut vars,
            &reg,
            Some(conversion_ctx),
        )
        .unwrap();
        assert_eq!(invoked, "800");

        let recipe = parse_invocation("Recipe/item|Oracle Lens");
        assert_eq!(reg.expand(&recipe, &ctx).unwrap().expanded, "* Oracle Lens");

        let ris = parse_invocation("ris|Font of Life");
        assert_eq!(reg.expand(&ris, &ctx).unwrap().expanded, "Font of Life");

        let sse = parse_invocation("sse|20");
        assert_eq!(reg.expand(&sse, &ctx).unwrap().expanded, "20");
    }

    #[test]
    fn rd_template_formats_melee_and_ranged_values() {
        let reg = TemplateRegistry::new();
        let result = reg.expand(&parse_invocation("rd|5|3"), &ctx()).unwrap();
        assert_eq!(result.expanded, "5 (melee) / 3 (ranged)");
    }

    #[test]
    fn buff_header_and_jungle_pet_info_are_neutralized() {
        let reg = TemplateRegistry::new();
        assert_eq!(
            reg.expand(&parse_invocation("Buff header|Ascended"), &ctx())
                .unwrap()
                .expanded,
            ""
        );
        assert_eq!(
            reg.expand(&parse_invocation("Jungle pet info"), &ctx())
                .unwrap()
                .expanded,
            ""
        );
    }

    #[test]
    fn inline_marker_templates_render_visible_text_not_empty() {
        // These carry reader-facing text on the wiki, so the registry expansion
        // path must render them instead of dropping them to empty (they used to
        // be neutralized, leaving holes in ability text).
        let reg = TemplateRegistry::new();
        let expand = |raw: &str| reg.expand(&parse_invocation(raw), &ctx()).unwrap().expanded;
        assert_eq!(expand("bug"), "[Bug]");
        assert_eq!(expand("pending for test"), "[Pending test]");
        assert_eq!(
            expand("Effect at cast time start"),
            "(effect determined at cast time start)"
        );
        assert_eq!(
            expand("Effect at cast time end"),
            "(effect determined at cast time end)"
        );
    }

    #[test]
    fn rutngt_resolves_champion_constant_lookups_in_argument() {
        // Senna-style argument: the duration is an #expr over {{ccd|...}}
        // champion-constant lookups, which only resolve with a conversion
        // context. 0.7 / 0.033 = 21.2..., rounds up to 22 ticks * 0.033 = 0.726.
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        let conversion_ctx = Arc::new(ConversionContext::new(&export_dir, 2).unwrap());
        let mut constants = HashMap::new();
        constants.insert("windup".to_string(), "0.7".to_string());
        conversion_ctx.insert_champion_constants("Tester", constants);
        let ctx = ExpanderCtx::new(2, &HashMap::new(), Some(conversion_ctx));
        let reg = TemplateRegistry::new();
        assert_eq!(
            reg.expand(
                &parse_invocation("rutngt|{{#expr:{{ccd|Tester|windup}}}}"),
                &ctx
            )
            .unwrap()
            .expanded,
            "0.726 seconds"
        );
    }

    #[test]
    fn champion_without_ap_ratio_renders_lead_sentence() {
        let reg = TemplateRegistry::new();
        assert_eq!(
            reg.expand(
                &parse_invocation("Champion without ability power ratio|Garen"),
                &ctx()
            )
            .unwrap()
            .expanded,
            "'''Garen''' is one of the champions that do not have a single ability power ratio on any of their abilities."
        );
    }

    #[test]
    fn highest_lowest_stats_resolves_champion_module_values() {
        let tmp = tempdir().unwrap();
        let export_dir = tmp.path().join("export_out");
        std::fs::create_dir_all(&export_dir).unwrap();
        std::fs::write(
            export_dir.join("Module%3AChampionData%2Fdata.txt"),
            r#"return {
    ["Mini Gnar"] = {
        ["name"] = "Mini Gnar",
        ["rangetype"] = "Ranged",
        ["stats"] = {
            ["hp_base"] = 400,
            ["hp_lvl"] = 80,
            ["dam_base"] = 50,
            ["dam_lvl"] = 3,
        },
    },
    ["Mega Gnar"] = {
        ["name"] = "Mega Gnar",
        ["rangetype"] = "Melee",
        ["stats"] = {
            ["hp_base"] = 500,
            ["hp_lvl"] = 100,
            ["dam_base"] = 60,
            ["dam_lvl"] = 5,
        },
    },
    ["Anivia"] = {
        ["name"] = "Anivia",
        ["rangetype"] = "Ranged",
        ["stats"] = {
            ["hp_base"] = 390,
            ["hp_lvl"] = 70,
            ["dam_base"] = 55,
            ["dam_lvl"] = 4,
        },
    },
    ["Corki"] = {
        ["name"] = "Corki",
        ["rangetype"] = "Ranged",
        ["stats"] = {
            ["hp_base"] = 410,
            ["hp_lvl"] = 75,
            ["dam_base"] = 52,
            ["dam_lvl"] = 2,
        },
    },
}"#,
        )
        .unwrap();

        let ctx = Arc::new(ConversionContext::new(tmp.path(), 2).unwrap());
        let registry = TemplateRegistry::new();
        let expander_ctx = ExpanderCtx::new(2, &HashMap::new(), Some(ctx));

        let third_lowest_hp = parse_invocation(
            "Highest lowest stats|hp|sortby=bot|size=1|lvl=18|show=3|rangetype=ranged",
        );
        assert_eq!(
            registry
                .expand(&third_lowest_hp, &expander_ctx)
                .unwrap()
                .expanded,
            "1760"
        );

        let highest_ad = parse_invocation("Highest lowest stats|ad|sortby=top|size=1|lvl=18");
        assert_eq!(
            registry
                .expand(&highest_ad, &expander_ctx)
                .unwrap()
                .expanded,
            "145"
        );

        let highest_ad_champ =
            parse_invocation("Highest lowest stats|ad|sortby=top|size=1|lvl=18|get=champ");
        assert_eq!(
            registry
                .expand(&highest_ad_champ, &expander_ctx)
                .unwrap()
                .expanded,
            "Mega Gnar"
        );
    }
}
