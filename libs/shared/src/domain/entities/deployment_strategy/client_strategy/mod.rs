use thiserror::Error;
use crate::shared_kernel::enums::DeploymentModality;

use super::rule_set::RuleSet;
use super::parameter_set::ParameterSet;
use super::strategy::StrategyConfig;

#[derive(Error, Debug)]
pub enum ClientStrategyError {
    #[error("{0}")]
    MissingRuleSets(String)
}

#[derive(Debug, Clone)]
pub struct ClientStrategy {
    pub name: String,
    pub description: Option<String>,
    deployment_modality: DeploymentModality,
    rule_sets: Option<Vec<RuleSet>>,
    parameter_set: Option<ParameterSet>,
    use_rule_sets: Option<Vec<String>>,
    use_parameter_set: Option<String>,
    config: Option<StrategyConfig>,
    enabled: bool,
}

impl ClientStrategy {
    pub fn new(
        name: String,
        description: Option<String>,
        deployment_modality: DeploymentModality,
        rule_sets: Option<Vec<RuleSet>>,
        parameter_set: Option<ParameterSet>,
        use_rule_sets: Option<Vec<String>>,
        use_parameter_set: Option<String>,
        config: Option<StrategyConfig>,
        enabled: bool,
    ) -> Result<Self, ClientStrategyError> {
        // Must be at least 1 rule set or reference to a client rule set
        if rule_sets.is_none() && use_rule_sets.is_none() {
            return Err(ClientStrategyError::MissingRuleSets("Must provide 1 or more rule set or 1 or more rule set reference.".into()))
        }

        if let Some(rs) = rule_sets.as_ref() {
            if rs.is_empty() {
                return Err(ClientStrategyError::MissingRuleSets("Must provide at least one rule set in the rule_sets array".into()))
            }
        }

        if let Some(rsr) = use_rule_sets.as_ref() {
            if rsr.is_empty() {
                return Err(ClientStrategyError::MissingRuleSets("Must provide at least one rule set reference in the use_rule_sets array".into()))
            }
        }
        
        Ok(Self {
            name,
            description,
            deployment_modality,
            rule_sets,
            parameter_set,
            use_rule_sets,
            use_parameter_set,
            config,
            enabled,
        })
    }

    pub fn rule_sets(&self) -> &Option<Vec<RuleSet>> {
        &self.rule_sets
    }

    pub fn rule_set_refs(&self) -> &Option<Vec<String>> {
        &self.use_rule_sets
    }

    pub fn parameter_set(&self) -> &Option<ParameterSet> {
        &self.parameter_set
    }

    pub fn parameter_set_ref(&self) -> &Option<String> {
        &self.use_parameter_set
    }

    pub fn deployment_modality(&self) -> &DeploymentModality {
        &self.deployment_modality
    }

    pub fn config(&self) -> &Option<StrategyConfig> {
        &self.config
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "client_strategy.test.rs"]
mod client_strategy_test;