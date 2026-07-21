pub mod entity_to_response;

// use super::rule_set::RuleSet;
use super::parameter_set::Parameter;
use platforms::Platform;
use utoipa::ToSchema;
use serde::Serialize;

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct Strategy {
    pub name: String,
    pub platform: Platform,
    pub description: Option<String>,
    pub deployment_modality: DeploymentModality,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Debug, ToSchema, Serialize)]
pub enum DeploymentModality {
    Batch,
    Service,
}