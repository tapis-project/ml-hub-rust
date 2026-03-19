use crate::application::inputs::artifacts as inputs;
use crate::domain::entities::artifact as entities;

impl From<inputs::ArtifactType> for entities::ArtifactType {
    fn from(value: inputs::ArtifactType) -> Self {
        match value {
            inputs::ArtifactType::Model => entities::ArtifactType::Model,
            inputs::ArtifactType::Dataset => entities::ArtifactType::Dataset,
        }
    }
}