//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Database;
use shared::common::application::errors::ApplicationError;
use crate::application::ports::repositories::{
    ArtifactRepository,
    ArtifactIngestionRepository,
};
use crate::application::services::artifact_service::ArtifactService;
use crate::infra::persistence::mongo::repositories::{
    ArtifactRepository as MongoArtifactRepository,
    ArtifactIngestionRepository as MongoArtifactIngestionRepository,
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

pub async fn artifact_service_factory(db: &Database) -> Result<ArtifactService, ApplicationError> {    
    Ok(ArtifactService::new(
        artifact_repo_factory(db),
        artifact_ingestion_repo_factory(db),
        Arc::new(RabbitMQArtifactOpMessagePublisher {})
    ))
}

