//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use amqprs::channel::Channel;
use mongodb::Client;
use shared::application::ports::cipher::Cipher;
use shared::application::ports::deployment::ModelDeploymentReconcilerProvider;
use shared::application::ports::deployment::ModelDeploymentRepository;
use shared::application::ports::deployment_argument::DeploymentArgumentRepository;
use shared::application::ports::deployment_option::DeploymentOptionRepository;
use shared::application::ports::events::EventPublisher;
use shared::application::ports::hpc_cluster::HpcClusterRepository;
use shared::application::ports::model::ExternalModelRepository;
use shared::application::services::deployment_argument_service::DeploymentArgumentService;
use shared::application::services::model_deployment_controller::ModelDeploymentController;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::domain::entities::site::SiteContext;
use shared::infra::argument::mongo::MongoDeploymentArgumentRepository;
use shared::infra::configuration::TapisJobsConfiguration;
use shared::infra::encryption::vault::VaultCipher;
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;
use shared::infra::persistence::mongo::repositories::{
    DeploymentOptionRepository as MongoDeploymentOptionRepository,
    ExternalModelRepository as MongoExternalModelRepository,
    HpcClusterRepository as MongoHpcClusterRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
};
use shared::infra::reconciliation::client_provider::ReconciliationClientProvider;
use std::sync::Arc;

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
    Arc::new(MongoModelDeploymentRepository::new(client, db_name))
}

pub fn event_publisher_factory(channel: Arc<Channel>) -> Arc<dyn EventPublisher> {
    Arc::new(RabbitMQModelDeploymentMessagePublisher::new(channel))
}

pub fn model_deployment_reconciler_provider_factory(
    tapis_jobs_configuration: TapisJobsConfiguration,
) -> Arc<dyn ModelDeploymentReconcilerProvider> {
    Arc::new(ReconciliationClientProvider::new(tapis_jobs_configuration))
}

pub fn deployment_option_repo_factory(
    client: &Client,
    db_name: String,
) -> Arc<dyn DeploymentOptionRepository> {
    Arc::new(MongoDeploymentOptionRepository::new(client, db_name))
}

pub fn hpc_cluster_repo_factory(client: &Client, db_name: String) -> Arc<dyn HpcClusterRepository> {
    Arc::new(MongoHpcClusterRepository::new(client, db_name))
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
) -> ModelDeploymentService {
    ModelDeploymentService::new(
        deployment_argument_service_builder(client, &db_name),
        model_deployment_repo_factory(client, db_name.clone()),
        deployment_option_repo_factory(client, db_name.clone()),
        external_model_repo_factory(client, db_name.clone()),
        hpc_cluster_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel),
    )
}

pub fn model_deployment_conroller_builder(
    site_context: SiteContext,
    client: &Client,
    db_name: String,
    channel: Arc<Channel>,
    tapis_jobs_configuration: TapisJobsConfiguration,
) -> Arc<ModelDeploymentController> {
    Arc::new(ModelDeploymentController::new(
        site_context,
        deployment_argument_service_builder(client, &db_name),
        model_deployment_service_builder(client, db_name.clone(), channel.clone()),
        external_model_repo_factory(client, db_name.clone()),
        event_publisher_factory(channel.clone()),
        model_deployment_reconciler_provider_factory(tapis_jobs_configuration),
    ))
}
