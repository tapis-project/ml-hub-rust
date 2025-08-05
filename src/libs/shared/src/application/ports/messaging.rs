use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;
use crate::application::inputs::artifacts::ArtifactType;

#[derive(Debug, Error)]
pub enum MessagePublisherError {
    #[error("Message broker error: {0}")]
    AmqpError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Connection: {0}")]
    ConnectionError(String),
}

#[derive(Clone)]
pub struct IngestArtifactMessagePayload {
    pub ingestion_id: Uuid,
    pub artifact_type: ArtifactType,
    pub platform: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

#[derive(Clone)]
pub enum Message {
    IngestArtifactMessage(IngestArtifactMessagePayload),
    Placeholder // TODO remove Placeholder when a second message variant is added to this enum
}

#[async_trait]
pub trait MessagePublisher: Send + Sync {
    async fn publish(&self, message: Message) -> Result<(), MessagePublisherError>;
}