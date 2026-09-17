mod output_to_response;

use serde::Serialize;
use serde_json::Value;

pub mod agent_records;
pub mod agents;
pub mod artifacts;
pub mod datasets;
pub mod deployment;
pub mod endpoints;
pub mod hpc_clusters;
pub mod models;
pub mod operators;
pub mod platform_details;
pub mod tasks;
pub mod visibility;

#[derive(Serialize)]
pub struct JsonResponse {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub metadata: Option<Value>,
    pub version: Option<String>,
}
