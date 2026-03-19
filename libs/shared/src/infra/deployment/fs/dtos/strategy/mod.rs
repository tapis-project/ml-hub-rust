pub mod dto_to_entity;

use serde::{Serialize, Deserialize};
use crate::infra::deployment::fs::dtos::rule_set::RuleSet;
use crate::infra::deployment::fs::dtos::parameter_set::ParameterSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Strategy {
    pub name: String,
    pub description: Option<String>,
    pub rule_sets: Vec<RuleSet>,
    pub parameter_set: Option<ParameterSet>,
}
