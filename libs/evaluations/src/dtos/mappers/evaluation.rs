use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    Evaluation, EvaluationStrategy, EvaluationType, EvaluatorError, Expression, Statement,
};

use super::{invalid, validate_groups, validate_name, validate_parameters};
use crate::dtos::{
    Evaluation as EvaluationDto, EvaluationStrategy as EvaluationStrategyDto,
    EvaluationType as EvaluationTypeDto,
};

pub(crate) fn convert_evaluation(
    value: EvaluationDto,
    statements: &HashMap<String, Rc<Statement>>,
) -> Result<Evaluation, EvaluatorError> {
    validate_name(&value.name, "evaluation")?;

    validate_parameters(&value.parameters, &format!("evaluation {}", value.name))?;

    validate_groups(&value.expressions, &format!("evaluation {}", value.name))?;

    let declared_parameters = value.parameters.iter().collect::<HashSet<_>>();
    let mut referenced_names = HashSet::new();
    let mut expressions = Vec::with_capacity(value.expressions.len());

    for group in value.expressions {
        let mut referenced_statements = Vec::with_capacity(group.len());

        for statement_name in group {
            if !referenced_names.insert(statement_name.clone()) {
                return Err(invalid(format!(
                    "evaluation {} references statement {} more than once",
                    value.name, statement_name
                )));
            }

            let statement = statements.get(&statement_name).ok_or_else(|| {
                EvaluatorError::MissingStatementReference {
                    evaluation: value.name.clone(),
                    statement: statement_name.clone(),
                }
            })?;

            if let Some(parameter) = statement
                .parameters()
                .iter()
                .find(|parameter| !declared_parameters.contains(parameter))
            {
                return Err(invalid(format!(
                    "evaluation {} does not declare parameter {} required by statement {}",
                    value.name,
                    parameter,
                    statement.name()
                )));
            }

            referenced_statements.push(statement.clone());
        }

        expressions.push(Expression::new(referenced_statements));
    }

    Ok(Evaluation::new(
        value.name,
        value.evaluation_strategy.into(),
        value.parameters,
        value.evaluation_type.map(Into::into),
        expressions,
    ))
}

impl From<EvaluationStrategyDto> for EvaluationStrategy {
    fn from(value: EvaluationStrategyDto) -> Self {
        match value {
            EvaluationStrategyDto::Dnf => Self::Dnf,
            EvaluationStrategyDto::Cnf => Self::Cnf,
        }
    }
}

impl From<EvaluationTypeDto> for EvaluationType {
    fn from(value: EvaluationTypeDto) -> Self {
        match value {
            EvaluationTypeDto::ModelTrust => Self::ModelTrust,
        }
    }
}
