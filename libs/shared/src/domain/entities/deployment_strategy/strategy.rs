use std::num::{NonZero, NonZeroU64};

use crate::domain::entities::deployment::ParallelismStrategy;
use crate::shared_kernel::enums::DeploymentModality;
use crate::shared_kernel::value_objects::Ttl;

use super::rule_set::RuleSet;
use super::parameter_set::ParameterSet;

use serde::Serialize;
use thiserror::Error;

use platforms::Platform;

#[derive(Error, Debug)]
pub enum StrategyError {
    #[error("Duplicate RuleSet name: {0}")]
    DuplicateRuleSetName(String)
}

#[derive(Clone, Debug, Serialize)]
pub struct Strategy {
    pub name: String,
    pub platform: Platform,
    pub description: Option<String>,
    pub deployment_modality: DeploymentModality,
    rule_sets: Vec<RuleSet>,
    parameter_set: Option<ParameterSet>,
    config: StrategyConfig,
    enabled: bool,
}

impl Strategy {
    pub fn new(
        name: String,
        platform: Platform,
        description: Option<String>,
        deployment_modality: DeploymentModality,
        rule_sets: Vec<RuleSet>,
        parameter_set: Option<ParameterSet>,
        config: Option<StrategyConfig>,
        enabled: bool
    ) -> Result<Self, StrategyError> {
        let mut rule_set_names: Vec<String> = Vec::new();
        for rule_set in &rule_sets {
            let rule_set_name = rule_set.name.clone();
            if rule_set_names.contains(&rule_set_name) {
                return Err(StrategyError::DuplicateRuleSetName(format!("Strategy '{}' contains rulesets with duplicate names. Duplicate found: {}", &name, rule_set_name)))
            }

            rule_set_names.push(rule_set_name)
        };

        Ok(Self {
            name,
            platform,
            deployment_modality,
            description,
            rule_sets,
            parameter_set,
            config: config.unwrap_or_default(),
            enabled,
        })
    }

    pub fn rule_sets(&self) -> &Vec<RuleSet> {
        &self.rule_sets
    }

    pub fn parameter_set(&self) -> &Option<ParameterSet> {
        &self.parameter_set
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StrategyConfig {
    /// Lifetime management
    pub max_ttl: Option<Ttl>,

    /// Replication and parallelism
    pub supported_paralellism_strategies: Option<Vec<ParallelismStrategy>>,
    pub min_replicas: Option<NonZero<u64>>,
    pub max_replicas: Option<NonZero<u64>>,
}

impl Default for StrategyConfig {
    fn default() -> Self {
        // Use constant block to evaluate at runtime. Unwraps at COMPILE time rather
        // than runtime
        let min_replicas = Some(const { NonZeroU64::new(1).unwrap() });
    
        Self {
            max_ttl: None,
            supported_paralellism_strategies: None,
            min_replicas,
            max_replicas: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ViableStrategy(Strategy);

impl ViableStrategy {
    pub fn new(strategy: Strategy) -> ViableStrategy {
        ViableStrategy(strategy)
    }
    
    pub fn into_inner(self) -> Strategy {
        self.0
    }
}

impl Into<Strategy> for ViableStrategy {
    fn into(self) -> Strategy {
        self.into_inner()
    }
}