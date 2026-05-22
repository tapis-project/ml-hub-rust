use std::path::PathBuf;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;
use retry_utils::{retry_async, RetryPolicy, ExponentialBackoff, FixedBackoff, Retry, Jitter};
use crate::application::errors::ApplicationError;
use crate::application::inputs::artifacts::{DownloadArtifactInput, GetModelArtifactInput, IngestArtifactInput, ListIngestionsByArtifactIdInput, ListPublicationsByArtifactIdInput, UploadArtifactInput};
use crate::application::inputs::artifact_publication::{GetModelPublicationInput, ListModelPublicationsInput, PublishArtifactInput};
use crate::application::inputs::artifact_ingestion::{GetModelIngestionInput, ListModelIngestionsInput};
use crate::application::outputs::artifacts::ModelArtifactOutput;
use crate::application::ports::commands::{Command, CommandPublisher, CommandPublisherError, IngestArtifactCommandPayload, PublishArtifactCommandPayload};
use crate::application::ports::artifacts::{ArtifactIngestionRepository, ArtifactPublicationRepository, ArtifactRepository};
use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::application::ports::dataset_metadata::DatasetMetadataRepository;
use crate::domain::entities::artifact::{Artifact, ArtifactType as ArtifactTypeEntity};
use crate::domain::entities::artifact_ingestion::{ArtifactIngestion, ArtifactIngestionError, ArtifactIngestionStatus};
use crate::domain::entities::artifact_publication::{ArtifactPublication, ArtifactPublicationStatus, ArtifactPublicationError};
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::entities::dataset_metadata::DatasetMetadata;
use crate::domain::services::{
    ArtifactService as DomainArtifactService,
    ArtifactServiceError as DomainArtifactServiceError};
use futures::lock::Mutex;
use thiserror::Error;
use once_cell::sync::Lazy;
use uuid::Uuid;
use crate::logging::GlobalLogger;
use crate::constants::ARTIFACT_CACHE_DIR_NAME;
use crate::infra::fs::stacking::FileStacker;
use crate::infra::system::Env;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("Message broker error: {0}")]
    PubisherError(#[from] CommandPublisherError),

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

    #[error("Incorrect artifact type: {0}")]
    IncorrectArtifactType(String),
}

pub enum UuidOrString {
    Uuid(Uuid),
    String(String),
}

pub struct ArtifactService {
    artifact_repo: Arc<dyn ArtifactRepository>,
    ingestion_repo: Arc<dyn ArtifactIngestionRepository>,
    publication_repo: Arc<dyn ArtifactPublicationRepository>,
    model_metadata_repo: Arc<dyn ModelMetadataRepository>,
    dataset_metadata_repo: Arc<dyn DatasetMetadataRepository>,
    command_publisher: Arc<dyn CommandPublisher>,
}

