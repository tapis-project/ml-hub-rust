use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;
use crate::application::inputs::artifacts::ArtifactType;

// TODO Message borker related errors should be factored out of these ports
#[derive(Debug, Error)]
pub enum CommandPublisherError {
    #[error("Event broker error: {0}")]
    AmqpError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Connection: {0}")]
    ConnectionError(String),
}

#[derive(Clone)]
pub struct IngestArtifactCommandPayload {
    pub ingestion_id: Uuid,
    pub artifact_type: ArtifactType,
    pub platform: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

#[derive(Clone)]
pub struct PublishArtifactCommandPayload {
    pub publication_id: Uuid,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

#[derive(Clone)]
pub enum Command {
    IngestArtifactCommand(IngestArtifactCommandPayload),
    PublishArtifactCommand(PublishArtifactCommandPayload),
}

#[async_trait]
pub trait CommandPublisher: Send + Sync {
    async fn publish(&self, command: &Command) -> Result<(), CommandPublisherError>;
}