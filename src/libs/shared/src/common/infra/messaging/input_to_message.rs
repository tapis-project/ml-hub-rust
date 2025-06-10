use crate::common::application::inputs;
use crate::common::infra::messaging::messages;

impl From<inputs::ArtifactType> for messages::ArtifactType {
    fn from(value: inputs::ArtifactType) -> Self {
        match value {
            inputs::ArtifactType::Model => messages::ArtifactType::Model,
            inputs::ArtifactType::Dataset => messages::ArtifactType::Dataset
        }
    }
}

impl From<inputs::IngestArtifactInput> for messages::IngestArtifactMessage {
    fn from(value: inputs::IngestArtifactInput) -> Self {
        Self {
            artifact_type: messages::ArtifactType::from(value.artifact_type),
            platform: value.platform,
            platform_artifact_id: value.platform_artifact_id,
            webhook_url: value.webhook_url,
            serialized_client_request: value.serialized_client_request,
        }
    }
}