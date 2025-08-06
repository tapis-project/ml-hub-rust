use std::path::PathBuf;

use crate::domain::entities;
use crate::infra::persistence::mongo::documents;
use uuid::Uuid;

impl From<documents::ArtifactType> for entities::artifact::ArtifactType {
    fn from(value: documents::ArtifactType) -> Self {
        match value {
            documents::ArtifactType::Model => entities::artifact::ArtifactType::Model,
            documents::ArtifactType::Dataset => entities::artifact::ArtifactType::Dataset,
        }
    }
}

impl From<documents::Artifact> for entities::artifact::Artifact {
    fn from(value: documents::Artifact) -> Self {
        let path = match value.path {
            Some(s) =>  Some(PathBuf::from(s)),
            None => None
        };

        Self {
            id: Uuid::from_bytes(value.id.bytes()),
            artifact_type: entities::artifact::ArtifactType::from(value.artifact_type),
            last_modified: entities::timestamp::TimeStamp::from(value.last_modified.to_chrono()),
            created_at: entities::timestamp::TimeStamp::from(value.created_at.to_chrono()),
            path
        }
    }
}

impl From<documents::ArtifactIngestion> for entities::artifact_ingestion::ArtifactIngestion {
    fn from(value: documents::ArtifactIngestion) -> Self {
        let artifact_path = match value.artifact_path {
            Some(s) =>  Some(PathBuf::from(s)),
            None => None
        };

        Self {
            id: Uuid::from_bytes(value.id.bytes()),
            last_modified: entities::timestamp::TimeStamp::from(value.last_modified.to_chrono()),
            created_at: entities::timestamp::TimeStamp::from(value.created_at.to_chrono()),
            artifact_id: Uuid::from_bytes(value.artifact_id.bytes()),
            artifact_path,
            last_message: value.last_message,
            platform: value.platform,
            status: entities::artifact_ingestion::ArtifactIngestionStatus::from(value.status),
            webhook_url: value.webhook_url
        }
    }
}

impl From<documents::ArtifactIngestionStatus> for entities::artifact_ingestion::ArtifactIngestionStatus {
    fn from(value: documents::ArtifactIngestionStatus) -> Self {
        match value {
            documents::ArtifactIngestionStatus::Submitted => entities::artifact_ingestion::ArtifactIngestionStatus::Submitted,
            documents::ArtifactIngestionStatus::Pending => entities::artifact_ingestion::ArtifactIngestionStatus::Pending,
            documents::ArtifactIngestionStatus::Resubmitted => entities::artifact_ingestion::ArtifactIngestionStatus::Resubmitted,
            documents::ArtifactIngestionStatus::Archived => entities::artifact_ingestion::ArtifactIngestionStatus::Archived,
            documents::ArtifactIngestionStatus::Archiving => entities::artifact_ingestion::ArtifactIngestionStatus::Archiving,
            documents::ArtifactIngestionStatus::Downloaded => entities::artifact_ingestion::ArtifactIngestionStatus::Downloaded,
            documents::ArtifactIngestionStatus::Downloading=> entities::artifact_ingestion::ArtifactIngestionStatus::Downloading,
            documents::ArtifactIngestionStatus::Finished => entities::artifact_ingestion::ArtifactIngestionStatus::Finished,
            documents::ArtifactIngestionStatus::Failed(r) => {
                entities::artifact_ingestion::ArtifactIngestionStatus::Failed(entities::artifact_ingestion::ArtifactIngestionFailureReason::from(r))
            }
        }
    }
}

impl From<documents::ArtifactIngestionFailureReason> for entities::artifact_ingestion::ArtifactIngestionFailureReason {
    fn from(value: documents::ArtifactIngestionFailureReason) -> Self {
        match value {
            documents::ArtifactIngestionFailureReason::FailedToArchive => entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToArchive,
            documents::ArtifactIngestionFailureReason::FailedToDownload => entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToDownload,
            documents::ArtifactIngestionFailureReason::FailedToQueue => entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToQueue,
            documents::ArtifactIngestionFailureReason::Unknown => entities::artifact_ingestion::ArtifactIngestionFailureReason::Unknown,
        }
    }
}