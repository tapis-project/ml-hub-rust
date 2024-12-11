use actix_web::{web, get, HttpResponse, Responder};
use crate::dtos::dataset_dto::DatasetDto;
use crate::dtos::responses::Response;
use log::debug;

#[get("/datasets/{dataset_id}")]
async fn get_dataset(
    path: web::Path<String>
) -> impl Responder {
    debug!("Operation get_dataset");
    let dataset_id = path.into_inner();
    let dataset_dto = DatasetDto {
        dataset_id
    };
    let resp = Response::new(
        String::from("test"),
        String::from("test"),
        String::from("test"),
        String::from("test"),
        dataset_dto
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(resp)
}
