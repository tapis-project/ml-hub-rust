use crate::domain::entities::deployment::ParallelismStrategy;
use crate::domain::entities::deployment_strategy::strategy::StrategyConfigError;
use crate::domain::entities::deployment_strategy as entities;
use crate::infra::deployment::fs::dtos;
use crate::shared_kernel::value_objects::Ttl;
use crate::shared_kernel::enums::DeploymentModality;

impl TryFrom<dtos::client_strategy::ClientStrategy> for entities::client_strategy::ClientStrategy {
    type Error = entities::client_strategy::ClientStrategyError;

    fn try_from(value: dtos::client_strategy::ClientStrategy) -> Result<Self, Self::Error> {
        let rule_sets = match value.rule_sets {
            Some(rs) => {
                Some(
                    rs.iter().map(|rs| {
                        entities::rule_set::RuleSet::from(rs.clone())
                    }).collect()
                )
            },
            None => None
        };
        
        let parameter_set = match value.parameter_set {
            Some(ps) => Some(entities::parameter_set::ParameterSet::from(ps)),
            None => None,
        };
        
        let client_strat = entities::client_strategy::ClientStrategy::reconstitute(
            value.name,
            value.description,
            rule_sets,
            parameter_set,
            value.use_rule_sets,
            value.use_parameter_set,
            entities::strategy::StrategyConfig::try_from(value.config)?,
            value.enabled,
        )?;

        Ok(client_strat)
    }
}

impl TryFrom<dtos::client_strategy::StrategyConfig> for entities::strategy::StrategyConfig {
    type Error = StrategyConfigError;

    fn try_from(value: dtos::client_strategy::StrategyConfig) -> Result<Self, Self::Error> {
        Ok(entities::strategy::StrategyConfig::reconstitute(
            entities::strategy::ReconstitueStrategyConfigProps {
                max_ttl: value.max_ttl.and_then(|ttl| Some(Ttl::from_minutes(ttl))),
                supported_paralellism_strategies: value.supported_paralellism_strategies
                    .unwrap_or(vec![])
                    .into_iter()
                    .map(|ps| ParallelismStrategy::from(ps))
                    .collect(),
                supported_deployment_modalities: value.supported_deployment_modalities
                    .iter()
                    .map(|dm| DeploymentModality::from(dm.clone()))
                    .collect(),
                min_replicas: value.min_replicas,
                max_replicas: value.max_replicas,
            }
        )?)
    }
}