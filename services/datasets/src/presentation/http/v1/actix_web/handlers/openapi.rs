use actix_web::{get, Responder, HttpResponse};
use crate::presentation::http::v1::actix_web::openapi::ApiDoc;
use utoipa::OpenApi;

#[get("datasets-api/spec/openapi.json")]
async fn openapi() -> impl Responder {
    HttpResponse::Ok().json(ApiDoc::openapi())
}