use crate::shared_kernel::enums::DeploymentModality;

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
}

impl Strategy {
    pub fn new(
        name: String,
        platform: Platform,
        description: Option<String>,
        deployment_modality: DeploymentModality,
        rule_sets: Vec<RuleSet>,
        parameter_set: Option<ParameterSet>,
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
            parameter_set
        })
    }

    pub fn rule_sets(&self) -> &Vec<RuleSet> {
        &self.rule_sets
    }

    pub fn parameter_set(&self) -> &Option<ParameterSet> {
        &self.parameter_set
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
