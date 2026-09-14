use std::sync::Arc;

use mongodb::Client;
use shared::application::errors::ApplicationError;
use shared::application::ports::deployment_strategy::DeploymentStrategyProvider;
use shared::application::ports::model::ExternalModelRepository;
use shared::application::services::external_model_ingestion_service::ExternalModelIngestionService;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use shared::infra::persistence::mongo::repositories::ExternalModelRepository as MongoExternalModelRepository;

pub fn external_model_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ExternalModelRepository> {
    Arc::new(MongoExternalModelRepository::new(client, db_name))
}

pub fn build_deployment_strategy_provider(
) -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    DeploymentStrategyProviderFs::new()
        .map(|provider| Arc::new(provider) as Arc<dyn DeploymentStrategyProvider>)
}

pub fn external_model_ingestion_service_factory(
    client: &Client,
    db_name: String,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>,
) -> ExternalModelIngestionService {
    ExternalModelIngestionService::new(
        external_model_repo_factory(client, db_name),
        client_strategy_sets,
    )
}
