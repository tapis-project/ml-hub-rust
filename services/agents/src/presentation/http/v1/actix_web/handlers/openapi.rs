use actix_web::{get, HttpResponse, Responder};
use utoipa::OpenApi;

use crate::presentation::http::v1::actix_web::openapi::ApiDoc;

#[get("/agents-api/spec/openapi.json")]
pub async fn openapi() -> impl Responder {
    HttpResponse::Ok().json(ApiDoc::openapi())
}
