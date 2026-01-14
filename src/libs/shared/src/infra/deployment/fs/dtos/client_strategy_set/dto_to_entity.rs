use crate::domain::entities::deployment_strategy as entities;
use crate::infra::deployment::fs::dtos;

impl TryFrom<dtos::client_strategy_set::ClientStrategySet> for entities::client_strategy_set::ClientStrategySet {
    type Error = entities::client_strategy_set::ClientStrategySetError;
    
    fn try_from(value: dtos::client_strategy_set::ClientStrategySet) -> Result<Self, Self::Error> {
        let mut client_strategies: Vec<entities::client_strategy::ClientStrategy> = vec![];

        for strat in value.strategies {
            let maybe_strat = entities::client_strategy::ClientStrategy::try_from(strat.clone());
            match maybe_strat {
                Ok(s) => client_strategies.push(s),
                Err(err) => return Err(entities::client_strategy_set::ClientStrategySetError::ClientStrategyError(err))
            };
        }

        let maybe_rule_sets: Option<Vec<entities::rule_set::RuleSet>> = match value.rule_sets {
            Some(rs) => {
                let rule_sets: Vec<entities::rule_set::RuleSet> = rs.iter()
                    .map(|r| {
                        entities::rule_set::RuleSet::from(r.clone())
                    })
                    .collect();

                Some(rule_sets)
            },
            None => None
        };

        let maybe_parameter_sets: Option<Vec<entities::parameter_set::ParameterSet>> = match value.parameter_sets {
            Some(ps) => {
                let parameter_sets: Vec<entities::parameter_set::ParameterSet> = ps.iter()
                    .map(|p| {
                        entities::parameter_set::ParameterSet::from(p.clone())
                    })
                    .collect();

                Some(parameter_sets)
            },
            None => None
        };
        
        let client_strategy_set = entities::client_strategy_set::ClientStrategySet::new(
            value.client,
            value.description,
            client_strategies,
            maybe_rule_sets,
            maybe_parameter_sets
        )?;
        
        Ok(client_strategy_set)
    }
}