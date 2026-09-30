use std::path::Path;
use std::sync::Arc;

use evaluations::{Evaluator, EvaluatorError};
use mongodb::Client;
use shared::application::errors::ApplicationError;
use shared::application::ports::deployment_option::DeploymentOptionRepository;
use shared::application::ports::deployment_strategy::DeploymentStrategyProvider;
use shared::application::ports::hpc_cluster::{HpcClusterRepository, HpcClusterRepositoryError};
use shared::application::ports::model::ExternalModelRepository;
use shared::application::services::external_model_ingestion_service::ExternalModelIngestionService;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use shared::infra::persistence::mongo::repositories::ExternalModelRepository as MongoExternalModelRepository;
use shared::infra::persistence::mongo::repositories::{
    DeploymentOptionRepository as MongoDeploymentOptionRepository,
    HpcClusterRepository as MongoHpcClusterRepository,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExternalModelIngestionBootstrapError {
    #[error(transparent)]
    Evaluation(#[from] EvaluatorError),

    #[error(transparent)]
    HpcClusterRepository(#[from] HpcClusterRepositoryError),
}

pub fn external_model_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ExternalModelRepository> {
    Arc::new(MongoExternalModelRepository::new(client, db_name))
}

pub fn deployment_option_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn DeploymentOptionRepository> {
    Arc::new(MongoDeploymentOptionRepository::new(client, db_name))
}

pub fn build_deployment_strategy_provider(
) -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    DeploymentStrategyProviderFs::new()
        .map(|provider| Arc::new(provider) as Arc<dyn DeploymentStrategyProvider>)
}

pub async fn external_model_ingestion_service_factory(
    client: &Client,
    db_name: String,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    evaluation_config_path: impl AsRef<Path>,
) -> Result<ExternalModelIngestionService, ExternalModelIngestionBootstrapError> {
    let evaluator = Evaluator::load(evaluation_config_path)?;
    let hpc_cluster_repository = MongoHpcClusterRepository::new(client, db_name.clone());
    let hpc_clusters = hpc_cluster_repository.list_all().await?;

    Ok(ExternalModelIngestionService::new(
        external_model_repo_factory(client, db_name.clone()),
        deployment_option_repo_factory(client, db_name),
        client_strategy_sets,
        evaluator,
        hpc_clusters,
    ))
}
