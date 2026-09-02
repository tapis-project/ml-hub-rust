use actix_web::{get, HttpResponse, Responder};

#[utoipa::path(
    get,
    path = "/datasets-api/healthcheck",
    tag = "Health",
    summary = "Check service health",
    responses(
        (status = 200, description = "Service is healthy"),
    )
)]
#[get("datasets-api/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().finish()
}
