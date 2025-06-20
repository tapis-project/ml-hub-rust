use crate::common::domain::entities;
use crate::common::infra::persistence::mongo::documents;
use mongodb::bson::{Uuid, DateTime};


impl From<entities::Artifact> for documents::Artifact {
    fn from(value: entities::Artifact) -> Self {
        let path = match value.path {
            Some(p) =>  p.to_str().map(|s| s.to_string()),
            None => None
        };

        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            created_at: DateTime::from_chrono(value.created_at.into_inner()),
            path
        }
    }
}

impl From<entities::ArtifactIngestion> for documents::ArtifactIngestion {
    fn from(value: entities::ArtifactIngestion) -> Self {
        
        let artifact_path = match value.artifact_path {
            Some(p) => p.to_str().map(|s| s.to_string()),
            None => None
        };

        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            created_at: DateTime::from_chrono(value.created_at.into_inner()),
            artifact_id: Uuid::from_bytes(value.artifact_id.into_bytes()),
            artifact_path,
            last_message: value.last_message,
            platform: value.platform,
            status: documents::ArtifactIngestionStatus::from(value.status),
            webhook_url: value.webhook_url
        }
    }
}

impl From<entities::ArtifactIngestion> for documents::UpdateArtifactIngestionStatusRequest {
    fn from(value: entities::ArtifactIngestion) -> Self {
        Self {
            id: Uuid::from_bytes(value.id.into_bytes()),
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            last_message: value.last_message,
            status: documents::ArtifactIngestionStatus::from(value.status),
        }
    }
}

impl From<entities::ArtifactIngestionStatus> for documents::ArtifactIngestionStatus {
    fn from(value: entities::ArtifactIngestionStatus) -> Self {
        match value {
            entities::ArtifactIngestionStatus::Submitted => documents::ArtifactIngestionStatus::Submitted,
            entities::ArtifactIngestionStatus::Pending => documents::ArtifactIngestionStatus::Pending,
            entities::ArtifactIngestionStatus::Resubmitted => documents::ArtifactIngestionStatus::Resubmitted,
            entities::ArtifactIngestionStatus::Archived => documents::ArtifactIngestionStatus::Archived,
            entities::ArtifactIngestionStatus::Archiving => documents::ArtifactIngestionStatus::Archiving,
            entities::ArtifactIngestionStatus::Downloaded => documents::ArtifactIngestionStatus::Downloaded,
            entities::ArtifactIngestionStatus::Downloading=> documents::ArtifactIngestionStatus::Downloading,
            entities::ArtifactIngestionStatus::Finished => documents::ArtifactIngestionStatus::Finished,
            entities::ArtifactIngestionStatus::Failed(r) => {
                documents::ArtifactIngestionStatus::Failed(documents::ArtifactIngestionFailureReason::from(r))
            }
        }
    }
}

impl From<entities::ArtifactIngestionFailureReason> for documents::ArtifactIngestionFailureReason {
    fn from(value: entities::ArtifactIngestionFailureReason) -> Self {
        match value {
            entities::ArtifactIngestionFailureReason::FailedToArchive => documents::ArtifactIngestionFailureReason::FailedToArchive,
            entities::ArtifactIngestionFailureReason::FailedToDownload => documents::ArtifactIngestionFailureReason::FailedToDownload,
            entities::ArtifactIngestionFailureReason::FailedToQueue => documents::ArtifactIngestionFailureReason::FailedToQueue,
            entities::ArtifactIngestionFailureReason::Unknown => documents::ArtifactIngestionFailureReason::Unknown,
        }
    }
}