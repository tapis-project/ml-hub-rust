use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::model_metadata::ModelMetadata;

pub struct ModelArtifactOutput {
    pub artifact: Artifact,
    pub metadata: Option<ModelMetadata>
}