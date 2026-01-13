use crate::domain::entities::automated_deployment_strategy::client_strategy_set as entities;
use crate::presentation::http::v1::responses::deployment::{
    strategy::Strategy,
    rule_set::RuleSet,
    parameter_set::ParameterSet,
};
use crate::presentation::http::v1::responses::deployment::client_strategy_set as dtos;

impl From<entities::ClientStrategySet> for dtos::ClientStrategySet {
    fn from(value: entities::ClientStrategySet) -> Self {
        Self {
            client: value.client.clone(),
            description: value.description.clone(),
            rule_sets: value.rule_sets()
                .clone()
                .and_then(|rss| Some(rss
                        .iter()
                        .map(|rs| RuleSet::from(rs.clone()))
                        .collect())),
            parameter_sets: value.parameter_sets()
                        .clone()
                        .and_then(|pss| Some(pss
                                .iter()
                                .map(|ps| ParameterSet::from(ps.clone()))
                                .collect())),
            strategies: value.strategies()
                .clone()
                .iter()
                .map(|s| Strategy::from(s.clone()))
                .collect()
        }
    }
}