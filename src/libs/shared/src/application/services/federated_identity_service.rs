use crate::domain::entities::identity::Authority;

pub struct FederatedIdentityService;

impl FederatedIdentityService {
    pub fn resolve_authority_from_token(&self, token: &String) -> Option<Authority> {
        Some(Authority::Tapis)
    }
}