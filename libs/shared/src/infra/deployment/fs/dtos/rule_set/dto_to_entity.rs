use crate::domain::entities::deployment_strategy as entities;
use crate::domain::entities::operator::Operator;
use crate::infra::deployment::fs::dtos;

impl From<dtos::rule_set::Rule> for entities::rule_set::Rule {
    fn from(value: dtos::rule_set::Rule) -> Self {
        Self {
            field_path: value.field_path,
            operator: Operator::from(value.operator),
            value: value.value
        }
    }
}

impl From<dtos::rule_set::RuleSet> for entities::rule_set::RuleSet {
    fn from(value: dtos::rule_set::RuleSet) -> Self {

        let mut rules: Vec<entities::rule_set::Rule> = Vec::with_capacity(value.rules.len());
        for rule in value.rules {
            rules.push(entities::rule_set::Rule::from(rule))
        }

        Self {
            name: value.name,
            rules: rules,
        }
    }
}