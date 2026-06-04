use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ArtifactIngestionStatus {
    Submitted,
    Resubmitted,
    Pending,
    Downloading,
    Downloaded,
    Archiving,
    Archived,
    Finished,
    Failed,
}
