use crate::domain::entities::operator::Operator;

#[derive(Clone)]
pub struct Rule {
    pub field_path: Vec<String>,
    pub operator: Operator,
    pub value: String,
}


#[derive(Clone)]
pub struct RuleSet {
    pub name: String,
    pub rules: Vec<Rule>
}