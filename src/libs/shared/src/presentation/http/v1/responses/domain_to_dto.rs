use crate::domain::entities;
use crate::presentation::http::v1::responses;

impl From<entities::artifact_ingestion::ArtifactIngestionStatus> for responses::ArtifactIngestionStatus {
    fn from(value: entities::artifact_ingestion::ArtifactIngestionStatus) -> Self {
        match value {
            entities::artifact_ingestion::ArtifactIngestionStatus::Submitted => responses::ArtifactIngestionStatus::Submitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Resubmitted => responses::ArtifactIngestionStatus::Resubmitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archived => responses::ArtifactIngestionStatus::Archived,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archiving => responses::ArtifactIngestionStatus::Archiving,
            entities::artifact_ingestion::ArtifactIngestionStatus::Pending => responses::ArtifactIngestionStatus::Pending,
            entities::artifact_ingestion::ArtifactIngestionStatus::Finished => responses::ArtifactIngestionStatus::Finished,
            entities::artifact_ingestion::ArtifactIngestionStatus::Failed(_) => responses::ArtifactIngestionStatus::Failed,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloaded => responses::ArtifactIngestionStatus::Downloaded,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloading => responses::ArtifactIngestionStatus::Downloading,
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for responses::ArtifactIngestion {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        responses::ArtifactIngestion {
            artifact_id: value.artifact_id.to_string(),
            id: value.id.to_string(),
            created_at: String::from(value.created_at),
            last_modified: String::from(value.last_modified),
            last_message: value.last_message,
            platform: value.platform,
            status: responses::ArtifactIngestionStatus::from(value.status),
            webhook_url: value.webhook_url
        }
    }
}