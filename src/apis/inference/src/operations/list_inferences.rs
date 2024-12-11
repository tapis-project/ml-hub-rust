use actix_web::{get, HttpResponse, Responder};
use crate::dtos::inference_dto::InferenceDto;
use crate::dtos::responses::Response;
use log::debug;

#[get("/inference")]
async fn list_inferences() -> impl Responder {
    debug!("Operation list_inferences");
    let mut inferences: Vec<InferenceDto> = Vec::new();
    let inference_dto = InferenceDto {
        inference_id: String::from("test")
    };
    inferences.push(inference_dto);
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        inferences
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}