use thiserror::Error;
use crate::application::ports::commands::CommandPublisherError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("{0}")]
    RepoError(String),

    #[error("{0}")]
    PublisherError(#[from] CommandPublisherError),

    #[error("{0}")]
    DomainError(String),

    #[error("{0}")]
    ConversionError(String),

    #[error("Site config loader initialization error: {0}")]
    SiteConfigLoaderInitialization(String),

    #[error("Deployment strategy provider initialization error: {0}")]
    DeploymentStrategyProviderInitialization(String),

    #[error("Model deployment failed: {0}")]
    ModelDeploymentFailed(String),
}