use serde::{Serialize, Deserialize};
use super::ArtifactPublicationStatus;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactPublication {
    pub artifact_id: String,
    pub attempts: i64,
    pub created_at: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message: Option<String>,
    pub last_modified: String,
    pub status: ArtifactPublicationStatus,
    pub target_platform: String,
}
impl std::fmt::Display for ArtifactPublication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
