use crate::error::{ConvertError, Result};
use crate::parse::lua::{lua_value_to_string, parse_champion_module, parse_item_module, LuaValue};
use crate::parse::templates::{ConversionContextTrait, TemplateRegistry};
use crate::wiki_export::WikiExport;
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Serialize)]
struct TemplateInventoryReport {
    total: usize,
    templates: Vec<String>,
}

#[derive(Serialize)]
struct SpecimenMatrixReport {
    total_entities: usize,
    entities: BTreeMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct TemplateCoverageReport {
    total_registered: usize,
    executed: usize,
    missing: Vec<String>,
    extra: Vec<String>,
    coverage_percent: f64,
}

use super::ConversionOutcome;

/// Shared state and caches used across conversions.
#[derive(Clone)]
pub struct ConversionContext {
    inner: Arc<ConversionContextInner>,
}

pub struct ConversionContextInner {
    wiki_root: PathBuf,
    precision: u8,
    export: WikiExport,
    registry: TemplateRegistry,
    champion_module_raw: OnceCell<Option<String>>,
    champion_module_map: OnceCell<HashMap<String, HashMap<String, LuaValue>>>,
    champion_getter_defaults: OnceCell<HashMap<String, String>>,
    champion_getter_fields: OnceCell<HashSet<String>>,
    item_module_raw: OnceCell<Option<String>>,
    item_module_map: OnceCell<HashMap<String, HashMap<String, LuaValue>>>,
    gold_value_data_map: OnceCell<HashMap<String, HashMap<String, LuaValue>>>,
    template_inventory: Mutex<HashSet<String>>,
    specimen_matrix: Mutex<HashMap<String, Vec<String>>>,
    champion_constants: Mutex<HashMap<String, HashMap<String, String>>>,
    template_parameters: Mutex<HashMap<String, (String, BTreeSet<String>)>>,
    template_include_cache: Mutex<HashMap<String, Option<String>>>,
    include_removed: AtomicBool,
}

impl ConversionContext {
    pub fn new(root: &Path, precision: u8) -> Result<Self> {
        let inner = ConversionContextInner {
            wiki_root: root.to_path_buf(),
            precision,
            export: WikiExport::new(root),
            registry: TemplateRegistry::new(),
            champion_module_raw: OnceCell::new(),
            champion_module_map: OnceCell::new(),
            champion_getter_defaults: OnceCell::new(),
            champion_getter_fields: OnceCell::new(),
            item_module_raw: OnceCell::new(),
            item_module_map: OnceCell::new(),
            gold_value_data_map: OnceCell::new(),
            template_inventory: Mutex::new(HashSet::new()),
            specimen_matrix: Mutex::new(HashMap::new()),
            champion_constants: Mutex::new(HashMap::new()),
            template_parameters: Mutex::new(HashMap::new()),
            template_include_cache: Mutex::new(HashMap::new()),
            include_removed: AtomicBool::new(false),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn precision(&self) -> u8 {
        self.inner.precision
    }

    /// Whether removed items should be converted and written to the output.
    /// Defaults to `false`; removed items are excluded unless explicitly opted in.
    pub fn include_removed(&self) -> bool {
        self.inner.include_removed.load(Ordering::Relaxed)
    }

    /// Toggle whether removed items are included in the output. Configure this
    /// once before running conversions; it is read (without mutation) by the
    /// parallel batch workers.
    pub fn set_include_removed(&self, value: bool) {
        self.inner.include_removed.store(value, Ordering::Relaxed);
    }

    pub fn export(&self) -> &WikiExport {
        &self.inner.export
    }

    pub fn registry(&self) -> &TemplateRegistry {
        &self.inner.registry
    }

    pub fn wiki_root(&self) -> &Path {
        &self.inner.wiki_root
    }

    pub fn record_templates<I>(&self, names: I)
    where
        I: IntoIterator<Item = String>,
    {
        if let Ok(mut guard) = self.inner.template_inventory.lock() {
            for name in names {
                if !name.trim().is_empty() {
                    guard.insert(name);
                }
            }
        }
    }

    pub fn record_template_params(&self, name: &str, params: &[String]) {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return;
        }
        let display_name = trimmed_name.to_string();
        if let Ok(mut guard) = self.inner.template_parameters.lock() {
            let key = trimmed_name.to_ascii_lowercase();
            let entry = guard
                .entry(key)
                .or_insert_with(|| (display_name.clone(), BTreeSet::new()));
            for (idx, param) in params.iter().enumerate() {
                let raw = param.trim();
                if raw.is_empty() {
                    continue;
                }
                let normalized = if let Some(eq) = raw.find('=') {
                    raw[..eq].trim().to_ascii_lowercase()
                } else {
                    format!("#{idx}", idx = idx + 1)
                };
                if normalized.is_empty() {
                    continue;
                }
                entry.1.insert(normalized);
            }
        }
        self.record_templates(std::iter::once(display_name));
    }

    pub fn template_inventory_snapshot(&self) -> Vec<String> {
        match self.inner.template_inventory.lock() {
            Ok(guard) => {
                let mut names: Vec<String> = guard.iter().cloned().collect();
                names.sort_unstable();
                names
            }
            Err(_) => Vec::new(),
        }
    }

    pub fn template_parameters_snapshot(&self) -> Vec<(String, Vec<String>)> {
        match self.inner.template_parameters.lock() {
            Ok(guard) => {
                let mut rows: Vec<(String, Vec<String>)> = guard
                    .values()
                    .map(|(display, params)| (display.clone(), params.iter().cloned().collect()))
                    .collect();
                rows.sort_by(|a, b| a.0.to_ascii_lowercase().cmp(&b.0.to_ascii_lowercase()));
                rows
            }
            Err(_) => Vec::new(),
        }
    }

    pub fn champion_module_raw(&self) -> Result<Option<&str>> {
        let raw = self
            .inner
            .champion_module_raw
            .get_or_try_init(|| self.inner.export.read_champion_module_data())?;
        Ok(raw.as_deref())
    }

    pub fn item_module_raw(&self) -> Result<Option<&str>> {
        let raw = self
            .inner
            .item_module_raw
            .get_or_try_init(|| self.inner.export.read_item_module_data())?;
        Ok(raw.as_deref())
    }

    pub fn item_module_map(&self) -> Result<&HashMap<String, HashMap<String, LuaValue>>> {
        self.inner.item_module_map.get_or_try_init(|| {
            let mut merged = HashMap::new();
            for (_title, raw) in self.inner.export.read_item_module_data_sources()? {
                for (name, data) in parse_item_module(&raw)? {
                    merged.entry(name).or_insert(data);
                }
            }
            Ok(merged)
        })
    }

    pub fn gold_value_data_map(&self) -> Result<&HashMap<String, HashMap<String, LuaValue>>> {
        self.inner.gold_value_data_map.get_or_try_init(|| {
            let Some(raw) = self
                .inner
                .export
                .read_optional_page("Module:Gold value/data")?
            else {
                return Ok(HashMap::new());
            };
            parse_item_module(&raw)
        })
    }

    pub fn set_specimen_sample(&self, entity: &str, artifact: &str) {
        if let Ok(mut guard) = self.inner.specimen_matrix.lock() {
            let trimmed = artifact.trim();
            if trimmed.is_empty() {
                return;
            }
            let entry = guard.entry(entity.to_string()).or_default();
            if entry.len() >= 8 {
                return;
            }
            if entry
                .iter()
                .any(|existing| existing.eq_ignore_ascii_case(trimmed))
            {
                return;
            }
            entry.push(trimmed.to_string());
        }
    }

    pub fn specimen_matrix_snapshot(&self) -> Vec<(String, Vec<String>)> {
        match self.inner.specimen_matrix.lock() {
            Ok(guard) => {
                let mut entries: Vec<(String, Vec<String>)> =
                    guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                entries.sort_by(|a, b| a.0.cmp(&b.0));
                entries
            }
            Err(_) => Vec::new(),
        }
    }

    /// Champion module data parsed once (name -> data table).
    pub fn champion_module_map(&self) -> Result<&HashMap<String, HashMap<String, LuaValue>>> {
        self.inner.champion_module_map.get_or_try_init(|| {
            match self.champion_module_raw()? {
                Some(raw) => parse_champion_module(raw),
                None => Ok(HashMap::new()),
            }
        })
    }

    /// Resolve a champion's flattened constants, loading them from the champion
    /// module on demand (and caching) when they were not seeded by a prior
    /// champion conversion. Lets `{{ccd}}` work from any conversion context.
    pub fn champion_constants_or_load(&self, entity: &str) -> Option<HashMap<String, String>> {
        if let Some(constants) = self.champion_constants(entity) {
            return Some(constants);
        }
        let map = self.champion_module_map().ok()?;
        let entry = map.get(entity).or_else(|| {
            map.iter()
                .find(|(name, _)| name.eq_ignore_ascii_case(entity))
                .map(|(_, value)| value)
        })?;
        let mut constants: HashMap<String, String> = HashMap::new();
        for (key, value) in entry {
            if let Some(raw) = lua_value_to_string(value) {
                constants.insert(key.clone(), raw);
            }
        }
        if let Some(LuaValue::Table(stats)) = entry.get("stats") {
            for (key, value) in stats {
                if let Some(raw) = lua_value_to_string(value) {
                    constants.entry(key.clone()).or_insert(raw);
                }
            }
        }
        self.insert_champion_constants(entity, constants.clone());
        Some(constants)
    }

    /// Resolve the fallback value the wiki applies for a champion stat that is
    /// absent from `Module:ChampionData/data`. These defaults live in
    /// `Module:ChampionData/getter` as one-line accessors of the form
    /// `getData(champname, ...).field or <default>` (e.g. `crit_base or 200`),
    /// so we parse them straight out of the dumped module rather than hardcoding
    /// any numbers — the wiki module is the single source of truth, and if it
    /// changes the next export picks it up automatically.
    pub fn champion_constant_default(&self, field: &str) -> Option<String> {
        let defaults = self.inner.champion_getter_defaults.get_or_init(|| {
            match self
                .inner
                .export
                .read_optional_page("Module:ChampionData/getter")
            {
                Ok(Some(raw)) => parse_getter_defaults(&raw),
                _ => HashMap::new(),
            }
        });
        defaults.get(&field.trim().to_ascii_lowercase()).cloned()
    }

    /// Whether `Module:ChampionData/getter` exposes an accessor for `field`
    /// (a `function p.<field>(champname)` declaration). Such a field is a real,
    /// wiki-known stat: when a champion does not set it and the accessor has no
    /// static `or <default>` fallback, the getter returns nil, which the wiki
    /// renders as an empty string. Callers use this to mirror that empty result
    /// instead of erroring, while still failing fast on genuinely unknown fields
    /// the getter never defines.
    pub fn champion_getter_defines_field(&self, field: &str) -> bool {
        let fields = self.inner.champion_getter_fields.get_or_init(|| {
            match self
                .inner
                .export
                .read_optional_page("Module:ChampionData/getter")
            {
                Ok(Some(raw)) => parse_getter_field_names(&raw),
                _ => HashSet::new(),
            }
        });
        fields.contains(&field.trim().to_ascii_lowercase())
    }

    pub fn champion_constants(&self, key: &str) -> Option<HashMap<String, String>> {
        match self.inner.champion_constants.lock() {
            Ok(guard) => guard.get(key).cloned().or_else(|| {
                guard
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(key))
                    .map(|(_, values)| values.clone())
            }),
            Err(_) => None,
        }
    }

