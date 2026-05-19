use thiserror::Error;

#[derive(Debug, Error)]
pub enum PresentationError {
    #[error("Validation error: {0}")]
    ValidationError(String),
}