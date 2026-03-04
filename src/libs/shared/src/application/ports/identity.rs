use crate::application::errors::ApplicationError;
use crate::domain::entities::identity::{FederatedIdentity, Authority};

#[async_trait::async_trait]
pub trait FederatedIdentityProvider: Send + Sync {
    async fn authenticate(&self, token: String) -> Result<Option<FederatedIdentity>, ApplicationError>;
    fn authority(&self) -> Authority;
}