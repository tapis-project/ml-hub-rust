mod domain_to_dto;

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct JsonResponse {
    pub status: Option<u16>,
    pub message: Option<String>,
    pub result: Option<Value>,
    pub metadata: Option<Value>,
    pub version: Option<String>
}

#[derive(Serialize)]
pub enum ArtifactIngestionStatus {
    Submitted,
    Resubmitted,
    Pending,
    Downloading,
    Downloaded,
    Archiving,
    Archived,
    Finished,
    Failed,
}

#[derive(Serialize)]
pub struct ArtifactIngestion {
    pub id: String,
    pub artifact_id: String, 
    pub platform: String,
    pub status: ArtifactIngestionStatus,
    pub last_message: Option<String>,
    pub created_at: String,
    pub last_modified: String,
    pub webhook_url: Option<String>,
}