use crate::domain::entities::deployment_strategy as entities;
use crate::infra::deployment::fs::dtos;

impl From<dtos::parameter_set::Parameter> for entities::parameter_set::Parameter {
    fn from(value: dtos::parameter_set::Parameter) -> Self {
        let choices = match value.choices {
            Some(cs) => {
                Some(
                    cs.iter()
                        .map(|c| entities::parameter_set::Choice::from(c.clone()))
                        .collect::<Vec<entities::parameter_set::Choice>>()
                )
            },
            None => None
        };

        Self {
            name: value.name,
            description: value.description,
            required: value.required.unwrap_or(false),
            secret: value.secret.unwrap_or(false),
            r#type: entities::parameter_set::ParameterType::from(value.r#type),
            choices,
            default: value.default,
        }
    }
}

impl From<dtos::parameter_set::ParameterType> for entities::parameter_set::ParameterType {
    fn from(value: dtos::parameter_set::ParameterType) -> Self {
        use dtos::parameter_set::ParameterType;
        match value {
            ParameterType::String => entities::parameter_set::ParameterType::String,
            ParameterType::Integer => entities::parameter_set::ParameterType::Integer,
            ParameterType::Float => entities::parameter_set::ParameterType::Float,
            ParameterType::Boolean => entities::parameter_set::ParameterType::Boolean,
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


impl From<dtos::parameter_set::Choice> for entities::parameter_set::Choice {
    fn from(value: dtos::parameter_set::Choice) -> Self {
        Self {
            value: value.value,
            description: value.description.clone(),
            enabled: value.enabled.unwrap_or(true),
        }
    }
}
