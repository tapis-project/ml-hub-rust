use crate::domain::entities;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
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

#[derive(Serialize, ToSchema)]
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

impl From<entities::artifact_ingestion::ArtifactIngestionStatus> for ArtifactIngestionStatus {
    fn from(value: entities::artifact_ingestion::ArtifactIngestionStatus) -> Self {
        match value {
            entities::artifact_ingestion::ArtifactIngestionStatus::Submitted => ArtifactIngestionStatus::Submitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Resubmitted => ArtifactIngestionStatus::Resubmitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archived => ArtifactIngestionStatus::Archived,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archiving => ArtifactIngestionStatus::Archiving,
            entities::artifact_ingestion::ArtifactIngestionStatus::Pending => ArtifactIngestionStatus::Pending,
            entities::artifact_ingestion::ArtifactIngestionStatus::Finished => ArtifactIngestionStatus::Finished,
            entities::artifact_ingestion::ArtifactIngestionStatus::Failed => ArtifactIngestionStatus::Failed,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloaded => ArtifactIngestionStatus::Downloaded,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloading => ArtifactIngestionStatus::Downloading,
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for ArtifactIngestion {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        ArtifactIngestion {
            artifact_id: value.artifact_id.to_string(),
            id: value.id.to_string(),
            created_at: String::from(value.created_at),
            last_modified: String::from(value.last_modified),
            last_message: value.last_message,
            platform: value.platform,
            status: ArtifactIngestionStatus::from(value.status),
            webhook_url: value.webhook_url
        }
    }
}