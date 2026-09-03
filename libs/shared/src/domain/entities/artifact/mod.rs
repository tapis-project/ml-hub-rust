use std::path::PathBuf;
use uuid::Uuid;
use crate::shared_kernel::value_objects::TimeStamp;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactType {
    Model,
    Dataset,
}

impl std::fmt::Display for ArtifactType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactType::Model => write!(f, "Model"),
            ArtifactType::Dataset => write!(f, "Dataset"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Artifact {
    pub id: Uuid,
    pub artifact_type: ArtifactType,
    pub path: Option<PathBuf>,
    pub created_at: TimeStamp,
    pub last_modified: TimeStamp,
}

impl Artifact {
    pub fn new(r#type: ArtifactType) -> Self {
        let now = TimeStamp::now();
        Self {
            id: Uuid::new_v4(),
            path: None,
            artifact_type: r#type,
            created_at: now.clone(),
            last_modified: now.clone()
        }
    }

    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
        
        // Update last modified
        self.touch();
    }

    pub fn is_fully_ingested(&self) -> bool {
        self.path.is_some()
    }

    /// Updates last modified to the UTC timestamp
    fn touch(&mut self) {
        self.last_modified = TimeStamp::now()
    }
}

// Unit tests
#[cfg(test)]
#[path = "artifact.test.rs"]
mod artifact_test;
