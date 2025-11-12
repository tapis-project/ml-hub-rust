#[derive(Clone)]
pub enum Operator {
    Contains,
    AnyIn,
    Lte,
    Lt,
    Gte,
    Gt,
    Eq,
    Neq,
}

#[derive(Clone)]
pub struct Rule {
    pub field: String,
    pub operator: Operator,
    pub value: String,
}


#[derive(Clone)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Rule>
}