use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct Response {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub metadata: Option<Value>,
    pub version: Option<String>
}