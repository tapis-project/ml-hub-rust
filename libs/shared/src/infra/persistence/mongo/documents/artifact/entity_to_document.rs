use crate::{application::errors::ApplicationError, domain::entities};
use crate::infra::persistence::mongo::documents;
use crate::infra::common::mongo::ToBsonDateTime;
use mongodb::bson::Uuid;


impl From<entities::artifact::ArtifactType> for documents::artifact::ArtifactType {
    fn from(value: entities::artifact::ArtifactType) -> Self {
        match value {
            entities::artifact::ArtifactType::Model => documents::artifact::ArtifactType::Model,
            entities::artifact::ArtifactType::Dataset => documents::artifact::ArtifactType::Dataset,
        }
    }
}

impl From<entities::artifact::Artifact> for documents::artifact::Artifact {
    fn from(value: entities::artifact::Artifact) -> Self {
        let path = match value.path {
            Some(p) =>  p.to_str().map(|s| s.to_string()),
            None => None
        };

        Self {
            _id: None,
            id: Uuid::from_bytes(value.id.into_bytes()),
            artifact_type: documents::artifact::ArtifactType::from(value.artifact_type),
            last_modified: value.last_modified.to_bson(),
            created_at: value.created_at.to_bson(),
            path
        }
    }
}

impl TryFrom<entities::artifact::Artifact> for documents::artifact::UpdateArtifactPathRequest {
    type Error = ApplicationError;

    fn try_from(value: entities::artifact::Artifact) -> Result<Self, Self::Error> {
        let path = match value.path {
            Some(p) => p,
            None => return Err(ApplicationError::ConversionError("Path".into()))
        };

        Ok(Self {
            last_modified: value.last_modified.to_bson(),
            path: path.to_string_lossy().into_owned()
        })
    }
}

impl TryFrom<entities::artifact::Artifact> for documents::artifact::UpdateArtifactRequest {
    type Error = ApplicationError;

    fn try_from(value: entities::artifact::Artifact) -> Result<Self, Self::Error> {
        let path = match value.path {
            Some(p) => p,
            None => return Err(ApplicationError::ConversionError("Path".into()))
        };

        Ok(Self {
            last_modified: value.last_modified.to_bson(),
            path: path.to_string_lossy().into_owned()
        })
    }
}