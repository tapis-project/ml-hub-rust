use actix_web::dev::ServiceResponse;
use actix_web::http::Method;
use actix_web::middleware::Next;
use actix_web::{
    body::{EitherBody, MessageBody},
    dev::ServiceRequest,
    web,
};
use actix_web::{HttpMessage, HttpResponse};
use log::{error, info, warn};
use serde_json::json;

use crate::application::ports::identity::FederatedIdentityProviderError;
use crate::application::services::authentication_service::{
    authenticate_token, AuthenticationServiceError,
};
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::federated_idp_registrar::FederatedIdpRegistrar;
use crate::application::services::principal_service::{PrincipalService, PrincipalServiceError};
use crate::domain::entities::tenancy::Tenant;
use crate::presentation::http::v1::actix_web::helpers::get_header_value;
use crate::presentation::http::v1::adapters::derive_header_keys_from_authorities;
use crate::presentation::http::v1::requests::common::headers::AuthToken;

pub async fn authenticate(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, actix_web::Error> {
    // First, we check that we are in the right tenant
    // Get the tenant from the requests extension data. May not be set
    let maybe_tenant = req.extensions().get::<Tenant>().map(|fid| fid.clone());

    // Respond with error if no tenant is found
    let tenant = match maybe_tenant {
        Some(t) => t,
        None => {
            error!("No tenant found in authentication middleware. Tenant is expected to be resolved and available by the time this middleware run");
            return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": "No Tenant found when authenticating. Tenant is expected to have been resolved previously"})))
                    .map_into_right_body()
            );
        }
    };

    // Get federated identity service
    let federated_identity_service = match req
        .app_data::<web::Data<FederatedIdentityService>>()
        .cloned()
    {
        Some(s) => s.into_inner(),
        None => {
            error!("Federated identity service not found in authentication middleware. This is very likely a bootstraping issue");
            return Ok(req
                .into_response(
                    HttpResponse::InternalServerError()
                        .json(json!({"error": "Federated identity service not found"})),
                )
                .map_into_right_body());
        }
    };

    // Get the IDP registrar
    let idp_registrar = match req.app_data::<web::Data<FederatedIdpRegistrar>>().cloned() {
        Some(r) => r.into_inner(),
        None => {
            error!("Federated Idp registrar not found in authentication middleware. This is very likely a bootstraping issue");
            return Ok(req
                .into_response(
                    HttpResponse::InternalServerError()
                        .json(json!({"error": "Federated idp registrar not found"})),
                )
                .map_into_right_body());
        }
    };

    // Check for token in headers. May be missing.
    let mut maybe_token: Option<AuthToken> = None;
    for header_key in derive_header_keys_from_authorities() {
        maybe_token = get_header_value(&header_key, req.request()).map(|t| AuthToken(t));

        if maybe_token.is_some() {
            break;
        }
    }

    // Explicitly allow ONLY the OPTIONS method to bypass authentication
    if req.method() == Method::OPTIONS {
        return Ok(next.call(req).await?.map_into_left_body());
    }

    // Respond with error if no token exists
    let token = match maybe_token.clone() {
        Some(t) => t,
        None => {
            error!("Auth token missing");
            return Ok(req
                .into_response(
                    HttpResponse::Unauthorized().json(json!({"error": "Missing auth token"})),
                )
                .map_into_right_body());
        }
    };

    let principal_service = match req.app_data::<web::Data<PrincipalService>>().cloned() {
        Some(service) => service.into_inner(),
        None => {
            error!("Principal service not found in authentication middleware");

            return Ok(req
                .into_response(
                    HttpResponse::InternalServerError()
                        .json(json!({"error": "Principal service not found"})),
                )
                .map_into_right_body());
        }
    };

    let request_context = match authenticate_token(
        token.into_inner(),
        &tenant.id,
        &federated_identity_service,
        &idp_registrar,
        &principal_service,
    )
    .await
    {
        Ok(context) => context,
        Err(AuthenticationServiceError::TenantMismatch { .. }) => {
            warn!("Authenticated identity does not belong to the requested tenant");

            return Ok(req
                .into_response(HttpResponse::Forbidden().json(json!({"error": "Federated user's tenant_id does not match the resolved tenant_id"})))
                .map_into_right_body());
        }
        Err(AuthenticationServiceError::IdentityProvider(
            FederatedIdentityProviderError::InvalidCredentials(message)
            | FederatedIdentityProviderError::MalformedCredentials(message),
        )) => {
            warn!("Malformed or invalid credentials");

            return Ok(req
                .into_response(HttpResponse::Unauthorized().json(json!({"error": message})))
                .map_into_right_body());
        }
        Err(AuthenticationServiceError::Principal(
            PrincipalServiceError::FederatedIdentityConflict(..)
            | PrincipalServiceError::PrincipalConflict,
        )) => {
            info!("Principal identity conflict while authenticating request");

            return Ok(req
                .into_response(
                    HttpResponse::Conflict().json(json!({"error": "Principal identity conflict"})),
                )
                .map_into_right_body());
        }
        Err(AuthenticationServiceError::IdentityProvider(
            FederatedIdentityProviderError::InternalIdpError(_)
            | FederatedIdentityProviderError::InitializationError(_, _),
        ))
        | Err(AuthenticationServiceError::Principal(PrincipalServiceError::InternalError(_))) => {
            error!("Authentication dependency failed");

            return Ok(req
                .into_response(
                    HttpResponse::InternalServerError()
                        .json(json!({"error": "Authentication service unavailable"})),
                )
                .map_into_right_body());
        }
        Err(error) => {
            error!("Authentication failed: {error}");

            return Ok(req
                .into_response(
                    HttpResponse::Unauthorized().json(json!({"error": error.to_string()})),
                )
                .map_into_right_body());
        }
    };

    req.extensions_mut().insert(request_context);

    // Call the next middleware
    Ok(next.call(req).await?.map_into_left_body())
}
