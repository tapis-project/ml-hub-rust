use crate::common::application::inputs;
use crate::common::application::ports::messaging::IngestArtifactMessagePayload;
use crate::common::infra::messaging::messages;

impl From<inputs::ArtifactType> for messages::ArtifactType {
    fn from(value: inputs::ArtifactType) -> Self {
        match value {
            inputs::ArtifactType::Model => messages::ArtifactType::Model,
            inputs::ArtifactType::Dataset => messages::ArtifactType::Dataset
        }
    }
}

impl From<IngestArtifactMessagePayload> for messages::IngestArtifactMessage {
    fn from(value: IngestArtifactMessagePayload) -> Self {
        Self {
            ingestion_id: value.ingestion_id.to_string(),
            artifact_type: messages::ArtifactType::from(value.artifact_type),
            platform: value.platform,
            webhook_url: value.webhook_url,
            serialized_client_request: value.serialized_client_request,
        }
    }
}