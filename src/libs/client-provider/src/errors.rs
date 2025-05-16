use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientProviderError<'a> {
    #[error("Platform '{0}' not found or does not have '{1}' functionality")]
    NotFound(&'a str, &'a str),

    #[error("Could not parse platform name: {0}")]
    ParseError(String)
}