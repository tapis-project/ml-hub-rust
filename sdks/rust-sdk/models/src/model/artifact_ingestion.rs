use serde::{Serialize, Deserialize};
use super::ArtifactIngestionStatus;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactIngestion {
    pub artifact_id: String,
    pub created_at: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message: Option<String>,
    pub last_modified: String,
    pub platform: String,
    pub status: ArtifactIngestionStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}
impl std::fmt::Display for ArtifactIngestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
