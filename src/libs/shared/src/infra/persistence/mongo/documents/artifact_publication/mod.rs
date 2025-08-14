pub mod entity_to_document;
pub mod document_to_entity;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use mongodb::bson::{DateTime, Uuid, oid::ObjectId};
use strum_macros::Display;

#[derive(Debug, Error)]
pub enum ArtifactPublicationError {
    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum ArtifactPublicationFailureReason {
    FailedToQueue(String),
    FailedToExtract(String),
    FailedToPublishArtifact(String),
    FailedToPublishMetadata(String),
    InternalError(String),
    PlatformError(String),
}

type Reason = ArtifactPublicationFailureReason;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
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

type Status = ArtifactPublicationStatus;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArtifactPublication  {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub id: Uuid,
    pub status: Status,
    pub artifact_id: Uuid,
    pub target_platform: String,
    pub last_message: Option<String>,
    pub attempts: u8,
    pub created_at: DateTime,
    pub last_modified: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateArtifactPublicationStatusRequest {
    pub status: ArtifactPublicationStatus,
    pub last_message: Option<String>,
    pub last_modified: DateTime,
}