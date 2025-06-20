use crate::common::domain::entities::{Artifact, ArtifactIngestion, ArtifactIngestionStatus, TimeStamp};
use crate::common::application::errors::ApplicationError;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ArtifactRepository: Send + Sync {
    async fn save(&self, artifact: &Artifact) -> Result<(), ApplicationError>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Artifact>, ApplicationError>;
    async fn list_all(&self) -> Result<Vec<Artifact>, ApplicationError>;
    // async fn update(&self, artifact: &Artifact) -> Result<(), ApplicationError>;
}

#[async_trait]
pub trait ArtifactIngestionRepository: Send + Sync {
    async fn save(&self, ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
    async fn update_status(&self, ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
    async fn find_by_artifact_id(&self, id: Uuid) -> Result<Vec<ArtifactIngestion>, ApplicationError>;
}