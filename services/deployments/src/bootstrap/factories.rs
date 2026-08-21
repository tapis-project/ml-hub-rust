//! This module contains factories/builders that wire together infrastructure-level concerns
//! with application level concerns
// use mongodb::Database;
use std::sync::Arc;
use amqprs::channel::Channel;
use shared::application::errors::ApplicationError;
use shared::application::ports::artifacts::ArtifactRepository;
use shared::application::ports::cipher::Cipher;
use shared::application::ports::deployment_argument::DeploymentArgumentRepository;
use shared::application::ports::deployment_strategy::DeploymentStrategyProvider;
use shared::application::services::deployment_argument_service::DeploymentArgumentService;
use shared::infra::argument::mongo::MongoDeploymentArgumentRepository;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use mongodb::Client;
use shared::application::ports::deployment::ModelDeploymentRepository;
use shared::application::ports::events::EventPublisher;
use shared::application::ports::model_metadata::ModelMetadataRepository;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::infra::artifacts::mongo::artifact_repository::ArtifactRepository as MongoArtifactRepository;
use shared::infra::encryption::vault::VaultCipher;
use shared::infra::persistence::mongo::repositories::{
    ModelMetadataRepository as MongoModelMetadataRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
};
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;

pub fn model_metadata_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(client, db_name.clone()))
}

pub fn model_deployment_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelDeploymentRepository> {
    Arc::new(MongoModelDeploymentRepository::new(client, db_name.clone()))
}

pub fn artifact_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(client, db_name.clone()))
}

pub fn event_publisher_factory(channel: Arc<Channel>) -> Arc<dyn EventPublisher> {
    Arc::new(RabbitMQModelDeploymentMessagePublisher::new(channel))
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(err) => Err(err)
    }
}

pub fn deployment_argument_repo_factory(client: &Client, db_name: &str) -> Arc<dyn DeploymentArgumentRepository> {
    Arc::new(MongoDeploymentArgumentRepository::new(client, db_name))
}

pub fn cipher_factory() -> Arc<dyn Cipher> {
    Arc::new(VaultCipher {})
}

pub fn deployment_argument_service_builder(client: &Client, db_name: &str) -> DeploymentArgumentService {
    DeploymentArgumentService::new(
        deployment_argument_repo_factory(client, db_name),
        cipher_factory()
    )
}

pub fn model_deployment_service_builder(client: &Client, db_name: String, channel: Arc<Channel>) -> Result<ModelDeploymentService, ApplicationError> {
    Ok(ModelDeploymentService::new(
        deployment_argument_service_builder(client, &db_name),
        model_deployment_repo_factory(client, db_name.clone()),
        model_metadata_repo_factory(client, db_name.clone()),
        artifact_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel.clone()),
        build_deployment_strategy_provider()?,
        cipher_factory(),
    ))
}