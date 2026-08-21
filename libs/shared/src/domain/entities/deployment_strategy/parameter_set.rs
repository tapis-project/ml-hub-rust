use std::collections::{HashMap, HashSet};

use serde::Serialize;
use thiserror::Error;

use crate::domain::entities::deployment::argument::Argument;

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
pub enum ParameterSetError {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String)
}

#[derive(Clone, Debug, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}

impl ParameterSet {
    pub fn validate_arguments(&self, args: &[Argument]) -> Result<(), ParameterSetError> {
        let params_map:HashMap<&str, &Parameter> = self.parameters
            .iter()
            .map(|p| (p.name.as_str(), p))
            .collect();

        let args_hashset: HashSet<&str> = args.iter()
            .map(|a| a.parameter_name.as_str())
            .collect();

        let extraneous_args: Vec<&str> = args_hashset
            .difference(&params_map.keys().copied().collect::<HashSet<&str>>())
            .copied()
            .collect();

        // Invairant: Extraneous arguments are NOT permitted
        if extraneous_args.len() > 0 {
            return Err(ParameterSetError::InvalidArgument(format!("Extraneous args: {:?}", &extraneous_args)))
        }

        let missing_args: Vec<&str> = params_map.keys()
            .copied()
            .collect::<HashSet<&str>>()
            .difference(&args_hashset)
            .copied()
            .collect();

        let missing_required_args: Vec<&str> = missing_args.iter()
            .copied()
            .filter(|name| {
                params_map.get(name).map_or(false, |p| p.required)
            })
            .collect();

        // Invariant: All required parameters MUST have a corresponding argument.
        if missing_required_args.len() > 0 {
            return Err(ParameterSetError::InvalidArgument(format!("Missing required arguments: {:?}", &missing_required_args)))
        }

        Ok(())
    }

    pub fn get_required_params(&self) -> Vec<&Parameter> {
        self.parameters.iter()
            .filter(|p| p.required)
            .collect()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Choice {
    pub value: String,
    pub description: Option<String>,
    pub enabled: bool
}