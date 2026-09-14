use crate::application::outputs::model::Model;

pub struct DiscoverModelsOutput {
    pub models: Vec<Model>,
    pub count: Option<i64>,
    pub cursor: Option<String>,
}