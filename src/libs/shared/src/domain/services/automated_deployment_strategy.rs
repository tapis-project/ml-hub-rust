use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::entities::automated_deployment_strategy::strategy::{Strategy, ViableStrategy};

pub fn resolve_viable_strategies(model_metadata: &ModelMetadata, strategies: &Vec<Strategy>) -> Vec<ViableStrategy> {
    let mut viable_strategies: Vec<ViableStrategy> = Vec::new();
    for strat in strategies {
        
    }

    viable_strategies
}