use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessagePublisherError {
    #[error("Error connecting to message broker: {0}")]
    ConnectionError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String)
}

#[async_trait]
pub trait MessagePublisher: Send + Sync {
    async fn publish(&self, message: &[u8]) -> Result<(), MessagePublisherError>;
}