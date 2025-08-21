use actix_web::{web, delete, HttpResponse, Responder};
use log::debug;

#[delete("/inference-api/inference-servers/{inference_server_name}")]
async fn delete_inference_server(
    _path: web::Path<String>
) -> impl Responder {
    debug!("Operation create_inference_server");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("create inference server")
}
