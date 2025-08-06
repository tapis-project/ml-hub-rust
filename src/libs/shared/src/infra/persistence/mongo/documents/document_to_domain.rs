use std::path::PathBuf;

use crate::domain::entities;
use crate::infra::persistence::mongo::documents;
use uuid::Uuid;

impl From<documents::artifact::ArtifactType> for entities::artifact::ArtifactType {
    fn from(value: documents::artifact::ArtifactType) -> Self {
        match value {
            documents::artifact::ArtifactType::Model => entities::artifact::ArtifactType::Model,
            documents::artifact::ArtifactType::Dataset => entities::artifact::ArtifactType::Dataset,
        }
    }
}

impl From<documents::artifact::Artifact> for entities::artifact::Artifact {
    fn from(value: documents::artifact::Artifact) -> Self {
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

impl From<documents::artifact_ingestion::ArtifactIngestion> for entities::artifact_ingestion::ArtifactIngestion {
    fn from(value: documents::artifact_ingestion::ArtifactIngestion) -> Self {
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

impl From<documents::artifact_ingestion::ArtifactIngestionStatus> for entities::artifact_ingestion::ArtifactIngestionStatus {
    fn from(value: documents::artifact_ingestion::ArtifactIngestionStatus) -> Self {
        match value {
            documents::artifact_ingestion::ArtifactIngestionStatus::Submitted => entities::artifact_ingestion::ArtifactIngestionStatus::Submitted,
            documents::artifact_ingestion::ArtifactIngestionStatus::Pending => entities::artifact_ingestion::ArtifactIngestionStatus::Pending,
            documents::artifact_ingestion::ArtifactIngestionStatus::Resubmitted => entities::artifact_ingestion::ArtifactIngestionStatus::Resubmitted,
            documents::artifact_ingestion::ArtifactIngestionStatus::Archived => entities::artifact_ingestion::ArtifactIngestionStatus::Archived,
            documents::artifact_ingestion::ArtifactIngestionStatus::Archiving => entities::artifact_ingestion::ArtifactIngestionStatus::Archiving,
            documents::artifact_ingestion::ArtifactIngestionStatus::Downloaded => entities::artifact_ingestion::ArtifactIngestionStatus::Downloaded,
            documents::artifact_ingestion::ArtifactIngestionStatus::Downloading=> entities::artifact_ingestion::ArtifactIngestionStatus::Downloading,
            documents::artifact_ingestion::ArtifactIngestionStatus::Finished => entities::artifact_ingestion::ArtifactIngestionStatus::Finished,
            documents::artifact_ingestion::ArtifactIngestionStatus::Failed(r) => {
                entities::artifact_ingestion::ArtifactIngestionStatus::Failed(entities::artifact_ingestion::ArtifactIngestionFailureReason::from(r))
            }
        }
    }
}

impl From<documents::artifact_ingestion::ArtifactIngestionFailureReason> for entities::artifact_ingestion::ArtifactIngestionFailureReason {
    fn from(value: documents::artifact_ingestion::ArtifactIngestionFailureReason) -> Self {
        match value {
            documents::artifact_ingestion::ArtifactIngestionFailureReason::FailedToArchive => entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToArchive,
            documents::artifact_ingestion::ArtifactIngestionFailureReason::FailedToDownload => entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToDownload,
            documents::artifact_ingestion::ArtifactIngestionFailureReason::FailedToQueue => entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToQueue,
            documents::artifact_ingestion::ArtifactIngestionFailureReason::Unknown => entities::artifact_ingestion::ArtifactIngestionFailureReason::Unknown,
        }
    }
}