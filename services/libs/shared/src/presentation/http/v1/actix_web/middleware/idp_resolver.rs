use std::sync::Arc;
use std::future::{ready, Ready};
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use crate::application::services::federated_identity_service::FederatedIdentityService;
use crate::presentation::http::v1::requests::headers::AuthToken;
use crate::application::services::federated_ipd_registrar::FederatedIdpRegistrar;
use crate::presentation::http::v1::actix_web::helpers::get_header_value;
use crate::presentation::http::v1::adapters::derive_header_keys_from_authorites;

pub struct IdpResolver {
    idp_registrar: Arc<FederatedIdpRegistrar>,
    federated_identity_service: Arc<FederatedIdentityService>,
}

impl IdpResolver {
    pub fn new(
        registrar: Arc<FederatedIdpRegistrar>,
        federated_identity_service: Arc<FederatedIdentityService>,
    ) -> Self {
        Self {
            idp_registrar: registrar,
            federated_identity_service
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for IdpResolver
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = IdpResolverMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(
            Ok(
                IdpResolverMiddleware {
                    federated_identity_service: self.federated_identity_service.clone(),
                    idp_registrar: self.idp_registrar.clone(),
                    service
                }
            )
        )
    }
}

pub struct IdpResolverMiddleware<S> {
    federated_identity_service: Arc<FederatedIdentityService>,
    idp_registrar: Arc<FederatedIdpRegistrar>,
    service: S,
}

impl<S, B> Service<ServiceRequest> for IdpResolverMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
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
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) })
            }
        };
        
        let authority = match self.federated_identity_service.resolve_authority_from_token(&token.into_inner()) {
            Some(a) => a,
            None => {
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) })
            }
        };

        let token_idp_pair = match self.idp_registrar.get_by_authority(authority) {
            Some(i) => {
                (token, i)
            },
            None => {
                let fut = self.service.call(req);
                return Box::pin(async move { Ok(fut.await?) })
            }
        };

        req.extensions_mut().insert(token_idp_pair);
        
        let fut = self.service.call(req);

        return Box::pin(async move { Ok(fut.await?) })
    }
}