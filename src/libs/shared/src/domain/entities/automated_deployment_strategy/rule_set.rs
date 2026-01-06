use serde::Serialize;
use serde_json::Value;

use crate::domain::entities::operator::Operator;

#[derive(Clone, Debug, Serialize)]
pub struct Rule {
    pub field_path: Vec<String>,
    pub operator: Operator,
    pub value: Value,
}


#[derive(Clone, Debug, Serialize)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Rule>
}