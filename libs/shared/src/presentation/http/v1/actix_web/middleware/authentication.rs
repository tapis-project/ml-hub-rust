use actix_web::dev::ServiceResponse;
use actix_web::middleware::Next;
use actix_web::{HttpMessage, HttpResponse};
use actix_web::{
    web,
    dev::ServiceRequest,
    body::{EitherBody, MessageBody},
};
use serde_json::json;

use crate::application::actor::Actor;
use crate::application::inputs::principal::GetOrCreateFromFederatedIdentity;
use crate::application::ports::identity::FederatedIdentityProviderError;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::application::services::principal_service::PrincipalService;
use crate::domain::entities::tenancy::Tenant;
use crate::presentation::http::v1::requests::headers::AuthToken;
use crate::application::services::federated_idp_registrar::FederatedIdpRegistrar;
use crate::presentation::http::v1::actix_web::helpers::get_header_value;
use crate::presentation::http::v1::adapters::derive_header_keys_from_authorities;

pub async fn authenticate(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, actix_web::Error> {
    let federated_identity_service = match req.app_data::<web::Data<FederatedIdentityService>>().cloned() {
        Some(s) => s.into_inner(),
        None => return Ok(
            req
                .into_response(HttpResponse::InternalServerError().json(json!({"error": "Federated identity service not found"})))
                .map_into_right_body()
        )
    };

    let idp_registrar = match req.app_data::<web::Data<FederatedIdpRegistrar>>().cloned() {
        Some(r) => r.into_inner(),
        None => return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "Federated idp registrar not found"})))
                    .map_into_right_body()
            )
    };
    
    let mut maybe_token: Option<AuthToken> = None;
    for header_key in derive_header_keys_from_authorities() {
        maybe_token = get_header_value(&header_key, req.request())
            .map(|t| AuthToken(t));
        
        if maybe_token.is_some() {
            break
        }
    }

    let token = match maybe_token.clone() {
        Some(t) => t,
        None => {
            return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": "Missing auth token"})))
                    .map_into_right_body()
            )
        }
    };
    
    let authority = match federated_identity_service.resolve_idp_from_token(&token.into_inner()) {
        Some(a) => a,
        None => return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": "Failed to derived Authority"})))
                    .map_into_right_body()
            )
    };

    let idp = match idp_registrar.get_by_authority(authority.clone()) {
        Some(i) => i,
        None => return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().finish())
                    .map_into_right_body()
            )
    };

    let maybe_federated_identity = match idp.authenticate(token.into_inner()).await {
        Ok(f) => f,
        Err(err) => {
            use FederatedIdentityProviderError as E;
            return match err {
                E::InvalidCredentials(msg) | E::MalformedCredentials(msg) => {
                    Ok(
                        req
                            .into_response(HttpResponse::Unauthorized().json(json!({"error": msg})))
                            .map_into_right_body()
                    )
                },
                E::InternalIdpError(msg) | E::InitializationError(_, msg)=> Ok(
                        req
                            .into_response(HttpResponse::InternalServerError().json(json!({"error": msg})))
                            .map_into_right_body()
                    )
            }
        }
    };

    let maybe_tenant  = req.extensions()
        .get::<Tenant>()
        .map(|fid| fid.clone());

    let tenant = match maybe_tenant {
        Some(t) => t,
        None => return Ok(
            req
                .into_response(HttpResponse::Unauthorized().json(json!({"error": "No Tenant found when authenticating. Then must have been resolved previously"})))
                .map_into_right_body()
        )
    };

    let maybe_federated_identity = match maybe_federated_identity {
        Some(i) => {
            if i.clone().tenant_id != tenant.id {
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

    if let Some(identity) = maybe_federated_identity {
        let principal_id = match authority.resolve_principal_id(&identity) {
            Ok(id) => id,
            Err(err) => return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": err.to_string()})))
                    .map_into_right_body()
            )
        };

        let principal_service = match req.app_data::<web::Data<PrincipalService>>().cloned() {
            Some(s) => s.into_inner(),
            None => return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "Principal service not found"})))
                    .map_into_right_body()
            )
        };

        let input = GetOrCreateFromFederatedIdentity {
            principal_id,
            identity,
        };

        let principal = match principal_service.get_or_create_from_identity(input).await {
            Ok(p) => p,
            Err(err) => return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": format!("Error fetching or creating principal: {}", err.to_string())})))
                    .map_into_right_body()
            )
        };

        req.extensions_mut().insert(Actor::from(principal));
    }
    
    Ok(next.call(req).await?.map_into_left_body())
}