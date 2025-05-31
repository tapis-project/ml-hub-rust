use std::sync::Arc;
use crate::application::inputs::IngestArtifactInput;
use crate::application::ports::messaging::{MessagePublisher, MessagePublisherError};
use crate::application::ports::repositories::{ArtifactRepository, ArtifactIngestionRepository};
use crate::domain::entities::{Artifact, ArtifactIngestion};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("Message broker error: {0}")]
    PubisherError(#[from] MessagePublisherError),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct ArtifactService {
    artifact_repo: Arc<dyn ArtifactRepository>,
    ingestion_repo: Arc<dyn ArtifactIngestionRepository>,
    publisher: Arc<dyn MessagePublisher>
}

impl ArtifactService {
    pub fn new(
        artifact_repo: Arc<dyn ArtifactRepository>,
        ingestion_repo: Arc<dyn ArtifactIngestionRepository>,
        publisher: Arc<dyn MessagePublisher>
    ) -> Self {
        Self {
            artifact_repo,
            ingestion_repo,
            publisher
        }
    }

    pub async fn ingest_artifact(&self, input: IngestArtifactInput) -> Result<(), ArtifactServiceError> {
        let artifact = Artifact::new();

        // Persist the new Artifact to the database
        self.artifact_repo.save(&artifact);

        let ingestion = ArtifactIngestion::new(
            artifact.id.clone(),
            input.platform,
            input.webhook_url
        );

        self.ingestion_repo.save(&ingestion);

        if let Err(err) = self.publisher.publish(&[72]).await {
            return Err(ArtifactServiceError::PubisherError(err))
        };

        return Ok(())
    }
}
