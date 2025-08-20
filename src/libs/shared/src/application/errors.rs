use thiserror::Error;
use crate::application::ports::events::EventPublisherError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("{0}")]
    RepoError(String),

    #[error("{0}")]
    PublisherError(#[from] EventPublisherError),

    #[error("{0}")]
    ConvesionError(String),
}