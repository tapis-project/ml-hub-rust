use actix_web::HttpResponse;
use shared::responses::JsonResponse;
use crate::config::VERSION;
use serde_json::Value;

pub fn build_error_response(status: u16, message: String) -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_type("application/json")
        .json(JsonResponse {
            status: Some(status),
            message: Some(message),
            result: None,
            metadata: None,
            version: Some(VERSION.to_string()),
        })
}

pub fn build_success_response(result: Option<Value>, status: Option<u16>, message: Option<String>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(JsonResponse {
            status,
            message,
            result,
            metadata: None,
            version: Some(VERSION.to_string()),
        })
}