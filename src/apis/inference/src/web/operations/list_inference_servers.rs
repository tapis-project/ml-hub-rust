use actix_web::{get, HttpResponse, Responder};
use log::debug;

#[get("/inference-api/inference-servers")]
async fn list_inference_servers() -> impl Responder {
    debug!("Operation list_inferences");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("list inference servers")
}