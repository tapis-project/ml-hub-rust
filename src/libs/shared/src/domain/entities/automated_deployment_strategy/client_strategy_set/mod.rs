use thiserror::Error;
use super::rule_set::RuleSet;
use super::strategy::{Strategy, StrategyError};
use super::parameter_set::ParameterSet;
use super::client_strategy::{ClientStrategy, ClientStrategyError};

#[derive(Error, Debug)]
pub enum ClientStrategySetError {
    #[error("{0}")]
    MissingStrategies(String),

    #[error("{0}")]
    InvalidClientRuleSetReference(String),

    #[error("{0}")]
    InvalidClientParameterSetReference(String),

    #[error("{0}")]
    ClientStrategyError(#[from] ClientStrategyError),

    #[error("{0}")]
    StrategyError(#[from] StrategyError)
}

#[derive(Clone)]
pub struct ClientStrategySet {
    pub client: String,
    pub description: Option<String>,
    rule_sets: Option<Vec<RuleSet>>,
    parameter_sets: Option<Vec<ParameterSet>>,
    strategies: Vec<Strategy>
}

impl ClientStrategySet {
    pub fn new(
        client: String,
        description: Option<String>,
        client_strategies: Vec<ClientStrategy>,
        rule_sets: Option<Vec<RuleSet>>,
        parameter_sets: Option<Vec<ParameterSet>>,
    ) -> Result<Self, ClientStrategySetError> {
        // Must be 1 or more strategies
        if client_strategies.len() == 0 {
            return Err(ClientStrategySetError::MissingStrategies("One or more strategies must be provided".into()))
        };

        // Static to allow borrowing in lazy evaluation with zero runtime cost
        static EMPTY_RULE_SET: Vec<RuleSet> = Vec::new();
        
        let client_rule_sets = rule_sets.as_ref().unwrap_or_else(|| &EMPTY_RULE_SET);

        // Covert ClientStrategies into Strategies
        let mut strategies: Vec<Strategy> = Vec::new();
        for client_strat in client_strategies {
            // Set the current strategy rulesets to the explcitly defined rulesets
            // from the ClientStrategy
            let mut strategy_rule_sets: Vec<RuleSet> = client_strat.rule_sets()
                .clone()
                .unwrap_or_else(|| EMPTY_RULE_SET.clone());

            // Resolve any client ruleset references in the client strategy.
            let resolved_rule_sets = Self::resolve_rule_set_refs(
                client_strat.rule_set_refs().clone().unwrap_or_else(|| Vec::new()),
                client_rule_sets
            )?;

            // Merge the resovled references to the existing strategy rule sets
            strategy_rule_sets.extend(resolved_rule_sets.iter().cloned());

            // Use explicitly defined parameter set if defined, if not, resolve any
            // references to client parameter sets
            let parameter_set = match client_strat.parameter_set() {
                Some(ps) => Ok(Some(ps.clone())),
                None => {
                    // Static to allow borrowing in lazy evaluation with zero runtime cost
                    static EMPTY_PARAMETER_SET: Vec<ParameterSet> = Vec::new();
                    let client_parameter_sets = parameter_sets.as_ref().unwrap_or_else(|| &EMPTY_PARAMETER_SET);
                    let parameter_set_ref = client_strat.parameter_set_ref();
                    
                    let maybe_parameter_set: Result<Option<ParameterSet>, ClientStrategySetError> = match parameter_set_ref {
                        Some(r) => {
                            Ok(Some(
                                Self::resolve_parameter_set_ref(
                                r.clone(),
                                    client_parameter_sets
                                )?
                            ))
                        },
                        None => Ok(None)
                    };

                    maybe_parameter_set
                }
            }?;
            
            // Create the Strategy
            strategies.push(
                Strategy::new(
                    client_strat.name,
                    client_strat.description,
                    strategy_rule_sets,
                    parameter_set,
                )?
            );
        }

        let strategies: Vec<Strategy> = Vec::new();

        Ok(Self {
            client,
            description,
            rule_sets,
            parameter_sets,
            strategies,
        })
    }

    pub fn rule_sets(&self) -> &Option<Vec<RuleSet>> {
        &self.rule_sets
    }

    pub fn parameter_sets(&self) -> &Option<Vec<ParameterSet>> {
        &self.parameter_sets
    }

    pub fn strategies(&self) -> &Vec<Strategy> {
        &self.strategies
    }

    fn resolve_rule_set_refs(rule_set_refs: Vec<String>, rule_sets: &Vec<RuleSet>) -> Result<Vec<RuleSet>, ClientStrategySetError>{
        let mut resolved_rule_sets: Vec<RuleSet> = Vec::new();
        for name in rule_set_refs {
            let maybe_resolved_rule_set = rule_sets.iter()
                .filter(|client_rule_set| client_rule_set.name == name)
                .next();

            match maybe_resolved_rule_set {
                Some(resolved_rule_set) => resolved_rule_sets.push(resolved_rule_set.clone()),
                None => return Err(ClientStrategySetError::InvalidClientRuleSetReference(format!("Failed to find client RuleSet with name '{}'", &name)))
            }
        };

        return Ok(resolved_rule_sets)
    }

    fn resolve_parameter_set_ref(parameter_set_ref: String, parameter_sets: &Vec<ParameterSet>) -> Result<ParameterSet, ClientStrategySetError>{
        let maybe_resolved_parameter_set = parameter_sets.iter()
            .filter(|client_parameter_set| client_parameter_set.name == parameter_set_ref.clone())
            .next();

        match maybe_resolved_parameter_set {
            Some(resolved_parameter_set) => Ok(resolved_parameter_set.clone()),
            None => Err(ClientStrategySetError::InvalidClientParameterSetReference(format!("Failed to find client ParameterSet with name '{}'", &parameter_set_ref)))
        }
    }
}

#[cfg(test)]
#[path = "client_strategy_set.test.rs"]
mod client_strategy_set_test;