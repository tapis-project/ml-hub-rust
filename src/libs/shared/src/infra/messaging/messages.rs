use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IngestArtifactMessage {
    pub ingestion_id: String,
    pub platform: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PublishArtifactMessage {
    pub publication_id: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}