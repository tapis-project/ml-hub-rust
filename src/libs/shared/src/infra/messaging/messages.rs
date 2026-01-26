use serde::{Serialize, Deserialize};
use platforms::Platform;
use serde_json::Value;

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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DeployModelWithStrategyMessage {
    pub owner: String,
    pub platform: Platform,
    pub model_name: String,
    pub model_author: String,
    pub strategy_name: String,
    pub params: Value,
}