use serde::{Serialize, Deserialize};
use super::Artifact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListModelArtifactResponse {
    pub message: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub metadata: serde_json::Value,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<Artifact>,
    pub status: i64,
    pub version: String,
}
impl std::fmt::Display for ListModelArtifactResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
