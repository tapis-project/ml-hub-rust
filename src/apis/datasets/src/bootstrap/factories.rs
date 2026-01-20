//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Database;
use shared::application::errors::ApplicationError;
use crate::application::ports::repositories::{
    ArtifactRepository,
    ArtifactIngestionRepository,
    DatasetMetadataRepository,
    ArtifactPublicationRepository,
};
use crate::application::services::artifact_service::{ArtifactService, MetadataRepoVariant};
use crate::application::services::dataset_metadata_service::DatasetMetadataService;
use crate::infra::persistence::mongo::repositories::{
    ArtifactRepository as MongoArtifactRepository,
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
    DatasetMetadataRepository as MongoDatasetMetadataRepository,
    ArtifactPublicationRepository as MongoArtifactPublicationRepository,
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
pub fn dataset_metadata_repo_factory(db: &Database) -> Arc<dyn DatasetMetadataRepository> {
    Arc::new(MongoDatasetMetadataRepository::new(db))
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
        MetadataRepoVariant::Other(dataset_metadata_repo_factory(db)),
        Arc::new(RabbitMQArtifactOpMessagePublisher {})
    ))
}

pub async fn dataset_metadata_service_factory(db: &Database) -> Result<DatasetMetadataService, ApplicationError> {
    Ok(DatasetMetadataService::new(
        dataset_metadata_repo_factory(db),
        artifact_repo_factory(db),
    ))
}
