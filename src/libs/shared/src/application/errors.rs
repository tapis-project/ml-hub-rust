use thiserror::Error;
use crate::application::ports::messaging::MessagePublisherError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("{0}")]
    RepoError(String),

    #[error("{0}")]
    PublisherError(#[from] MessagePublisherError),

    #[error("{0}")]
    ConvesionError(String),
}