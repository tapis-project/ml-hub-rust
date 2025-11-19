//! This module contains factories that wire together infrastructure-level concerns
//! with application-level concerns
use mongodb::Database;
use shared::application::errors::ApplicationError;
use shared::application::ports::repositories::{ModelMetadataRepository, ArtifactRepository};
use shared::application::services::model_metadata_service::ModelMetadataService;
use shared::infra::persistence::mongo::repositories::{
    ModelMetadataRepository as MongoModelMetadataRepository,
    ArtifactRepository as MongoArtifactRepository,
};
use std::sync::Arc;

pub fn artifact_repo_factory(db: &Database) -> Arc<dyn ArtifactRepository> {
    Arc::new(MongoArtifactRepository::new(db))
}

pub fn model_metadata_repo_factory(db: &Database) -> Arc<dyn ModelMetadataRepository> {
    Arc::new(MongoModelMetadataRepository::new(db))
}

pub async fn model_metadata_service_factory(db: &Database) -> Result<ModelMetadataService, ApplicationError> {    
    Ok(ModelMetadataService::new(
        model_metadata_repo_factory(db),
        artifact_repo_factory(db),
    ))
}