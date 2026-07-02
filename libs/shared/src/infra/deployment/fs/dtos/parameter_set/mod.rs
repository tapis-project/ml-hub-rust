pub mod dto_to_entity;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    pub r#type: ParameterType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ParameterType {
    String {
        choices: Option<Vec<String>>,
        default: Option<String>,
    },
    Integer {
        choices: Option<Vec<u128>>,
        default: Option<u128>,
    },
    Float {
        choices: Option<Vec<i128>>,
        default: Option<i128>,
    },
    Boolean{
        default: Option<bool>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}