use crate::application::outputs::model_metadata::ModelMetadata;

pub struct DiscoverModelsOutput {
    pub models: Vec<ModelMetadata>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}