use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    pub r#type: ParameterType,
    pub choices: Option<Vec<String>>,
    pub default: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
}

#[derive(Clone, Debug, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}