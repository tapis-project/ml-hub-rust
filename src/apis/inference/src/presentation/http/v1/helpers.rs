use actix_web::HttpResponse;
use shared::common::presentation::http::v1::actix_web::helpers::{
    build_error_response as error,
    build_success_response as success
};
use crate::config::VERSION;
use serde_json::Value;

pub fn build_error_response(status: u16, message: String) -> HttpResponse {
    error(status, message, Some(String::from(VERSION)), None)
}

pub fn build_success_response(result: Option<Value>, message: Option<String>, metadata: Option<Value>) -> HttpResponse {
    let meta = if metadata.is_some() {
        metadata
    } else {
        Some(Value::Object(serde_json::Map::new()))
    };

    success(result,  message, Some(String::from(VERSION)), meta)
}