pub mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, ToSchema)]
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