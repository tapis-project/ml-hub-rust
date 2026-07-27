use thiserror::Error;

use super::rule_set::RuleSet;
use super::parameter_set::ParameterSet;
use super::strategy::{StrategyConfig, StrategyConfigError};

#[derive(Error, Debug)]
pub enum ClientStrategyError {
    #[error("{0}")]
    MissingRuleSetsAndReferences(String),

    #[error("{0}")]
    EmptyRuleSets(String),

    #[error("{0}")]
    EmptyRuleSetReferences(String),

    #[error(transparent)]
    ConfigError(#[from] StrategyConfigError),
}

#[derive(Debug, Clone)]
pub struct ClientStrategy {
    pub name: String,
    pub description: Option<String>,
    rule_sets: Option<Vec<RuleSet>>,
    parameter_set: Option<ParameterSet>,
    use_rule_sets: Option<Vec<String>>,
    use_parameter_set: Option<String>,
    config: StrategyConfig,
    enabled: Option<bool>,
}

impl ClientStrategy {
    pub fn reconstitute(
        name: String,
        description: Option<String>,
        rule_sets: Option<Vec<RuleSet>>,
        parameter_set: Option<ParameterSet>,
        use_rule_sets: Option<Vec<String>>,
        use_parameter_set: Option<String>,
        config: StrategyConfig,
        enabled: Option<bool>,
    ) -> Result<Self, ClientStrategyError> {
        // Invariant: Rule sets MUST contain either rule sets or rule set references
        if rule_sets.is_none() && use_rule_sets.is_none() {
            return Err(ClientStrategyError::MissingRuleSetsAndReferences(
                "Invariant Violation: Client Strategy MUST provide at least one inline rule set or one rule set reference.".into()
            ));
        }

        // Invariant: Rule sets array must not be empty if provided
        if rule_sets.as_ref().is_some_and(|rs| rs.is_empty()) {
            return Err(ClientStrategyError::EmptyRuleSets(
                "Invariant Violation: The inline rule_sets array MUST NOT be empty if provided.".into()
            ));
        }

        // Invariant 3: Rule set references array must not be empty if provided
        if use_rule_sets.as_ref().is_some_and(|rsr| rsr.is_empty()) {
            return Err(ClientStrategyError::EmptyRuleSetReferences(
                "Invariant Violation: The use_rule_sets reference array cannot be empty if provided.".into()
            ));
        }
        
        Ok(Self {
            name,
            description,
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

    pub fn config(&self) -> &StrategyConfig {
        &self.config
    }

    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }
}

#[cfg(test)]
#[path = "client_strategy.test.rs"]
mod client_strategy_test;