use crate::application::ports::messaging::IngestArtifactMessagePayload;
use crate::infra::messaging::messages;

impl From<IngestArtifactMessagePayload> for messages::IngestArtifactMessage {
    fn from(value: IngestArtifactMessagePayload) -> Self {
        Self {
            ingestion_id: value.ingestion_id.to_string(),
            platform: value.platform,
            webhook_url: value.webhook_url,
            serialized_client_request: value.serialized_client_request,
        }
    }
}