use crate::bootstrap::TenancyResolutionMode;
use crate::bootstrap::SiteConfiguration;
use crate::domain::entities::tenancy::Tenant;
use actix_web::HttpResponse;
use actix_web::{
    web,
    middleware::Next,
    body::{EitherBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    Error,
    HttpMessage
};
use serde_json::json;
use url_parse::core::Parser;
use log::error;

pub async fn resolve_tenancy(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, Error> {
    let config = match req.app_data::<web::Data<SiteConfiguration>>().cloned() {
        Some(c) => c.into_inner(),
        None => {
            error!("Site configuration missing from the request's app data");
            return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "SiteConfiguration missing"})))
                    .map_into_right_body()
            )
        }
    };

    // Get the FQDN from X-Forwarded-Host first, then fallback to Host
    let maybe_fqdn = req
        .headers()
        .get("X-Forwarded-Host")
        .or_else(|| req.headers().get("Host"))
        .and_then(|val| val.to_str().map(|v| String::from(v)).ok());

    let fqdn = match maybe_fqdn {
        Some(domain) => domain,
        None => {
            error!("Failed to resolve FQDN from X-Forwarded-Host and Host headers");
            return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "Failed to resolve FQDN"})))
                    .map_into_right_body()
            )
        }
    };
    
    let url = match Parser::new(None).parse(&fqdn) {
        Ok(u) => u,
        Err(err) => {
            error!("Failed to parse the URL from the FQDN resolved in the headers. Resolved FQDN: {:#?}", &fqdn);
            return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": format!("Failed parse url when resolving tenancy: {}", err.to_string())})))
                    .map_into_right_body()
            )
        }
    };

    let maybe_tenant_id = match config.tenancy_resolution_mode {
        TenancyResolutionMode::Subdomain => {
            url.subdomain
                .as_ref()
                .and_then(|s| s.split(".").next())
        }
    };

    let tenant_id = match maybe_tenant_id {
        Some(t) => t,
        None => {
            error!("Unable to resolve tenant id using the following tenancy resolution mode: {}", &config.tenancy_resolution_mode);
            return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "Unable to resolve tenant id"})))
                    .map_into_right_body()
            )
        }
    };

    req.extensions_mut().insert(Tenant { id: tenant_id.to_string() });

    Ok(next.call(req).await?.map_into_left_body())
}

