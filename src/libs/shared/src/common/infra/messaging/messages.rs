use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ArtifactType {
    Model,
    Dataset
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IngestArtifactMessage {
    pub artifact_type: ArtifactType,
    pub platform: String,
    pub platform_artifact_id: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}