pub mod ingestions;
pub mod publications;

use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::entities;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum ArtifactType {
    Model,
    Dataset
}

#[derive(Serialize, ToSchema)]
pub struct Artifact {
    pub id: String,
    pub artifact_type: ArtifactType,
    pub created_at: String,
    pub last_modified: String,
}

impl From<entities::artifact::ArtifactType> for ArtifactType {
    fn from(value: entities::artifact::ArtifactType) -> Self {
        match value {
            entities::artifact::ArtifactType::Dataset => ArtifactType::Dataset,
            entities::artifact::ArtifactType::Model => ArtifactType::Model
        }
    }
}

impl From<entities::artifact::Artifact> for Artifact {
    fn from(value: entities::artifact::Artifact) -> Self {
        Artifact {
            id: value.id.to_string(),
            created_at: String::from(value.created_at),
            last_modified: String::from(value.last_modified),
            artifact_type: ArtifactType::from(value.artifact_type),
        }
    }
}
