use actix_web::{get, HttpResponse, Responder};
use log::debug;

#[get("/inference-api/inference-server/{inference_server}/deployments")]
async fn list_inference_server_deployments() -> impl Responder {
    debug!("Operation list_inferences");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("list inference server deployments")
}