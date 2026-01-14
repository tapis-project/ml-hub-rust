use crate::domain::entities::deployment_strategy::parameter_set as entities;
use crate::presentation::http::v1::responses::deployment::parameter_set as dtos;

impl From<entities::Parameter> for dtos::Parameter {
    fn from(value: entities::Parameter) -> Self {
        Self {
            name: value.name
        }
    }
}

impl From<entities::ParameterSet> for dtos::ParameterSet {
    fn from(value: entities::ParameterSet) -> Self {
        Self {
            name: value.name,
            parameters: value.parameters
                .iter()
                .map(|p| dtos::Parameter::from(p.clone()))
                .collect()
        }
    }
}