    pub fn insert_champion_constants(&self, key: &str, values: HashMap<String, String>) {
        if let Ok(mut guard) = self.inner.champion_constants.lock() {
            guard.insert(key.to_string(), values);
        }
    }

    pub fn write_inventory_reports(&self, output_dir: &Path) -> Result<()> {
        let mut templates = self.template_inventory_snapshot();
        let specimens = self.specimen_matrix_snapshot();
        if templates.is_empty() && specimens.is_empty() {
            return Ok(());
        }
        let inventory_dir = output_dir.join("_inventory");
        std::fs::create_dir_all(&inventory_dir)?;
        let mut seen_lower: HashSet<String> = HashSet::new();
        let mut canonical_map: BTreeMap<String, String> = BTreeMap::new();
        if !templates.is_empty() {
            for name in templates.drain(..) {
                let key = name.to_ascii_lowercase();
                seen_lower.insert(key.clone());
                canonical_map.entry(key).or_insert(name);
            }
            let templates: Vec<String> = canonical_map.values().cloned().collect();
            let total = templates.len();
            let report = TemplateInventoryReport { total, templates };
            let json = serde_json::to_string_pretty(&report).map_err(|e| {
                ConvertError::Internal(format!("failed to serialize template inventory: {e}"))
            })?;
            std::fs::write(inventory_dir.join("template_inventory.json"), json)?;
        }
        let registered = self.inner.registry.list_names();
        let registry_lower: HashSet<String> =
            registered.iter().map(|n| n.to_ascii_lowercase()).collect();
        let extra: Vec<String> = canonical_map
            .iter()
            .filter_map(|(key, name)| {
                if registry_lower.contains(key) {
                    None
                } else {
                    Some(name.clone())
                }
            })
            .collect();
        let mut executed_names: Vec<String> = Vec::new();
        let mut missing: Vec<String> = Vec::new();
        for name in &registered {
            let key = name.to_ascii_lowercase();
            if seen_lower.contains(&key) {
                executed_names.push(name.to_string());
            } else {
                missing.push(name.to_string());
            }
        }
        executed_names.sort();
        missing.sort();
        let executed = executed_names.len();
        let total_registered = registered.len();
        let coverage_percent = if total_registered == 0 {
            100.0
        } else {
            (executed as f64 / total_registered as f64) * 100.0
        };
        let coverage_report = TemplateCoverageReport {
            total_registered,
            executed,
            missing,
            extra,
            coverage_percent,
        };
        let coverage_json = serde_json::to_string_pretty(&coverage_report).map_err(|e| {
            ConvertError::Internal(format!("failed to serialize template coverage report: {e}"))
        })?;
        std::fs::write(inventory_dir.join("template_coverage.json"), coverage_json)?;
        let parameter_snapshot = self.template_parameters_snapshot();
        if !parameter_snapshot.is_empty() {
            let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for (name, params) in parameter_snapshot {
                map.insert(name, params);
            }
            let parameters_json = serde_json::to_string_pretty(&map).map_err(|e| {
                ConvertError::Internal(format!(
                    "failed to serialize template parameter report: {e}"
                ))
            })?;
            std::fs::write(
                inventory_dir.join("template_parameters.json"),
                parameters_json,
            )?;
        }
        if !specimens.is_empty() {
            let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for (entity, mut entries) in specimens {
                entries.retain(|s| !s.trim().is_empty());
                if entries.is_empty() {
                    continue;
                }
                entries.sort();
                entries.dedup();
                map.insert(entity, entries);
            }
            if !map.is_empty() {
                let report = SpecimenMatrixReport {
                    total_entities: map.len(),
                    entities: map,
                };
                let json = serde_json::to_string_pretty(&report).map_err(|e| {
                    ConvertError::Internal(format!(
                        "failed to serialize specimen matrix report: {e}"
                    ))
                })?;
                let specimen_path = inventory_dir.join("specimen_matrix.json");
                std::fs::write(&specimen_path, json)?;
            }
        }
        Ok(())
    }

