use actix_web::{get, HttpResponse, Responder};
use crate::models::models::Model;
use crate::models::responses::Response;
use log::debug;

#[get("/models")]
async fn list_models() -> impl Responder {
    debug!("Operation list_models");
    let mut models: Vec<Model> = Vec::new();
    let model = Model {
        model_id: String::from("test")
    };
    models.push(model);
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        models
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}