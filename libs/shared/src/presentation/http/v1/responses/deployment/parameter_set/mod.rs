pub mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    pub choices: Option<Vec<Choice>>,
    pub default: Option<String>,
    pub r#type: ParameterType,
}

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct Choice {
    value: String,
    description: Option<String>,
    enabled: bool
}