use crate::domain::entities::deployment_strategy::parameter_set as entities;
use crate::presentation::http::v1::responses::deployment::parameter_set as dtos;

impl From<entities::Parameter> for dtos::Parameter {
    fn from(value: entities::Parameter) -> Self {
        Self {
            name: value.name,
            description: value.description,
            required: value.required,
            secret: value.secret,
            r#type: dtos::ParameterType::from(value.r#type),
            choices: value.choices,
            default: value.default,
        }
    }
}

impl From<entities::ParameterType> for dtos::ParameterType {
    fn from(value: entities::ParameterType) -> Self {
        use entities::ParameterType;
        match value {
            ParameterType::String => dtos::ParameterType::String,
            ParameterType::Integer => dtos::ParameterType::Integer,
            ParameterType::Float => dtos::ParameterType::Float,
            ParameterType::Boolean => dtos::ParameterType::Boolean,
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