use crate::error::{ConvertError, Result};
use crate::parse::lua::{parse_item_module, LuaValue};
use crate::parse::templates::{ConversionContextTrait, TemplateRegistry};
use crate::wiki_export::WikiExport;
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
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
    item_module_raw: OnceCell<Option<String>>,
    item_module_map: OnceCell<HashMap<String, HashMap<String, LuaValue>>>,
    template_inventory: Mutex<HashSet<String>>,
    specimen_matrix: Mutex<HashMap<String, Vec<String>>>,
    champion_constants: Mutex<HashMap<String, HashMap<String, String>>>,
    template_parameters: Mutex<HashMap<String, (String, BTreeSet<String>)>>,
}

impl ConversionContext {
    pub fn new(root: &Path, precision: u8) -> Result<Self> {
        let inner = ConversionContextInner {
            wiki_root: root.to_path_buf(),
            precision,
            export: WikiExport::new(root),
            registry: TemplateRegistry::new(),
            champion_module_raw: OnceCell::new(),
            item_module_raw: OnceCell::new(),
            item_module_map: OnceCell::new(),
            template_inventory: Mutex::new(HashSet::new()),
            specimen_matrix: Mutex::new(HashMap::new()),
            champion_constants: Mutex::new(HashMap::new()),
            template_parameters: Mutex::new(HashMap::new()),
        };
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    pub fn precision(&self) -> u8 {
        self.inner.precision
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
            match self.inner.export.read_item_module_data()? {
                Some(raw) => parse_item_module(&raw),
                None => Ok(HashMap::new()),
            }
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

    pub fn champion_constants(&self, key: &str) -> Option<HashMap<String, String>> {
        match self.inner.champion_constants.lock() {
            Ok(guard) => guard.get(key).cloned(),
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
}

impl ConversionContextTrait for ConversionContextInner {
    fn champion_constants(&self, key: &str) -> Option<HashMap<String, String>> {
        match self.champion_constants.lock() {
            Ok(guard) => guard.get(key).cloned(),
            Err(_) => None,
        }
    }

    fn item_module_map(&self) -> Result<&HashMap<String, HashMap<String, LuaValue>>> {
        self.item_module_map
            .get_or_try_init(|| match self.export.read_item_module_data()? {
                Some(raw) => parse_item_module(&raw),
                None => Ok(HashMap::new()),
            })
    }
}

impl ConversionContextInner {
    pub fn precision(&self) -> u8 {
        self.precision
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use tempfile::tempdir;

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
