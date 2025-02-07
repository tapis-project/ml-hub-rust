use crate::config::VERSION;
use actix_web::{
    get,
    HttpResponse,
    Responder as ActixResponder
};
use log::debug;
use shared::responses::Response;

#[get("/models-api")]
pub async fn index() -> impl ActixResponder {
    debug!("Index operation");

    HttpResponse::Ok()
        .content_type("application/json")
        .json(Response {
            status: Some(200),
            message: Some(String::from("success")),
            result: None,
            metadata: None,
            version: Some(VERSION.to_string())
        })
}