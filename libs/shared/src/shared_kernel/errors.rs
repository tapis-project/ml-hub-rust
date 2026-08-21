use thiserror::Error;

#[derive(Debug, Error)]
pub enum BootstrapError {
    #[error("Failed to initialize '{0}': {1}")]
    FailedToInitialize(String, String)
}