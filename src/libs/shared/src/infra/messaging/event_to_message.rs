use crate::application::ports::events::{
    IngestArtifactEventPayload,
    PublishArtifactEventPayload,
    DeployModelWithStrategyEventPayload,
};
use crate::infra::messaging::messages;

impl From<&IngestArtifactEventPayload> for messages::IngestArtifactMessage {
    fn from(value: &IngestArtifactEventPayload) -> Self {
        Self {
            ingestion_id: value.ingestion_id.to_string(),
            platform: value.platform.clone(),
            webhook_url: value.webhook_url.clone(),
            serialized_client_request: value.serialized_client_request.clone(),
        }
    }
}

impl From<&PublishArtifactEventPayload> for messages::PublishArtifactMessage {
    fn from(value: &PublishArtifactEventPayload) -> Self {
        Self {
            publication_id: value.publication_id.to_string(),
            webhook_url: value.webhook_url.clone(),
            serialized_client_request: value.serialized_client_request.clone(),
        }
    }
}

impl From<&DeployModelWithStrategyEventPayload> for messages::DeployModelWithStrategyMessage {
    fn from(value: &DeployModelWithStrategyEventPayload) -> Self {
        Self {
            owner: value.owner.clone(),
            platform: value.platform.clone(),
            model_name: value.model_name.clone(),
            model_author: value.model_author.clone(),
            strategy_name: value.strategy_name.clone(),
            params: value.params.clone(),
        }
    }
}