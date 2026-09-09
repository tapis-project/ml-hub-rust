//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Client;
use shared::application::errors::ApplicationError;
use shared::application::ports::deployment_strategy::DeploymentStrategyProvider;
use shared::application::ports::artifacts::ArtifactRepository;
use shared::application::ports::model::ModelRepository;
use shared::application::services::model_service::ModelService;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use shared::infra::artifacts::mongo::artifact_repository::ArtifactRepository as MongoArtifactRepository;
use shared::infra::persistence::mongo::repositories::{
    ModelRepository as MongoModelRepository,
};
use std::sync::Arc;

pub fn artifact_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(client, db_name.clone()))
}

pub fn model_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelRepository> {
    Arc::new(MongoModelRepository::new(client, db_name.clone()))
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(e) => Err(e)
    }
}

pub async fn model_service_factory(
    client: &Client,
    db_name: String,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>
) -> Result<ModelService, ApplicationError> {
    Ok(ModelService::new(
        model_repo_factory(client, db_name.clone()),
        artifact_repo_factory(client, db_name.clone()),
        client_strategy_sets,
    ))
}
