use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::artifact_ingestion::ArtifactIngestion;
use crate::application::errors::ApplicationError;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ArtifactRepository: Send + Sync {
    async fn save(&self, artifact: &Artifact) -> Result<(), ApplicationError>;
    async fn update(&self, ingestion: &Artifact) -> Result<(), ApplicationError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Artifact>, ApplicationError>;
    async fn list_all(&self) -> Result<Vec<Artifact>, ApplicationError>;
    async fn update_path(&self, artifact: &Artifact) -> Result<(), ApplicationError>;
}

#[async_trait]
pub trait ArtifactIngestionRepository: Send + Sync {
    async fn save(&self, ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
    async fn update(&self, ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
    async fn update_status(&self, ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
    async fn find_by_artifact_id(&self, id: Uuid) -> Result<Vec<ArtifactIngestion>, ApplicationError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ArtifactIngestion>, ApplicationError>;
}

// #[async_trait]
// pub trait ModelMetadataRepository: Send + Sync {
//     async fn save(&self, metadata: &ModelMetadata) -> Result<(), ApplicationError>;
//     async fn update(&self, ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
//     async fn update_status(&self, ingestion: &ArtifactIngestion) -> Result<(), ApplicationError>;
//     async fn find_by_artifact_id(&self, id: Uuid) -> Result<Vec<ArtifactIngestion>, ApplicationError>;
//     async fn find_by_id(&self, id: Uuid) -> Result<Option<ArtifactIngestion>, ApplicationError>;
// }