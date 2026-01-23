//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Database;
use shared::application::errors::ApplicationError;
use shared::application::ports::deployment::DeploymentStrategyProvider;
use shared::application::ports::artifacts::{ModelMetadataRepository, ArtifactRepository};
use shared::application::services::model_metadata_service::ModelMetadataService;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use shared::infra::persistence::mongo::repositories::{
    ModelMetadataRepository as MongoModelMetadataRepository,
    ArtifactRepository as MongoArtifactRepository,
};
use std::sync::Arc;

pub fn artifact_repo_factory(db: &Database) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(db))
}

pub fn model_metadata_repo_factory(db: &Database) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(db))
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn DeploymentStrategyProvider>, ()> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(_) => Err(())
    }
}

pub async fn model_metadata_service_factory(
    db: &Database,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>
) -> Result<ModelMetadataService, ApplicationError> {    
    Ok(ModelMetadataService::new(
        model_metadata_repo_factory(db),
        artifact_repo_factory(db),
        client_strategy_sets,
    ))
}