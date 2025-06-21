use async_trait::async_trait;
use thiserror::Error;
use crate::common::application::inputs::IngestArtifactInput;

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

pub enum Message {
    IngestArtifactInput(IngestArtifactInput),
    Placeholer // TODO remove Placeholder when a second message variant is added to this enum
}

#[async_trait]
pub trait MessagePublisher: Send + Sync {
    async fn publish(&self, message: Message) -> Result<(), MessagePublisherError>;
}