    pub fn convert_champion(&self, output_dir: &Path, name: &str) -> Result<ConversionOutcome> {
        super::champion::convert_champion(self, output_dir, name)
    }

    pub fn convert_item(&self, output_dir: &Path, name: &str) -> Result<ConversionOutcome> {
        super::item::convert_item(self, output_dir, name)
    }

    pub fn convert_rune(&self, output_dir: &Path, name: &str) -> Result<ConversionOutcome> {
        super::rune::convert_rune(self, output_dir, name)
    }

    pub fn template_includeonly(&self, name: &str) -> Result<Option<String>> {
        self.inner.template_includeonly(name)
    }
}

impl ConversionContextTrait for ConversionContextInner {
    fn champion_constants(&self, key: &str) -> Option<HashMap<String, String>> {
        match self.champion_constants.lock() {
            Ok(guard) => guard.get(key).cloned().or_else(|| {
                guard
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(key))
                    .map(|(_, values)| values.clone())
            }),
            Err(_) => None,
        }
    }

    fn item_module_map(&self) -> Result<&HashMap<String, HashMap<String, LuaValue>>> {
        self.item_module_map.get_or_try_init(|| {
            let mut merged = HashMap::new();
            for (_title, raw) in self.export.read_item_module_data_sources()? {
                for (name, data) in parse_item_module(&raw)? {
                    merged.entry(name).or_insert(data);
                }
            }
            Ok(merged)
        })
    }
}

