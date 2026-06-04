use crate::domain::entities::model_metadata::{ModelMetadata, ModelMetadataError};
use crate::domain::entities::deployment_strategy::strategy::{Strategy, ViableStrategy};
use crate::domain::entities::deployment_strategy::rule_set::Rule;
use crate::domain::entities::operator::{Operator, OperandError};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StrategyEvaluationError {
    #[error("Error evaluating strategy rule: {0}")]
    RuleError(#[from] OperandError),
    
    #[error("Model Metadata Error: {0}")]
    MetadataError(#[from] ModelMetadataError),
}

/// Iterates over the provided stragies and determines whether a strategy is viable
/// for the provided metadata. If any of the rules in the rule sets of the strategy
/// are false, then the strategy is not valid.
pub fn resolve_viable_strategies(model_metadata: &ModelMetadata, strategies: &Vec<Strategy>) -> Result<Vec<ViableStrategy>, StrategyEvaluationError> {
    let mut viable_strategies: Vec<ViableStrategy> = Vec::new();
    
    for strat in strategies {
        let mut is_viable_strat = true;

        for rule_set in strat.rule_sets() {
            for rule in rule_set.rules.clone() {
                is_viable_strat = is_viable_strat && evaluate_rule(model_metadata, &rule)?;
                if !is_viable_strat {
                    break
                }
            }

            if !is_viable_strat {
                break
            }
        }

        if is_viable_strat {
            viable_strategies.push(ViableStrategy::new(strat.clone()));
        }
    }

    Ok(viable_strategies)
}

pub(super) fn evaluate_rule(model_metadata: &ModelMetadata, rule: &Rule) -> Result<bool, StrategyEvaluationError> {
    let value: Value = model_metadata.get_field_value_at_field_path(&rule.field_path)?.into();
    
    match rule.operator {
        Operator::Eq => Ok(Operator::Eq.evaluate(&value, &rule.value)?),
        Operator::Neq => Ok(Operator::Neq.evaluate(&value, &rule.value)?),
        Operator::Gte => Ok(Operator::Gte.evaluate(&value, &rule.value)?),
        Operator::Gt => Ok(Operator::Gt.evaluate(&value, &rule.value)?),
        Operator::Lte => Ok(Operator::Lte.evaluate(&value, &rule.value)?),
        Operator::Lt => Ok(Operator::Lt.evaluate(&value, &rule.value)?),
        Operator::In => Ok(Operator::In.evaluate(&value, &rule.value)?),
        Operator::Contains => Ok(Operator::Contains.evaluate(&value, &rule.value)?),
        Operator::NotIn => Ok(Operator::NotIn.evaluate(&value, &rule.value)?),
        Operator::AllIn => Ok(Operator::AllIn.evaluate(&value, &rule.value)?),
        Operator::AnyIn => Ok(Operator::AnyIn.evaluate(&value, &rule.value)?),
        Operator::NoneIn => Ok(Operator::NoneIn.evaluate(&value, &rule.value)?),
    }
}

#[cfg(test)]
#[path = "deployment_strategy.test.rs"]
mod deployment_strategy_test;