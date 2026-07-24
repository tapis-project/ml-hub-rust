pub mod dto_to_entity;

use std::num::NonZero;

use serde::{Serialize, Deserialize};
use crate::infra::deployment::fs::dtos::rule_set::RuleSet;
use crate::infra::deployment::fs::dtos::parameter_set::ParameterSet;
use crate::infra::persistence::mongo::documents::deployment::{DeploymentModality, ParallelismStrategy};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientStrategy {
    pub name: String,
    pub description: Option<String>,
    pub deployment_modality: DeploymentModality,
    pub rule_sets: Option<Vec<RuleSet>>,
    pub parameter_set: Option<ParameterSet>,
    pub use_rule_sets: Option<Vec<String>>,
    pub use_parameter_set: Option<String>,
    pub enabled: bool,
    pub config: Option<StrategyConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StrategyConfig {
    pub max_ttl: Option<u64>,
    pub supported_paralellism_strategies: Option<Vec<ParallelismStrategy>>,
    pub min_replicas: Option<NonZero<u64>>,
    pub max_replicas: Option<NonZero<u64>>,
}
