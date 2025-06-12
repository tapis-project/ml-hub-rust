use std::sync::Arc;
use crate::retry::{retry_async, RetryPolicy, ExponentionalBackoff, FixedBackoff, Retry, Jitter};
use crate::common::application::errors::ApplicationError;
use crate::common::application::inputs::IngestArtifactInput;
use crate::common::application::ports::messaging::{MessagePublisher, MessagePublisherError, Message};
use crate::common::application::ports::repositories::{ArtifactRepository, ArtifactIngestionRepository};
use crate::common::domain::entities::{Artifact, ArtifactIngestion, ArtifactIngestionError, ArtifactIngestionFailureReason as Reason, ArtifactIngestionStatus};
use thiserror::Error;
use once_cell::sync::Lazy;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("Message broker error: {0}")]
    PubisherError(#[from] MessagePublisherError),

    #[error("Repository error: {0}")]
    RepoError(#[from] ApplicationError),

    #[error("Artifact ingestion error: {0}")]
    ArtifactIngestionError(#[from] ArtifactIngestionError),
}

pub struct ArtifactService {
    artifact_repo: Arc<dyn ArtifactRepository>,
    ingestion_repo: Arc<dyn ArtifactIngestionRepository>,
    publisher: Arc<dyn MessagePublisher>
}

impl ArtifactService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::FixedBackoff(
        FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        }
    ));

    const MQ_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::ExponentionalBackoff(
        ExponentionalBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
            base: Some(2),
            max_delay: 500,
            jitter: Some(Jitter::Full)
        }
    ));

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

    pub async fn ingest_artifact(&self, input: IngestArtifactInput) -> Result<ArtifactIngestion, ArtifactServiceError> {
        let artifact = Artifact::new();
        
        // Closure for saving the artifact
        let save_artifact = || self.artifact_repo.save(&artifact);

        // Persist the new Artifact to the database
        retry_async(save_artifact, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let mut ingestion = ArtifactIngestion::new(
            artifact.id.clone(),
            input.platform.clone(),
            input.webhook_url.clone()
        );

        // Closure for saving the ingestion
        let save_ingestion = || self.ingestion_repo.save(&ingestion);

        // Persist the new ArtifactIngestion to the database
        // TODO need to attempt to clean up the Artifact that was just persisted if ingestion fails
        let _ = retry_async(save_ingestion, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err));

        // Closure for publishing the artifact ingestion request
        let publish_ingestion = || self.publisher.publish(
            Message::IngestArtifactInput(input.clone())
        );
        
        // Publish the artifact ingestion request to the queue
        let publish_result = retry_async(publish_ingestion, &Self::MQ_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::PubisherError(err));

        if let Err(err) = publish_result {
            ingestion.change_status(ArtifactIngestionStatus::Failed(Reason::FailedToQueue))
                .map_err(|err| ArtifactServiceError::ArtifactIngestionError(err))?;
            
            let update_ingestion = || self.ingestion_repo.update_status(&ingestion.id, &ingestion.status);
            
            let _ = retry_async(update_ingestion, &Self::REPO_RETRY_POLICY).await
                .map_err(|err| ArtifactServiceError::RepoError(err))?;
            
            return Err(err)
        };

        return Ok(ingestion)
    }
}
