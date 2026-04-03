use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatsFile {
    #[serde(rename = "DataVersion", default)]
    pub data_version: Option<i64>,
    #[serde(default)]
    pub stats: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
