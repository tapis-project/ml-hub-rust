mod output_to_response;

use serde::Serialize;
use serde_json::Value;

pub mod models;
pub mod agent_records;
pub mod agents;
pub mod deployment;
pub mod operators;
pub mod visibility;
pub mod tasks;
pub mod artifacts;
pub mod platform_details;


#[derive(Serialize)]
pub struct JsonResponse {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub metadata: Option<Value>,
    pub version: Option<String>
}
