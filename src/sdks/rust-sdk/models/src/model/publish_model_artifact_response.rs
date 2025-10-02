use serde::{Serialize, Deserialize};
use super::ArtifactPublication;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishModelArtifactResponse {
    pub message: String,
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub metadata: serde_json::Value,
    pub result: ArtifactPublication,
    pub status: i64,
    pub version: String,
}
impl std::fmt::Display for PublishModelArtifactResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
