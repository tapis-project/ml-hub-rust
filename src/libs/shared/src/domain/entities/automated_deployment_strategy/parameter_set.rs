#[derive(Clone)]
pub struct Parameter {
    pub name: String
}

#[derive(Clone)]
pub struct ParameterSet {
    pub name: String,
    pub parameters: Vec<Parameter>
}