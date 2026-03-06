use std::collections::HashSet;
use std::sync::Arc;
use crate::application::ports::identity::{FederatedIdentityProvider, FederatedIdentityProviderError};
use crate::domain::entities::identity::Authority;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum FederatedIdpRegistrarError {
    #[error("IDP already registered with Authority {0}")]
    DuplicateAuthorityRegistration(String),

    #[error("IDP error: {0}")]
    FederatedIdentityError(#[from] FederatedIdentityProviderError)
}

pub struct FederatedIdpRegistrar {
    authorities: HashSet<Authority>,
    providers: Vec<Arc<dyn FederatedIdentityProvider>>
}

impl FederatedIdpRegistrar {
    pub fn new() -> Self {
        Self { authorities: HashSet::new(), providers: Vec::new() }
    }
}

impl FederatedIdpRegistrar {
    pub fn register(&mut self, idp: Arc<dyn FederatedIdentityProvider>) -> Result<(), FederatedIdpRegistrarError> {
        // If false, an IDP was already registered with this Authority
        let authority = idp.authority();
        if !self.authorities.insert(authority.clone()) {
            return Err(FederatedIdpRegistrarError::DuplicateAuthorityRegistration(authority.to_string()))
        }

        self.providers.push(idp);

        Ok(())
    }

    pub fn get_by_authority(&self, authority: Authority) -> Option<Arc<dyn FederatedIdentityProvider>> {
        self.providers
            .iter()
            .find(|p| p.authority() == authority)
            .cloned()
    }
}