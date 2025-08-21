use crate::domain::entities::artifact_publication as entities;
use crate::infra::persistence::mongo::documents::artifact_publication as documents;
use mongodb::bson::{Uuid, DateTime};

impl From<&entities::ArtifactPublication> for documents::ArtifactPublication {
    fn from(value: &entities::ArtifactPublication) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            artifact_id: Uuid::from_bytes(value.artifact_id.into_bytes()),
            attempts: value.attempts,
            last_message: value.last_message.clone(),
            target_platform: value.target_platform.clone(),
            created_at: DateTime::from_chrono(value.created_at.into_inner()),
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            status: documents::ArtifactPublicationStatus::from(value.status.clone())
        }
    }
}

impl From<entities::ArtifactPublicationStatus> for documents::ArtifactPublicationStatus {
    fn from(value: entities::ArtifactPublicationStatus) -> Self {
        match value {
            entities::ArtifactPublicationStatus::Submitted => documents::ArtifactPublicationStatus::Submitted,
            entities::ArtifactPublicationStatus::Pending => documents::ArtifactPublicationStatus::Pending,
            entities::ArtifactPublicationStatus::Extracted => documents::ArtifactPublicationStatus::Extracted,
            entities::ArtifactPublicationStatus::Extracting => documents::ArtifactPublicationStatus::Extracting,
            entities::ArtifactPublicationStatus::PublishingMetadata => documents::ArtifactPublicationStatus::PublishingMetadata,
            entities::ArtifactPublicationStatus::PublishedMetadata => documents::ArtifactPublicationStatus::PublishedMetadata,
            entities::ArtifactPublicationStatus::PublishingArtifact => documents::ArtifactPublicationStatus::PublishingArtifact,
            entities::ArtifactPublicationStatus::PublishedArtifact => documents::ArtifactPublicationStatus::PublishedArtifact,
            entities::ArtifactPublicationStatus::Finished => documents::ArtifactPublicationStatus::Finished,
            entities::ArtifactPublicationStatus::Failed(r) => {
                documents::ArtifactPublicationStatus::Failed(documents::ArtifactPublicationFailureReason::from(r))
            },
        }
    }
}

impl From<entities::ArtifactPublicationFailureReason> for documents::ArtifactPublicationFailureReason {
    fn from(value: entities::ArtifactPublicationFailureReason) -> Self {
        match value {
            entities::ArtifactPublicationFailureReason::FailedToQueue(s) => documents::ArtifactPublicationFailureReason::FailedToQueue(s),
            entities::ArtifactPublicationFailureReason::FailedToExtract(s) => documents::ArtifactPublicationFailureReason::FailedToExtract(s),
            entities::ArtifactPublicationFailureReason::FailedToPublishArtifact(s) => documents::ArtifactPublicationFailureReason::FailedToPublishArtifact(s),
            entities::ArtifactPublicationFailureReason::FailedToPublishMetadata(s) => documents::ArtifactPublicationFailureReason::FailedToPublishMetadata(s),
            entities::ArtifactPublicationFailureReason::InternalError(s) => documents::ArtifactPublicationFailureReason::InternalError(s),
            entities::ArtifactPublicationFailureReason::PlatformError(s) => documents::ArtifactPublicationFailureReason::PlatformError(s),
        }
    }
}

impl From<&entities::ArtifactPublication> for documents::UpdateArtifactPublicationStatusRequest {
    fn from(value: &entities::ArtifactPublication) -> Self {
        Self {
            last_modified: DateTime::from_chrono(value.last_modified.into_inner()),
            last_message: value.last_message.clone(),
            status: documents::ArtifactPublicationStatus::from(value.status.clone()),
        }
    }
}