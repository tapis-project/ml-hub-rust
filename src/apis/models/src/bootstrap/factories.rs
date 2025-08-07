//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Database;
use shared::application::errors::ApplicationError;
use crate::application::ports::repositories::{
    ArtifactRepository,
    ArtifactIngestionRepository,
    ModelMetadataRepository
};
use crate::application::services::artifact_service::ArtifactService;
use crate::application::services::model_metadata_service::ModelMetadataService;
use crate::infra::persistence::mongo::repositories::{
    ArtifactRepository as MongoArtifactRepository,
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
    ModelMetadataRepository as MongoModelMetadataRepository,
};
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

pub async fn artifact_service_factory(db: &Database) -> Result<ArtifactService, ApplicationError> {    
    Ok(ArtifactService::new(
        artifact_repo_factory(db),
        artifact_ingestion_repo_factory(db),
        Arc::new(RabbitMQArtifactOpMessagePublisher {})
    ))
}

pub async fn model_metadata_service_factory(db: &Database) -> Result<ModelMetadataService, ApplicationError> {    
    Ok(ModelMetadataService::new(
        model_metadata_repo_factory(db),
        artifact_repo_factory(db),
    ))
}
