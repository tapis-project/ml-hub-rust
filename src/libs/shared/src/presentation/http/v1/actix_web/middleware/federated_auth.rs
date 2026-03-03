use std::future::{ready, Ready};

use actix_web::{
    HttpMessage,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use crate::domain::entities::identity::FederatedIdentity;
use crate::presentation::http::v1::actix_web::helpers::get_header_value;


pub struct FederatedAuth;

impl<S, B> Transform<S, ServiceRequest> for FederatedAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = FederatedAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(FederatedAuthMiddleware { service }))
    }
}

pub struct FederatedAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for FederatedAuthMiddleware<S>
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
        let maybe_authorization_value = get_header_value("Authorization", req.request());
        
        let maybe_identity:Option<FederatedIdentity> = None;
        
        if let Some(auth_value) = maybe_authorization_value {
            
        }

        req.extensions_mut().insert(maybe_identity);
        
        let fut = self.service.call(req);

        Box::pin(async move { Ok(fut.await?) })
    }
}