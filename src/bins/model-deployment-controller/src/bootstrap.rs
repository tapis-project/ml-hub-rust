//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use amqprs::channel::Channel;
use mongodb::Database;
use shared::application::ports::artifacts::ArtifactRepository;
use shared::application::ports::deployment::ModelDeploymentRepository;
use shared::application::ports::events::EventPublisher;
use shared::application::ports::model_metadata::ModelMetadataRepository;
use shared::application::services::model_deployment_service::ModelDeploymentService;
use shared::application::ports::deployment::ModelDeploymentPlatformReconcilerProvider;
use shared::infra::persistence::mongo::repositories::{
    ModelMetadataRepository as MongoModelMetadataRepository,
    ModelDeploymentRepository as MongoModelDeploymentRepository,
    ArtifactRepository as MongoArtifactRepository,
};
use shared::infra::reconciliation::client_provider::ReconciliationClientProvider;
use shared::infra::messaging::rabbitmq::model_deployment_message_publisher::RabbitMQModelDeploymentMessagePublisher;
use shared::application::services::model_deployment_controller::ModelDeploymentController;
use std::sync::Arc;

pub fn model_metadata_repo_factory(db: &Database) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(db))
}

pub fn model_deployment_repo_factory(db: &Database) -> Arc<dyn ModelDeploymentRepository> {
    Arc::new(MongoModelDeploymentRepository::new(db))
}

pub fn event_publisher_factory(channel: Arc<Channel>) -> Arc<dyn EventPublisher> {
    Arc::new(RabbitMQModelDeploymentMessagePublisher::new(channel))
}

pub fn model_deployment_platform_reconciler_provider_factory() -> Arc<dyn ModelDeploymentPlatformReconcilerProvider> {
    Arc::new(ReconciliationClientProvider::new())
}

pub fn artifact_repo_factory(db: &Database) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(db))
}

pub fn model_deployment_service_builder(db: &Database, channel: Arc<Channel>) -> ModelDeploymentService {
    ModelDeploymentService::new(
        model_deployment_repo_factory(db),
        model_metadata_repo_factory(db),
        artifact_repo_factory(db),
        event_publisher_factory(channel),
    )
}

pub fn model_deployment_conroller_builder(db: &Database, channel: Arc<Channel>) -> Arc<ModelDeploymentController> {
    Arc::new(ModelDeploymentController::new(
        model_deployment_service_builder(db, channel.clone()),
        model_metadata_repo_factory(db),
        event_publisher_factory(channel.clone()),
        model_deployment_platform_reconciler_provider_factory(),
    ))
}