use actix_web::dev::ServiceResponse;
use actix_web::middleware::Next;
use actix_web::{HttpMessage, HttpResponse};
use actix_web::{
    web,
    dev::ServiceRequest,
    body::{EitherBody, MessageBody},
};
use crate::application::ports::identity::FederatedIdentityProviderError;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::presentation::http::v1::requests::headers::AuthToken;
use crate::application::services::federated_ipd_registrar::FederatedIdpRegistrar;
use crate::presentation::http::v1::actix_web::helpers::get_header_value;
use crate::presentation::http::v1::adapters::derive_header_keys_from_authorites;
use serde_json::json;
use log::debug;

pub async fn authenticate(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, actix_web::Error> {
    debug!("Authenticating");

    let federated_identity_service = match req.app_data::<web::Data<FederatedIdentityService>>().cloned() {
        Some(s) => s.into_inner(),
        None => return Ok(
            req
                .into_response(HttpResponse::InternalServerError().json(json!({"error": "FederatedIdentityService not found"})))
                .map_into_right_body()
        )
    };

    let idp_registrar = match req.app_data::<web::Data<FederatedIdpRegistrar>>().cloned() {
        Some(r) => r.into_inner(),
        None => return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "FederatedIpdRegistrar not found"})))
                    .map_into_right_body()
            )
    };
    
    let mut maybe_token: Option<AuthToken> = None;
    for header_key in derive_header_keys_from_authorites() {
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
    
    let authority = match federated_identity_service.resolve_authority_from_token(&token.into_inner()) {
        Some(a) => a,
        None => return Ok(
                req
                    .into_response(HttpResponse::Unauthorized().json(json!({"error": "Failed to derived Authority"})))
                    .map_into_right_body()
            )
    };

    let idp = match idp_registrar.get_by_authority(authority) {
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

    req.extensions_mut().insert(maybe_federated_identity);
    
    Ok(next.call(req).await?.map_into_left_body())
}