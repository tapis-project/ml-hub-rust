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
use log::debug;
use url_parse::core::Parser;

pub async fn resolve_tenancy(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<EitherBody<impl MessageBody>>, Error> {
    let config = match req.app_data::<web::Data<SiteConfiguration>>().cloned() {
        Some(c) => c.into_inner(),
        None => return Ok(
                req
                    .into_response(HttpResponse::InternalServerError().json(json!({"error": "SiteConfiguration missing"})))
                    .map_into_right_body()
        )
    };

    let conn = req.connection_info().clone();
    let url_string = format!("{}://{}{}", conn.scheme(), conn.host(), req.uri());

    debug!("{}", &url_string);
    
    let url = match Parser::new(None).parse(&url_string) {
        Ok(u) => u,
        Err(err) => return Ok(
            req
                .into_response(HttpResponse::InternalServerError().json(json!({"error": format!("Failed parse url when resolving tenancy: {}", err.to_string())})))
                .map_into_right_body()
        )
    };

    debug!("{:#?}", &url);

    let maybe_tenant_id = match config.tenancy_resolution_mode {
        TenancyResolutionMode::Subdomain => {
            url.subdomain
                .as_ref()
                .and_then(|s| s.split(".").next())
        }
    };

    let tenant_id = match maybe_tenant_id {
        Some(t) => t,
        None => return Ok(
            req
                .into_response(HttpResponse::InternalServerError().json(json!({"error": "Unable to resolve tenant id"})))
                .map_into_right_body()
        )
    };

    req.extensions_mut().insert(Tenant { id: tenant_id.to_string() });

    Ok(next.call(req).await?.map_into_left_body())
}

