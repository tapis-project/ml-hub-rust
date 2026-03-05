use crate::domain::entities::identity::Authority;
use log::debug;

pub struct FederatedIdentityService;

impl FederatedIdentityService {
    pub fn resolve_authority_from_token(&self, _token: &String) -> Option<Authority> {
        debug!("Automatically resolving to Tapis Authority");
        Some(Authority::Tapis)
    }
}