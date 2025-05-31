use crate::domain::entities::{Artifact, ArtifactIngestion};
use crate::application::errors::ApplicationError;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ArtifactRepository {
    async fn save(&self, artifact: &Artifact) -> Result<(), ApplicationError>;
    // fn update(&self, artifact: &Artifact) -> Result<(), ()>;
    // fn get_by_id(&self, id: Uuid) -> Result<Artifact, ()>;
}

#[async_trait]
pub trait ArtifactIngestionRepository {
    async fn save(&self, artifact_ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
    // fn update(&self, artifact_ingestion: &ArtifactIngestion) -> Result<(), ()>;
    // fn find_by_artifact_id(&self, id: Uuid) -> Result<Vec<ArtifactIngestion>, ()>;
}