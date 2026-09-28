use crate::{EvaluatorError, Statement};

use super::{convert_condition, validate_groups, validate_name, validate_parameters};
use crate::dtos::Statement as StatementDto;

pub(crate) fn convert_statement(value: StatementDto) -> Result<Statement, EvaluatorError> {
    validate_name(&value.name, "statement")?;

    validate_parameters(&value.parameters, &format!("statement {}", value.name))?;

    validate_groups(&value.conditions, &format!("statement {}", value.name))?;

    let conditions = value
        .conditions
        .into_iter()
        .map(|group| {
            group
                .into_iter()
                .map(|condition| convert_condition(condition, &value.parameters))
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Statement::new(
        value.name,
        value.evaluation_strategy.into(),
        value.parameters,
        conditions,
    ))
}
