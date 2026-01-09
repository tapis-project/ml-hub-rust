use crate::domain::entities::automated_deployment_strategy as entities;
use crate::infra::deployment::fs::dtos;

impl From<dtos::parameter_set::Parameter> for entities::parameter_set::Parameter {
    fn from(value: dtos::parameter_set::Parameter) -> Self {
        Self {
            name: value.name
        }
    }
}

impl From<dtos::parameter_set::ParameterSet> for entities::parameter_set::ParameterSet {
    fn from(value: dtos::parameter_set::ParameterSet) -> Self {
        Self {
            name: value.name,
            parameters: value.parameters.iter()
                .map(|p| entities::parameter_set::Parameter::from(p.clone()))
                .collect()
        }
    }
}