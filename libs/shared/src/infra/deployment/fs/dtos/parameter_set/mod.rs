pub mod dto_to_entity;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}