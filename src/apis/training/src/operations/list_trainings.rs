use actix_web::{get, HttpResponse, Responder};
use crate::dtos::training_dto::TrainingDto;
use crate::dtos::responses::Response;
use log::debug;

#[get("/training")]
async fn list_trainings() -> impl Responder {
    debug!("Operation list_trainings");
    let mut trainings: Vec<TrainingDto> = Vec::new();
    let training = TrainingDto {
        training_id: String::from("test")
    };
    trainings.push(training);
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        trainings
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}