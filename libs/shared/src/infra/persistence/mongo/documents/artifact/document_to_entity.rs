use std::path::PathBuf;

use crate::domain::entities;
use crate::infra::persistence::mongo::documents;
use uuid::Uuid;

impl From<documents::artifact::ArtifactType> for entities::artifact::ArtifactType {
    fn from(value: documents::artifact::ArtifactType) -> Self {
        match value {
            documents::artifact::ArtifactType::Model => entities::artifact::ArtifactType::Model,
            documents::artifact::ArtifactType::Dataset => entities::artifact::ArtifactType::Dataset,
        }
    }
}

impl From<documents::artifact::Artifact> for entities::artifact::Artifact {
    fn from(value: documents::artifact::Artifact) -> Self {
        let path = match value.path {
            Some(s) =>  Some(PathBuf::from(s)),
            None => None
        };

        Self {
            id: Uuid::from_bytes(value.id.bytes()),
            artifact_type: entities::artifact::ArtifactType::from(value.artifact_type),
            last_modified: entities::timestamp::TimeStamp::from(value.last_modified.to_chrono()),
            created_at: entities::timestamp::TimeStamp::from(value.created_at.to_chrono()),
            path
        }
    }
}