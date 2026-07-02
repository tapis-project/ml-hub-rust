use crate::domain::entities::deployment_strategy::parameter_set as entities;
use crate::presentation::http::v1::responses::deployment::parameter_set as dtos;

impl From<entities::Parameter> for dtos::Parameter {
    fn from(value: entities::Parameter) -> Self {
        Self {
            name: value.name,
            description: value.description,
            required: value.required,
            secret: value.secret,
            r#type: dtos::ParameterType::from(value.r#type)
        }
    }
}

impl From<entities::ParameterType> for dtos::ParameterType {
    fn from(value: entities::ParameterType) -> Self {
        use entities::ParameterType;
        match value {
            ParameterType::String { choices, default } => dtos::ParameterType::String { choices, default },
            ParameterType::Integer { choices, default } => dtos::ParameterType::Integer { choices, default },
            ParameterType::Float { choices, default } => dtos::ParameterType::Float { choices, default },
            ParameterType::Boolean { default } => dtos::ParameterType::Boolean { default },
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