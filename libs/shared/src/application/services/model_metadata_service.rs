use std::sync::Arc;
use crate::application::identity_context::IdentityContext;
use crate::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::domain::entities::model_metadata::ModelMetadata;
use crate::domain::entities::tenancy::GLOBAL_TENANT;
use crate::domain::services::deployment_strategy::resolve_viable_strategies;
use retry_utils::{retry_async, RetryPolicy, FixedBackoff, Retry};
use crate::application::errors::ApplicationError;
use crate::application::ports::artifacts::ArtifactRepository;
use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::application::inputs::model_metadata::{
    ModelMetadata as ModelMetadataRegistrationInput,
    AssociateModelMetadata,
    UpsertModelMetadata,
    UpdateModelMetadataArtifactId,
};
use crate::application::inputs::discover_models::DiscoverModelsInput;
use crate::application::outputs::discover_models::DiscoverModelsOutput;
use crate::domain::services::{
    ModelMetadataService as ModelMetadataDomainService,
    ModelMetadataServiceError as ModelMetadataDomainServiceError
};
use crate::domain::entities;
use thiserror::Error;
use once_cell::sync::Lazy;
use serde_json::{Value, to_value, json};
use log::error;
// use crate::logging::GlobalLogger;

#[derive(Debug, Error)]
pub enum ModelMetadataServiceError {
    #[error("Repository error: {0}")]
    RepoError(#[from] ApplicationError),

    #[error("Artifact not found: {0}")]
    ArtifactNotFound(String),

    #[error("Metadata not found: {0}")]
    MetdataNotFound(String),

    #[error("{0}")]
    DomainServiceError(#[from] ModelMetadataDomainServiceError),

    #[error("Metadata already exists for Artifact '{0}'")]
    DuplicateMetadataError(String),
}

pub struct ModelMetadataService {
    model_metadata_repo: Arc<dyn ModelMetadataRepository>,
    artifact_repo: Arc<dyn ArtifactRepository>,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>,
}

impl ModelMetadataService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::FixedBackoff(
        FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        }
    ));

    pub fn new(
        model_metadata_repo: Arc<dyn ModelMetadataRepository>,
        artifact_repo: Arc<dyn ArtifactRepository>,
        client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    ) -> Self {
        Self {
            model_metadata_repo,
            artifact_repo,
            client_strategy_sets
        }
    }

    pub async fn associate_metadata_with_artifact(&self, input: AssociateModelMetadata) -> Result<(), ModelMetadataServiceError> {
        // Get the artifact_id from the input
        let artifact_id = input.artifact_id.clone();

        let find_artifact = || self.artifact_repo.get_by_id(&artifact_id);

        // Find the artifact by id
        let artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY, None)
            .await?
            .ok_or_else(|| ModelMetadataServiceError::ArtifactNotFound(format!("Artifact with id {} does not exist", &artifact_id)))?;

        // Ensure no metadata already exists for this artifact
        let find_metadata = || self.model_metadata_repo.find_by_artifact_id(&artifact_id);

        let maybe_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY, None)
            .await?;

        let metadata = match maybe_metadata {
            Some(m) => m,
            None => return Err(ModelMetadataServiceError::MetdataNotFound(format!("No metadata found with author {} and name {}", input.author, input.name)))
        };

        // Determine if we are allowed to create the metadata for this artifact
        ModelMetadataDomainService::associate_metadata_with_artifact(&artifact, metadata)?;

        let update_input = UpdateModelMetadataArtifactId::from(input);
        
        let update_metadata = || self.model_metadata_repo.update_artifact_id(&update_input);

        retry_async(update_metadata, &Self::REPO_RETRY_POLICY, None)
            .await?;

        return Ok(())
    }

    pub async fn register_model_metadata(&self, input: UpsertModelMetadata) -> Result<(), ModelMetadataServiceError> {
        let metadata_entity = entities::model_metadata::ModelMetadata::try_from(input.metadata.clone())?;
        
        let modified_metadata = self.annotate_with_deployment_strategies(&metadata_entity);

        let modified_input = UpsertModelMetadata {
            metadata:  ModelMetadataRegistrationInput::try_from(modified_metadata)?
        };

        let upsert_metadata = || self.model_metadata_repo.upsert(&modified_input);

        retry_async(upsert_metadata, &Self::REPO_RETRY_POLICY, None)
            .await?;

        return Ok(())
    }

    pub async fn discover_models(&self, input: DiscoverModelsInput, identity_context: &IdentityContext) -> Result<DiscoverModelsOutput, ModelMetadataServiceError> {
        // By default search in the user's tenant. Search the global tenant if
        // specificed
        let mut tenant_ids = vec![identity_context.actor_tenant_id().clone()];
        if input.options.include_global_models().unwrap_or(false) {
            tenant_ids.push(String::from(GLOBAL_TENANT))
        }
        
        let discover_models = || self.model_metadata_repo.filter_model_metadata_by_criteria(&input, &tenant_ids);

        // Find model metadata by discovery criteria
        let output = retry_async(discover_models, &Self::REPO_RETRY_POLICY, None).await
            .map_err(|err| ModelMetadataServiceError::RepoError(err))?;

        let modified_models: Vec<_> = output.models
            .iter()
            .map(|m| self.annotate_with_deployment_strategies(m))
            .collect();

        let modified_output = DiscoverModelsOutput {
            models: modified_models,
            ..output
        };
        
        Ok(modified_output)
    }

    fn annotate_with_deployment_strategies(&self, model: &ModelMetadata) -> ModelMetadata {
        let mut modified_annotations = serde_json::Map::new();
        
        let mut deployment_strategies: Vec<Value> = vec![];
        
        for set in self.client_strategy_sets.iter() {
            // Ignore if there is in error resolving strategies
            match resolve_viable_strategies(model, set.strategies()) {
                Ok(viable_strategies) => {
                    let mut strategies: Vec<Value> = Vec::with_capacity(viable_strategies.len());
                
                    // Ignore strategies that cannot be converted to a Value.
                    for viable_strat in viable_strategies {
                        to_value(viable_strat.into_inner())
                            .map_err(|err| {
                                error!("Error converting viable strategy to Value: {}", err.to_string())
                            })
                            .ok()
                            .map(|v| strategies.push(v));
                    }
                    
                    deployment_strategies.push(
                        json!({
                            "platform": set.platform,
                            "strategies": Value::Array(strategies)
                        })
                    );
                }
                Err(err) => {
                    error!("Error resolving viable strategies for model annotation: {}", err.to_string())
                }
            } 
        }

        modified_annotations.insert("deployment_strategies".into(), Value::Array(deployment_strategies));
        
        if let Some(value) = model.annotations.clone() {
            for (k, v) in value.as_object().unwrap_or(&serde_json::Map::new()) {
                modified_annotations.insert(k.clone(), v.clone());
            }
        };

        ModelMetadata {
            annotations: Some(Value::from(modified_annotations)),
            ..model.clone()
        }
    }
}
