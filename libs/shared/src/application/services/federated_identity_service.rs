// TODO This should not be comming from infra as this is an application-level
// service.
use crate::infra::identity::Idp;

pub struct FederatedIdentityService;

impl FederatedIdentityService {
    pub fn resolve_idp_from_token(&self, _token: &String) -> Option<Idp> {
        // TODO Build out authority resolution logic once more IDPs are added
        Some(Idp::Tapis)
    }
}