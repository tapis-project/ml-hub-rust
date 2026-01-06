//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Database;
use shared::application::errors::ApplicationError;
use shared::application::ports::deployment::AutomatedDeploymentStrategyProvider;
use shared::domain::entities::automated_deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::application::ports::repositories::{
    ArtifactRepository,
    ArtifactIngestionRepository,
    ModelMetadataRepository,
    ArtifactPublicationRepository,
};
use crate::application::services::artifact_service::ArtifactService;
use crate::application::services::model_metadata_service::ModelMetadataService;
use crate::infra::persistence::mongo::repositories::{
    ArtifactRepository as MongoArtifactRepository,
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
    ModelMetadataRepository as MongoModelMetadataRepository,
    ArtifactPublicationRepository as MongoArtifactPublicationRepository,
};
use crate::infra::deployment::fs::automated_deployment_strategy_provider::AutomatedDeploymentStrategyProviderFs;
use crate::infra::messaging::rabbitmq::artifact_op_message_publisher::RabbitMQArtifactOpMessagePublisher;
use std::sync::Arc;

#[cfg(feature = "mongo")]
pub fn artifact_repo_factory(db: &Database) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(db))
}

#[cfg(feature = "mongo")]
pub fn artifact_ingestion_repo_factory(db: &Database) -> Arc<dyn ArtifactIngestionRepository> {
    Arc::new(MongoArtifactIngestionRepository::new(db))
}

#[cfg(feature = "mongo")]
pub fn model_metadata_repo_factory(db: &Database) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(db))
}

#[cfg(feature = "mongo")]
pub fn artifact_publication_repo_factory(db: &Database) -> Arc<dyn ArtifactPublicationRepository> {
    Arc::new(MongoArtifactPublicationRepository::new(db))
}

pub fn artifact_service_factory(db: &Database) -> Result<ArtifactService, ApplicationError> {    
    Ok(ArtifactService::new(
        artifact_repo_factory(db),
        artifact_ingestion_repo_factory(db),
        artifact_publication_repo_factory(db),
        model_metadata_repo_factory(db),
        Arc::new(RabbitMQArtifactOpMessagePublisher {})
    ))
}

pub fn build_deployment_strategy_provider() -> Result<Arc<dyn AutomatedDeploymentStrategyProvider>, ApplicationError> {
    let provider = AutomatedDeploymentStrategyProviderFs::new();
    match provider {
        Ok(p) => Ok(Arc::new(p)),
        Err(err) => Err(err)
    }
}

pub async fn model_metadata_service_factory(
    db: &Database,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>
) -> Result<ModelMetadataService, ApplicationError> {    
    Ok(ModelMetadataService::new(
        model_metadata_repo_factory(db),
        artifact_repo_factory(db),
        client_strategy_sets,
    ))
}
