use actix_web::{web, get, HttpResponse, Responder};
use crate::dtos::training_dto::TrainingDto;
use crate::dtos::responses::Response;
use log::debug;

#[get("/training/{training_id}")]
async fn get_training(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_training");
    let training_id = path.into_inner();
    let training_dto = TrainingDto {
        training_id
    };
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        training_dto
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}
