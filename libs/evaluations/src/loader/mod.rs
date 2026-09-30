use std::{
    path::{Path, PathBuf},
    rc::Rc,
};

use thiserror::Error;

use crate::{Arguments, Evaluate, Evaluation, Statement, StatementEvaluationError, dtos::Config};

#[derive(Debug)]
pub struct Evaluator {
    evaluations: Vec<Evaluation>,
    statements: Vec<Rc<Statement>>,
}

impl Evaluator {
    pub fn load(config_path: impl AsRef<Path>) -> Result<Self, EvaluatorError> {
        let path = config_path.as_ref();
        let contents =
            std::fs::read_to_string(path).map_err(|source| EvaluatorError::FileRead {
                path: path.to_path_buf(),
                source,
            })?;

        if contents.trim().is_empty() {
            return Err(EvaluatorError::InvalidConfiguration(
                "evaluation configuration cannot be empty".into(),
            ));
        }

        let config = serde_json::from_str::<Config>(&contents).map_err(|source| {
            EvaluatorError::Deserialization {
                path: path.to_path_buf(),
                source,
            }
        })?;

        config.try_into()
    }

    pub(crate) fn from_parts(evaluations: Vec<Evaluation>, statements: Vec<Rc<Statement>>) -> Self {
        Self {
            evaluations,
            statements,
        }
    }

    pub fn evaluations(&self) -> &[Evaluation] {
        &self.evaluations
    }

    pub fn statements(&self) -> &[Rc<Statement>] {
        &self.statements
    }

    pub fn evaluate(
        &self,
        evaluation_name: &str,
        arguments: &Arguments,
    ) -> Result<bool, EvaluatorError> {
        let evaluation = self
            .evaluations
            .iter()
            .find(|evaluation| evaluation.name() == evaluation_name)
            .ok_or_else(|| EvaluatorError::EvaluationNotFound(evaluation_name.into()))?;

        Self::ensure_arguments(evaluation, arguments)?;

        evaluation
            .evaluate(arguments)
            .map_err(|source| EvaluatorError::Evaluation {
                evaluation: evaluation.name().into(),
                source,
            })
    }

    pub fn evaluate_all(&self, arguments: &Arguments) -> Result<EvaluationResult, EvaluatorError> {
        for evaluation in &self.evaluations {
            Self::ensure_arguments(evaluation, arguments)?;
        }

        let mut passes = Vec::new();
        let mut fails = Vec::new();

        for evaluation in &self.evaluations {
            let passes_evaluation =
                evaluation
                    .evaluate(arguments)
                    .map_err(|source| EvaluatorError::Evaluation {
                        evaluation: evaluation.name().into(),
                        source,
                    })?;

            if passes_evaluation {
                passes.push(evaluation.name().into());
            } else {
                fails.push(evaluation.name().into());
            }
        }

        Ok(EvaluationResult { passes, fails })
    }

    fn ensure_arguments(
        evaluation: &Evaluation,
        arguments: &Arguments,
    ) -> Result<(), EvaluatorError> {
        for parameter in evaluation.parameters() {
            if !arguments.contains_key(parameter) {
                return Err(EvaluatorError::MissingArgument {
                    evaluation: evaluation.name().into(),
                    parameter: parameter.clone(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct EvaluationResult {
    passes: Vec<String>,
    fails: Vec<String>,
}

impl EvaluationResult {
    pub fn passes(&self) -> &[String] {
        &self.passes
    }

    pub fn fails(&self) -> &[String] {
        &self.fails
    }
}

#[derive(Debug, Error)]
pub enum EvaluatorError {
    #[error("Unable to read evaluation configuration at {path}: {source}")]
    FileRead {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Unable to deserialize evaluation configuration at {path}: {source}")]
    Deserialization {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },

    #[error("Invalid evaluation configuration: {0}")]
    InvalidConfiguration(String),

    #[error("Evaluation {evaluation} references missing statement {statement}")]
    MissingStatementReference {
        evaluation: String,
        statement: String,
    },

    #[error("Evaluation not found: {0}")]
    EvaluationNotFound(String),

    #[error("Evaluation {evaluation} requires argument {parameter}")]
    MissingArgument {
        evaluation: String,
        parameter: String,
    },

    #[error("Unable to evaluate {evaluation}: {source}")]
    Evaluation {
        evaluation: String,
        #[source]
        source: StatementEvaluationError,
    },
}

#[cfg(test)]
#[path = "loader.test.rs"]
mod loader_test;
