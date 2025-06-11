use crate::common::domain::entities;
use crate::common::presentation::http::v1::responses;

impl From<entities::ArtifactIngestionStatus> for responses::ArtifactIngestionStatus {
    fn from(value: entities::ArtifactIngestionStatus) -> Self {
        match value {
            entities::ArtifactIngestionStatus::Submitted => responses::ArtifactIngestionStatus::Submitted,
            entities::ArtifactIngestionStatus::Resubmitted => responses::ArtifactIngestionStatus::Resubmitted,
            entities::ArtifactIngestionStatus::Archived => responses::ArtifactIngestionStatus::Archived,
            entities::ArtifactIngestionStatus::Archiving => responses::ArtifactIngestionStatus::Archiving,
            entities::ArtifactIngestionStatus::Pending => responses::ArtifactIngestionStatus::Pending,
            entities::ArtifactIngestionStatus::Finished => responses::ArtifactIngestionStatus::Finished,
            entities::ArtifactIngestionStatus::Failed(_) => responses::ArtifactIngestionStatus::Failed,
            entities::ArtifactIngestionStatus::Downloaded => responses::ArtifactIngestionStatus::Downloaded,
            entities::ArtifactIngestionStatus::Downloading => responses::ArtifactIngestionStatus::Downloading,
        }
    }
}

impl From<entities::ArtifactIngestion> for responses::ArtifactIngestion {
    fn from(value: entities::ArtifactIngestion) -> Self {
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