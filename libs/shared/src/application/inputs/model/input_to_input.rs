use crate::application::inputs::model as inputs;

impl From<inputs::AssociateModel> for inputs::UpdateModelArtifactId {
    fn from(value: inputs::AssociateModel) -> Self {
        return Self {
            artifact_id: value.artifact_id,
            name: value.name,
            author: value.author,
        };
    }
}
