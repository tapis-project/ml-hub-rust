pub mod dto_to_entity;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

