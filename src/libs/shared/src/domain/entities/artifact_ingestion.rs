use std::path::PathBuf;
use uuid::Uuid;
use thiserror::Error;
use crate::domain::entities::timestamp::TimeStamp;

#[derive(Debug, Error)]
pub enum ArtifactIngestionError {
    #[error("Invalid status change. Cannot move from status '{0}' to {1}")]
    InvalidStatusTransition(String, String),

    #[error("Artifact path error: {0}")]
    ArtifactPath(String)
}

// Private type alias to make things less verbose
type IngestionError = ArtifactIngestionError;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ArtifactIngestion {
    pub id: Uuid,
    pub artifact_id: Uuid, 
    pub platform: String,
    pub status: ArtifactIngestionStatus,
    pub last_message: Option<String>,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
    pub artifact_path: Option<PathBuf>,
    pub webhook_url: Option<String>,
}

/// Represent the ingestion
impl ArtifactIngestion {
    pub fn new(artifact_id: Uuid, platform: String, webhook_url: Option<String>) -> Self {
        let now = TimeStamp::now();
        Self {
            id: Uuid::new_v4(),
            artifact_id,
            platform,
            last_message: None,
            status: ArtifactIngestionStatus::Submitted,
            created_at: now.clone(),
            last_modified: now.clone(),
            artifact_path: None,
            webhook_url,
        }
    }

    /// Updates last modified to the UTC timestamp
    fn touch(&mut self) {
        self.last_modified = TimeStamp::now()
    }

    /// Returns whether a transition from one status to another is valid
    fn is_valid_status_transition(from: &Status, to: &Status) -> bool {
        match from {
            Status::Submitted | Status::Resubmitted => {
                match to {
                    Status::Pending | Status::Failed(_)  => true,
                    _ => false
                }
            }
            Status::Pending => {
                match to {
                    Status::Downloading | Status::Failed(_) => true,
                    _ => false
                }
            }
            Status::Downloading => {
                match to {
                    Status::Downloaded | Status::Failed(_) => true,
                    _ => false
                }
            }, 
            Status::Downloaded => {
                match to {
                    Status::Archiving | Status::Finished | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::Archiving => {
                match to {
                    Status::Archived | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::Archived => {
                match to {
                    Status::Finished | Status::Failed(_) => true,
                    _ => false
                }
            },
            Status::Finished | Status::Failed(_)=> match to {
                Status::Resubmitted => true,
                _ => false
            },
        }
    }

    pub fn set_artifact_path(&mut self, path: PathBuf) -> Result<(), IngestionError> {
        if !(self.status == Status::Downloaded || self.status == Status::Archived) {
            return Err(IngestionError::ArtifactPath("This ingestion's artifact_path can only be set while the ingestion has a status of Downloaded or Archived".into()))
        }

        self.artifact_path = Some(path);

        // Updates last modifified
        self.touch();

        return Ok(())
    }

    /// Changes the status. Returns an error if invalid status transition is detected
    pub fn change_status(&mut self, new_status: Status) -> Result<(), IngestionError> {
        if !Self::is_valid_status_transition(&self.status, &new_status) {
            return Err(IngestionError::InvalidStatusTransition(self.status.clone().into(), new_status.into()))
        }

        // The artifact_path must be set before the ingestion is moved to a status of finished
        if new_status == Status::Finished && self.artifact_path == None {
            return Err(IngestionError::ArtifactPath("The artifact_path must be set before moving the ingestion into a Finished state".into()));
        }

        // Changes the status
        self.status = new_status;

        // Updates last_modified
        self.touch();

        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ArtifactIngestionStatus {
    Submitted,
    Resubmitted,
    Pending,
    Downloading,
    Downloaded,
    Archiving,
    Archived,
    Finished,
    Failed(Reason),
}

type Status = ArtifactIngestionStatus;

impl From<Status> for String {
    fn from(value: Status) -> Self {
        match value {
            Status::Submitted => "Submitted".into(),
            Status::Resubmitted => "Submitted".into(),
            Status::Pending => "Pending".into(),
            Status::Downloading => "Downloading".into(),
            Status::Downloaded => "Downloaded".into(),
            Status::Archiving => "Archiving".into(),
            Status::Archived => "Archived".into(),
            Status::Finished => "Finished".into(),
            Status::Failed(_) => "Failed".into(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum ArtifactIngestionFailureReason {
    FailedToQueue,
    FailedToDownload,
    FailedToArchive,
    Unknown
}

type Reason = ArtifactIngestionFailureReason;

// Unit tests
#[cfg(test)]
#[path = "artifact_ingestion.test.rs"]
mod artifact_ingestion_test;