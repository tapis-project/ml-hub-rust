use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Parameter {
    pub name: String
}

#[derive(Clone, Debug, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}