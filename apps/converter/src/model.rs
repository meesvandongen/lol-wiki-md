use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Champion {
    pub name: String,
    pub basic: BasicInfo,
    pub summary: Option<String>,
    pub stats: Stats,
    pub advanced: Option<AdvancedStats>,
    pub primary_stat_label: Option<String>,
    pub special_stats: Vec<ChampionSpecialMode>,
    pub stat_variants: Vec<ChampionStatVariant>,
    pub abilities: Vec<Ability>,
    pub pets: Vec<Pet>,
    pub trivia: Vec<String>,
    pub patch_history: Vec<PatchEntry>,
    pub notes: Vec<String>,
    pub source_appendices: Vec<SourceAppendix>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SourceAppendix {
    pub title: String,
    pub format: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BasicInfo {
    pub title: Option<String>,
    pub roles: Vec<String>,
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Stats {
    pub base: HashMap<String, StatLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatLine {
    pub base: f32,
    pub growth: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AdvancedStats {
    pub metrics: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ChampionSpecialMode {
    pub mode: String,
    pub metrics: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ChampionStatVariant {
    pub label: String,
    pub stats: Stats,
    pub advanced: Option<AdvancedStats>,
    pub special_stats: Vec<ChampionSpecialMode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ability {
    pub key: AbilityKey,
    pub name: String,
    pub descriptions: Vec<String>,
    pub cooldowns: Vec<String>,
    pub costs: Vec<String>,
    pub ranges: Vec<String>,
    pub leveling_tables: Vec<SkillTable>,
    pub notes: Vec<String>,
    pub extra: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AbilityKey {
    Passive,
    BasicAttack,
    Q,
    W,
    E,
    R,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SkillTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pet {
    pub name: String,
    /// Flat fallback description (used for plain list-item pet entries).
    #[serde(default)]
    pub description: String,
    /// Structured stat lines from a `{{Infobox/Pet}}`, rendered as sub-bullets.
    #[serde(default)]
    pub stats: Vec<PetStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PetStat {
    pub label: String,
    #[serde(default)]
    pub value: String,
    /// Nested sub-bullets (e.g. per-ability or per-modifier list entries).
    #[serde(default)]
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchEntry {
    pub version: String,
    pub changes: Vec<Change>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Change {
    pub section: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Item {
    pub name: String,
    pub tier: Option<String>,
    pub categories: Vec<String>,
    pub stats: HashMap<String, String>,
    pub effects: Vec<ItemEffect>,
    pub recipe: Vec<String>,
    pub cost_total: Option<u32>,
    pub cost_combine: Option<u32>,
    pub cost_sell: Option<u32>,
    pub sell_ratio: Option<f32>,
    pub gold_value: Option<String>,
    pub gold_efficiency: Option<String>,
    pub caption: Option<String>,
    pub limit: Option<String>,
    pub modes: Vec<String>,
    pub ornn_forged: bool,
    pub removed_patch: Option<String>,
    pub upgrades: Vec<String>,
    pub similar_items: Vec<String>,
    pub background: Option<String>,
    pub strategy: Option<String>,
    pub notes: Vec<String>,
    pub trivia: Vec<String>,
    pub patch_history: Vec<PatchEntry>,
    pub source_appendices: Vec<SourceAppendix>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ItemEffect {
    pub kind: String,
    pub name: Option<String>,
    pub description: String,
    pub cooldown: Option<String>,
    pub range: Option<String>,
    pub unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Rune {
    pub name: String,
    pub path: Option<String>,
    pub slot: Option<String>,
    pub description: String,
    pub caption: Option<String>,
    pub map_changes: Option<String>,
    pub notes: Vec<String>,
    pub trivia: Vec<String>,
    pub patch_history: Vec<PatchEntry>,
    pub source_appendices: Vec<SourceAppendix>,
    pub warnings: Vec<String>,
}
