use actix_web::{get, HttpResponse, Responder};
use crate::dtos::dataset_dto::DatasetDto;
use crate::dtos::responses::Response;
use log::debug;

#[get("/datasets")]
async fn list_datasets() -> impl Responder {
    debug!("Operation list_dataset");
    let mut datasets: Vec<DatasetDto> = Vec::new();
    let dataset = DatasetDto {
        dataset_id: String::from("test")
    };
    datasets.push(dataset);
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        datasets
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}