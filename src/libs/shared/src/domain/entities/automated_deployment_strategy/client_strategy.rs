use thiserror::Error;
use super::rule_set::RuleSet;
use super::parameter_set::ParameterSet;

#[derive(Error, Debug)]
pub enum ClientStrategyError {
    #[error("{0}")]
    MissingRuleSets(String)
}

pub struct ClientStrategy {
    pub name: String,
    pub description: Option<String>,
    rule_sets: Option<Vec<RuleSet>>,
    parameter_set: Option<ParameterSet>,
    use_rule_sets: Option<Vec<String>>,
    use_parameter_set: Option<String>,
}

impl ClientStrategy {
    pub fn new(
        name: String,
        description: Option<String>,
        rule_sets: Option<Vec<RuleSet>>,
        parameter_set: Option<ParameterSet>,
        use_rule_sets: Option<Vec<String>>,
        use_parameter_set: Option<String>,
    ) -> Result<Self, ClientStrategyError> {
        // Must be at least 1 rule set or reference to a client rule set
        if rule_sets.is_none() && use_rule_sets.is_none() {
            return Err(ClientStrategyError::MissingRuleSets("Must provide 1 or more rule set or 1 or more rule set reference.".into()))
        }
        
        Ok(Self {
            name,
            description,
            rule_sets,
            parameter_set,
            use_rule_sets,
            use_parameter_set,
        })
    }

    pub fn rule_sets(&self) -> &Option<Vec<RuleSet>> {
        &self.rule_sets
    }

    pub fn rule_set_refs(&self) -> &Option<Vec<String>> {
        &self.use_rule_sets
    }

    pub fn parameter_set(&self) -> &Option<ParameterSet> {
        &self.parameter_set
    }

    pub fn parameter_set_ref(&self) -> &Option<String> {
        &self.use_parameter_set
    }
}