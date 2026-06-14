use actix_web::dev::ServiceResponse;
use actix_web::middleware::Next;
use actix_web::{HttpMessage, HttpResponse};
use actix_web::http::Method;
use actix_web::{
    web,
    dev::ServiceRequest,
    body::{EitherBody, MessageBody},
};
use serde_json::json;
use log::{info, warn, error};

use crate::application::identity_context::{IdentityContext, Actor};
use crate::application::inputs::principal::GetOrCreateFromFederatedIdentity;
use crate::application::ports::identity::FederatedIdentityProviderError;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::principal_service::{PrincipalService, PrincipalServiceError};
use crate::domain::entities::tenancy::Tenant;
use crate::presentation::http::v1::requests::headers::AuthToken;
use crate::application::services::federated_idp_registrar::FederatedIdpRegistrar;
use crate::presentation::http::v1::actix_web::helpers::get_header_value;
use crate::presentation::http::v1::adapters::derive_header_keys_from_authorities;

pub async fn authenticate(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, actix_web::Error> {
    // First, we check that we are in the right tenant
    // Get the tenant from the requests extension data. May not be set
    let maybe_tenant  = req.extensions()
        .get::<Tenant>()
        .map(|fid| fid.clone());

    // Respond with error if no tenant is found
    let tenant = match maybe_tenant {
        Some(t) => t,
        None => {
            error!("No tenant found in authentication middleware. Tenant is expected to be resolved and available by the time this middleware run");
            return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": "No Tenant found when authenticating. Tenant is expected to have been resolved previously"})))
                    .map_into_right_body()
            )
        }
    };

    // Get federated identity service
    let federated_identity_service = match req.app_data::<web::Data<FederatedIdentityService>>().cloned() {
        Some(s) => s.into_inner(),
        None => {
            error!("Federated identity service not found in authentication middleware. This is very likely a bootstraping issue");
            return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "Federated identity service not found"})))
                    .map_into_right_body()
            )
        }
    };

    // Get the IDP registrar
    let idp_registrar = match req.app_data::<web::Data<FederatedIdpRegistrar>>().cloned() {
        Some(r) => r.into_inner(),
        None => {
            error!("Federated Idp registrar not found in authentication middleware. This is very likely a bootstraping issue");
            return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "Federated idp registrar not found"})))
                    .map_into_right_body()
            )
        }
    };
    
    // Check for token in headers. May be missing.
    let mut maybe_token: Option<AuthToken> = None;
    for header_key in derive_header_keys_from_authorities() {
        maybe_token = get_header_value(&header_key, req.request())
            .map(|t| AuthToken(t));
        
        if maybe_token.is_some() {
            break
        }
    }

    // Explicitly allow ONLY the OPTIONS method to bypass authentication
    if req.method() == Method::OPTIONS {
        return Ok(next.call(req).await?.map_into_left_body())
    }

    // Respond with error if no token exists
    let token = match maybe_token.clone() {
        Some(t) => t,
        None => {
            error!("Auth token missing");
            return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": "Missing auth token"})))
                    .map_into_right_body()
            )
        }
    };
    
    // Determine IDP name from the token
    let authority = match federated_identity_service.resolve_idp_from_token(&token.into_inner()) {
        Some(a) => a,
        None => {
            error!("Failed to derive Authority from token");
            return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": "Failed to derive Authority"})))
                    .map_into_right_body()
            )
        }
    };

    // Get the federated identitiy service from the registrar
    let idp = match idp_registrar.get_by_authority(authority.clone()) {
        Some(i) => i,
        None => {
            error!("Failed to find an IDP from the IDP registrar using authority {}", &authority);
            return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().finish())
                    .map_into_right_body()
            )
        }
    };

    // Authenticate with the IDP and get the federated identity. Might be missing.
    let maybe_federated_identity = match idp.authenticate(token.into_inner()).await {
        Ok(f) => f,
        Err(err) => {
            use FederatedIdentityProviderError as E;
            return match err {
                E::InvalidCredentials(msg) | E::MalformedCredentials(msg) => {
                    warn!("Malformed or invalid credentials found when attempting to authenticate with IDP {}. Error: {}", &authority, &msg);
                    Ok(
                        req
                            .into_response(HttpResponse::Unauthorized().json(json!({"error": msg})))
                            .map_into_right_body()
                    )
                },
                E::InternalIdpError(msg) | E::InitializationError(_, msg) => {
                    error!("Internal IDP error: {}", &msg);
                    Ok(
                        req
                            .into_response(HttpResponse::InternalServerError().json(json!({"error": msg})))
                            .map_into_right_body()
                    )
                }
            }
        }
    };

    // Check that the tenant of the federated identity and the tenant resolved from
    // the requested fqdn are the same
    let maybe_federated_identity = match maybe_federated_identity {
        Some(i) => {
            if i.clone().tenant_id != tenant.id {
                warn!("The federated identity's tenant ID does not match the tenant ID derived in a previous step. Expected: {}. Found: {}", tenant.id, i.clone().tenant_id);
                return Ok(
                    req
                        .into_response(HttpResponse::Forbidden().json(json!({"error": "Federated user's tenant_id does not match the resolved tenanat_id"})))
                        .map_into_right_body()
                )
            }

            Some(i)
        },
        None => None,
    };

    // Get the Principal that owns this federated identity
    if let Some(identity) = maybe_federated_identity {
        let principal_id = match authority.resolve_principal_id(&identity) {
            Ok(id) => id,
            Err(err) => {
                info!("Unable to resolve principal id: {}", &err);
                return Ok(
                    req
                        .into_response(HttpResponse::Unauthorized().json(json!({"error": err.to_string()})))
                        .map_into_right_body()
                )
            }
        };

        let principal_service = match req.app_data::<web::Data<PrincipalService>>().cloned() {
            Some(s) => s.into_inner(),
            None => {
                error!("Principal service not found in authentication middleware. This is very likely a bootstraping issue");
                return Ok(
                    req
                        .into_response(HttpResponse::InternalServerError().json(json!({"error": "Principal service not found"})))
                        .map_into_right_body()
                )
            }
        };

        let input = GetOrCreateFromFederatedIdentity {
            principal_id,
            identity,
        };

        let principal = match principal_service.get_or_create_from_identity(input).await {
            Ok(p) => p,
            Err(err) => {
                let resp = match err {
                    PrincipalServiceError::FederatedIdentityConflict(..) => {
                        info!("Federated identity conflict: {}", err.to_string());
                        HttpResponse::Conflict().json(json!({"error": format!("Error fetching or creating principal: {}", err.to_string())}))
                    },
                    PrincipalServiceError::PrincipalConflict => {
                        info!("Principal conflict: {}", err.to_string());
                        HttpResponse::Conflict().json(json!({"error": format!("Error fetching or creating principal: {}", err.to_string())}))
                    },
                    PrincipalServiceError::InternalError(..) => {
                        error!("Internal error creating principal from federated identity: {}", err.to_string());
                        HttpResponse::InternalServerError().json(json!({"error": format!("Error fetching or creating principal: {}", err.to_string())}))
                    }
                };

                return Ok(req.into_response(resp).map_into_right_body())
            }
        };

        let identity_conext = IdentityContext::new(
            Actor::from(principal),
            "".into()
        );

        req.extensions_mut().insert(identity_conext);
    }
    
    // Call the next middleware
    Ok(next.call(req).await?.map_into_left_body())
}