use std::path::PathBuf;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use crate::retry::{retry_async, RetryPolicy, ExponentionalBackoff, FixedBackoff, Retry, Jitter};
use crate::application::errors::ApplicationError;
use crate::application::inputs::artifacts::{DownloadArtifactInput, IngestArtifactInput, UploadArtifactInput};
use crate::application::inputs::artifact_publication::PublishArtifactInput;
use crate::application::ports::events::{Event, EventPublisher, EventPublisherError, IngestArtifactEventPayload, PublishArtifactEventPayload};
use crate::application::ports::repositories::{ArtifactIngestionRepository, ArtifactPublicationRepository, ArtifactRepository, ModelMetadataRepository};
use crate::domain::entities::artifact::{Artifact, ArtifactType as ArtifactTypeEntity};
use crate::domain::entities::artifact_ingestion::{ArtifactIngestion, ArtifactIngestionError, ArtifactIngestionFailureReason, ArtifactIngestionStatus};
use crate::domain::entities::artifact_publication::{ArtifactPublication, ArtifactPublicationStatus, ArtifactPublicationError, ArtifactPublicationFailureReason};
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::services::{
    ArtifactService as DomainArtifactService,
    ArtifactServiceError as DomainArtifactServiceError};
use thiserror::Error;
use once_cell::sync::Lazy;
use uuid::Uuid;
use crate::logging::GlobalLogger;
use crate::constants::ARTIFACT_INGEST_DIR_NAME;
use crate::infra::fs::stacking::FileStacker;
use crate::infra::system::Env;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("Message broker error: {0}")]
    PubisherError(#[from] EventPublisherError),

    #[error("Repository error: {0}")]
    RepoError(#[from] ApplicationError),

    #[error("Artifact ingestion error: {0}")]
    ArtifactIngestionError(#[from] ArtifactIngestionError),

    #[error("Artifact publication error: {0}")]
    ArtifactPublicationError(#[from] ArtifactPublicationError),

    #[error("Artifact service error: {0}")]
    DomainArtifactServiceError(#[from] DomainArtifactServiceError),

    #[error("Not Found Error: {0}")]
    NotFound(String),

    #[error("Missing artifact file(s) error: {0}")]
    MissingArtifactFiles(String),

    #[error("Missing artifact: {0}")]
    MissingArtifact(String),

    #[error("Missing metadata: {0}")]
    MissingMetadata(String),

    #[error("Artifact not ingested: {0}")]
    AritfactNotIngested(String),

    #[error("UnexpectedState: {0}")]
    UnexpectedState(String),
}

pub enum UuidOrString {
    Uuid(Uuid),
    String(String),
}

pub struct ArtifactService {
    artifact_repo: Arc<dyn ArtifactRepository>,
    ingestion_repo: Arc<dyn ArtifactIngestionRepository>,
    publication_repo: Arc<dyn ArtifactPublicationRepository>,
    metadata_repo: Arc<dyn ModelMetadataRepository>,
    event_publisher: Arc<dyn EventPublisher>,
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
        publication_repo: Arc<dyn ArtifactPublicationRepository>,
        metadata_repo: Arc<dyn ModelMetadataRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            artifact_repo,
            ingestion_repo,
            publication_repo,
            metadata_repo,
            event_publisher,
        }
    }

    /// Creates an artifact publication
    pub async fn submit_artifact_publication(&self, input: PublishArtifactInput) -> Result<ArtifactPublication, ArtifactServiceError> {
        // Closure for fetching the artifact
        let find_artifact = || self.artifact_repo.find_by_id(&input.artifact_id);
        
        // Find the artifact with retries
        let maybe_artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        // Check that the artifact exists
        if maybe_artifact.is_none() {
            return Err(ArtifactServiceError::MissingArtifact("Artifact must exist in order to publish it".into()))
        }

        // Fetch artifact metadata
        let _ = self.find_metadata_by_artifact_id(&input.artifact_id)
            .await?;

        // Instantiate the ArtifactPublication
        let mut publication = ArtifactPublication::new(
            input.artifact_id,
            input.target_platform,
        );

        // Closure for saving the publication
        let save_publication = || self.publication_repo.save(&publication);

        // Save publication with retries. Propogate error
        retry_async(save_publication, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        
        let payload = PublishArtifactEventPayload {
            publication_id: publication.id.clone(),
            webhook_url: input.webhook_url.clone(),
            serialized_client_request: input.serialized_client_request.clone(),
        };

        let event = Event::PublishArtifactEvent(payload.clone());

        // Closure for publishing artifact
        let publish_artifact = || self.event_publisher.publish(
            &event
        );
        
        // Handle the artifact publication with retries
        let publish_result = retry_async(publish_artifact, &Self::MQ_RETRY_POLICY).await
            .map_err(|err| {ArtifactServiceError::PubisherError(err)});

        if let Err(err) = publish_result {
            GlobalLogger::error(format!("Failed to publish ArtifactIngestion: {}", &err.to_string()).as_str());

            publication.change_status(&ArtifactPublicationStatus::Failed(ArtifactPublicationFailureReason::FailedToQueue("Failed to queue".into())))
                .map_err(|err| ArtifactServiceError::ArtifactPublicationError(err))?;

            let update_ingestion = || 
                self.publication_repo.update_status(&publication);
            
            let _ = retry_async(update_ingestion, &Self::REPO_RETRY_POLICY).await
                .map_err(|err| ArtifactServiceError::RepoError(err))?;
            
            return Err(err)
        };

        return Ok(publication)
    }

    pub async fn find_metadata_by_artifact_id(&self, artifact_id: &Uuid) -> Result<ModelMetadata, ArtifactServiceError> {
        // Closure for fetching the metadata for this artifact
        let find_metadata = || self.metadata_repo.find_by_artifact_id(&artifact_id);

        // Find the metadata with retries
        let maybe_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        // Check that the artifact exists
        match maybe_metadata {
            Some(m) => Ok(m),
            None => Err(ArtifactServiceError::MissingMetadata("Artifact must exist in order to publish it".into()))
        }
    }

    pub async fn change_publication_status_by_publication_id(
        &self,
        publication_id: Uuid,
        status: ArtifactPublicationStatus,
        message: Option<String>
    ) -> Result<(), ArtifactServiceError> {
        let find_publication = || self.publication_repo.find_by_id(publication_id);

        // Find the publication
        let maybe_publication = retry_async(find_publication, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let mut publication = match maybe_publication {
            Some(i) => i,
            None => {
                GlobalLogger::error(format!("Cannot find any record of ArtifactPublication '{}'.", publication_id).as_str());
                return Err(ArtifactServiceError::NotFound(format!("Cannot find any record of ArtifactPublication '{}'.", publication_id)))
            }
        };

        publication.change_status(&status)?;

        // Only set the message if one was provided
        if let Some(msg) = message {
            publication.last_message = Some(msg)
        }

        let update_publication = || self.publication_repo.update_status(&publication);

        retry_async(update_publication, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        Ok(())
    }

    pub async fn find_publication_by_publication_id(&self, publication_id: Uuid) -> Result<Option<ArtifactPublication>, ArtifactServiceError> {
        let find_publication = || self.publication_repo.find_by_id(publication_id);

        let maybe_publication = retry_async(find_publication, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        return Ok(maybe_publication)
    }

    pub async fn submit_artifact_ingestion(&self, input: IngestArtifactInput) -> Result<ArtifactIngestion, ArtifactServiceError> {
        let artifact = Artifact::new(ArtifactTypeEntity::from(input.artifact_type.clone()));
        
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
        let payload = IngestArtifactEventPayload {
            ingestion_id: ingestion.id.clone(),
            artifact_type: input.artifact_type.clone(),
            platform: ingestion.platform.clone(),
            serialized_client_request: input.serialized_client_request.clone(),
            webhook_url: input.webhook_url.clone()
        };

        let event = Event::IngestArtifactEvent(payload.clone());
        let publish_ingestion = || self.event_publisher.publish(
            &event
        );
        
        // Publish the artifact ingestion request to the queue
        let publish_result = retry_async(publish_ingestion, &Self::MQ_RETRY_POLICY).await
            .map_err(|err| {ArtifactServiceError::PubisherError(err)});

        if let Err(err) = publish_result {
            GlobalLogger::error(format!("Failed to publish ArtifactIngestion: {}", &err.to_string()).as_str());

            ingestion.change_status(ArtifactIngestionStatus::Failed(ArtifactIngestionFailureReason::FailedToQueue))
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
        let find_artifact = || self.artifact_repo.find_by_id(&ingestion.artifact_id);
        
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
        // Check if the artifact path actually exists
        if !artifact_path.exists() {
            return Err(ArtifactServiceError::MissingArtifactFiles(format!("No files found for Artifact '{}' at path '{}'", artifact.id.to_string(), artifact_path.to_string_lossy())))
        }

        ingestion.set_artifact_path(artifact_path.clone())?;

        ingestion.change_status(ArtifactIngestionStatus::Finished)?;

        // Closure for saving the updated ingestion
        let update = || self.ingestion_repo.update(ingestion);

        // Update the ingestions
        retry_async(update, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        DomainArtifactService::finish_artifact_ingestion(artifact, ingestion)?;  

        // Closure for saving the updated artifact
        let update = || self.artifact_repo.update(artifact);

        // Update the artifact
        retry_async(update, &Self::REPO_RETRY_POLICY).await
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
        let mut artifact = Artifact::new(ArtifactTypeEntity::from(input.artifact_type.clone()));
        
        // Closure for saving the artifact
        let save_artifact = || self.artifact_repo.save(&artifact);

        // Persist the new Artifact to the database
        retry_async(save_artifact, &Self::REPO_RETRY_POLICY).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let environment = Env::new().expect("Env could not be initialized");

        // Set the artifact ingest dir on the 
        artifact.set_path(PathBuf::from(&environment.shared_data_dir)
            .join(ARTIFACT_INGEST_DIR_NAME)
            .join(artifact.id.to_string()));

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
        let find_artifact = || self.artifact_repo.find_by_id(&artifact_uuid);

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

    pub fn get_ingested_artifact_path(&self, artifact: &Artifact) -> Result<PathBuf, ArtifactServiceError> {
        if !artifact.is_fully_ingested() {
            return Err(ArtifactServiceError::AritfactNotIngested("Attempting to get the path of an Artifact that is not fully ingested".into()))
        };

        let path = artifact.path.clone()
            .ok_or_else(|| ArtifactServiceError::UnexpectedState("Attempting to access path on a fully ingested artifact, but the path is None".into()))?;

        Ok(path)
    }

    // TODO This should not be internally loading in the artifact just to get the
    // path. The artifact should be fetched before calling this method and the 
    // reference to that should be passed an an argument
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
