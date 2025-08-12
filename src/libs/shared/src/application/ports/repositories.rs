use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::artifact_ingestion::ArtifactIngestion;
use crate::domain::entities::artifact_publication::ArtifactPublication;
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::application::errors::ApplicationError;
use crate::application::inputs::model_metadata::CreateModelMetadata;
use uuid::Uuid;
use async_trait::async_trait;

#[async_trait]
pub trait ArtifactRepository: Send + Sync {
    async fn save(&self, artifact: &Artifact) -> Result<(), ApplicationError>;
    async fn update(&self, ingestion: &Artifact) -> Result<(), ApplicationError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Artifact>, ApplicationError>;
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

#[async_trait]
pub trait ModelMetadataRepository: Send + Sync {
    async fn save(&self, input: &CreateModelMetadata) -> Result<(), ApplicationError>;
    async fn find_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<ModelMetadata>, ApplicationError>;
    // async fn update(&self, metadata: &ModelMetadata) -> Result<(), ApplicationError>;
    // async fn list(&self) -> Result<Vec<ModelMetadata>, ApplicationError>;
}

#[async_trait]
pub trait ArtifactPublicationRepository: Send + Sync {
    async fn save(&self, publication: &ArtifactPublication) -> Result<(), ApplicationError>;
    async fn update_status(&self, ingestion: &ArtifactPublication) -> Result<(), ApplicationError>;
    // async fn find_by_id(&self, id: Uuid) -> Result<Option<ArtifactPublication>, ApplicationError>;
    // async fn find_all_by_id(&self, artifact_id: Uuid) -> Result<Vec<ArtifactPublication>, ApplicationError>;
}