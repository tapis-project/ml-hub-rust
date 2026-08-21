pub mod entity_to_response;

// use super::rule_set::RuleSet;
use super::{parameter_set::Parameter, ParallelismStrategy};
use platforms::Platform;
use utoipa::ToSchema;
use serde::Serialize;

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct Strategy {
    pub name: String,
    pub platform: Platform,
    pub description: Option<String>,
    pub parameters: Vec<Parameter>,
    pub config: StrategyConfig,
    pub enabled: bool,
}

#[derive(Clone, Debug, ToSchema, Serialize)]
pub enum DeploymentModality {
    Batch,
    Service,
}

#[derive(Debug, Clone, ToSchema, Serialize)]
pub struct StrategyConfig {
    pub max_ttl: Option<u64>,
    pub supported_paralellism_strategies: Vec<ParallelismStrategy>,
    pub supported_deployment_modalities: Vec<DeploymentModality>,
    pub min_replicas: u64,
    pub max_replicas: Option<u64>,
}