use std::collections::{HashMap, HashSet};

use serde::Serialize;
use thiserror::Error;

#[derive(Clone, Debug, Serialize)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    pub r#type: ParameterType,
    pub choices: Option<Vec<Choice>>,
    pub default: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
}

#[derive(Clone, Debug, Error)]
pub enum DeploymentParameterError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Deployment parameter name is defined more than once: {0}")]
    DuplicateParameter(String),
}

#[derive(Clone, Debug, Serialize)]
pub struct DeploymentParameters {
    parameters: Vec<Parameter>,
}

impl DeploymentParameters {
    pub fn new(parameters: Vec<Parameter>) -> Result<Self, DeploymentParameterError> {
        let mut names = HashSet::new();

        for parameter in &parameters {
            if !names.insert(parameter.name.as_str()) {
                return Err(DeploymentParameterError::DuplicateParameter(
                    parameter.name.clone(),
                ));
            }
        }

        Ok(Self { parameters })
    }

    pub fn definitions(&self) -> &[Parameter] {
        &self.parameters
    }

    pub fn into_definitions(self) -> Vec<Parameter> {
        self.parameters
    }

    pub fn resolve(
        &self,
        values: &[(String, String)],
    ) -> Result<Vec<ResolvedDeploymentParameter>, DeploymentParameterError> {
        let mut names = HashSet::new();

        for (name, _) in values {
            if !names.insert(name.as_str()) {
                return Err(DeploymentParameterError::InvalidArgument(format!(
                    "Duplicate argument: {name}"
                )));
            }
        }

        let parameters = self
            .parameters
            .iter()
            .map(|parameter| (parameter.name.as_str(), parameter))
            .collect::<HashMap<_, _>>();

        for (name, _) in values {
            if !parameters.contains_key(name.as_str()) {
                return Err(DeploymentParameterError::InvalidArgument(format!(
                    "Extraneous argument: {name}"
                )));
            }
        }

        let mut resolved = Vec::with_capacity(self.parameters.len());

        for parameter in &self.parameters {
            let value = values
                .iter()
                .find(|(name, _)| name == &parameter.name)
                .map(|(_, value)| value.clone())
                .or_else(|| parameter.default.clone());

            let value = match value {
                Some(value) => value,
                None if parameter.required => {
                    return Err(DeploymentParameterError::InvalidArgument(format!(
                        "Missing required argument: {}",
                        parameter.name
                    )));
                }
                None => continue,
            };

            validate_parameter_value(parameter, &value)?;

            resolved.push(ResolvedDeploymentParameter {
                name: parameter.name.clone(),
                value,
                secret: parameter.secret,
            });
        }

        Ok(resolved)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedDeploymentParameter {
    name: String,
    value: String,
    secret: bool,
}

impl ResolvedDeploymentParameter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn secret(&self) -> bool {
        self.secret
    }
}

fn validate_parameter_value(
    parameter: &Parameter,
    value: &str,
) -> Result<(), DeploymentParameterError> {
    let valid_type = match parameter.r#type {
        ParameterType::String => true,
        ParameterType::Integer => value.parse::<i64>().is_ok(),
        ParameterType::Float => value.parse::<f64>().is_ok(),
        ParameterType::Boolean => value.parse::<bool>().is_ok(),
    };

    if !valid_type {
        return Err(DeploymentParameterError::InvalidArgument(format!(
            "Argument '{}' has an invalid value for its type",
            parameter.name
        )));
    }

    if let Some(choices) = &parameter.choices {
        let choice = choices
            .iter()
            .find(|choice| choice.value == value)
            .ok_or_else(|| {
                DeploymentParameterError::InvalidArgument(format!(
                    "Argument '{}' has an unsupported value",
                    parameter.name
                ))
            })?;

        if !choice.enabled {
            return Err(DeploymentParameterError::InvalidArgument(format!(
                "Argument '{}' selected a disabled value",
                parameter.name
            )));
        }
    }

    Ok(())
}

#[derive(Clone, Debug, Serialize)]
pub struct Choice {
    pub value: String,
    pub description: Option<String>,
    pub enabled: bool,
}

#[cfg(test)]
#[path = "deployment_parameters.test.rs"]
mod deployment_parameters_test;
