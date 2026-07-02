use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Parameter {
    pub name: String,
    pub description: Option<String>,
    pub required: bool,
    pub secret: bool,
    pub r#type: ParameterType,
}

#[derive(Clone, Debug, Serialize)]
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

#[derive(Clone, Debug, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}