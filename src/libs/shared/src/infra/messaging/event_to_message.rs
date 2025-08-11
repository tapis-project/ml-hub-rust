use crate::application::ports::events::IngestArtifactEventPayload;
use crate::infra::messaging::messages;

impl From<IngestArtifactEventPayload> for messages::IngestArtifactMessage {
    fn from(value: IngestArtifactEventPayload) -> Self {
        Self {
            ingestion_id: value.ingestion_id.to_string(),
            platform: value.platform,
            webhook_url: value.webhook_url,
            serialized_client_request: value.serialized_client_request,
        }
    }
}