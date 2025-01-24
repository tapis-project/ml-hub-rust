use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Serialize)]
pub struct Response {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub version: Option<String>,
    pub metadata: Option<Value>,
}

pub struct ResponseBuilder {}

impl ResponseBuilder {
    pub fn new() -> Self {
        return ResponseBuilder {};
    }

    pub fn build(&self, success: bool, response: Response) -> Response {
        // Default response field values
        let default_metadata = Value::Object(Map::new());
        let default_version = String::from("unknown");
        let default_result = Value::Null;

        // Default success response field values
        let default_message_success = String::from("success");
        let default_status_success = 200;

        // Default failed response field values
        let default_message_failed = String::from("failed");
        let default_status_failed = 500;

        if success {
            Response {
                status: Some(response.status.unwrap_or(default_status_success)),
                message: Some(response.message.unwrap_or(default_message_success)),
                result: Some(response.result.unwrap_or(default_result)),
                version: Some(response.version.unwrap_or(default_version)),
                metadata: Some(response.metadata.unwrap_or(default_metadata)),
            }
        } else {
            Response {
                status: Some(response.status.unwrap_or(default_status_failed)),
                message: Some(response.message.unwrap_or(default_message_failed)),
                result: Some(default_result),
                version: Some(response.version.unwrap_or(default_version)),
                metadata: Some(response.metadata.unwrap_or(default_metadata)),
            }
        }
    }
}
