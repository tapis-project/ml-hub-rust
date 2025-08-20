use serde::{Deserialize, Serialize};
use mongodb::bson::{oid::ObjectId, DateTime, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactIngestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub artifact_id: Uuid, 
    pub platform: String,
    pub status: ArtifactIngestionStatus,
    pub last_message: Option<String>,
    pub created_at: DateTime,
    pub last_modified: DateTime,
    pub artifact_path: Option<String>,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateArtifactIngestionStatusRequest {
    pub status: ArtifactIngestionStatus,
    pub last_message: Option<String>,
    pub last_modified: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateArtifactIngestionRequest {
    pub status: ArtifactIngestionStatus,
    pub last_message: Option<String>,
    pub last_modified: DateTime,
    pub artifact_path: Option<String>,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ArtifactIngestionStatus {
    Submitted,
    Resubmitted,
    Pending,
    Downloading,
    Downloaded,
    Archiving,
    Archived,
    Finished,
    Failed(Reason),
}

type Status = ArtifactIngestionStatus;

impl From<Status> for String {
    fn from(value: Status) -> Self {
        match value {
            Status::Submitted => "Submitted".into(),
            Status::Resubmitted => "Submitted".into(),
            Status::Pending => "Pending".into(),
            Status::Downloading => "Downloading".into(),
            Status::Downloaded => "Downloaded".into(),
            Status::Archiving => "Archiving".into(),
            Status::Archived => "Archived".into(),
            Status::Finished => "Finished".into(),
            Status::Failed(_) => "Failed".into(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum ArtifactIngestionFailureReason {
    FailedToQueue,
    FailedToDownload,
    FailedToArchive,
    Unknown
}

type Reason = ArtifactIngestionFailureReason;