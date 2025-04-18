use actix_web::{web, get, HttpResponse, Responder};
use log::debug;

#[get("/inference-api/inference-servers/{inference_server_name}")]
async fn get_inference_server(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_inference");
    HttpResponse::Ok()
    .content_type("text/html")
    .body("get inference server")
}
