use actix_web::{get, HttpResponse, Responder};
use crate::dtos::model_dto::ModelDto;
use crate::dtos::responses::Response;
use log::debug;

#[get("/models")]
async fn list_models() -> impl Responder {
    debug!("Operation list_models");
    let mut models: Vec<ModelDto> = Vec::new();
    let model_dto = ModelDto {
        model_id: String::from("test")
    };
    models.push(model_dto);
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