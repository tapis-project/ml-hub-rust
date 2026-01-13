use crate::domain::entities::automated_deployment_strategy::strategy as entities;
use crate::presentation::http::v1::responses::deployment::{
    rule_set::RuleSet,
    parameter_set::ParameterSet,
};
use crate::presentation::http::v1::responses::deployment::strategy as dtos;

impl From<entities::Strategy> for dtos::Strategy {
    fn from(value: entities::Strategy) -> Self {
        Self {
            name: value.name.clone(),
            description: value.description.clone(),
            rule_sets: value.rule_sets()
                .iter()
                .map(|rs| RuleSet::from(rs.clone()))
                .collect(),
            parameter_set: value.parameter_set()
                .clone()
                .and_then(|ps| Some(ParameterSet::from(ps)))
        }
    }
}