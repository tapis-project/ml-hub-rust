use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

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
pub struct ModelDeploymentStateDriftDetectedPayload {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub desired_state: String,
    pub actual_state: String,
    pub message: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelDeploymentStartedPayload {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelDeploymentStoppedPayload {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ModelDeploymentDeletedPayload {
    pub deployment_id: String,
    pub deployment_revision: u32,
    pub message: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EventEnvelope {
    pub payload: Value,
    pub metadata: EventMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventMetadata {
    pub id: Uuid,
    pub correlation_id: Uuid,
    pub causation_id: Uuid,
    pub timestamp: String,
    pub kind: String,
}

// This struct services as an envelope for messages. The consumer can desirialize
// this struct, determine which ...Message struct corresponds to the event_type field
// then desirialize the event using the correct struct.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct EventMessageEnvelope {
    pub kind: String,
    pub event_envelope: EventEnvelope,
}