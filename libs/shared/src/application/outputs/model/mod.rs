use crate::domain::entities::model::{external_model::ExternalModel, Model};

#[derive(Debug, Clone)]
pub struct ModelWithExternalModel {
    pub model: Model,
    pub external_model: ExternalModel,
}

#[derive(Debug, Clone)]
pub struct ModelListOutput {
    pub models: Vec<ModelWithExternalModel>,
    pub count: Option<u64>,
    pub cursor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExternalModelListOutput {
    pub external_models: Vec<ExternalModel>,
    pub count: Option<u64>,
    pub cursor: Option<String>,
}
