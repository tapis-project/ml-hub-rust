pub mod entity_to_response;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    #[serde(flatten)]
    pub r#type: ParameterType,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum ParameterType {
    String {
        choices: Option<Vec<String>>,
        default: Option<String>,
    },
    Integer {
        default: Option<u128>,
        choices: Option<Vec<u128>>,
    },
    Float {
        default: Option<i128>,
        choices: Option<Vec<i128>>,
    },
    Boolean{
        default: Option<bool>,
    },
}

#[derive(Clone, Debug, ToSchema, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}