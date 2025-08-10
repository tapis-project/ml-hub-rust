pub mod entity_to_document;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use mongodb::bson::{DateTime, Uuid, oid::ObjectId};

#[derive(Debug, Error)]
pub enum ArtifactPublicationError {
    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ArtifactPublicationFailedReason {
    FailedToExtract(String),
    FailedToPublishArtifact(String),
    FailedToPublishMetadata(String),
    InternalError(String),
    PlatformError(String),
}

impl ArtifactPublicationFailedReason {
    fn _kind(&self) -> &str {
        match self {
            Self::FailedToExtract(_) => "FailedToExtract",
            Self::FailedToPublishArtifact(_) => "FailedToPublishArtifact",
            Self::FailedToPublishMetadata(_) => "FailedToPublishMetadata",
            Self::InternalError(_) => "InternalError",
            Self::PlatformError(_) => "PlatformError",
        }
    }
}

type Reason = ArtifactPublicationFailedReason;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize,)]
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
    Failed(Reason)
}

impl ArtifactPublicationStatus {
    fn kind(&self) -> &str {
        match self {
            Self::Submitted => "Submitted",
            Self::Pending => "Pending",
            Self::Extracting => "Extracting",
            Self::Extracted => "Extracted",
            Self::PublishingMetadata => "PublishingMetadata",
            Self::PublishedMetadata => "PublishedMetadata",
            Self::PublishingArtifact => "PublishingArtifact",
            Self::PublishedArtifact => "PublishedArtifact",
            Self::Finished => "Finished",
            Self::Failed(_) => "Failed"
        }
    }
}

type Status = ArtifactPublicationStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactPublication  {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub status: Status,
    pub artifact_id: Uuid,
    pub platform: String,
    pub platform_artifact_id: String,
    pub last_message: String,
    pub attempts: u8,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}