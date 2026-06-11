//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use amqprs::channel::Channel;
use mongodb::Client;
use shared::application::errors::ApplicationError;
use shared::application::ports::artifacts::ArtifactRepository;
use shared::application::ports::deployment::{DeploymentStrategyProvider, ModelDeploymentRepository};
use shared::application::ports::events::EventPublisher;
use shared::application::ports::model_metadata::ModelMetadataRepository;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::application::ports::deployment::ModelDeploymentPlatformReconcilerProvider;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use shared::infra::artifacts::mongo::artifact_repository::ArtifactRepository as MongoArtifactRepository;
use shared::infra::persistence::mongo::repositories::{
    ModelMetadataRepository as MongoModelMetadataRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
};
use shared::infra::reconciliation::client_provider::ReconciliationClientProvider;
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;
use shared::application::services::model_deployment_controller::ModelDeploymentController;
use std::sync::Arc;

pub fn model_metadata_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(client, db_name))
}

pub fn model_deployment_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelDeploymentRepository> {
    Arc::new(MongoModelDeploymentRepository::new(client, db_name))
}

pub fn event_publisher_factory(channel: Arc<Channel>) -> Arc<dyn EventPublisher> {
    Arc::new(RabbitMQModelDeploymentMessagePublisher::new(channel))
}

pub fn model_deployment_platform_reconciler_provider_factory() -> Arc<dyn ModelDeploymentPlatformReconcilerProvider> {
    Arc::new(ReconciliationClientProvider::new())
}

pub fn artifact_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(client, db_name))
}

pub fn model_deployment_service_builder(client: &Client, db_name: String, channel: Arc<Channel>) -> ModelDeploymentService {
    ModelDeploymentService::new(
        model_deployment_repo_factory(client, db_name.clone()),
        model_metadata_repo_factory(client, db_name.clone()),
        artifact_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel),
    )
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(err) => Err(err)
    }
}

pub fn model_deployment_conroller_builder(client: &Client, db_name: String, channel: Arc<Channel>, client_strategy_sets: Arc<Vec<ClientStrategySet>>) -> Arc<ModelDeploymentController> {
    Arc::new(ModelDeploymentController::new(
        client_strategy_sets,
        model_deployment_service_builder(client, db_name.clone(), channel.clone()),
        model_metadata_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel.clone()),
        model_deployment_platform_reconciler_provider_factory(),
    ))
}