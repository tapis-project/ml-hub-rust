pub mod dto_to_entity;

use serde::{Serialize, Deserialize};
use crate::infra::deployment::fs::dtos::rule_set::RuleSet;
use crate::infra::deployment::fs::dtos::parameter_set::ParameterSet;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientStrategy {
    pub name: String,
    pub description: Option<String>,
    pub rule_sets: Option<Vec<RuleSet>>,
    pub parameter_set: Option<ParameterSet>,
    pub use_rule_sets: Option<Vec<String>>,
    pub use_parameter_set: Option<String>,
}