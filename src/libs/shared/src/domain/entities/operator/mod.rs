use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

fn get_type(value: &Value) -> String {
    match value {
        Value::Null => "null".into(),
        Value::Bool(_) => "bool".into(),
        Value::Number(_) => "number".into(),
        Value::String(_) => "string".into(),
        Value::Array(_) => "array".into(),
        Value::Object(_) => "object".into(),
    }
}

#[derive(Error, Debug)]
pub enum OperandError {
    #[error("{0}")]
    InvalidOperand(String),
    #[error("Operand Error: Invalid left-hand operand: Expected type {0} but found type {1}")]
    InvalidLeftOperand(String, String),
    #[error("Operand Error: Invalid right-hand operand: Expected type {0} but found type {1}")]
    InvalidRightOperand(String, String),
}

#[derive(Clone)]
pub enum Operator {
    Eq,
    Neq,
    Gte,
    Lte,
    Gt,
    Lt,
    Contains,
    NotIn,
    NoneIn,
    AnyIn,
    AllIn,
}

impl Operator {
    pub fn evaluate<L, R>(&self, l_operand: &L, r_operand: &R) -> Result<bool, OperandError>
    where
        L: Serialize,
        R: Serialize
    {
        let left = serde_json::to_value(l_operand).map_err(|err| OperandError::InvalidOperand(err.to_string()))?;
        let right = serde_json::to_value(r_operand).map_err(|err| OperandError::InvalidOperand(err.to_string()))?;

        match self {
            Operator::Eq =>  Ok(left == right),
            Operator::Neq =>  Ok(left != right),
            Operator::Gte =>  {
                let l = left.as_number()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("number".into(), get_type(&left)))?
                    .clone();
                let r = right.as_number()
                    .ok_or_else(|| OperandError::InvalidRightOperand("number".into(), get_type(&right)))?
                    .clone();

                Ok(l.as_f64() >= r.as_f64())
            },
            Operator::Lte => {
                let l = left.as_number()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("number".into(), get_type(&left)))?
                    .clone();
                let r = right.as_number()
                    .ok_or_else(|| OperandError::InvalidRightOperand("number".into(), get_type(&right)))?
                    .clone();

                Ok(l.as_f64() <= r.as_f64())
            },
            Operator::Gt => {
                let l = left.as_number()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("number".into(), get_type(&left)))?
                    .clone();
                let r = right.as_number()
                    .ok_or_else(|| OperandError::InvalidRightOperand("number".into(), get_type(&left)))?
                    .clone();

                Ok(l.as_f64() > r.as_f64())
            },
            Operator::Lt => {
                let l = left.as_number()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("number".into(), get_type(&left)))?
                    .clone();
                let r = right.as_number()
                    .ok_or_else(|| OperandError::InvalidRightOperand("number".into(), get_type(&left)))?
                    .clone();

                Ok(l.as_f64() < r.as_f64())
            },
            Operator::Contains => {
                let l = left.as_array()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("array".into(), get_type(&left)))?
                    .clone();
                Ok(l.contains(&right))
            },
            Operator::NotIn => {
                let r = right.as_array()
                    .ok_or_else(|| OperandError::InvalidRightOperand("array".into(), get_type(&left)))?
                    .clone();

                Ok(!r.contains(&left))
            },
            Operator::AnyIn => {
                let l = left.as_array()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("array".into(), get_type(&left)))?
                    .clone();
                let r = right.as_array()
                    .ok_or_else(|| OperandError::InvalidRightOperand("array".into(), get_type(&left)))?
                    .clone();
                for item in l {
                    if r.contains(&item) {
                        return Ok(true)
                    }
                }

                Ok(false)
            },
            Operator::AllIn => {
                let l = left.as_array()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("array".into(), get_type(&left)))?
                    .clone();
                let r = right.as_array()
                    .ok_or_else(|| OperandError::InvalidRightOperand("array".into(), get_type(&left)))?
                    .clone();

                for item in l {
                    if !r.contains(&item) {
                        return Ok(false)
                    }
                }

                Ok(true)
            },
            Operator::NoneIn => {
                let l = left.as_array()
                    .ok_or_else(|| OperandError::InvalidLeftOperand("array".into(), get_type(&left)))?
                    .clone();
                let r = right.as_array()
                    .ok_or_else(|| OperandError::InvalidRightOperand("array".into(), get_type(&left)))?
                    .clone();

                for item in l {
                    if r.contains(&item) {
                        return Ok(false)
                    }
                }

                Ok(true)
            }
        }
    }
}

#[cfg(test)]
#[path = "operator.test.rs"]
mod operator_test;