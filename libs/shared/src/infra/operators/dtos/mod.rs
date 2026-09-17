pub mod dto_to_entity;

use serde::{Deserialize, Serialize};

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
