use uuid::Uuid;
use crate::domain::entities::timestamp::TimeStamp;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtifactPublicationError {
    #[error("Invalid status transition: {0}")]
    InvalidStatusTransition(String)
}

pub struct ArtifactPublication  {
    pub id: Uuid,
    pub status: Status,
    pub artifact_id: Uuid,
    pub platform: String,
    pub last_message: String,
    pub attempts: u8,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
}

/// Represents the life cycle of an attempt to publish an artifact
impl ArtifactPublication {
    pub fn new(artifact_id: Uuid, platform: String) -> Self {
        let now = TimeStamp::now();
        Self {
            id: Uuid::new_v4(),
            artifact_id,
            status: ArtifactPublicationStatus::Submitted,
            platform,
            last_message: "Submitted".into(),
            attempts: 0,
            created_at: now.clone(),
            last_modified: now.clone(),
        }
    }

    /// Sets the status of the of the ArtifactPublication. This will also check
    /// if the transition from the current status to the new status is valid.
    /// Additionally the the ArtifactPublications last_modified will be updated
    /// to "now"
    pub fn set_status(&mut self, status: &Status) -> Result<&mut Self, ArtifactPublicationError> {
        if !self.is_valid_status_transition(&self.status, status) {
            return Err(ArtifactPublicationError::InvalidStatusTransition(format!("Invalid status transition: ArtifactPublication cannot move from status {} to status {}", self.status.kind(), status.kind())))
        }

        self.status = status.clone();

        self.touch();

        Ok(self)
    }

    /// Updates last modified to the UTC timestamp
    fn touch(&mut self) {
        self.last_modified = TimeStamp::now()
    }

    /// Checks if the transition from the current status to the new status is valid
    fn is_valid_status_transition(&self, from: &Status,  to: &Status) -> bool {
        let is_valid: bool = match from {
            Status::Submitted => {
                match to {
                    Status::Pending
                    | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::Pending => {
                match to {
                    Status::Extracting
                    | Status::PublishingArtifact
                    | Status::PublishedMetadata
                    | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::Extracting => {
                match to {
                    Status::Extracted
                    | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::Extracted => {
                match to {
                    Status::PublishingArtifact
                    | Status::PublishingMetadata
                    | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::PublishingMetadata => {
                match to {
                    Status::PublishedMetadata
                    | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::PublishedMetadata => {
                match to {
                    Status::PublishingArtifact
                    | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::PublishingArtifact => {
                match to {
                    Status::PublishedArtifact => true,
                    _ => false
                }
            },
            Status::PublishedArtifact => {
                match to {
                    Status::Finished | Status::Failed(_) => true,
                    _ => false
                }
            },
            // Cannot transition from finished to any other status
            Status::Finished => false,

            // Cannot transition from Failed to any other status (NOTE This will not be true once publication is a recoverable operation)
            Status::Failed(_) => false,
        };

        is_valid
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArtifactPublicationFailureReason {
    FailedToExtract(String),
    FailedToPublishArtifact(String),
    FailedToPublishMetadata(String),
    InternalError(String),
    PlatformError(String),
}

type Reason = ArtifactPublicationFailureReason;

impl ArtifactPublicationFailureReason {
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

#[cfg(test)]
#[path = "artifact_publication.test.rs"]
mod artifact_publication_test;