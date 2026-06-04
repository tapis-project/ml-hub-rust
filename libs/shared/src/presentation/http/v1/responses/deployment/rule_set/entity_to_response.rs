use crate::domain::entities::deployment_strategy::rule_set as entities;
use crate::presentation::http::v1::responses::deployment::rule_set as dtos;
use crate::presentation::http::v1::responses::operators::Operator;

impl From<entities::Rule> for dtos::Rule {
    fn from(value: entities::Rule) -> Self {
        Self {
            field_path: value.field_path,
            operator: Operator::from(value.operator),
            value: value.value,
        }
    }
}

impl From<entities::RuleSet> for dtos::RuleSet {
    fn from(value: entities::RuleSet) -> Self {
        Self {
            name: value.name,
            rules: value.rules
                .iter()
                .map(|r| dtos::Rule::from(r.clone()))
                .collect()
        }
    }
}