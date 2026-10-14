use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/endpoints-api/healthcheck",
    tag = "Health",
    summary = "Check service health",
    responses(
        (
            status = 200,
            description = "Service is healthy"
        ),
    ),
)]
#[get("endpoints-api/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().finish()
}
