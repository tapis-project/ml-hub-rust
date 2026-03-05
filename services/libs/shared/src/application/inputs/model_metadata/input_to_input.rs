use crate::application::inputs::model_metadata as inputs;

impl From<inputs::AssociateModelMetadata> for inputs::UpdateModelMetadataArtifactId {
    fn from(value: inputs::AssociateModelMetadata) -> Self {
        return Self {
            artifact_id: value.artifact_id,
            name: value.name,
            author: value.author,
        }
    }
}