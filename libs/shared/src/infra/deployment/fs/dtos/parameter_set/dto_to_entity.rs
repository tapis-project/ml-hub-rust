use crate::domain::entities::deployment_strategy as entities;
use crate::infra::deployment::fs::dtos;

impl From<dtos::parameter_set::Parameter> for entities::parameter_set::Parameter {
    fn from(value: dtos::parameter_set::Parameter) -> Self {
        Self {
            name: value.name,
            description: value.description,
            required: value.required,
            secret: value.secret,
            r#type: entities::parameter_set::ParameterType::from(value.r#type)
        }
    }
}

impl From<dtos::parameter_set::ParameterType> for entities::parameter_set::ParameterType {
    fn from(value: dtos::parameter_set::ParameterType) -> Self {
        use dtos::parameter_set::ParameterType;
        match value {
            ParameterType::String { choices, default } => entities::parameter_set::ParameterType::String { choices, default },
            ParameterType::Integer { choices, default } => entities::parameter_set::ParameterType::Integer { choices, default },
            ParameterType::Float { choices, default } => entities::parameter_set::ParameterType::Float { choices, default },
            ParameterType::Boolean { default } => entities::parameter_set::ParameterType::Boolean { default },
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