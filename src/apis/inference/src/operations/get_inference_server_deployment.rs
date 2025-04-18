use actix_web::{web, get, HttpResponse, Responder};
use log::debug;

#[get("/inference-api/{inference_server_name}/deploymets/{deployment_name}")]
async fn get_inference_server_deployment(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_inference_server_deployment");
    HttpResponse::Ok()
        .content_type("text/html")
        .body("get inference server deployment")
}
