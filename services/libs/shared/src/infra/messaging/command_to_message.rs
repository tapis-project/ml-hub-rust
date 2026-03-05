use crate::application::ports::commands::{
    IngestArtifactCommandPayload,
    PublishArtifactCommandPayload,
};
use crate::infra::messaging::messages;

impl From<&IngestArtifactCommandPayload> for messages::IngestArtifactMessage {
    fn from(value: &IngestArtifactCommandPayload) -> Self {
        Self {
            ingestion_id: value.ingestion_id.to_string(),
            platform: value.platform.clone(),
            webhook_url: value.webhook_url.clone(),
            serialized_client_request: value.serialized_client_request.clone(),
        }
    }
}

impl From<&PublishArtifactCommandPayload> for messages::PublishArtifactMessage {
    fn from(value: &PublishArtifactCommandPayload) -> Self {
        Self {
            publication_id: value.publication_id.to_string(),
            webhook_url: value.webhook_url.clone(),
            serialized_client_request: value.serialized_client_request.clone(),
        }
    }
}