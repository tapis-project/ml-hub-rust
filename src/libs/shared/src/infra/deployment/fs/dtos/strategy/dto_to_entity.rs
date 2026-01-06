use crate::domain::entities::automated_deployment_strategy as entities;
use crate::infra::deployment::fs::dtos;

impl TryFrom<dtos::strategy::Strategy> for entities::strategy::Strategy {
    type Error = entities::strategy::StrategyError;

    fn try_from(value: dtos::strategy::Strategy) -> Result<Self, Self::Error> {
        let parameter_set = match value.parameter_set {
            Some(ps) => Some(entities::parameter_set::ParameterSet::from(ps)),
            None => None,
        };
        
        let strat = entities::strategy::Strategy::new(
            value.name,
            value.description,
            value.rule_sets.iter().map(|rs| {
                entities::rule_set::RuleSet::from(rs.clone())
            }).collect(),
            parameter_set
        )?;

        Ok(strat)
    }
}