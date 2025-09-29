use actix_web::{get, HttpResponse, Responder};
use log::debug;

#[get("/inference-api/inference-servers/{inference_server}/interfaces")]
async fn list_inference_server_interfaces() -> impl Responder {
    debug!("Operation list_inference_server_interfaces");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("list inference server deployments")
}