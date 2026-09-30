pub mod dto_to_entity;

use crate::infra::deployment::fs::dtos::client_strategy::ClientStrategy;
use crate::infra::deployment::fs::dtos::parameter_set::ParameterSet;
use crate::infra::deployment::fs::dtos::rule_set::RuleSet;
use platforms::Platform;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientStrategySet {
    pub platform: Platform,
    pub description: Option<String>,
    pub rule_sets: Option<Vec<RuleSet>>,
    pub parameter_sets: Option<Vec<ParameterSet>>,
    pub strategies: Vec<ClientStrategy>,
    pub use_rule_sets: Option<Vec<String>>,
    pub use_parameter_set: Option<String>,
}
