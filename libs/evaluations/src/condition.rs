use serde_json::{Number, Value};
use thiserror::Error;

use crate::{Arguments, Evaluate, FieldPath, OperandError, Operator, ValueResolutionError};

#[derive(Debug)]
pub struct Condition {
    negate: bool,
    case_insensitive: bool,
    operator: Operator,
    operands: Operands,
}

impl Condition {
    pub(crate) fn new(
        negate: bool,
        case_insensitive: bool,
        operator: Operator,
        operands: Operands,
    ) -> Self {
        Self {
            negate,
            case_insensitive,
            operator,
            operands,
        }
    }

    pub fn negate(&self) -> bool {
        self.negate
    }

    pub fn case_insensitive(&self) -> bool {
        self.case_insensitive
    }

    pub fn operator(&self) -> Operator {
        self.operator
    }

    pub fn operands(&self) -> &Operands {
        &self.operands
    }

    fn resolve_operand_value(
        operand: &Operand,
        arguments: &Arguments,
    ) -> Result<Value, ConditionEvaluationError> {
        let (mut value, operations) = match operand {
            Operand::Literal(operand) => (operand.value.clone(), operand.operations.as_slice()),
            Operand::Parameter(operand) => {
                let value = arguments
                    .get(&operand.parameter)
                    .ok_or_else(|| {
                        ConditionEvaluationError::MissingArgument(operand.parameter.clone())
                    })?
                    .resolve_value(operand.accessor.clone())?;

                (Value::from(value), operand.operations.as_slice())
            }
        };

        for operation in operations {
            value = operation.apply(value)?;
        }

        Ok(value)
    }
}

impl Evaluate for Condition {
    type Error = ConditionEvaluationError;

    fn evaluate(&self, arguments: &Arguments) -> Result<bool, Self::Error> {
        let left = Self::resolve_operand_value(self.operands.left(), arguments)?;
        let right = Self::resolve_operand_value(self.operands.right(), arguments)?;

        let result = self
            .operator
            .evaluate_with_options(&left, &right, self.case_insensitive)?;

        Ok(self.negate ^ result)
    }
}

#[derive(Debug)]
pub struct Operands {
    left: Operand,
    right: Operand,
}

impl Operands {
    pub(crate) fn new(left: Operand, right: Operand) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> &Operand {
        &self.left
    }

    pub fn right(&self) -> &Operand {
        &self.right
    }
}

#[derive(Debug)]
pub enum Operand {
    Parameter(ParameterOperand),
    Literal(LiteralOperand),
}

impl Operand {
    pub fn operations(&self) -> &[Operation] {
        match self {
            Self::Parameter(operand) => &operand.operations,
            Self::Literal(operand) => &operand.operations,
        }
    }
}

#[derive(Debug)]
pub struct ParameterOperand {
    parameter: String,
    accessor: Option<FieldPath>,
    operations: Vec<Operation>,
}

impl ParameterOperand {
    pub(crate) fn new(
        parameter: String,
        accessor: Option<FieldPath>,
        operations: Vec<Operation>,
    ) -> Self {
        Self {
            parameter,
            accessor,
            operations,
        }
    }

    pub fn parameter(&self) -> &str {
        &self.parameter
    }

    pub fn accessor(&self) -> Option<&FieldPath> {
        self.accessor.as_ref()
    }
}

#[derive(Debug)]
pub struct LiteralOperand {
    value: Value,
    operations: Vec<Operation>,
}

impl LiteralOperand {
    pub(crate) fn new(value: Value, operations: Vec<Operation>) -> Self {
        Self { value, operations }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Coalesce { from: Value, to: Value },
    Multiply(Vec<f64>),
}

impl Operation {
    pub fn apply(&self, value: Value) -> Result<Value, OperationError> {
        match self {
            Self::Coalesce { from, to } if &value == from => Ok(to.clone()),
            Self::Coalesce { .. } => Ok(value),
            Self::Multiply(factors) => {
                let value = value.as_f64().ok_or_else(|| {
                    OperationError::InvalidOperand("multiply requires a numeric value".into())
                })?;
                let value = factors.iter().fold(value, |value, factor| value * factor);
                let value = Number::from_f64(value).ok_or_else(|| {
                    OperationError::InvalidOperand(
                        "multiply produced a non-finite numeric value".into(),
                    )
                })?;

                Ok(Value::Number(value))
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("Invalid operation operand: {0}")]
    InvalidOperand(String),
}

#[derive(Error, Debug)]
pub enum ConditionEvaluationError {
    #[error(transparent)]
    Operand(#[from] OperandError),

    #[error(transparent)]
    Operation(#[from] OperationError),

    #[error(transparent)]
    ValueResolution(#[from] ValueResolutionError),

    #[error("Missing argument: Expected a value for {0} but none was found")]
    MissingArgument(String),
}
