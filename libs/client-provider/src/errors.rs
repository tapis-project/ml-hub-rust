use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientProviderError {
    #[error("Platform '{0}' not found or does not have '{1}' functionality")]
    NotFound(String, String),

    #[error("Could not parse platform name: {0}")]
    ParseError(String)
}