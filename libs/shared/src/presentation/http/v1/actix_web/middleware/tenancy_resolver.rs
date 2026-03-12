use std::future::{ready, Ready};
use actix_web::HttpMessage;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use crate::domain::entities::identity::FederatedIdentity;
use crate::domain::entities::tenant::Tenant;

pub struct TenancyResolver;

impl<S, B> Transform<S, ServiceRequest> for TenancyResolver
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TenancyResolverMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(
            Ok(TenancyResolverMiddleware { service })
        )
    }
}

pub struct TenancyResolverMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for TenancyResolverMiddleware<S>
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
        let federated_identity: Option<FederatedIdentity> = req.extensions()
            .get::<FederatedIdentity>()
            .map(|fid| fid.clone());

        // TODO Dispatch the tenant resolver for the provided federated identity
        // here and ensure that the tenant's id exists on enumerated list of tenants
        // on the federated identity
        let maybe_tenant_id = federated_identity
            .as_ref()
            .map(|i| i.tenants.clone())
            .map(|t| {
                t.iter()
                    .next()
                    .map(|id| id.clone())
            })
            .flatten();
        
        let mut maybe_tenant: Option<Tenant> = None;
        if let Some(id) = maybe_tenant_id{
            maybe_tenant = Some(Tenant { id })
        }

        req.extensions_mut().insert(maybe_tenant);
        
        let fut = self.service.call(req);

        return Box::pin(async move { Ok(fut.await?) })
    }
}