use actix_web::HttpResponse;
use shared::responses::helpers::{
    build_error_response as error,
    build_success_response as success
};
use crate::config::VERSION;
use serde_json::Value;

pub fn build_error_response(status: u16, message: String) -> HttpResponse {
    error(status, message, Some(String::from(VERSION)))
}

pub fn build_success_response(result: Option<Value>, status: Option<u16>, message: Option<String>) -> HttpResponse {
    success(result, status, message, Some(String::from(VERSION)))
}