use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{Evaluator, EvaluatorError};

use super::{convert_evaluation, convert_statement, invalid};
use crate::dtos::Config;

impl TryFrom<Config> for Evaluator {
    type Error = EvaluatorError;

    fn try_from(value: Config) -> Result<Self, Self::Error> {
        if value.evaluations.is_empty() {
            return Err(invalid("at least one evaluation is required"));
        }

        if value.statements.is_empty() {
            return Err(invalid("at least one statement is required"));
        }

        let mut statement_names = HashSet::new();
        let mut statements = Vec::with_capacity(value.statements.len());
        let mut statements_by_name = HashMap::with_capacity(value.statements.len());

        for statement in value.statements {
            if !statement_names.insert(statement.name.clone()) {
                return Err(invalid(format!(
                    "duplicate statement name: {}",
                    statement.name
                )));
            }

            let statement = Rc::new(convert_statement(statement)?);

            statements_by_name.insert(statement.name().to_owned(), statement.clone());
            statements.push(statement);
        }

        let mut evaluation_names = HashSet::new();
        let mut evaluations = Vec::with_capacity(value.evaluations.len());

        for evaluation in value.evaluations {
            if !evaluation_names.insert(evaluation.name.clone()) {
                return Err(invalid(format!(
                    "duplicate evaluation name: {}",
                    evaluation.name
                )));
            }

            evaluations.push(convert_evaluation(evaluation, &statements_by_name)?);
        }

        Ok(Evaluator::from_parts(evaluations, statements))
    }
}
