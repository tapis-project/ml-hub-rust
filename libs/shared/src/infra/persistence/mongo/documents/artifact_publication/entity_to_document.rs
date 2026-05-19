use crate::domain::entities::artifact_publication as entities;
use crate::domain::entities::artifact::ArtifactType;
use crate::infra::persistence::mongo::documents::artifact_publication as documents;
use crate::infra::common::mongo::ToBsonDateTime;
use mongodb::bson::Uuid;

impl From<ArtifactType> for documents::ArtifactType {
    fn from(value: ArtifactType) -> Self {
        match value {
            ArtifactType::Model => documents::ArtifactType::Model,
            ArtifactType::Dataset => documents::ArtifactType::Dataset
        }
    }
}

impl From<&entities::ArtifactPublication> for documents::ArtifactPublication {
    fn from(value: &entities::ArtifactPublication) -> Self {
        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            artifact_id: Uuid::from_bytes(value.artifact_id.into_bytes()),
            artifact_type: documents::ArtifactType::from(value.artifact_type.clone()),
            attempts: value.attempts,
            last_message: value.last_message.clone(),
            target_platform: value.target_platform.clone(),
            created_at: value.created_at.to_bson(),
            last_modified: value.last_modified.to_bson(),
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
            entities::ArtifactPublicationStatus::Failed => documents::ArtifactPublicationStatus::Failed,
        }
    }
}

impl From<&entities::ArtifactPublication> for documents::UpdateArtifactPublicationStatusRequest {
    fn from(value: &entities::ArtifactPublication) -> Self {
        Self {
            last_modified: value.last_modified.to_bson(),
            last_message: value.last_message.clone(),
            status: documents::ArtifactPublicationStatus::from(value.status.clone()),
        }
    }
}