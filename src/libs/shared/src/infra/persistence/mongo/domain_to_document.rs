use crate::{application::errors::ApplicationError, domain::entities};
use crate::infra::persistence::mongo::documents;
use mongodb::bson::{Uuid, DateTime};


impl From<entities::artifact::ArtifactType> for documents::ArtifactType {
    fn from(value: entities::artifact::ArtifactType) -> Self {
        match value {
            entities::artifact::ArtifactType::Model => documents::ArtifactType::Model,
            entities::artifact::ArtifactType::Dataset => documents::ArtifactType::Dataset,
        }
    }
}

impl From<entities::artifact::Artifact> for documents::Artifact {
    fn from(value: entities::artifact::Artifact) -> Self {
        let path = match value.path {
            Some(p) =>  p.to_str().map(|s| s.to_string()),
            None => None
        };

        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            artifact_type: documents::ArtifactType::from(value.artifact_type),
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            created_at: DateTime::from_chrono(value.created_at.into_inner()),
            path
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for documents::ArtifactIngestion {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        
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

impl From<entities::artifact_ingestion::ArtifactIngestion> for documents::UpdateArtifactIngestionStatusRequest {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        Self {
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            last_message: value.last_message,
            status: documents::ArtifactIngestionStatus::from(value.status),
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for documents::UpdateArtifactIngestionRequest {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        let artifact_path = match value.artifact_path {
            Some(p) => p.to_str().map(|s| s.to_string()),
            None => None
        };

        Self {
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            last_message: value.last_message,
            status: documents::ArtifactIngestionStatus::from(value.status),
            artifact_path,
            webhook_url: value.webhook_url,
        }
    }
}

impl TryFrom<entities::artifact::Artifact> for documents::UpdateArtifactPathRequest {
    type Error = ApplicationError;

    fn try_from(value: entities::artifact::Artifact) -> Result<Self, Self::Error> {
        let path = match value.path {
            Some(p) => p,
            None => return Err(ApplicationError::ConvesionError("Path".into()))
        };

        Ok(Self {
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            path: path.to_string_lossy().into_owned()
        })
    }
}

impl TryFrom<entities::artifact::Artifact> for documents::UpdateArtifactRequest {
    type Error = ApplicationError;

    fn try_from(value: entities::artifact::Artifact) -> Result<Self, Self::Error> {
        let path = match value.path {
            Some(p) => p,
            None => return Err(ApplicationError::ConvesionError("Path".into()))
        };

        Ok(Self {
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            path: path.to_string_lossy().into_owned()
        })
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestionStatus> for documents::ArtifactIngestionStatus {
    fn from(value: entities::artifact_ingestion::ArtifactIngestionStatus) -> Self {
        match value {
            entities::artifact_ingestion::ArtifactIngestionStatus::Submitted => documents::ArtifactIngestionStatus::Submitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Pending => documents::ArtifactIngestionStatus::Pending,
            entities::artifact_ingestion::ArtifactIngestionStatus::Resubmitted => documents::ArtifactIngestionStatus::Resubmitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archived => documents::ArtifactIngestionStatus::Archived,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archiving => documents::ArtifactIngestionStatus::Archiving,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloaded => documents::ArtifactIngestionStatus::Downloaded,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloading=> documents::ArtifactIngestionStatus::Downloading,
            entities::artifact_ingestion::ArtifactIngestionStatus::Finished => documents::ArtifactIngestionStatus::Finished,
            entities::artifact_ingestion::ArtifactIngestionStatus::Failed(r) => {
                documents::ArtifactIngestionStatus::Failed(documents::ArtifactIngestionFailureReason::from(r))
            }
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestionFailureReason> for documents::ArtifactIngestionFailureReason {
    fn from(value: entities::artifact_ingestion::ArtifactIngestionFailureReason) -> Self {
        match value {
            entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToArchive => documents::ArtifactIngestionFailureReason::FailedToArchive,
            entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToDownload => documents::ArtifactIngestionFailureReason::FailedToDownload,
            entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToQueue => documents::ArtifactIngestionFailureReason::FailedToQueue,
            entities::artifact_ingestion::ArtifactIngestionFailureReason::Unknown => documents::ArtifactIngestionFailureReason::Unknown,
        }
    }
}