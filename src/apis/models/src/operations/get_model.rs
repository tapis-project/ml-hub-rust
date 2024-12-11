use actix_web::{web, get, HttpResponse, Responder};
use crate::dtos::model_dto::ModelDto;
use crate::dtos::responses::Response;
use log::debug;
use huggingface_client::HuggingFaceClient;

#[get("/models/{model_id}")]
async fn get_model(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_model");
    
    let c = HuggingFaceClient::new();
    c.call();
    
    let model_id = path.into_inner();
    let model_dto = ModelDto {
        model_id
    };

    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        model_dto
    );

    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}
