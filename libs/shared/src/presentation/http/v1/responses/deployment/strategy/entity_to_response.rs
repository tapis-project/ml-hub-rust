use crate::domain::entities::deployment_strategy::strategy as entities;
use crate::presentation::http::v1::responses::deployment:: parameter_set::Parameter;
use crate::presentation::http::v1::responses::deployment::strategy as dtos;
use crate::shared_kernel::enums::DeploymentModality;

impl From<entities::Strategy> for dtos::Strategy {
    fn from(value: entities::Strategy) -> Self {
        let parameters: Vec<Parameter> = value
                .parameter_set()
                .clone()
                .map(|ps| ps.parameters )
                .unwrap_or(vec![])
                .into_iter()
                .map(|p| Parameter::from(p))
                .collect();

        Self {
            name: value.name.clone(),
            description: value.description.clone(),
            platform: value.platform.clone(),
            parameters,
            config: dtos::StrategyConfig::from(value.config().clone()),
            enabled: value.enabled(),
        }
    }
}

impl From<DeploymentModality> for dtos::DeploymentModality {
    fn from(value: DeploymentModality) -> Self {
        match value {
            DeploymentModality::Batch => dtos::DeploymentModality::Batch,
            DeploymentModality::Service => dtos::DeploymentModality::Service,
        }
    }
}

impl From<entities::StrategyConfig> for dtos::StrategyConfig {
    fn from(value: entities::StrategyConfig) -> Self {
        Self {
            max_ttl: value.max_ttl.map(|ttl| ttl.as_minutes()),
            min_replicas: value.min_replicas.into(),
            max_replicas: value.max_replicas.map(|max| max.into()),
            supported_deployment_modalities: Vec::from(value.supported_deployment_modalities)
                .iter()
                .map(|dm| dtos::DeploymentModality::from(dm.clone()))
                .collect(),
            supported_paralellism_strategies: value.supported_paralellism_strategies
                .iter()
                .map(|ps| dtos::ParallelismStrategy::from(ps))
                .collect()
        }
    }
}