impl ConversionContextInner {
    pub fn precision(&self) -> u8 {
        self.precision
    }

    fn template_includeonly(&self, name: &str) -> Result<Option<String>> {
        let canonical = name.trim().to_ascii_lowercase();
        if let Ok(cache) = self.template_include_cache.lock() {
            if let Some(cached) = cache.get(&canonical) {
                return Ok(cached.clone());
            }
        }

        let mut include: Option<String> = None;
        for title in template_title_candidates(name) {
            if let Some(raw) = self.export.read_template_page(&title)? {
                if let Some(body) = extract_template_body(&raw) {
                    include = Some(body);
                    break;
                }
            }
        }

        if let Ok(mut cache) = self.template_include_cache.lock() {
            cache.insert(canonical, include.clone());
        }

        Ok(include)
    }
}

/// Parse the per-field fallbacks out of `Module:ChampionData/getter`.
///
/// The module exposes one accessor per stat, each of the shape
/// `... = getData(champname, ...).field or <default>`, where `<default>` is a
/// bare number (`200`, `65`) or a quoted string (`"Physical"`). We capture the
/// field name and its literal default verbatim so callers get exactly what the
/// wiki would render. Anything that does not match (multi-line bodies, computed
/// defaults) is simply skipped — those fields have no static fallback to mirror.
fn parse_getter_defaults(raw: &str) -> HashMap<String, String> {
    use regex::Regex;
    static RE: OnceCell<Regex> = OnceCell::new();
    let re = RE.get_or_init(|| {
        Regex::new(r#"getData\([^)]*\)\.(\w+)\s+or\s+("[^"]*"|[-+0-9.]+)"#)
            .expect("valid getter-default regex")
    });
    let mut defaults = HashMap::new();
    for caps in re.captures_iter(raw) {
        let field = caps[1].trim().to_ascii_lowercase();
        let value = caps[2].trim().trim_matches('"').to_string();
        defaults.entry(field).or_insert(value);
    }
    defaults
}

/// Collect the set of field names the getter module exposes as accessors, i.e.
/// every `function p.<field>(...)` declaration. These are the stats the wiki
/// knows how to look up; a field outside this set is never resolvable through
/// `{{ccd}}` and should fail fast rather than silently render empty.
fn parse_getter_field_names(raw: &str) -> HashSet<String> {
    use regex::Regex;
    static RE: OnceCell<Regex> = OnceCell::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"(?m)^\s*function\s+p\.(\w+)\s*\(").expect("valid getter-field regex")
    });
    re.captures_iter(raw)
        .map(|caps| caps[1].trim().to_ascii_lowercase())
        .collect()
}

