use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entities;

#[derive(Serialize, ToSchema)]
pub enum ArtifactPublicationStatus {
    Submitted,
    Pending,
    Extracting,
    Extracted,
    PublishingMetadata,
    PublishedMetadata,
    PublishingArtifact,
    PublishedArtifact,
    Finished,
    Failed
}

#[derive(Serialize, ToSchema)]
pub struct ArtifactPublication  {
    pub id: String,
    pub status: ArtifactPublicationStatus,
    pub artifact_id: String,
    pub target_platform: String,
    pub last_message: Option<String>,
    pub attempts: u8,
    pub created_at: String,
    pub last_modified: String,
}

impl From<entities::artifact_publication::ArtifactPublicationStatus> for ArtifactPublicationStatus {
    fn from(value: entities::artifact_publication::ArtifactPublicationStatus) -> Self {
        match value {
            entities::artifact_publication::ArtifactPublicationStatus::Submitted => ArtifactPublicationStatus::Submitted,
            entities::artifact_publication::ArtifactPublicationStatus::Pending => ArtifactPublicationStatus::Pending,
            entities::artifact_publication::ArtifactPublicationStatus::Extracted => ArtifactPublicationStatus::Extracted,
            entities::artifact_publication::ArtifactPublicationStatus::Extracting => ArtifactPublicationStatus::Extracting,
            entities::artifact_publication::ArtifactPublicationStatus::PublishingArtifact => ArtifactPublicationStatus::PublishingArtifact,
            entities::artifact_publication::ArtifactPublicationStatus::PublishedArtifact => ArtifactPublicationStatus::PublishedArtifact,
            entities::artifact_publication::ArtifactPublicationStatus::PublishingMetadata => ArtifactPublicationStatus::PublishingMetadata,
            entities::artifact_publication::ArtifactPublicationStatus::PublishedMetadata => ArtifactPublicationStatus::PublishedMetadata,
            entities::artifact_publication::ArtifactPublicationStatus::Finished => ArtifactPublicationStatus::Finished,
            entities::artifact_publication::ArtifactPublicationStatus::Failed => ArtifactPublicationStatus::Failed,
        }
    }
}

impl From<entities::artifact_publication::ArtifactPublication> for ArtifactPublication {
    fn from(value: entities::artifact_publication::ArtifactPublication) -> Self {
        ArtifactPublication {
            artifact_id: value.artifact_id.to_string(),
            id: value.id.to_string(),
            created_at: String::from(value.created_at),
            last_modified: String::from(value.last_modified),
            last_message: value.last_message,
            target_platform: value.target_platform,
            attempts: value.attempts,
            status: ArtifactPublicationStatus::from(value.status),
        }
    }
}