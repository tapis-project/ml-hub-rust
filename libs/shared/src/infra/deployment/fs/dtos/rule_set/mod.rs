pub mod dto_to_entity;

use crate::infra::operators::dtos::Operator;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rule {
    pub field_path: Vec<String>,
    pub operator: Operator,
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Rule>,
}
