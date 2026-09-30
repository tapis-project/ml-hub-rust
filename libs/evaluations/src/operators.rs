use regex::RegexBuilder;
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;

fn get_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
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

    #[error("Invalid regular expression: {0}")]
    InvalidPattern(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Operator {
    Eq,
    Neq,
    Gte,
    Lte,
    Gt,
    Lt,
    In,
    Contains,
    NotIn,
    NoneIn,
    AnyIn,
    AllIn,
    Pattern,
}

impl Operator {
    pub fn evaluate<L, R>(&self, left: &L, right: &R) -> Result<bool, OperandError>
    where
        L: Serialize,
        R: Serialize,
    {
        self.evaluate_with_options(left, right, false)
    }

    pub fn evaluate_with_options<L, R>(
        &self,
        left: &L,
        right: &R,
        case_insensitive: bool,
    ) -> Result<bool, OperandError>
    where
        L: Serialize,
        R: Serialize,
    {
        let left = serde_json::to_value(left)
            .map_err(|error| OperandError::InvalidOperand(error.to_string()))?;
        let right = serde_json::to_value(right)
            .map_err(|error| OperandError::InvalidOperand(error.to_string()))?;

        match self {
            Self::Eq => Ok(values_equal(&left, &right, case_insensitive)),
            Self::Neq => Ok(!values_equal(&left, &right, case_insensitive)),
            Self::Gte => compare_numbers(&left, &right, |left, right| left >= right),
            Self::Lte => compare_numbers(&left, &right, |left, right| left <= right),
            Self::Gt => compare_numbers(&left, &right, |left, right| left > right),
            Self::Lt => compare_numbers(&left, &right, |left, right| left < right),
            Self::In => collection_contains(&right, &left, case_insensitive),
            Self::Contains => collection_contains(&left, &right, case_insensitive),
            Self::NotIn => collection_contains(&right, &left, case_insensitive).map(|value| !value),
            Self::AnyIn => collections_match(&left, &right, case_insensitive, CollectionMatch::Any),
            Self::AllIn => collections_match(&left, &right, case_insensitive, CollectionMatch::All),
            Self::NoneIn => {
                collections_match(&left, &right, case_insensitive, CollectionMatch::None)
            }
            Self::Pattern => evaluate_patterns(&left, &right, case_insensitive),
        }
    }
}

fn compare_numbers(
    left: &Value,
    right: &Value,
    compare: impl FnOnce(f64, f64) -> bool,
) -> Result<bool, OperandError> {
    let left = left
        .as_f64()
        .ok_or_else(|| OperandError::InvalidLeftOperand("number".into(), get_type(left).into()))?;

    let right = right.as_f64().ok_or_else(|| {
        OperandError::InvalidRightOperand("number".into(), get_type(right).into())
    })?;

    Ok(compare(left, right))
}

fn values_equal(left: &Value, right: &Value, case_insensitive: bool) -> bool {
    match (left, right) {
        (Value::String(left), Value::String(right)) if case_insensitive => {
            left.to_lowercase() == right.to_lowercase()
        }
        (Value::Array(left), Value::Array(right)) if left.len() == right.len() => left
            .iter()
            .zip(right)
            .all(|(left, right)| values_equal(left, right, case_insensitive)),
        (Value::Object(left), Value::Object(right)) if left.len() == right.len() => {
            left.iter().all(|(key, left)| {
                right
                    .get(key)
                    .is_some_and(|right| values_equal(left, right, case_insensitive))
            })
        }
        _ => left == right,
    }
}

fn collection_values(value: &Value) -> Option<Vec<&Value>> {
    match value {
        Value::Null => None,
        Value::Array(values) => Some(values.iter().collect()),
        value => Some(vec![value]),
    }
}

fn collection_contains(
    collection: &Value,
    item: &Value,
    case_insensitive: bool,
) -> Result<bool, OperandError> {
    let Some(collection) = collection_values(collection) else {
        return Ok(false);
    };

    Ok(collection
        .into_iter()
        .any(|candidate| values_equal(candidate, item, case_insensitive)))
}

enum CollectionMatch {
    Any,
    All,
    None,
}

fn collections_match(
    left: &Value,
    right: &Value,
    case_insensitive: bool,
    mode: CollectionMatch,
) -> Result<bool, OperandError> {
    let Some(left) = collection_values(left) else {
        return Ok(false);
    };

    let Some(right) = collection_values(right) else {
        return Ok(false);
    };

    let contains = |item: &&Value| {
        right
            .iter()
            .any(|candidate| values_equal(item, candidate, case_insensitive))
    };

    Ok(match mode {
        CollectionMatch::Any => left.iter().any(contains),
        CollectionMatch::All => left.iter().all(contains),
        CollectionMatch::None => !left.iter().any(contains),
    })
}

fn evaluate_patterns(
    left: &Value,
    right: &Value,
    case_insensitive: bool,
) -> Result<bool, OperandError> {
    let Some(values) = collection_values(left) else {
        return Ok(false);
    };

    let Some(patterns) = collection_values(right) else {
        return Ok(false);
    };

    for pattern in patterns {
        let pattern = pattern.as_str().ok_or_else(|| {
            OperandError::InvalidRightOperand(
                "string or string array".into(),
                get_type(pattern).into(),
            )
        })?;

        let regex = RegexBuilder::new(pattern)
            .case_insensitive(case_insensitive)
            .build()
            .map_err(|error| OperandError::InvalidPattern(error.to_string()))?;

        for value in &values {
            let value = value.as_str().ok_or_else(|| {
                OperandError::InvalidLeftOperand(
                    "string or string array".into(),
                    get_type(value).into(),
                )
            })?;

            if regex.is_match(value) {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
