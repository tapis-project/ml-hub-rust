use std::path::PathBuf;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use crate::retry::{retry_async, RetryPolicy, ExponentionalBackoff, FixedBackoff, Retry, Jitter};
use crate::common::application::errors::ApplicationError;
use crate::common::application::inputs::{ArtifactType, DownloadArtifactInput, IngestArtifactInput, UploadArtifactInput};
use crate::common::application::ports::messaging::{MessagePublisher, MessagePublisherError, Message, IngestArtifactMessagePayload};
use crate::common::application::ports::repositories::{ArtifactRepository, ArtifactIngestionRepository};
use crate::common::domain::entities::{Artifact, ArtifactIngestion, ArtifactIngestionError, ArtifactIngestionFailureReason as Reason, ArtifactIngestionStatus};
use crate::common::domain::services::{
    ArtifactService as DomainArtifactService,
    ArtifactServiceError as DomainArtifactServiceError};
use thiserror::Error;
use once_cell::sync::Lazy;
use uuid::Uuid;
use crate::logging::GlobalLogger;
use crate::constants::{DATASET_INGEST_DIR_NAME, MODEL_INGEST_DIR_NAME};
use crate::common::infra::fs::stacking::FileStacker;
use crate::common::infra::system::Env;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("Message broker error: {0}")]
    PubisherError(#[from] MessagePublisherError),

    #[error("Repository error: {0}")]
    RepoError(#[from] ApplicationError),

    #[error("Artifact ingestion error: {0}")]
    ArtifactIngestionError(#[from] ArtifactIngestionError),

    #[error("Artifact service error: {0}")]
    DomainArtifactServiceError(#[from] DomainArtifactServiceError),

    #[error("Not Found Error: {0}")]
    NotFound(String),

    #[error("Missing artifact file(s) error: {0}")]
    MissingArtifactFiles(String),
}

pub enum UuidOrString {
    Uuid(Uuid),
    String(String),
}

pub struct ArtifactService {
    artifact_repo: Arc<dyn ArtifactRepository>,
    ingestion_repo: Arc<dyn ArtifactIngestionRepository>,
    publisher: Arc<dyn MessagePublisher>,
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
            publisher,
        }
    }

    pub async fn submit_artifact_ingestion(&self, input: IngestArtifactInput) -> Result<ArtifactIngestion, ArtifactServiceError> {
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
        let message_payload = IngestArtifactMessagePayload {
            ingestion_id: ingestion.id.clone(),
            artifact_type: input.artifact_type.clone(),
            platform: ingestion.platform.clone(),
            serialized_client_request: input.serialized_client_request.clone(),
            webhook_url: input.webhook_url.clone()
        };

        let publish_ingestion = || self.publisher.publish(
            Message::IngestArtifactMessage(message_payload.clone())
        );
        
        // Publish the artifact ingestion request to the queue
        let publish_result = retry_async(publish_ingestion, &Self::MQ_RETRY_POLICY).await
            .map_err(|err| {ArtifactServiceError::PubisherError(err)});

        if let Err(err) = publish_result {
            GlobalLogger::error(format!("Failed to publish ArtifactIngestion: {}", &err.to_string()).as_str());

            ingestion.change_status(ArtifactIngestionStatus::Failed(Reason::FailedToQueue))
                .map_err(|err| ArtifactServiceError::ArtifactIngestionError(err))?;

            let update_ingestion = || 
                self.ingestion_repo.update_status(&ingestion);
            
            let _ = retry_async(update_ingestion, &Self::REPO_RETRY_POLICY).await
                .map_err(|err| ArtifactServiceError::RepoError(err))?;
            
            return Err(err)
        };

        return Ok(ingestion)
    }

    pub async fn find_artifact_by_ingestion_id(&self, ingestion_id: Uuid) -> Result<Option<Artifact>, ArtifactServiceError> {
        // Closure for fetching the ingestion
        let find_ingestion = || self.ingestion_repo.find_by_id(ingestion_id);

        // Find the ingestion
        let maybe_ingestion = retry_async(find_ingestion, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let ingestion = match maybe_ingestion {
            Some(i) => i,
            None => return Ok(None)
        };

        // Closure for fetching the artifact
        let find_artifact = || self.artifact_repo.find_by_id(ingestion.artifact_id);

        // Find the artifact
        let maybe_artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let artifact = match maybe_artifact {
            Some(a) => a,
            None => {
                GlobalLogger::error(format!("Cannot find any record of the Artifact associated with ArtifactIngestion '{}'.", ingestion.id).as_str());
                return Err(ArtifactServiceError::NotFound("Cannot find any record of the artifact associated with this ingestion".into()))
            }
        };

        Ok(Some(artifact))
    }

    pub async fn change_ingestion_status_by_ingestion_id(
        &self,
        ingestion_id: Uuid,
        status: ArtifactIngestionStatus,
        message: Option<String>
    ) -> Result<(), ArtifactServiceError> {
        let find_ingestion = || self.ingestion_repo.find_by_id(ingestion_id);

        // Find the ingestion
        let maybe_ingestion = retry_async(find_ingestion, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let mut ingestion = match maybe_ingestion {
            Some(i) => i,
            None => {
                GlobalLogger::error(format!("Cannot find any record of ArtifactIngestion '{}'.", ingestion_id).as_str());
                return Err(ArtifactServiceError::NotFound(format!("Cannot find any record of ArtifactIngestion '{}'.", ingestion_id)))
            }
        };

        ingestion.change_status(status)?;

        // Only set the message if one was provided
        if let Some(msg) = message {
            ingestion.last_message = Some(msg)
        }

        let update_ingestion = || self.ingestion_repo.update_status(&ingestion);

        retry_async(update_ingestion, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        Ok(())
    }

    pub async fn find_ingestion_by_ingestion_id(&self, ingestion_id: Uuid) -> Result<Option<ArtifactIngestion>, ArtifactServiceError> {
        let find_ingestion = || self.ingestion_repo.find_by_id(ingestion_id);

        let maybe_ingestion = retry_async(find_ingestion, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        return Ok(maybe_ingestion)
    }

    pub async fn finish_artifact_ingestion(&self, artifact_path: PathBuf, artifact: &mut Artifact, ingestion: &mut ArtifactIngestion) -> Result<(), ArtifactServiceError> {
        GlobalLogger::debug(format!("Artifact path: {}", artifact_path.clone().to_string_lossy().to_string()).as_str());
        // Check if the artifact path actually exists
        if !artifact_path.exists() {
            return Err(ArtifactServiceError::MissingArtifactFiles(format!("No files found for Artifact '{}' at path '{}'", artifact.id.to_string(), artifact_path.to_string_lossy())))
        }

        ingestion.set_artifact_path(artifact_path)?;

        // Closure for saving the updated ingestion
        let save_ingestion = || self.ingestion_repo.save(ingestion);

        // Save updated artifact
        retry_async(save_ingestion, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        DomainArtifactService::finish_artifact_ingestion(artifact, ingestion)?;

        // Closure for saving the updated artifact
        let save_artifact = || self.artifact_repo.save(artifact);

        // Save updated artifact
        retry_async(save_artifact, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        Ok(())
    }

    // Uploads an artifact and returns a tuple containing the artifact ID and a closure for saving chunks of the artifact
    pub async fn upload_artifact<'a>(
        &'a self,
        input: &'a UploadArtifactInput
    ) -> Result<
        (
            String,
            impl FnMut(Vec<u8>) -> Pin<Box<dyn Future<Output = Result<(), ArtifactServiceError>> + Send + 'a>>,
        ),
        ArtifactServiceError,
    > {
        GlobalLogger::debug(format!("Uploading start").as_str());
        let mut artifact = Artifact::new();
        GlobalLogger::debug(format!("New Artifact: {:#?}", artifact).as_str());
        // Closure for saving the artifact
        let save_artifact = || self.artifact_repo.save(&artifact);

        // Persist the new Artifact to the database
        retry_async(save_artifact, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let environment = Env::new().expect("Env could not be initialized");
        artifact.set_path(PathBuf::from(&environment.shared_data_dir).join(
            match input.artifact_type {
                ArtifactType::Dataset => {
                    DATASET_INGEST_DIR_NAME
                },
                ArtifactType::Model => {
                    MODEL_INGEST_DIR_NAME
                }
            }
        ).join(artifact.id.to_string()));

        // Closure for updating the artifact
        let update_artifact_path = || self.artifact_repo.update_path(&artifact);

        // Persist the new Artifact to the database
        retry_async(update_artifact_path, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let stacker = move |chunk: Vec<u8>| -> Pin<Box<dyn Future<Output = Result<(), ArtifactServiceError>> + Send + 'a>> {
            let filepath = artifact.path.clone();
            Box::pin(async move {
                if let Some(filepath) = filepath {
                    FileStacker::stack(&filepath, chunk)
                        .await
                        .map_err(|e| ArtifactServiceError::NotFound(format!("Fail to stack file: {}", e)))?;
                    Ok(())
                } else {
                    Err(ArtifactServiceError::NotFound(
                        "Artifact file path is not set.".into(),
                    ))
                }
            })
        };

        Ok((artifact.id.to_string(), stacker))
    }

    pub async fn find_artifact_by_artifact_id(&self, artifact_id: String) -> Result<Option<Artifact>, ArtifactServiceError> {
        let artifact_uuid = match Uuid::parse_str(&artifact_id) {
                Ok(uuid) => uuid,
                Err(_) => return Err(ArtifactServiceError::NotFound(format!("Invalid UUID string: {}", artifact_id)))
            };

        // Closure for fetching the artifact
        let find_artifact = || self.artifact_repo.find_by_id(artifact_uuid);

        // Find the artifact
        let maybe_artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let artifact = match maybe_artifact {
            Some(a) => a,
            None => {
                GlobalLogger::error(format!("Cannot find any record of the Artifact associated with ID '{}'.", artifact_uuid).as_str());
                return Err(ArtifactServiceError::NotFound("Cannot find any record of the artifact associated with ID".into()))
            }
        };

        Ok(Some(artifact))
    }

    pub async fn get_artifact_path(&self, input: DownloadArtifactInput) -> Result<PathBuf, ArtifactServiceError> {
        let artifact = self.find_artifact_by_artifact_id(input.artifact_id).await?;

        let artifact = match artifact {
            Some(a) => a,
            None => return Err(ArtifactServiceError::NotFound("Artifact not found".into()))
        };

        let path = artifact.path.clone().ok_or_else(|| ArtifactServiceError::NotFound("Artifact path is not set".into()))?;

        Ok(path)
    }
}
