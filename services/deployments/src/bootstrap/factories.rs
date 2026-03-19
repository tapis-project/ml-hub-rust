//! This module contains factories/builders that wire together infrastructure-level concerns
//! with application level concerns
// use mongodb::Database;
use std::sync::Arc;
use amqprs::channel::Channel;
use shared::application::errors::ApplicationError;
use shared::application::ports::artifacts::ArtifactRepository;
use shared::application::ports::deployment::DeploymentStrategyProvider;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use mongodb::Database;
use shared::application::ports::deployment::ModelDeploymentRepository;
use shared::application::ports::events::EventPublisher;
use shared::application::ports::model_metadata::ModelMetadataRepository;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::infra::persistence::mongo::repositories::{
    ModelMetadataRepository as MongoModelMetadataRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
    ArtifactRepository as MongoArtifactRepository,
};
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;

pub fn model_metadata_repo_factory(db: &Database) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(db))
}

pub fn model_deployment_repo_factory(db: &Database) -> Arc<dyn ModelDeploymentRepository> {
    Arc::new(MongoModelDeploymentRepository::new(db))
}

pub fn artifact_repo_factory(db: &Database) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(db))
}

pub fn event_publisher_factory(channel: Arc<Channel>) -> Arc<dyn EventPublisher> {
    Arc::new(RabbitMQModelDeploymentMessagePublisher::new(channel))
}

pub fn model_deployment_service_builder(db: &Database, channel: Arc<Channel>) -> ModelDeploymentService {
    ModelDeploymentService::new(
        model_deployment_repo_factory(db),
        model_metadata_repo_factory(db),
        artifact_repo_factory(db),
        event_publisher_factory(channel.clone()),
    )
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(err) => Err(err)
    }
}