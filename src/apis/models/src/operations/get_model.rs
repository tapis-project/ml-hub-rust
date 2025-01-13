use actix_web::{web, get, HttpResponse, Responder};
use crate::dtos::model_dto::ModelDto;
use crate::dtos::responses::OkResponse;
use log::debug;

#[get("/models/{model_id}")]
async fn get_model(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_model");
    
    let model_id = path.into_inner();
    let model_dto = ModelDto {
        model_id
    };

    let resp = OkResponse {
        status: String::from("test"),
        message: String::from("test"),
        result: model_dto,
        metadata: String::from("test"),
        version: String::from("test"),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}
