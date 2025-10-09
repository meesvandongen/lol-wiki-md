use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Champion {
    pub name: String,
    pub basic: BasicInfo,
    pub stats: Stats,
    pub advanced: Option<AdvancedStats>,
    pub abilities: Vec<Ability>,
    pub pets: Vec<Pet>,
    pub trivia: Vec<String>,
    pub patch_history: Vec<PatchEntry>,
    pub notes: Option<String>,
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
pub struct StatLine { pub base: f32, pub growth: f32 }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AdvancedStats { pub metrics: HashMap<String, String> }

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
pub enum AbilityKey { Passive, Q, W, E, R, Other(String) }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SkillTable { pub headers: Vec<String>, pub rows: Vec<Vec<String>> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pet { pub name: String, pub description: String }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchEntry { pub version: String, pub changes: Vec<Change> }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Change { pub section: String, pub text: String }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item { pub name: String }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rune { pub name: String }
