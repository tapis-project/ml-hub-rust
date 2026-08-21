//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use amqprs::channel::Channel;
use mongodb::Client;
use shared::application::ports::artifacts::ArtifactRepository;
use shared::application::ports::cipher::Cipher;
use shared::application::ports::deployment::ModelDeploymentRepository;
use shared::application::ports::deployment_argument::DeploymentArgumentRepository;
use shared::application::ports::deployment_strategy::DeploymentStrategyProvider;
use shared::application::ports::events::EventPublisher;
use shared::application::ports::model_metadata::ModelMetadataRepository;
use shared::application::services::deployment_argument_service::DeploymentArgumentService;
use shared::application::services::deployment_strategy_service::DeploymentStrategyService;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::application::ports::deployment::ModelDeploymentPlatformReconcilerProvider;
use shared::domain::entities::site::SiteContext;
use shared::infra::argument::mongo::MongoDeploymentArgumentRepository;
use shared::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use shared::infra::artifacts::mongo::artifact_repository::ArtifactRepository as MongoArtifactRepository;
use shared::infra::encryption::vault::VaultCipher;
use shared::infra::persistence::mongo::repositories::{
    ModelMetadataRepository as MongoModelMetadataRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
};
use shared::infra::reconciliation::client_provider::ReconciliationClientProvider;
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;
use shared::application::services::model_deployment_controller::ModelDeploymentController;
use shared::shared_kernel::errors::BootstrapError;
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

pub fn model_deployment_service_builder(client: &Client, db_name: String, channel: Arc<Channel>) -> Result<ModelDeploymentService, BootstrapError> {
    Ok(ModelDeploymentService::new(
        deployment_argument_service_builder(client, &db_name),
        model_deployment_repo_factory(client, db_name.clone()),
        model_metadata_repo_factory(client, db_name.clone()),
        artifact_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel),
        build_deployment_strategy_provider()?,
        cipher_factory(),
    ))
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn DeploymentStrategyProvider>, BootstrapError> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(e) => Err(BootstrapError::FailedToInitialize("DeploymentStrategyProvider".into(), e.to_string()))
    }
}

pub fn deployment_strategy_service_builder() -> Result<DeploymentStrategyService, BootstrapError> {
    Ok(DeploymentStrategyService::new(
        build_deployment_strategy_provider()?
    ))
}

pub fn model_deployment_conroller_builder(site_context: SiteContext, client: &Client, db_name: String, channel: Arc<Channel>) -> Result<Arc<ModelDeploymentController>, BootstrapError> {
    Ok(Arc::new(
        ModelDeploymentController::new(
            site_context,
            deployment_strategy_service_builder()?,
            deployment_argument_service_builder(client, &db_name),
            model_deployment_service_builder(client, db_name.clone(), channel.clone())?,
            model_metadata_repo_factory(client, db_name.clone()),
            event_publisher_factory(channel.clone()),
            model_deployment_platform_reconciler_provider_factory(),
        )
    ))
}