impl ArtifactService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::FixedBackoff(
        FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        }
    ));

    const MQ_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::ExponentialBackoff(
        ExponentialBackoff {
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
        model_metadata_repo: Arc<dyn ModelMetadataRepository>,
        dataset_metadata_repo: Arc<dyn DatasetMetadataRepository>,
        command_publisher: Arc<dyn CommandPublisher>,
    ) -> Self {
        Self {
            artifact_repo,
            ingestion_repo,
            publication_repo,
            model_metadata_repo,
            dataset_metadata_repo,
            command_publisher,
        }
    }

    /// Creates an artifact publication
    pub async fn submit_artifact_publication(&self, input: PublishArtifactInput) -> Result<ArtifactPublication, ArtifactServiceError> {
        // Closure for fetching the artifact
        let find_artifact = || self.artifact_repo.get_by_id(&input.artifact_id);
        
        // Find the artifact with retries
        let maybe_artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        // Check that the artifact exists
        let artifact = match maybe_artifact {
            Some(a) => a,
            None => return Err(ArtifactServiceError::MissingArtifact("Artifact must exist in order to publish it".into()))
        };

        // Fetch artifact metadata
        match artifact.artifact_type {
            ArtifactTypeEntity::Model => {
                if let None = self.find_model_metadata_by_artifact_id(&input.artifact_id).await? {
                    return Err(ArtifactServiceError::MissingMetadata("Artifact must have an associated metadata entry in order to be published. Create a metadata entry for this artifact and try again".into()))
                };
            }
            ArtifactTypeEntity::Dataset => {
                if let None = self.find_dataset_metadata_by_artifact_id(&input.artifact_id).await? {
                    return Err(ArtifactServiceError::MissingMetadata("Artifact must have an associated metadata entry in order to be published. Create a metadata entry for this artifact and try again".into()))
                };
            }
        }

        // Instantiate the ArtifactPublication
        let mut publication = ArtifactPublication::new(
            artifact.id,
            artifact.artifact_type,
            input.target_platform,
        );

        // Closure for saving the publication
        let save_publication = || self.publication_repo.save(&publication);

        // Save publication with retries. Propagate error
        retry_async(save_publication, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        
        let payload = PublishArtifactCommandPayload {
            publication_id: publication.id.clone(),
            webhook_url: input.webhook_url.clone(),
            serialized_client_request: input.serialized_client_request.clone(),
        };

        let command = Command::PublishArtifactCommand(payload.clone());

        // Closure for publishing artifact
        let publish_artifact = || self.command_publisher.publish(
            &command
        );
        
        // Handle the artifact publication with retries
        let publish_result = retry_async(publish_artifact, &Self::MQ_RETRY_POLICY, None).await
            .map_err(|err| { ArtifactServiceError::PubisherError(err) });

        if let Err(err) = publish_result {
            GlobalLogger::error(format!("Failed to publish ArtifactIngestion: {}", &err.to_string()).as_str());

            publication.change_status(&ArtifactPublicationStatus::Failed)
                .map_err(|err| ArtifactServiceError::ArtifactPublicationError(err))?;

            let update_ingestion = || 
                self.publication_repo.update_status(&publication);
            
            let _ = retry_async(update_ingestion, &Self::REPO_RETRY_POLICY, None).await
                .map_err(|err| ArtifactServiceError::RepoError(err))?;
            
            return Err(err)
        };

        return Ok(publication)
    }

    pub async fn find_model_metadata_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<ModelMetadata>, ArtifactServiceError> {
        // Closure for fetching the metadata for this artifact
        let find_metadata = || self.model_metadata_repo.find_by_artifact_id(&artifact_id);

        // Find the metadata with retries
        let maybe_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        
        // Check that the artifact exists
        match maybe_metadata {
            Some(m) => 
            {
                Ok(Some(m))
            },
            None => Ok(None)
        }
    }

    pub async fn find_dataset_metadata_by_artifact_id(&self, artifact_id: &Uuid) -> Result<Option<DatasetMetadata>, ArtifactServiceError> {
        // Closure for fetching the metadata for this artifact
        let find_metadata = || self.dataset_metadata_repo.find_by_artifact_id(&artifact_id);

        // Find the metadata with retries
        let maybe_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        // Check that the artifact exists
        match maybe_metadata {
            Some(m) =>
                {
                    Ok(Some(m))
                },
            None => Ok(None)
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
        let maybe_publication = retry_async(find_publication, &Self::REPO_RETRY_POLICY, None).await
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

        retry_async(update_publication, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        Ok(())
    }

    pub async fn find_publication_by_publication_id(&self, publication_id: Uuid) -> Result<Option<ArtifactPublication>, ArtifactServiceError> {
        let find_publication = || self.publication_repo.find_by_id(publication_id);

        let maybe_publication = retry_async(find_publication, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        return Ok(maybe_publication)
    }

    pub async fn submit_artifact_ingestion(&self, input: IngestArtifactInput) -> Result<ArtifactIngestion, ArtifactServiceError> {
        let artifact = Artifact::new(ArtifactTypeEntity::from(input.artifact_type.clone()));
        
        // Closure for saving the artifact
        let save_artifact = || self.artifact_repo.save(&artifact);
        
        // Persist the new Artifact to the database
        retry_async(save_artifact, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;
        
        let mut ingestion = ArtifactIngestion::new(
            artifact.id.clone(),
            artifact.artifact_type.clone(),
            input.platform.clone(),
            input.webhook_url.clone()
        );
        
        // Closure for saving the ingestion
        let save_ingestion = || self.ingestion_repo.save(&ingestion);

        // Persist the new ArtifactIngestion to the database
        // TODO need to attempt to clean up the Artifact that was just persisted if ingestion fails
        let _ = retry_async(save_ingestion, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err));

        // Closure for submitting the artifact ingestion request
        let payload = IngestArtifactCommandPayload {
            ingestion_id: ingestion.id.clone(),
            artifact_type: input.artifact_type.clone(),
            platform: ingestion.platform.clone(),
            serialized_client_request: input.serialized_client_request.clone(),
            webhook_url: input.webhook_url.clone()
        };

        let command = Command::IngestArtifactCommand(payload.clone());
        let submit_ingestion = || self.command_publisher.publish(
            &command
        );
        
        // Submit the artifact ingestion request to the queue
        let submit_result = retry_async(submit_ingestion, &Self::MQ_RETRY_POLICY, None).await
            .map_err(|err| {ArtifactServiceError::PubisherError(err)});

        if let Err(err) = submit_result {
            GlobalLogger::error(format!("Failed to submit ArtifactIngestion: {}", &err.to_string()).as_str());

            ingestion.change_status(ArtifactIngestionStatus::Failed)
                .map_err(|err| ArtifactServiceError::ArtifactIngestionError(err))?;

            let update_ingestion = || 
                self.ingestion_repo.update_status(&ingestion);
            
            let _ = retry_async(update_ingestion, &Self::REPO_RETRY_POLICY, None).await
                .map_err(|err| ArtifactServiceError::RepoError(err))?;
            
            return Err(err)
        };

        return Ok(ingestion)
    }

    pub async fn find_artifact_by_ingestion_id(&self, ingestion_id: Uuid) -> Result<Option<Artifact>, ArtifactServiceError> {
        // Closure for fetching the ingestion
        let find_ingestion = || self.ingestion_repo.find_by_id(ingestion_id);

        // Find the ingestion
        let maybe_ingestion = retry_async(find_ingestion, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let ingestion = match maybe_ingestion {
            Some(i) => i,
            None => return Ok(None)
        };

        // Closure for fetching the artifact
        let find_artifact = || self.artifact_repo.get_by_id(&ingestion.artifact_id);
        
        // Find the artifact
        let maybe_artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY, None).await
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
        let maybe_ingestion = retry_async(find_ingestion, &Self::REPO_RETRY_POLICY, None).await
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

        retry_async(update_ingestion, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        Ok(())
    }

    pub async fn find_ingestion_by_ingestion_id(&self, ingestion_id: Uuid) -> Result<Option<ArtifactIngestion>, ArtifactServiceError> {
        let find_ingestion = || self.ingestion_repo.find_by_id(ingestion_id);

        let maybe_ingestion = retry_async(find_ingestion, &Self::REPO_RETRY_POLICY, None).await
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
        retry_async(update, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        DomainArtifactService::finish_artifact_ingestion(artifact, ingestion)?;  

        // Closure for saving the updated artifact
        let update = || self.artifact_repo.update(artifact);

        // Update the artifact
        retry_async(update, &Self::REPO_RETRY_POLICY, None).await
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
        retry_async(save_artifact, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let environment = Env::new().expect("Env could not be initialized");

        // Set the artifact ingest dir on the 
        artifact.set_path(PathBuf::from(&environment.shared_data_dir)
            .join(ARTIFACT_CACHE_DIR_NAME)
            .join(artifact.id.to_string()));

        
        let filepath: Arc<Mutex<Option<PathBuf>>>  = Arc::new(Mutex::new(artifact.path.clone()));
        let stacker_filepath: Arc<Mutex<Option<PathBuf>>> = filepath.clone(); 
        let stacker = move |chunk: Vec<u8>| {
            let filepath: Arc<Mutex<Option<PathBuf>>> = stacker_filepath.clone();
            Box::pin(async move {
                let path = filepath.lock().await.as_ref().unwrap().clone();
                FileStacker::stack(&path, chunk)
                    .await
                    .map_err(|e| ArtifactServiceError::NotFound(format!("Fail to stack file: {}", e)))
            }) as Pin<Box<dyn Future<Output = Result<(), ArtifactServiceError>> + Send + 'a>>
        };
    
        // Closure for updating the artifact
        let update_artifact_path = || self.artifact_repo.update_path(&artifact);

        // Persist the new Artifact to the database
        retry_async(update_artifact_path, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;
        
        Ok((artifact.id.to_string(), stacker))
    }

    pub async fn find_artifact_by_artifact_id(&self, artifact_id: Uuid) -> Result<Option<Artifact>, ArtifactServiceError> {
        // Closure for fetching the artifact
        let find_artifact = || self.artifact_repo.get_by_id(&artifact_id);

        // Find the artifact
        let maybe_artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ArtifactServiceError::RepoError(err))?;

        let artifact = match maybe_artifact {
            Some(a) => a,
            None => {
                GlobalLogger::error(format!("Cannot find any record of the Artifact associated with ID '{}'.", artifact_id).as_str());
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

    pub async fn get_model_artifact(&self, input: GetModelArtifactInput) -> Result<ModelArtifactOutput, ArtifactServiceError> {
        let artifact_id = input.artifact_id;
        
        let maybe_artifact = self.find_artifact_by_artifact_id(artifact_id.clone()).await?;

        let artifact = match maybe_artifact {
            Some(a) => a,
            None => return Err(ArtifactServiceError::MissingArtifact(format!("Cannot find artifact with id {}", artifact_id)))
        };

        if artifact.artifact_type != ArtifactTypeEntity::Model {
            return Err(ArtifactServiceError::MissingArtifact(format!("Cannot find artifact with id {}", artifact_id)))
        };

        Ok(ModelArtifactOutput {
            artifact,
            metadata: self.find_model_metadata_by_artifact_id(&input.artifact_id).await?
        })
    }

    pub async fn get_model_publication(&self, input: GetModelPublicationInput) -> Result<Option<ArtifactPublication>, ArtifactServiceError> {
        let publication = self.publication_repo.find_by_id(input.publication_id)
            .await?;

            match publication {
                Some(i) => {
                    if i.artifact_type != ArtifactTypeEntity::Model {
                       return Err(ArtifactServiceError::IncorrectArtifactType("ArtifactPublication is not associated with a Model artifact".into()))
                    };
    
                    Ok(Some(i))
                },
                None => Ok(None)
            }
    }

    pub async fn list_model_publications(&self, _input: ListModelPublicationsInput) -> Result<Vec<ArtifactPublication>, ArtifactServiceError> {
        let publications = self.publication_repo.find_by_artifact_type(ArtifactTypeEntity::Model)
            .await?;

        return Ok(publications)
    }

    pub async fn get_model_ingestion(&self, input: GetModelIngestionInput) -> Result<Option<ArtifactIngestion>, ArtifactServiceError> {
        let ingestion = self.ingestion_repo.find_by_id(input.ingestion_id)
            .await?;

        match ingestion {
            Some(i) => {
                if i.artifact_type != ArtifactTypeEntity::Model {
                   return Err(ArtifactServiceError::IncorrectArtifactType("ArtifactIngestion is not associated with a Model artifact".into()))
                };

                Ok(Some(i))
            },
            None => Ok(None)
        }
    }

    pub async fn list_model_ingestions(&self, _input: ListModelIngestionsInput) -> Result<Vec<ArtifactIngestion>, ArtifactServiceError> {
        let ingestions = self.ingestion_repo.find_by_artifact_type(ArtifactTypeEntity::Model)
            .await?;

        return Ok(ingestions)
    }

    pub async fn list_publications_by_artifact_id(&self, input: ListPublicationsByArtifactIdInput) -> Result<Vec<ArtifactPublication>, ArtifactServiceError> {
        let maybe_artifact = self.artifact_repo.get_by_id(&input.artifact_id)
            .await?;

        let artifact = match maybe_artifact {
            Some(a) => a,
            None => return Err(ArtifactServiceError::NotFound(format!("Artifact with id '{}' not found", &input.artifact_id)))
        };

        if artifact.artifact_type != ArtifactTypeEntity::Model {
            return Err(ArtifactServiceError::IncorrectArtifactType(format!("Expected type Model found type {}", &artifact.artifact_type)))
        }
        
        let publications = self.publication_repo.find_by_artifact_id(&input.artifact_id)
            .await?;

        return Ok(publications)
    }

    pub async fn list_ingestions_by_artifact_id(&self, input: ListIngestionsByArtifactIdInput) -> Result<Vec<ArtifactIngestion>, ArtifactServiceError> {
        let ingestions = self.ingestion_repo.find_by_artifact_id(&input.artifact_id)
            .await?;

        return Ok(ingestions)
    }

    pub async fn list_model_artifacts(&self) -> Result<Vec<Artifact>, ArtifactServiceError> {
        let artifacts = self.artifact_repo.list_by_artifact_type(ArtifactTypeEntity::Model)
            .await?;

        return Ok(artifacts)
    }

    pub async fn list_dataset_artifacts(&self) -> Result<Vec<Artifact>, ArtifactServiceError> {
        let artifacts = self.artifact_repo.list_by_artifact_type(ArtifactTypeEntity::Dataset)
            .await?;

        return Ok(artifacts)
    }
}
