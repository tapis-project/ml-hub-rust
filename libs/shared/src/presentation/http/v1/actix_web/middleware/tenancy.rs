use crate::domain::entities::identity::FederatedIdentity;
use crate::domain::entities::tenant::Tenant;
use actix_web::{
    middleware::Next,
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error,
    HttpMessage
};

pub async fn resolve_tenancy(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, Error> {
    let federated_identity: Option<FederatedIdentity> = req.extensions()
        .get::<FederatedIdentity>()
        .map(|fid| fid.clone());

    // TODO Dispatch the tenant resolver for the provided federated identity
    // here and ensure that the tenant's id exists on enumerated list of tenants
    // on the federated identity
    let maybe_tenant_id = federated_identity
        .as_ref()
        .and_then(|i| i.tenants.first().cloned());
    
    let mut maybe_tenant: Option<Tenant> = None;
    if let Some(id) = maybe_tenant_id{
        maybe_tenant = Some(Tenant { id })
    }

    req.extensions_mut().insert(maybe_tenant);

    Ok(next.call(req).await?.map_into_left_body())
}

