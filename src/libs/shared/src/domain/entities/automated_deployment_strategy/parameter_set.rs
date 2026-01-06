use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Parameter {
    pub name: String
}

#[derive(Clone, Serialize)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}