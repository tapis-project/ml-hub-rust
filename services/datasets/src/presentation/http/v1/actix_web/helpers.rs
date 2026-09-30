use crate::config::VERSION;
use actix_web::HttpResponse;
use serde_json::Value;
use shared::presentation::http::v1::actix_web::helpers::{
    build_error_response as error, build_success_response as success,
};

pub fn build_error_response(status: u16, message: String) -> HttpResponse {
    error(status, message, Some(VERSION.into()), None)
}

pub fn build_success_response(
    result: Option<Value>,
    message: Option<String>,
    metadata: Option<Value>,
) -> HttpResponse {
    success(
        result,
        message,
        Some(VERSION.into()),
        metadata.or_else(|| Some(Value::Object(serde_json::Map::new()))),
    )
}
