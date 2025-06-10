use std::path::PathBuf;

use crate::common::domain::entities;
use crate::common::infra::persistence::mongo::documents;
use uuid::Uuid;

impl From<documents::Artifact> for entities::Artifact {
    fn from(value: documents::Artifact) -> Self {
        let path = match value.path {
            Some(s) =>  Some(PathBuf::from(s)),
            None => None
        };

        Self {
            id: Uuid::from_bytes(value.id.bytes()),
            last_modified: entities::TimeStamp::from(value.last_modified),
            created_at: entities::TimeStamp::from(value.created_at),
            path
        }
    }
}

impl From<documents::ArtifactIngestion> for entities::ArtifactIngestion {
    fn from(value: documents::ArtifactIngestion) -> Self {
        let artifact_path = match value.artifact_path {
            Some(s) =>  Some(PathBuf::from(s)),
            None => None
        };

        Self {
            id: Uuid::from_bytes(value.id.bytes()),
            last_modified: entities::TimeStamp::from(value.last_modified),
            created_at: entities::TimeStamp::from(value.created_at),
            artifact_id: Uuid::from_bytes(value.artifact_id.bytes()),
            artifact_path,
            last_message: value.last_message,
            platform: value.platform,
            status: entities::ArtifactIngestionStatus::from(value.status),
            webhook_url: value.webhook_url
        }
    }
}

impl From<documents::ArtifactIngestionStatus> for entities::ArtifactIngestionStatus {
    fn from(value: documents::ArtifactIngestionStatus) -> Self {
        match value {
            documents::ArtifactIngestionStatus::Submitted => entities::ArtifactIngestionStatus::Submitted,
            documents::ArtifactIngestionStatus::Pending => entities::ArtifactIngestionStatus::Pending,
            documents::ArtifactIngestionStatus::Resubmitted => entities::ArtifactIngestionStatus::Resubmitted,
            documents::ArtifactIngestionStatus::Archived => entities::ArtifactIngestionStatus::Archived,
            documents::ArtifactIngestionStatus::Archiving => entities::ArtifactIngestionStatus::Archiving,
            documents::ArtifactIngestionStatus::Downloaded => entities::ArtifactIngestionStatus::Downloaded,
            documents::ArtifactIngestionStatus::Downloading=> entities::ArtifactIngestionStatus::Downloading,
            documents::ArtifactIngestionStatus::Finished => entities::ArtifactIngestionStatus::Finished,
            documents::ArtifactIngestionStatus::Failed(r) => {
                entities::ArtifactIngestionStatus::Failed(entities::ArtifactIngestionFailureReason::from(r))
            }
        }
    }
}

impl From<documents::ArtifactIngestionFailureReason> for entities::ArtifactIngestionFailureReason {
    fn from(value: documents::ArtifactIngestionFailureReason) -> Self {
        match value {
            documents::ArtifactIngestionFailureReason::FailedToArchive => entities::ArtifactIngestionFailureReason::FailedToArchive,
            documents::ArtifactIngestionFailureReason::FailedToDownload => entities::ArtifactIngestionFailureReason::FailedToDownload,
            documents::ArtifactIngestionFailureReason::FailedToQueue => entities::ArtifactIngestionFailureReason::FailedToQueue,
            documents::ArtifactIngestionFailureReason::Unknown => entities::ArtifactIngestionFailureReason::Unknown,
        }
    }
}