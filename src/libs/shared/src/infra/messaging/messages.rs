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

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelDeploymentStateDriftDetectedMessage {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub desired_state: String,
    pub acutal_state: String,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelDeploymentStartedMessage {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelDeploymentStoppedMessage {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelDeploymentDeletedMessage {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub timestamp: String,
}