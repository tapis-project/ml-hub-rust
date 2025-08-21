use actix_web::HttpResponse;
use clients::ClientError;
use shared::presentation::http::v1::actix_web::helpers::{
    build_error_response as error,
    build_success_response as success
};
use crate::config::VERSION;
use serde_json::Value;

pub fn build_client_error_response(err: ClientError) -> HttpResponse {
    let status_code = err.status_code();
    match err {
        ClientError::Internal { msg, scope: _ } => build_error_response(status_code, msg),
        ClientError::BadRequest { msg, scope: _ } => build_error_response(status_code, msg),
        ClientError::Unauthorized { msg, scope: _ } => build_error_response(status_code, msg),
        ClientError::Forbidden { msg, scope: _ } => build_error_response(status_code, msg),
        ClientError::NotFound { msg, scope: _ } => build_error_response(status_code, msg),
        ClientError::Unavailable(msg) => build_error_response(status_code, msg),
        ClientError::MissingInvalidCredentials(msg) => build_error_response(status_code, msg),
        ClientError::Unimplemented => build_error_response(status_code, "Unimplemented".into()),
    }
}

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