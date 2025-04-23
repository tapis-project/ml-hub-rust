use actix_web::{web, put, HttpResponse, Responder};
use log::debug;

#[put("/inference-api/inference-servers/{inference_server_name}")]
async fn update_inference_server(
    _path: web::Path<String>
) -> impl Responder {
    debug!("Operation create_inference_server");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("create inference server")
}
