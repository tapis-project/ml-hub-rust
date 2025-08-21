use crate::{application::errors::ApplicationError, domain::entities};
use crate::infra::persistence::mongo::documents;
use mongodb::bson::{Uuid, DateTime};


impl From<entities::artifact::ArtifactType> for documents::artifact::ArtifactType {
    fn from(value: entities::artifact::ArtifactType) -> Self {
        match value {
            entities::artifact::ArtifactType::Model => documents::artifact::ArtifactType::Model,
            entities::artifact::ArtifactType::Dataset => documents::artifact::ArtifactType::Dataset,
        }
    }
}

impl From<entities::artifact::Artifact> for documents::artifact::Artifact {
    fn from(value: entities::artifact::Artifact) -> Self {
        let path = match value.path {
            Some(p) =>  p.to_str().map(|s| s.to_string()),
            None => None
        };

        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            artifact_type: documents::artifact::ArtifactType::from(value.artifact_type),
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            created_at: DateTime::from_chrono(value.created_at.into_inner()),
            path
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for documents::artifact_ingestion::ArtifactIngestion {
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
            status: documents::artifact_ingestion::ArtifactIngestionStatus::from(value.status),
            webhook_url: value.webhook_url
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for documents::artifact_ingestion::UpdateArtifactIngestionStatusRequest {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        Self {
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            last_message: value.last_message,
            status: documents::artifact_ingestion::ArtifactIngestionStatus::from(value.status),
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for documents::artifact_ingestion::UpdateArtifactIngestionRequest {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        let artifact_path = match value.artifact_path {
            Some(p) => p.to_str().map(|s| s.to_string()),
            None => None
        };

        Self {
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            last_message: value.last_message,
            status: documents::artifact_ingestion::ArtifactIngestionStatus::from(value.status),
            artifact_path,
            webhook_url: value.webhook_url,
        }
    }
}

impl TryFrom<entities::artifact::Artifact> for documents::artifact::UpdateArtifactPathRequest {
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

impl TryFrom<entities::artifact::Artifact> for documents::artifact::UpdateArtifactRequest {
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

impl From<entities::artifact_ingestion::ArtifactIngestionStatus> for documents::artifact_ingestion::ArtifactIngestionStatus {
    fn from(value: entities::artifact_ingestion::ArtifactIngestionStatus) -> Self {
        match value {
            entities::artifact_ingestion::ArtifactIngestionStatus::Submitted => documents::artifact_ingestion::ArtifactIngestionStatus::Submitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Pending => documents::artifact_ingestion::ArtifactIngestionStatus::Pending,
            entities::artifact_ingestion::ArtifactIngestionStatus::Resubmitted => documents::artifact_ingestion::ArtifactIngestionStatus::Resubmitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archived => documents::artifact_ingestion::ArtifactIngestionStatus::Archived,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archiving => documents::artifact_ingestion::ArtifactIngestionStatus::Archiving,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloaded => documents::artifact_ingestion::ArtifactIngestionStatus::Downloaded,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloading=> documents::artifact_ingestion::ArtifactIngestionStatus::Downloading,
            entities::artifact_ingestion::ArtifactIngestionStatus::Finished => documents::artifact_ingestion::ArtifactIngestionStatus::Finished,
            entities::artifact_ingestion::ArtifactIngestionStatus::Failed(r) => {
                documents::artifact_ingestion::ArtifactIngestionStatus::Failed(documents::artifact_ingestion::ArtifactIngestionFailureReason::from(r))
            }
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestionFailureReason> for documents::artifact_ingestion::ArtifactIngestionFailureReason {
    fn from(value: entities::artifact_ingestion::ArtifactIngestionFailureReason) -> Self {
        match value {
            entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToArchive => documents::artifact_ingestion::ArtifactIngestionFailureReason::FailedToArchive,
            entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToDownload => documents::artifact_ingestion::ArtifactIngestionFailureReason::FailedToDownload,
            entities::artifact_ingestion::ArtifactIngestionFailureReason::FailedToQueue => documents::artifact_ingestion::ArtifactIngestionFailureReason::FailedToQueue,
            entities::artifact_ingestion::ArtifactIngestionFailureReason::Unknown => documents::artifact_ingestion::ArtifactIngestionFailureReason::Unknown,
        }
    }
}