use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ArtifactType {
    Model,
    Dataset
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IngestArtifactMessage {
    pub artifact_type: ArtifactType,
    pub ingestion_id: String,
    pub platform: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}