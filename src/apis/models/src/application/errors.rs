use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("{0}")]
    RepoError(String),

    #[error("{0}")]
    PublisherError(String),
}