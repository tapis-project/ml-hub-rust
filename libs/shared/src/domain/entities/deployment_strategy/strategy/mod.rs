

use std::collections::HashMap;
use std::num::{NonZero, NonZeroU64};

use crate::domain::entities::deployment::argument::Argument;
use crate::domain::entities::deployment_strategy::rule_set::RuleSet;
use crate::domain::entities::deployment_strategy::parameter_set::{ParameterSet, ParameterSetError};
use crate::domain::entities::deployment::ParallelismStrategy;
use crate::shared_kernel::enums::DeploymentModality;
use crate::shared_kernel::value_objects::Ttl;

use nonempty::NonEmpty;
use serde::Serialize;
use thiserror::Error;

use platforms::Platform;

#[derive(Error, Debug, Clone)]
pub enum StrategyError {
    #[error("Duplicate RuleSet name: {0}")]
    DuplicateRuleSetName(String),

    #[error(transparent)]
    ConfigurationError(#[from] StrategyConfigError),

    #[error(transparent)]
    InvalidArgumentsForParameterSet(#[from] ParameterSetError),

    #[error("Extraneous arguments provided for strategy with no parameter set")]
    UnexpectedArguments,
}

#[derive(Clone, Debug, Serialize)]
pub struct Strategy {
    pub name: String,
    pub platform: Platform,
    pub description: Option<String>,
    rule_sets: Vec<RuleSet>,
    parameter_set: Option<ParameterSet>,
    config: StrategyConfig,
    enabled: bool,
    // Any additional configuration data that deployment clients might need to deploy models
    data: Option<HashMap<String, String>>,
}

impl Strategy {
    pub fn reconstitute(
        name: String,
        platform: Platform,
        description: Option<String>,
        rule_sets: Vec<RuleSet>,
        parameter_set: Option<ParameterSet>,
        config: StrategyConfig,
        enabled: Option<bool>,
        data: Option<HashMap<String, String>>
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
            description,
            rule_sets,
            parameter_set,
            config,
            enabled: enabled.unwrap_or(true),
            data,
        })
    }

    pub fn rule_sets(&self) -> &Vec<RuleSet> {
        &self.rule_sets
    }

    pub fn parameter_set(&self) -> &Option<ParameterSet> {
        &self.parameter_set
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn config(&self) -> &StrategyConfig {
        &self.config
    }

    pub fn data(&self) -> &Option<HashMap<String, String>> {
        &self.data
    }

    pub fn is_parameter_secret(&self, parameter_name: &str) -> bool {
        let parameter_set = self.parameter_set();

        let parameters = parameter_set
            .as_ref()
            .map_or(vec![], |ps| ps.get_required_params());

        parameters.iter()
            .filter(|p| p.name == parameter_name && p.secret)
            .collect::<Vec<_>>()
            .len() > 0
    }

    pub fn validate_arguments(&self, args: &[Argument]) -> Result<(), StrategyError> {
        match &self.parameter_set {
            Some(ps) => ps
                .validate_arguments(args)
                .map_err(StrategyError::InvalidArgumentsForParameterSet),
            None if !args.is_empty() => Err(StrategyError::UnexpectedArguments),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct StrategyConfig {
    /// Lifetime management
    pub max_ttl: Option<Ttl>,

    /// Deployment modalities
    pub supported_deployment_modalities: NonEmpty<DeploymentModality>,

    /// Replication and parallelism
    pub supported_paralellism_strategies: Vec<ParallelismStrategy>,
    pub min_replicas: NonZero<u64>,
    pub max_replicas: Option<NonZero<u64>>,
}

pub struct ReconstitueStrategyConfigProps {
    pub max_ttl: Option<Ttl>,
    pub supported_deployment_modalities: Vec<DeploymentModality>,
    pub supported_paralellism_strategies: Vec<ParallelismStrategy>,
    pub min_replicas: Option<u64>,
    pub max_replicas: Option<u64>,
}

#[derive(Debug, Error, Clone)]
pub enum StrategyConfigError {
    #[error("Data integrity error: {0}")]
    DataIntegrityError(String)
} 

impl StrategyConfig {
    pub fn reconstitute(props: ReconstitueStrategyConfigProps) -> Result<Self, StrategyConfigError> {
        let supported_deployment_modalities = match NonEmpty::from_vec(props.supported_deployment_modalities) {
            Some(d) => d,
            None => return Err(StrategyConfigError::DataIntegrityError("Strategy configuration MUST have at least one supported deployment modality".into()))
        };

        let min_replicas = NonZeroU64::new(props.min_replicas.unwrap_or(1))
            .ok_or_else(|| StrategyConfigError::DataIntegrityError("Min replicas MUST be greater than 0".into()))?;

        let max_replicas = props.max_replicas.and_then(NonZeroU64::new);

        Self::validate_replica_bounds(min_replicas, max_replicas)
            .map_err(StrategyConfigError::DataIntegrityError)?;
    
        Ok(Self {
            max_ttl: props.max_ttl,
            supported_deployment_modalities,
            supported_paralellism_strategies: props.supported_paralellism_strategies,
            min_replicas,
            max_replicas,
        })
    }

    fn validate_replica_bounds(min: NonZero<u64>, maybe_max: Option<NonZero<u64>>) -> Result<(), String> {
        if maybe_max.is_some_and(|max| min > max) {
            return Err("Min replicas MUST NOT be greater than max replicas".into());
        }

        return Ok(())
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

#[cfg(test)]
#[path = "strategy.test.rs"]
mod strategy_test;