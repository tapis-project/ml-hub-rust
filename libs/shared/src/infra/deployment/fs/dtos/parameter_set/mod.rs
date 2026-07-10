pub mod dto_to_entity;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: Option<bool>,
    pub secret: Option<bool>,
    pub r#type: ParameterType,
    pub choices: Option<Vec<String>>,
    pub default: Option<String>,
}

// We will be flexible with the casing and abbreviation of the parameter values
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    #[serde(alias = "str", alias = "Str", alias = "String")] // Inline stacking
    String,
    #[serde(alias = "int", alias = "Int", alias = "Integer")]
    Integer,
    #[serde(alias = "float", alias = "Float")]
    Float,
    #[serde(alias = "bool", alias = "Bool", alias = "Boolean")]
    Boolean,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}