use thiserror::Error;

use crate::application::inputs::principal::GetOrCreateFromFederatedIdentity;
use crate::application::ports::identity::FederatedIdentityProviderError;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::federated_idp_registrar::FederatedIdpRegistrar;
use crate::application::services::principal_service::{PrincipalService, PrincipalServiceError};
use crate::shared_kernel::context::{Actor, RequestContext};

pub async fn authenticate_token(
    token: String,
    expected_tenant_id: &str,
    federated_identity_service: &FederatedIdentityService,
    idp_registrar: &FederatedIdpRegistrar,
    principal_service: &PrincipalService,
) -> Result<RequestContext, AuthenticationServiceError> {
    let forwarded_token = token.clone();

    let authority = federated_identity_service
        .resolve_idp_from_token(&token)
        .ok_or(AuthenticationServiceError::UnresolvedAuthority)?;

    let idp = idp_registrar
        .get_by_authority(authority.clone())
        .ok_or_else(|| AuthenticationServiceError::UnregisteredAuthority(authority.to_string()))?;

    let identity = idp
        .authenticate(token)
        .await?
        .ok_or(AuthenticationServiceError::IdentityNotFound)?;

    if identity.tenant_id != expected_tenant_id {
        return Err(AuthenticationServiceError::TenantMismatch {
            expected: expected_tenant_id.into(),
            actual: identity.tenant_id,
        });
    }

    let principal_id = authority
        .resolve_principal_id(&identity)
        .map_err(|error| AuthenticationServiceError::InvalidPrincipal(error.to_string()))?;

    let principal = principal_service
        .get_or_create_from_identity(GetOrCreateFromFederatedIdentity {
            principal_id,
            identity,
        })
        .await?;

    Ok(RequestContext::new(
        Actor::from(principal),
        forwarded_token,
        None,
        None,
        None,
    ))
}

#[derive(Debug, Error)]
pub enum AuthenticationServiceError {
    #[error("Unable to resolve identity provider from token")]
    UnresolvedAuthority,

    #[error("Identity provider is not registered: {0}")]
    UnregisteredAuthority(String),

    #[error("Federated identity was not found")]
    IdentityNotFound,

    #[error("Federated identity tenant does not match the requested tenant")]
    TenantMismatch { expected: String, actual: String },

    #[error("Unable to resolve principal identity: {0}")]
    InvalidPrincipal(String),

    #[error(transparent)]
    IdentityProvider(#[from] FederatedIdentityProviderError),

    #[error(transparent)]
    Principal(#[from] PrincipalServiceError),
}
