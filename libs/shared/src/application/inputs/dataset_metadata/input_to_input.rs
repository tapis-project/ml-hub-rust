use crate::application::inputs::dataset_metadata as inputs;

impl From<inputs::AssociateDatasetMetadata> for inputs::UpdateDatasetMetadataArtifactId {
    fn from(value: inputs::AssociateDatasetMetadata) -> Self {
        return Self {
            artifact_id: value.artifact_id,
            name: value.name,
            author: value.author,
        }
    }
}