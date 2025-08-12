use crate::domain::entities;
use crate::presentation::http::v1::responses;

impl From<entities::artifact_ingestion::ArtifactIngestionStatus> for responses::ArtifactIngestionStatus {
    fn from(value: entities::artifact_ingestion::ArtifactIngestionStatus) -> Self {
        match value {
            entities::artifact_ingestion::ArtifactIngestionStatus::Submitted => responses::ArtifactIngestionStatus::Submitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Resubmitted => responses::ArtifactIngestionStatus::Resubmitted,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archived => responses::ArtifactIngestionStatus::Archived,
            entities::artifact_ingestion::ArtifactIngestionStatus::Archiving => responses::ArtifactIngestionStatus::Archiving,
            entities::artifact_ingestion::ArtifactIngestionStatus::Pending => responses::ArtifactIngestionStatus::Pending,
            entities::artifact_ingestion::ArtifactIngestionStatus::Finished => responses::ArtifactIngestionStatus::Finished,
            entities::artifact_ingestion::ArtifactIngestionStatus::Failed(_) => responses::ArtifactIngestionStatus::Failed,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloaded => responses::ArtifactIngestionStatus::Downloaded,
            entities::artifact_ingestion::ArtifactIngestionStatus::Downloading => responses::ArtifactIngestionStatus::Downloading,
        }
    }
}

impl From<entities::artifact_ingestion::ArtifactIngestion> for responses::ArtifactIngestion {
    fn from(value: entities::artifact_ingestion::ArtifactIngestion) -> Self {
        responses::ArtifactIngestion {
            artifact_id: value.artifact_id.to_string(),
            id: value.id.to_string(),
            created_at: String::from(value.created_at),
            last_modified: String::from(value.last_modified),
            last_message: value.last_message,
            platform: value.platform,
            status: responses::ArtifactIngestionStatus::from(value.status),
            webhook_url: value.webhook_url
        }
    }
}

impl From<entities::artifact_publication::ArtifactPublicationStatus> for responses::ArtifactPublicationStatus {
    fn from(value: entities::artifact_publication::ArtifactPublicationStatus) -> Self {
        match value {
            entities::artifact_publication::ArtifactPublicationStatus::Submitted => responses::ArtifactPublicationStatus::Submitted,
            entities::artifact_publication::ArtifactPublicationStatus::Pending => responses::ArtifactPublicationStatus::Pending,
            entities::artifact_publication::ArtifactPublicationStatus::Extracted => responses::ArtifactPublicationStatus::Extracted,
            entities::artifact_publication::ArtifactPublicationStatus::Extracting => responses::ArtifactPublicationStatus::Extracting,
            entities::artifact_publication::ArtifactPublicationStatus::PublishingArtifact => responses::ArtifactPublicationStatus::PublishingArtifact,
            entities::artifact_publication::ArtifactPublicationStatus::PublishedArtifact => responses::ArtifactPublicationStatus::PublishedArtifact,
            entities::artifact_publication::ArtifactPublicationStatus::PublishingMetadata => responses::ArtifactPublicationStatus::PublishingMetadata,
            entities::artifact_publication::ArtifactPublicationStatus::PublishedMetadata => responses::ArtifactPublicationStatus::PublishedMetadata,
            entities::artifact_publication::ArtifactPublicationStatus::Finished => responses::ArtifactPublicationStatus::Finished,
            entities::artifact_publication::ArtifactPublicationStatus::Failed(_) => responses::ArtifactPublicationStatus::Failed,
        }
    }
}

impl From<entities::artifact_publication::ArtifactPublication> for responses::ArtifactPublication {
    fn from(value: entities::artifact_publication::ArtifactPublication) -> Self {
        responses::ArtifactPublication {
            artifact_id: value.artifact_id.to_string(),
            id: value.id.to_string(),
            created_at: String::from(value.created_at),
            last_modified: String::from(value.last_modified),
            last_message: value.last_message,
            target_platform: value.target_platform,
            attempts: value.attempts,
            status: responses::ArtifactPublicationStatus::from(value.status),
        }
    }
}