use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;
use crate::application::inputs::artifacts::ArtifactType;

#[derive(Debug, Error)]
pub enum EventPublisherError {
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
pub struct IngestArtifactEventPayload {
    pub ingestion_id: Uuid,
    pub artifact_type: ArtifactType,
    pub platform: String,
    pub webhook_url: Option<String>,
    pub serialized_client_request: Vec<u8>,
}

#[derive(Clone)]
pub enum Event {
    IngestArtifactEvent(IngestArtifactEventPayload),
    Placeholder // TODO remove Placeholder when a second event variant is added to this enum
}

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: Event) -> Result<(), EventPublisherError>;
}