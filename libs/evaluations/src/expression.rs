use std::rc::Rc;

use crate::{Arguments, Evaluate, Statement, StatementEvaluationError};

#[derive(Debug)]
pub struct Expression {
    statements: Vec<Rc<Statement>>,
}

impl Expression {
    pub(crate) fn new(statements: Vec<Rc<Statement>>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &[Rc<Statement>] {
        &self.statements
    }

    pub(crate) fn evaluate_all(
        &self,
        arguments: &Arguments,
    ) -> Result<bool, StatementEvaluationError> {
        for statement in &self.statements {
            if !statement.evaluate(arguments)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub(crate) fn evaluate_any(
        &self,
        arguments: &Arguments,
    ) -> Result<bool, StatementEvaluationError> {
        for statement in &self.statements {
            if statement.evaluate(arguments)? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
