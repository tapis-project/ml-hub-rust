pub mod entity_to_response;

use super::super::operators::Operator;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct Rule {
    pub field_path: Vec<String>,
    pub operator: Operator,
    pub value: Value,
}


#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Rule>
}