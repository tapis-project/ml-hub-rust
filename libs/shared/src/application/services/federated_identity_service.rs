use crate::domain::entities::identity::Authority;
use log::warn;

pub struct FederatedIdentityService;

impl FederatedIdentityService {
    pub fn resolve_authority_from_token(&self, _token: &String) -> Option<Authority> {
        warn!("Automatically resolving to Tapis Authority");
        Some(Authority::Tapis)
    }
}