fn extract_template_body(raw: &str) -> Option<String> {
    if let Some(includeonly) = extract_includeonly_sections(raw) {
        return Some(includeonly);
    }
    let stripped = strip_noinclude_sections(raw);
    let trimmed = stripped.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn template_title_candidates(name: &str) -> Vec<String> {
    let trimmed = name.trim();
    let without_prefix = trimmed.strip_prefix("Template:").unwrap_or(trimmed).trim();
    let canonical_space = without_prefix.replace('_', " ");
    let mut variants: Vec<String> = Vec::new();
    if !canonical_space.is_empty() {
        variants.push(canonical_space.clone());
    }
    let underscore = canonical_space.replace(' ', "_");
    if !underscore.is_empty() && !variants.iter().any(|v| v.eq_ignore_ascii_case(&underscore)) {
        variants.push(underscore);
    }
    if !without_prefix.eq_ignore_ascii_case(&canonical_space)
        && !variants
            .iter()
            .any(|v| v.eq_ignore_ascii_case(without_prefix))
    {
        variants.push(without_prefix.to_string());
    }

    let mut titles: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for variant in variants {
        let title = format!("Template:{}", variant);
        let key = title.to_ascii_lowercase();
        if seen.insert(key) {
            titles.push(title);
        }
    }
    titles
}

fn extract_includeonly_sections(raw: &str) -> Option<String> {
    const OPEN: &str = "<includeonly>";
    const CLOSE: &str = "</includeonly>";
    let mut remaining = raw;
    let mut collected = String::new();
    while let Some(start) = remaining.find(OPEN) {
        let after_open = &remaining[start + OPEN.len()..];
        if let Some(end) = after_open.find(CLOSE) {
            let section = &after_open[..end];
            collected.push_str(section);
            if !section.ends_with('\n') {
                collected.push('\n');
            }
            remaining = &after_open[end + CLOSE.len()..];
        } else {
            collected.push_str(after_open);
            break;
        }
    }
    let trimmed = collected.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn strip_noinclude_sections(raw: &str) -> String {
    const OPEN: &str = "<noinclude>";
    const CLOSE: &str = "</noinclude>";
    let mut out = String::new();
    let mut remaining = raw;
    loop {
        let Some(start) = remaining.find(OPEN) else {
            out.push_str(remaining);
            break;
        };
        out.push_str(&remaining[..start]);
        let after_open = &remaining[start + OPEN.len()..];
        let Some(end) = after_open.find(CLOSE) else {
            break;
        };
        remaining = &after_open[end + CLOSE.len()..];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use tempfile::tempdir;

    #[test]
    fn parse_getter_defaults_extracts_number_and_string_fallbacks() {
        let raw = r#"
function p.acquisition_radius(champname)
	return getData(champname, true).acquisition_radius or 750
end
function p.gameplay_radius(champname)
	return getData(champname, true).gameplay_radius or 65
end
function p.crit_base(champname)
	return getData(champname, true).crit_base or 200
end
function p.adaptivetype(champname)
	return getData(champname).adaptivetype or "Physical"
end
function p.computed(champname)
	-- no static fallback to mirror
	return getData(champname).computed
end
"#;
        let defaults = parse_getter_defaults(raw);
        assert_eq!(defaults.get("crit_base"), Some(&"200".to_string()));
        assert_eq!(defaults.get("gameplay_radius"), Some(&"65".to_string()));
        assert_eq!(defaults.get("acquisition_radius"), Some(&"750".to_string()));
        assert_eq!(defaults.get("adaptivetype"), Some(&"Physical".to_string()));
        assert_eq!(defaults.get("computed"), None);
    }

    #[test]
    fn champion_constant_default_reads_from_getter_module() {
        let td = tempdir().unwrap();
        std::fs::write(
            td.path().join("Module%3AChampionData%2Fgetter.txt"),
            "function p.crit_base(champname)\n\treturn getData(champname, true).crit_base or 200\nend\n",
        )
        .unwrap();
        let ctx = ConversionContext::new(td.path(), 2).unwrap();
        assert_eq!(
            ctx.champion_constant_default("crit_base").as_deref(),
            Some("200")
        );
        // A field with no static fallback in the module yields nothing — the
        // caller then fails fast rather than inventing a value.
        assert_eq!(ctx.champion_constant_default("missile_speed"), None);
    }

    #[test]
    fn inventory_reports_written_with_deduplication() {
        let root_dir = tempdir().unwrap();
        let output_dir = root_dir.path().join("artifacts");
        std::fs::create_dir_all(&output_dir).unwrap();
        let ctx = ConversionContext::new(root_dir.path(), 2).unwrap();
        ctx.record_templates(vec![
            "TemplateA".to_string(),
            "TemplateB".to_string(),
            "templatea".to_string(),
        ]);
        ctx.record_template_params("TemplateA", &["foo = 1".to_string(), "value".to_string()]);
        ctx.record_template_params("TemplateA", &["bar=2".to_string()]);
        ctx.set_specimen_sample("champion", "Akshan.md");
        ctx.set_specimen_sample("champion", "Akshan.md"); // duplicate ignored
        ctx.set_specimen_sample("item", "Infinity_Edge.md");
        ctx.write_inventory_reports(&output_dir).unwrap();

        let inventory_path = output_dir
            .join("_inventory")
            .join("template_inventory.json");
        assert!(inventory_path.exists());
        let inv_contents = std::fs::read_to_string(&inventory_path).unwrap();
        let inv_json: Value = serde_json::from_str(&inv_contents).unwrap();
        assert_eq!(inv_json["total"].as_u64().unwrap(), 2);
        let templates = inv_json["templates"].as_array().unwrap();
        assert_eq!(templates.len(), 2);
        assert!(templates.iter().any(|v| v.as_str() == Some("TemplateA")));
        assert!(templates.iter().any(|v| v.as_str() == Some("TemplateB")));

        let coverage_path = output_dir.join("_inventory").join("template_coverage.json");
        assert!(coverage_path.exists());
        let coverage_contents = std::fs::read_to_string(&coverage_path).unwrap();
        let coverage_json: Value = serde_json::from_str(&coverage_contents).unwrap();
        assert_eq!(
            coverage_json["total_registered"].as_u64().unwrap() > 0,
            true
        );
        assert_eq!(coverage_json["executed"].as_u64().unwrap(), 0);
        let extra = coverage_json["extra"].as_array().unwrap();
        assert!(extra.iter().any(|v| v.as_str() == Some("TemplateA")));
        assert!(extra.iter().any(|v| v.as_str() == Some("TemplateB")));

        let params_path = output_dir
            .join("_inventory")
            .join("template_parameters.json");
        assert!(params_path.exists());
        let params_contents = std::fs::read_to_string(&params_path).unwrap();
        let params_json: Value = serde_json::from_str(&params_contents).unwrap();
        let template_a_params = params_json["TemplateA"].as_array().unwrap();
        let collected: Vec<String> = template_a_params
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        assert!(collected.contains(&"foo".to_string()));
        assert!(collected.contains(&"bar".to_string()));
        assert!(collected.contains(&"#2".to_string()));

        let specimen_path = output_dir.join("_inventory").join("specimen_matrix.json");
        assert!(specimen_path.exists());
        let spec_contents = std::fs::read_to_string(&specimen_path).unwrap();
        let spec_json: Value = serde_json::from_str(&spec_contents).unwrap();
        assert_eq!(spec_json["total_entities"].as_u64().unwrap(), 2);
        let champ_samples = spec_json["entities"]["champion"].as_array().unwrap();
        assert_eq!(champ_samples.len(), 1);
        assert_eq!(champ_samples[0].as_str().unwrap(), "Akshan.md");
        let item_samples = spec_json["entities"]["item"].as_array().unwrap();
        assert_eq!(item_samples.len(), 1);
        assert_eq!(item_samples[0].as_str().unwrap(), "Infinity_Edge.md");
    }
}
