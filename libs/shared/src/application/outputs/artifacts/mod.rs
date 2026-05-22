use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::entities::dataset_metadata::DatasetMetadata;

pub struct ModelArtifactOutput {
    pub artifact: Artifact,
    pub metadata: Option<ModelMetadata>
}

pub struct DatasetArtifactOutput {
    pub artifact: Artifact,
    pub metadata: Option<DatasetMetadata>
}