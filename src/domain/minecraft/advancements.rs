use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AdvancementProgress {
    #[serde(default)]
    pub done: bool,
    #[serde(default)]
    pub criteria: BTreeMap<String, serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

pub type AdvancementsFile = BTreeMap<String, AdvancementProgress>;
