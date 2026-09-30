use thiserror::Error;

use crate::{Arguments, Condition, ConditionEvaluationError, Evaluate, EvaluationStrategy};

#[derive(Debug)]
pub struct Statement {
    name: String,
    evaluation_strategy: EvaluationStrategy,
    parameters: Vec<String>,
    conditions: Vec<Vec<Condition>>,
}

impl Statement {
    pub(crate) fn new(
        name: String,
        evaluation_strategy: EvaluationStrategy,
        parameters: Vec<String>,
        conditions: Vec<Vec<Condition>>,
    ) -> Self {
        Self {
            name,
            evaluation_strategy,
            parameters,
            conditions,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn evaluation_strategy(&self) -> EvaluationStrategy {
        self.evaluation_strategy
    }

    pub fn parameters(&self) -> &[String] {
        &self.parameters
    }

    pub fn conditions(&self) -> &[Vec<Condition>] {
        &self.conditions
    }

    fn evaluate_dnf(&self, arguments: &Arguments) -> Result<bool, StatementEvaluationError> {
        for group in &self.conditions {
            let mut passes = true;

            for condition in group {
                if !condition.evaluate(arguments)? {
                    passes = false;
                    break;
                }
            }

            if passes {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn evaluate_cnf(&self, arguments: &Arguments) -> Result<bool, StatementEvaluationError> {
        for group in &self.conditions {
            let mut passes = false;

            for condition in group {
                if condition.evaluate(arguments)? {
                    passes = true;
                    break;
                }
            }

            if !passes {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

impl Evaluate for Statement {
    type Error = StatementEvaluationError;

    fn evaluate(&self, arguments: &Arguments) -> Result<bool, Self::Error> {
        match self.evaluation_strategy {
            EvaluationStrategy::Dnf => self.evaluate_dnf(arguments),
            EvaluationStrategy::Cnf => self.evaluate_cnf(arguments),
        }
    }
}

#[derive(Error, Debug)]
pub enum StatementEvaluationError {
    #[error(transparent)]
    Condition(#[from] ConditionEvaluationError),
}
