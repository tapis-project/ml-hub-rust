use crate::domain::entities::artifact::{Artifact, ArtifactType};
use crate::domain::entities::artifact_ingestion::ArtifactIngestion;
use crate::domain::entities::artifact_publication::ArtifactPublication;
use crate::application::errors::ApplicationError;
use crate::application::ports::errors::CommonRepositoryError;
use uuid::Uuid;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtifactRepositoryError {
    #[error(transparent)]
    Persistence(#[from] CommonRepositoryError),
}

#[async_trait]
pub trait ArtifactRepository: Send + Sync {
    async fn save(&self, artifact: &Artifact) -> Result<(), ArtifactRepositoryError>;
    async fn update(&self, artifact: &Artifact) -> Result<(), ArtifactRepositoryError>;
    async fn update_path(&self, artifact: &Artifact) -> Result<(), ArtifactRepositoryError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Artifact>, ArtifactRepositoryError>;
    async fn list_by_artifact_type(&self, artifact_type: ArtifactType) -> Result<Vec<Artifact>, ArtifactRepositoryError>;
}

#[derive(Debug, Error)]
pub enum ArtifactIngestionRepositoryError {
    #[error(transparent)]
    Persistence(#[from] CommonRepositoryError),
}

#[async_trait]
pub trait ArtifactIngestionRepository: Send + Sync {
    async fn save(&self, ingestion: &ArtifactIngestion) -> Result<(), ArtifactIngestionRepositoryError>;
    async fn update(&self, ingestion: &ArtifactIngestion) -> Result<(), ArtifactIngestionRepositoryError>;
    async fn update_status(&self, ingestion: &ArtifactIngestion) -> Result<(), ArtifactIngestionRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ArtifactIngestion>, ArtifactIngestionRepositoryError>;
    async fn find_by_artifact_id(&self, id: &Uuid) -> Result<Vec<ArtifactIngestion>, ArtifactIngestionRepositoryError>;
    async fn find_by_artifact_type(&self, artifact_type: ArtifactType) -> Result<Vec<ArtifactIngestion>, ArtifactIngestionRepositoryError>;
}

#[derive(Debug, Error)]
pub enum ArtifactPublicationRepositoryError {
    #[error(transparent)]
    Persistence(#[from] CommonRepositoryError),
}

#[async_trait]
pub trait ArtifactPublicationRepository: Send + Sync {
    async fn save(&self, publication: &ArtifactPublication) -> Result<(), ArtifactPublicationRepositoryError>;
    async fn update_status(&self, ingestion: &ArtifactPublication) -> Result<(), ArtifactPublicationRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ArtifactPublication>, ArtifactPublicationRepositoryError>;
    async fn find_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Vec<ArtifactPublication>, ArtifactPublicationRepositoryError>;
    async fn find_by_artifact_type(&self, artifact_type: ArtifactType) -> Result<Vec<ArtifactPublication>, ArtifactPublicationRepositoryError>;
}