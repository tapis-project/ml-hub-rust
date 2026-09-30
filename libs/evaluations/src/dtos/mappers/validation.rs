use std::collections::HashSet;

use crate::EvaluatorError;

pub(crate) fn validate_name(name: &str, kind: &str) -> Result<(), EvaluatorError> {
    if name.trim().is_empty() {
        return Err(invalid(format!("{kind} name cannot be empty")));
    }

    Ok(())
}

pub(crate) fn validate_parameters(
    parameters: &[String],
    owner: &str,
) -> Result<(), EvaluatorError> {
    let mut unique = HashSet::new();

    for parameter in parameters {
        if parameter.trim().is_empty() {
            return Err(invalid(format!("{owner} contains an empty parameter")));
        }

        if !unique.insert(parameter) {
            return Err(invalid(format!(
                "{owner} contains duplicate parameter {parameter}"
            )));
        }
    }

    Ok(())
}

pub(crate) fn validate_groups<T>(groups: &[Vec<T>], owner: &str) -> Result<(), EvaluatorError> {
    if groups.is_empty() {
        return Err(invalid(format!("{owner} requires at least one group")));
    }

    if groups.iter().any(Vec::is_empty) {
        return Err(invalid(format!("{owner} cannot contain an empty group")));
    }

    Ok(())
}

pub(crate) fn invalid(message: impl Into<String>) -> EvaluatorError {
    EvaluatorError::InvalidConfiguration(message.into())
}
