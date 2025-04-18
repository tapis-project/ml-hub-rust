use actix_web::{web, post, HttpResponse, Responder};
use log::debug;

#[post("/inference-api/inference-servers")]
async fn create_inference_server(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation create_inference_server");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("create inference server")
}
