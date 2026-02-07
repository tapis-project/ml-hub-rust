use thiserror::Error;

#[derive(Debug, Error)]
pub enum JsonError {
    #[error("Serialization Error: {0}")]
    SerializationFailed(String),

    #[error("Deserialization Error: {0}")]
    DeserializationFailed(String)
}