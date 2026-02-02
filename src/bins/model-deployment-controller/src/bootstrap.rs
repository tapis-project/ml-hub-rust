//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Database;
use shared::application::errors::ApplicationError;
use shared::application::ports::artifacts::{
    ArtifactRepository,
    ArtifactIngestionRepository,
    ArtifactPublicationRepository,
};
use shared::application::ports::model_metadata::ModelMetadataRepository;
use shared::application::services::artifact_service::ArtifactService;
use shared::infra::persistence::mongo::repositories::{
    ArtifactRepository as MongoArtifactRepository,
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
    ArtifactPublicationRepository as MongoArtifactPublicationRepository,
    ModelMetadataRepository as MongoModelMetadataRepository,
};
use shared::infra::messaging::rabbitmq::artifact_op_message_publisher::RabbitMQArtifactOpMessagePublisher;
use std::sync::Arc;

pub fn artifact_repo_factory(db: &Database) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(db))
}

pub fn artifact_ingestion_repo_factory(db: &Database) -> Arc<dyn ArtifactIngestionRepository> {
    Arc::new(MongoArtifactIngestionRepository::new(db))
}

pub fn artifact_publication_repo_factory(db: &Database) -> Arc<dyn ArtifactPublicationRepository> {
    Arc::new(MongoArtifactPublicationRepository::new(db))
}

pub fn model_metadata_repo_factory(db: &Database) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(db))
}

pub fn artifact_service_factory(db: &Database) -> Result<ArtifactService, ApplicationError> {    
    Ok(ArtifactService::new(
        artifact_repo_factory(db),
        artifact_ingestion_repo_factory(db),
        artifact_publication_repo_factory(db),
        model_metadata_repo_factory(db),
        Arc::new(RabbitMQArtifactOpMessagePublisher {
            host: std::env::var("ARTIFACT_OP_MQ_HOST").expect("ARTIFACT_OP_MQ_URL missing from environment variables"),
            port: std::env::var("ARTIFACT_OP_MQ_PORT").expect("ARTIFACT_OP_MQ_PORT missing from environment variables"),
            username: std::env::var("ARTIFACT_OP_MQ_USER").expect("ARTIFACT_OP_MQ_USER missing from environment variables"),
            password: std::env::var("ARTIFACT_OP_MQ_PASSWORD").expect("ARTIFACT_OP_MQ_PASSWORD missing from environment variables"),
        })
    ))
}