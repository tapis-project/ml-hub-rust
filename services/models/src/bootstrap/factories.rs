//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use amqprs::channel::Channel;
#[cfg(feature = "mongo")]
use mongodb::Client;
use shared::application::errors::ApplicationError;
use shared::application::ports::deployment::DeploymentStrategyProvider;
use shared::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::application::ports::artifacts::{
    ArtifactRepository,
    ArtifactIngestionRepository,
    ArtifactPublicationRepository,
};
use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::application::services::artifact_service::ArtifactService;
use crate::application::services::model_metadata_service::ModelMetadataService;
use crate::infra::persistence::mongo::repositories::{
    ArtifactRepository as MongoArtifactRepository,
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
    ModelMetadataRepository as MongoModelMetadataRepository,
    ArtifactPublicationRepository as MongoArtifactPublicationRepository,
};
use crate::infra::deployment::fs::deployment_strategy_provider::DeploymentStrategyProviderFs;
use crate::infra::messaging::rabbitmq::artifact_op_message_publisher::RabbitMQArtifactOpMessagePublisher;
use std::sync::Arc;

#[cfg(feature = "mongo")]
pub fn artifact_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(client, db_name.clone()))
}

#[cfg(feature = "mongo")]
pub fn artifact_ingestion_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactIngestionRepository> {
    Arc::new(MongoArtifactIngestionRepository::new(client, db_name.clone()))
}

#[cfg(feature = "mongo")]
pub fn model_metadata_repo_factory(client: &Client, db_name: String) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(client, db_name.clone()))
}

#[cfg(feature = "mongo")]
pub fn artifact_publication_repo_factory(client: &Client, db_name: String) -> Arc<dyn ArtifactPublicationRepository> {
    Arc::new(MongoArtifactPublicationRepository::new(client, db_name.clone()))
}

pub fn artifact_service_factory(client: &Client, db_name: String, channel: Arc<Channel>) -> ArtifactService {    
    ArtifactService::new(
        artifact_repo_factory(client, db_name.clone()),
        artifact_ingestion_repo_factory(client, db_name.clone()),
        artifact_publication_repo_factory(client, db_name.clone()),
        model_metadata_repo_factory(client, db_name.clone()),
        Arc::new(RabbitMQArtifactOpMessagePublisher::new(channel.clone()))
    )
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn DeploymentStrategyProvider>, ApplicationError> {
    let provider = DeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(err) => Err(err)
    }
}

pub async fn model_metadata_service_factory(
    client: &Client,
    db_name: String,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>
) -> Result<ModelMetadataService, ApplicationError> {    
    Ok(ModelMetadataService::new(
        model_metadata_repo_factory(client, db_name.clone()),
        artifact_repo_factory(client, db_name.clone()),
        client_strategy_sets,
    ))
}
