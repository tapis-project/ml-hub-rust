use actix_web::{get, HttpResponse, Responder};
use log::debug;

#[get("/inference-api/inference-servers/{inference_server}/interfaces/{interface_name}")]
async fn get_inference_server_interface() -> impl Responder {
    debug!("Operation get_inference_server_interface");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("Get inference server interface")
}