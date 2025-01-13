use serde_json::Value;
use serde::Serialize;
use crate::wrappers::{JsonObject, JsonValue};

#[derive(Serialize)]
pub struct OkResponse {
    pub status: u16,
    pub message: String,
    pub result: JsonValue,
    pub version: String,
    pub metadata: JsonObject,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
    pub result: JsonValue,
    pub version: String,
    pub metadata: JsonObject,
}

#[derive(Serialize)]
pub enum ResponseType {
    Ok(OkResponse),
    Err(ErrorResponse),
}

#[derive(Serialize)]
pub struct Response {
    pub success: bool,
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<JsonValue>,
    pub version: Option<String>,
    pub metadata: Option<JsonObject>,
}

pub struct ResponseFactory {}

impl ResponseFactory {
    pub fn new() -> Self {
        return ResponseFactory {};
    }

    pub fn build(&self, response: Response) -> ResponseType {
        // Default response field values
        let default_metadata = JsonObject::new();
        let default_version = String::from("unknown");
        let default_result = JsonValue::new(Value::Null);

        // Default success response field values
        let default_message_success = String::from("success");
        let default_status_success = 200;

        // Default failed response field values
        let default_message_failed = String::from("failed");
        let default_status_failed = 500;

        if response.success {
            ResponseType::Ok(OkResponse {
                status: response.status.unwrap_or(default_status_success),
                message: response.message.unwrap_or(default_message_success),
                result: response.result.unwrap_or(default_result),
                version: response.version.unwrap_or(default_version),
                metadata: response.metadata.unwrap_or(default_metadata),
            })
        } else {
            ResponseType::Err(ErrorResponse {
                status: response.status.unwrap_or(default_status_failed),
                message: response.message.unwrap_or(default_message_failed),
                result: default_result,
                version: response.version.unwrap_or(default_version),
                metadata: response.metadata.unwrap_or(default_metadata),
            })
        }
    }
}