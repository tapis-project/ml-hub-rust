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
    // pub rule_sets: Vec<RuleSet>,
    pub parameters: Vec<Parameter>,
}
