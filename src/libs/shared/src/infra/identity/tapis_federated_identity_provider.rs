use crate::application::errors::ApplicationError;
use crate::application::ports::identity::FederatedIdentityProvider;
use crate::domain::entities::identity::{FederatedIdentity, Authority};

pub struct TapisFederatedIdentityProvider;

#[async_trait::async_trait]
impl FederatedIdentityProvider for TapisFederatedIdentityProvider {    
    async fn authenticate(&self, token: String) -> Result<Option<FederatedIdentity>, ApplicationError> {
        return Ok(None)
    }

    fn authority(&self) -> Authority {
        Authority::Tapis
    }
}