use crate::domain::entities::principal::Principal;
use crate::application::inputs::principal::FindByFederatedIdentity;

use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum PrincipalRepositoryError {
    #[error("Failed to save Principal: {0}")]
    FailedToSavePrincipal(String),

    #[error("Failed to start transaction: {0}")]
    FailedToStartTransaction(String),

    #[error("Failed to start session: {0}")]
    FailedToStartSession(String),

    #[error("Duplicate Principal")]
    PrincipalAlreadyExists,

    #[error("The FederatedIdentity with subject {0} and issued by {1} that is attached to the Principal is already owned by another Principal")]
    FederatedIdentityAlreadyOwned(String, String),
}

#[async_trait::async_trait]
pub trait PrincipalRepository: Send + Sync {
    async fn save(&self, principal: &Principal) -> Result<(), PrincipalRepositoryError>;
    async fn find_by_identity(&self, input: &FindByFederatedIdentity) -> Result<Option<Principal>, PrincipalRepositoryError>;
}