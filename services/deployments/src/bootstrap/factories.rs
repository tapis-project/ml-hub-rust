//! This module contains factories/builders that wire together infrastructure-level concerns
//! with application level concerns
// use mongodb::Database;
use amqprs::channel::Channel;
use mongodb::Client;
use shared::application::errors::ApplicationError;
use shared::application::ports::artifacts::ArtifactRepository;
use shared::application::ports::cipher::Cipher;
use shared::application::ports::deployment::ModelDeploymentRepository;
use shared::application::ports::deployment_argument::DeploymentArgumentRepository;
use shared::application::ports::deployment_strategy::DeploymentStrategyProvider;
use shared::application::ports::events::EventPublisher;
use shared::application::ports::hpc_cluster::HpcClusterRepository;
use shared::application::ports::model::{ExternalModelRepository, ModelRepository};
use shared::application::services::deployment_argument_service::DeploymentArgumentService;
use shared::application::services::hpc_cluster_query_service::HpcClusterQueryService;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::infra::argument::mongo::MongoDeploymentArgumentRepository;
use shared::infra::artifacts::mongo::artifact_repository::ArtifactRepository as MongoArtifactRepository;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use shared::infra::encryption::vault::VaultCipher;
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;
use shared::infra::persistence::mongo::repositories::{
    ExternalModelRepository as MongoExternalModelRepository,
    HpcClusterRepository as MongoHpcClusterRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
    ModelRepository as MongoModelRepository,
};
use std::sync::Arc;

pub fn model_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelRepository> {
    Arc::new(MongoModelRepository::new(client, db_name.clone()))
}

pub fn hpc_cluster_repo_factory(client: &Client, db_name: String) -> Arc<dyn HpcClusterRepository> {
    Arc::new(MongoHpcClusterRepository::new(client, db_name))
}

pub fn hpc_cluster_query_service_builder(
    client: &Client,
    db_name: String,
) -> HpcClusterQueryService {
    HpcClusterQueryService::new(hpc_cluster_repo_factory(client, db_name))
}

pub fn external_model_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ExternalModelRepository> {
    Arc::new(MongoExternalModelRepository::new(client, db_name))
}

pub fn model_deployment_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn ModelDeploymentRepository> {
    Arc::new(MongoModelDeploymentRepository::new(client, db_name.clone()))
}

pub fn artifact_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(client, db_name.clone()))
}

pub fn event_publisher_factory(channel: Arc<Channel>) -> Arc<dyn EventPublisher> {
    Arc::new(RabbitMQModelDeploymentMessagePublisher::new(channel))
}

pub fn build_deployment_strategy_provider(
) -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(err) => Err(err),
    }
}

pub fn deployment_argument_repo_factory(
    client: &Client,
    db_name: &str,
) -> Arc<dyn DeploymentArgumentRepository> {
    Arc::new(MongoDeploymentArgumentRepository::new(client, db_name))
}

pub fn cipher_factory() -> Arc<dyn Cipher> {
    Arc::new(VaultCipher {})
}

pub fn deployment_argument_service_builder(
    client: &Client,
    db_name: &str,
) -> DeploymentArgumentService {
    DeploymentArgumentService::new(
        deployment_argument_repo_factory(client, db_name),
        cipher_factory(),
    )
}

pub fn model_deployment_service_builder(
    client: &Client,
    db_name: String,
    channel: Arc<Channel>,
) -> Result<ModelDeploymentService, ApplicationError> {
    Ok(ModelDeploymentService::new(
        deployment_argument_service_builder(client, &db_name),
        model_deployment_repo_factory(client, db_name.clone()),
        model_repo_factory(client, db_name.clone()),
        external_model_repo_factory(client, db_name.clone()),
        artifact_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel.clone()),
        build_deployment_strategy_provider()?,
        cipher_factory(),
    ))
}
