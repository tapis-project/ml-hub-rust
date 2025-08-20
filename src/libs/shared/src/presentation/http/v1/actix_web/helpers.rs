use crate::presentation::http::v1::responses::JsonResponse;
use crate::presentation::http::v1::dto::Parameters;
use crate::errors::Error;
use serde_json::Value;
use actix_web::{HttpRequest, HttpResponse, http::StatusCode};

pub fn param_to_string(params: Option<Parameters>, prop: &str) -> Result<Option<String>, Error> {
    let val = match params.and_then(|mut m| m.remove(prop)) {
        Some(Value::String(s)) => Some(s),
        Some(v) => Some(v.to_string()), // fallback if it's not a string
        None => None,
    };
    Ok(val)
}

pub fn get_header_value(header_key: &str, request: &HttpRequest) -> Option<String> {
    request
        .headers()
        .get(header_key)
        .and_then(|value| value.to_str().ok())
        .map(|value| String::from(value))
}


pub fn build_error_response(status: u16, message: String, version: Option<String>, metadata: Option<Value>) -> HttpResponse {
    match StatusCode::from_u16(status) {
        Ok(code) => {
            return HttpResponse::build(code)
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(status),
                    message: Some(message),
                    result: None,
                    metadata,
                    version,
                })
        },
        Err(err) => {
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .json(JsonResponse {
                    status: Some(500),
                    message: Some(format!("Invalid http status code found: {}", err.to_string())),
                    result: None,
                    metadata,
                    version,
                })
        }
    }
    
    
}

pub fn build_success_response(result: Option<Value>, message: Option<String>, version: Option<String>, metadata: Option<Value>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(JsonResponse {
            status: Some(200),
            message,
            result,
            metadata,
            version,
        })
}