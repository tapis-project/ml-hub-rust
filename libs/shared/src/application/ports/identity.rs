use crate::{application::errors::ApplicationError, domain::entities::identity::FederatedIdentity};
use crate::bootstrap::Idp;
use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum FederatedIdentityProviderError {
    #[error("Federated identity provider intialization error: Authority '{0}'. Error: {1}")]
    InitializationError(Idp, String),

    #[error("Malformed credentials: {0}")]
    MalformedCredentials(String),

    #[error("Internal Idp error: {0}")]
    InternalIdpError(String),

    #[error("Invalid Credentials: {0}")]
    InvalidCredentials(String),
}

#[async_trait::async_trait]
pub trait FederatedIdentityProvider: Send + Sync {
    async fn authenticate(&self, token: String) -> Result<Option<FederatedIdentity>, FederatedIdentityProviderError>;
    fn authority(&self) -> Idp;
}

#[async_trait::async_trait]
pub trait FederatedIdentityRepository: Send + Sync {
    async fn save(&self, identity: &FederatedIdentity) -> Result<(), ApplicationError>;
}