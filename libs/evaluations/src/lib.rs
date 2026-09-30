mod condition;
mod dtos;
mod expression;
mod field_path;
mod loader;
mod operators;
mod statement;
mod traits;

use std::{collections::HashMap, rc::Rc};

pub use condition::*;
pub use expression::*;
pub use field_path::*;
pub use loader::*;
pub use operators::*;
pub use statement::*;
pub use traits::*;

pub type Arguments = HashMap<String, Rc<dyn ResolveValue>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvaluationStrategy {
    Dnf,
    Cnf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvaluationType {
    ModelTrust,
}

#[derive(Debug)]
pub struct Evaluation {
    name: String,
    evaluation_strategy: EvaluationStrategy,
    parameters: Vec<String>,
    evaluation_type: Option<EvaluationType>,
    expressions: Vec<Expression>,
}

impl Evaluation {
    pub(crate) fn new(
        name: String,
        evaluation_strategy: EvaluationStrategy,
        parameters: Vec<String>,
        evaluation_type: Option<EvaluationType>,
        expressions: Vec<Expression>,
    ) -> Self {
        Self {
            name,
            evaluation_strategy,
            parameters,
            evaluation_type,
            expressions,
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

    pub fn evaluation_type(&self) -> Option<EvaluationType> {
        self.evaluation_type
    }

    pub fn expressions(&self) -> &[Expression] {
        &self.expressions
    }

    fn evaluate_dnf(&self, arguments: &Arguments) -> Result<bool, StatementEvaluationError> {
        for expression in &self.expressions {
            if expression.evaluate_all(arguments)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn evaluate_cnf(&self, arguments: &Arguments) -> Result<bool, StatementEvaluationError> {
        for expression in &self.expressions {
            if !expression.evaluate_any(arguments)? {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

impl Evaluate for Evaluation {
    type Error = StatementEvaluationError;

    fn evaluate(&self, arguments: &Arguments) -> Result<bool, Self::Error> {
        match self.evaluation_strategy {
            EvaluationStrategy::Dnf => self.evaluate_dnf(arguments),
            EvaluationStrategy::Cnf => self.evaluate_cnf(arguments),
        }
    }
}
