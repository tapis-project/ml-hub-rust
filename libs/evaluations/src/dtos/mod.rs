mod mappers;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    pub evaluations: Vec<Evaluation>,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Evaluation {
    pub name: String,
    pub evaluation_strategy: EvaluationStrategy,
    pub parameters: Vec<String>,
    pub evaluation_type: Option<EvaluationType>,
    pub expressions: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Statement {
    pub name: String,
    pub evaluation_strategy: EvaluationStrategy,
    pub parameters: Vec<String>,
    pub conditions: Vec<Vec<Condition>>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub(crate) enum EvaluationStrategy {
    #[serde(rename = "DNF")]
    Dnf,
    #[serde(rename = "CNF")]
    Cnf,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub(crate) enum EvaluationType {
    ModelTrust,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Condition {
    #[serde(default)]
    pub negate: bool,
    #[serde(default)]
    pub case_insensitive: bool,
    pub operator: Operator,
    pub operands: Vec<Operand>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub(crate) enum Operator {
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

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Operand {
    #[serde(rename = "type")]
    pub operand_type: OperandType,
    pub parameter: Option<String>,
    pub accessor: Option<Accessor>,
    #[serde(default, deserialize_with = "deserialize_value_field")]
    pub value: ValueField,
    #[serde(default)]
    pub operations: Vec<Operation>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OperandType {
    Parameter,
    Literal,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Accessor {
    pub field_path: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Operation {
    pub operation: OperationType,
    pub coalesce: Option<Coalesce>,
    pub multiply: Option<Vec<f64>>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum OperationType {
    Coalesce,
    Multiply,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Coalesce {
    pub from: Value,
    pub to: Value,
}

#[derive(Debug, Default)]
pub(crate) enum ValueField {
    #[default]
    Missing,
    Present(Value),
}

fn deserialize_value_field<'de, D>(deserializer: D) -> Result<ValueField, D::Error>
where
    D: Deserializer<'de>,
{
    Value::deserialize(deserializer).map(ValueField::Present)
}
