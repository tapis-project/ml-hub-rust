use actix_web::{web, get, HttpResponse, Responder};
use crate::dtos::inference_dto::InferenceDto;
use crate::dtos::responses::Response;
use log::debug;

#[get("/inference/{inference_id}")]
async fn get_inference(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_inference");
    let inference_id = path.into_inner();
    let inference_dto = InferenceDto {
        inference_id
    };
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        inference_dto
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}
