use crate::domain::entities::deployment_strategy as entities;
use crate::shared_kernel::enums::DeploymentModality as DeploymentModalityEntity;
use crate::infra::deployment::fs::dtos;

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

        
        let client_strat = entities::client_strategy::ClientStrategy::new(
            value.name,
            value.description,
            DeploymentModalityEntity::from(value.deployment_modality),
            rule_sets,
            parameter_set,
            value.use_rule_sets,
            value.use_parameter_set
        )?;

        Ok(client_strat)
    }
}