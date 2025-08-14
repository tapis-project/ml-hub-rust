use crate::domain::entities::artifact_publication as entities;
use crate::domain::entities::timestamp::TimeStamp;
use crate::infra::persistence::mongo::documents::artifact_publication as documents;
use uuid::Uuid;

impl From<&documents::ArtifactPublication> for entities::ArtifactPublication {
    fn from(value: &documents::ArtifactPublication) -> Self {
        Self {
            id: Uuid::from_bytes(value.id.bytes()),
            artifact_id: Uuid::from_bytes(value.artifact_id.bytes()),
            attempts: value.attempts,
            last_message: value.last_message.clone(),
            target_platform: value.target_platform.clone(),
            created_at: TimeStamp::from(value.created_at.to_chrono()),
            last_modified: TimeStamp::from(value.last_modified.to_chrono()),
            status: entities::ArtifactPublicationStatus::from(value.status.clone())
        }
    }
}

impl From<documents::ArtifactPublicationStatus> for entities::ArtifactPublicationStatus {
    fn from(value: documents::ArtifactPublicationStatus) -> Self {
        match value {
            documents::ArtifactPublicationStatus::Submitted => entities::ArtifactPublicationStatus::Submitted,
            documents::ArtifactPublicationStatus::Pending => entities::ArtifactPublicationStatus::Pending,
            documents::ArtifactPublicationStatus::Extracted => entities::ArtifactPublicationStatus::Extracted,
            documents::ArtifactPublicationStatus::Extracting => entities::ArtifactPublicationStatus::Extracting,
            documents::ArtifactPublicationStatus::PublishingMetadata => entities::ArtifactPublicationStatus::PublishingMetadata,
            documents::ArtifactPublicationStatus::PublishedMetadata => entities::ArtifactPublicationStatus::PublishedMetadata,
            documents::ArtifactPublicationStatus::PublishingArtifact => entities::ArtifactPublicationStatus::PublishingArtifact,
            documents::ArtifactPublicationStatus::PublishedArtifact => entities::ArtifactPublicationStatus::PublishedArtifact,
            documents::ArtifactPublicationStatus::Finished => entities::ArtifactPublicationStatus::Finished,
            documents::ArtifactPublicationStatus::Failed(r) => {
                entities::ArtifactPublicationStatus::Failed(entities::ArtifactPublicationFailureReason::from(r))
            },
        }
    }
}

impl From<documents::ArtifactPublicationFailureReason> for entities::ArtifactPublicationFailureReason {
    fn from(value: documents::ArtifactPublicationFailureReason) -> Self {
        match value {
            documents::ArtifactPublicationFailureReason::FailedToQueue(s) => entities::ArtifactPublicationFailureReason::FailedToQueue(s),
            documents::ArtifactPublicationFailureReason::FailedToExtract(s) => entities::ArtifactPublicationFailureReason::FailedToExtract(s),
            documents::ArtifactPublicationFailureReason::FailedToPublishArtifact(s) => entities::ArtifactPublicationFailureReason::FailedToPublishArtifact(s),
            documents::ArtifactPublicationFailureReason::FailedToPublishMetadata(s) => entities::ArtifactPublicationFailureReason::FailedToPublishMetadata(s),
            documents::ArtifactPublicationFailureReason::InternalError(s) => entities::ArtifactPublicationFailureReason::InternalError(s),
            documents::ArtifactPublicationFailureReason::PlatformError(s) => entities::ArtifactPublicationFailureReason::PlatformError(s),
        }
    }
}