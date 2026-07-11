use crate::domain::entities::deployment_strategy::strategy as entities;
use crate::presentation::http::v1::responses::deployment:: parameter_set::Parameter;
use crate::presentation::http::v1::responses::deployment::strategy as dtos;

impl From<entities::Strategy> for dtos::Strategy {
    fn from(value: entities::Strategy) -> Self {
        let parameters: Vec<Parameter> = value
                .parameter_set()
                .clone()
                .map(|ps| ps.parameters )
                .unwrap_or(vec![])
                .into_iter()
                .map(|p| Parameter::from(p))
                .collect();

        Self {
            name: value.name.clone(),
            description: value.description.clone(),
            platform: value.platform.clone(),
            parameters,
            // rule_sets: value.rule_sets()
            //     .iter()
            //     .map(|rs| RuleSet::from(rs.clone()))
            //     .collect(),
            
        }
    }
}