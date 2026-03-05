pub mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct Parameter {
    pub name: String
}

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}