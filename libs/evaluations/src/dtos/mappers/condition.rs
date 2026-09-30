use regex::RegexBuilder;
use serde_json::Value;

use crate::{Condition, EvaluatorError, Operands, Operator};

use super::{convert_operand, invalid};
use crate::dtos::{
    Condition as ConditionDto, Operand as OperandDto, OperandType, Operator as OperatorDto,
    ValueField,
};

pub(crate) fn convert_condition(
    value: ConditionDto,
    parameters: &[String],
) -> Result<Condition, EvaluatorError> {
    if value.operands.len() != 2 {
        return Err(invalid(format!(
            "operator {:?} requires exactly two operands",
            value.operator
        )));
    }

    if matches!(value.operator, OperatorDto::Pattern) {
        validate_pattern_operand(&value.operands[1], value.case_insensitive)?;
    }

    let mut operands = value.operands.into_iter();
    let left = convert_operand(
        operands
            .next()
            .ok_or_else(|| invalid("missing left operand"))?,
        parameters,
    )?;

    let right = convert_operand(
        operands
            .next()
            .ok_or_else(|| invalid("missing right operand"))?,
        parameters,
    )?;

    Ok(Condition::new(
        value.negate,
        value.case_insensitive,
        value.operator.into(),
        Operands::new(left, right),
    ))
}

fn validate_pattern_operand(
    operand: &OperandDto,
    case_insensitive: bool,
) -> Result<(), EvaluatorError> {
    if !matches!(operand.operand_type, OperandType::Literal) {
        return Err(invalid("Pattern requires a literal right operand"));
    }

    let ValueField::Present(value) = &operand.value else {
        return Err(invalid("Pattern requires a literal right operand value"));
    };

    let patterns = match value {
        Value::String(pattern) => vec![pattern.as_str()],
        Value::Array(patterns) => patterns
            .iter()
            .map(|pattern| {
                pattern
                    .as_str()
                    .ok_or_else(|| invalid("Pattern values must be strings"))
            })
            .collect::<Result<Vec<_>, _>>()?,
        _ => return Err(invalid("Pattern requires a string or string array")),
    };

    if patterns.is_empty() {
        return Err(invalid("Pattern requires at least one regular expression"));
    }

    for pattern in patterns {
        RegexBuilder::new(pattern)
            .case_insensitive(case_insensitive)
            .build()
            .map_err(|error| invalid(format!("invalid regular expression: {error}")))?;
    }

    Ok(())
}

impl From<OperatorDto> for Operator {
    fn from(value: OperatorDto) -> Self {
        match value {
            OperatorDto::Eq => Self::Eq,
            OperatorDto::Neq => Self::Neq,
            OperatorDto::Gte => Self::Gte,
            OperatorDto::Lte => Self::Lte,
            OperatorDto::Gt => Self::Gt,
            OperatorDto::Lt => Self::Lt,
            OperatorDto::In => Self::In,
            OperatorDto::Contains => Self::Contains,
            OperatorDto::NotIn => Self::NotIn,
            OperatorDto::NoneIn => Self::NoneIn,
            OperatorDto::AnyIn => Self::AnyIn,
            OperatorDto::AllIn => Self::AllIn,
            OperatorDto::Pattern => Self::Pattern,
        }
    }
}
