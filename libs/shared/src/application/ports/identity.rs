use thiserror::Error;

// Domain
use crate::domain::entities::identity::FederatedIdentity;

// Infra
use crate::infra::identity::Idp;

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