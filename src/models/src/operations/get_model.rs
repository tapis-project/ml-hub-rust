use actix_web::{web, get, HttpResponse, Responder};
use crate::models::models::Model;
use crate::models::responses::Response;

#[get("/models/{model_id}")]
async fn get_model(
    path: web::Path<String>
) -> impl Responder {
    let model_id = path.into_inner();
    let model = Model {
        model_id
    };
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        model
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}
