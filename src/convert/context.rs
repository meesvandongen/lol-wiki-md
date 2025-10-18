use crate::error::Result;
use crate::parse::lua::{parse_item_module, LuaValue};
use crate::parse::templates::TemplateRegistry;
use crate::wiki_export::WikiExport;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

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
            guard
                .entry(entity.to_string())
                .or_default()
                .push(artifact.to_string());
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

impl ConversionContextInner {
    pub fn precision(&self) -> u8 {
        self.precision
    }
}
