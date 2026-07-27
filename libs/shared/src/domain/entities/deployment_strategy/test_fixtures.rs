#![cfg(test)]

use platforms::Platform;

use crate::{
    domain::entities::{deployment::ParallelismStrategy, operator::Operator},
    shared_kernel::{
        enums::DeploymentModality,
        value_objects::Ttl
    }
};

use super::{client_strategy::{ClientStrategy, ClientStrategyError}, parameter_set::ParameterSet, rule_set::{Rule, RuleSet}, strategy::{ReconstitueStrategyConfigProps, Strategy, StrategyConfig, StrategyConfigError, StrategyError}};

#[derive(Debug, Clone)]
pub struct ReconstitutedClientStrategyBuilder {
    pub name: String,
    pub description: Option<String>,
    rule_sets: Option<Vec<RuleSet>>,
    parameter_set: Option<ParameterSet>,
    use_rule_sets: Option<Vec<String>>,
    use_parameter_set: Option<String>,
    config: StrategyConfig,
    enabled: Option<bool>,
}

impl ReconstitutedClientStrategyBuilder {
    pub fn new() -> Self {
        Self {
            name: "Test Name".into(),
            description: None,
            rule_sets: Some(vec![
                RuleSet {
                    name: "default-rule-set".into(),
                    rules: vec![
                        Rule {
                            field_path: vec!["visibility".into()],
                            operator: Operator::Eq,
                            value: "Private".into(),
                        },
                    ]
                }
            ]),
            use_rule_sets: None,
            parameter_set: None,
            use_parameter_set: None,
            config: ReconstitutedStrategyConfigBuilder::new().build(),
            enabled: Some(true)
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    pub fn with_rule_set_reference(mut self, reference: String) -> Self {
        self.use_rule_sets.get_or_insert_with(Vec::new).push(reference);
        self
    }

    pub fn with_rule_sets(mut self, rule_sets: Option<Vec<RuleSet>>) -> Self {
        self.rule_sets = rule_sets;
        self
    }

    pub fn with_rule_set_references(mut self, references: Vec<String>) -> Self {
        self.use_rule_sets = Some(references);
        self
    }

    pub fn with_rule_set(mut self, rule_set: RuleSet) -> Self {
        self.rule_sets.get_or_insert_with(Vec::new).push(rule_set);
        self
    }

    pub fn with_use_parameter_set(mut self, parameter_set_name: String) -> Self {
        self.use_parameter_set = Some(parameter_set_name);
        self
    }

    pub fn with_parameter_set(mut self, parameter_set: ParameterSet) -> Self {
        self.parameter_set = Some(parameter_set);
        self
    }

    pub fn build_reconstituted(&self) -> Result<ClientStrategy, ClientStrategyError> {
        Ok(ClientStrategy::reconstitute(
            self.name.clone(),
            self.description.clone(),
            self.rule_sets.clone(),
            self.parameter_set.clone(),
            self.use_rule_sets.clone(),
            self.use_parameter_set.clone(),
            self.config.clone(),
            self.enabled.clone(),
        )?)
    }
}

pub struct ReconstituteStrategyBuilder {
    name: String,
    platform: Platform,
    description: Option<String>,
    rule_sets: Vec<RuleSet>,
    parameter_set: Option<ParameterSet>,
    config: StrategyConfig,
    enabled: Option<bool>,
}

impl ReconstituteStrategyBuilder {
    pub fn new() -> Self {
        Self {
            name: "Test Name".into(),
            platform: Platform::TapisJobs,
            description: None,
            parameter_set: None,
            config: ReconstitutedStrategyConfigBuilder::new().build(),
            rule_sets: vec![],
            enabled: Some(true)
        }
    }

    pub fn build_reconstituted(&self) -> Result<Strategy, StrategyError> {
        Ok(Strategy::reconstitute(
            self.name.clone(),
            self.platform.clone(),
            self.description.clone(),
            self.rule_sets.clone(),
            self.parameter_set.clone(),
            self.config.clone(),
            self.enabled.clone(),
        )?)
    }
}

pub struct ReconstitutedStrategyConfigBuilder {
    max_ttl: Option<Ttl>,
    min_replicas: Option<u64>,
    max_replicas: Option<u64>,
    supported_deployment_modalities: Vec<DeploymentModality>,
    supported_parallelism_strategies: Vec<ParallelismStrategy>
}

impl ReconstitutedStrategyConfigBuilder {
    /// By default, this creates a perfectly valid setup
    pub fn new() -> Self {
        Self {
            max_ttl: None,
            min_replicas: Some(1),
            max_replicas: Some(5),
            supported_deployment_modalities: vec![DeploymentModality::Batch],
            supported_parallelism_strategies: vec![]
        }
    }

    pub fn with_min_replicas(mut self, min: u64) -> Self {
        self.min_replicas = Some(min);
        self
    }

    pub fn with_max_replicas(mut self, max: u64) -> Self {
        self.max_replicas = Some(max);
        self
    }

    pub fn with_modalities(mut self, modalities: Vec<DeploymentModality>) -> Self {
        self.supported_deployment_modalities = modalities;
        self
    }

    pub fn build_reconstituted(self) -> Result<StrategyConfig, StrategyConfigError> {
        StrategyConfig::reconstitute(
            ReconstitueStrategyConfigProps {
                max_ttl: self.max_ttl,
                supported_deployment_modalities: self.supported_deployment_modalities,
                supported_paralellism_strategies: self.supported_parallelism_strategies,
                min_replicas: self.min_replicas,
                max_replicas: self.max_replicas
            }
        )
    }

    pub fn build(self) -> StrategyConfig {
        self.build_reconstituted()
            .expect("ReconstitutedStrategyConfigBuilder standard fixture data mapping failed")
    }
}