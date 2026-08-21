use std::sync::Arc;

use crate::domain::entities::deployment_strategy::strategy::Strategy;
use crate::shared_kernel::context::RequestContext;
use crate::shared_kernel::constants::GLOBAL_TENANT;
use crate::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::domain::entities::model_metadata::{DeploymentStrategyReference, ModelMetadata};
use crate::domain::services::deployment_strategy::resolve_viable_strategies;
use retry_utils::{retry_async, RetryPolicy, FixedBackoff, Retry};
use crate::application::ports::artifacts::{ArtifactRepository, ArtifactRepositoryError};
use crate::application::ports::model_metadata::{ModelMetadataRepository, ModelMetadataRepositoryError};
use crate::application::inputs::model_metadata::{
    AssociateModelMetadata,
    GetModelMetadataByAuthorAndNameInput,
    ListModelMetadataByAuthorInput,
    RegisterModelMetadataInput,
    UpdateModelMetadataArtifactId,
};
use crate::application::inputs::discover_models::SearchModelsInput;
use crate::application::outputs::model_metadata::{ModelMetadataListOutput, ModelMetadataOutput};
use crate::application::outputs::model_metadata::ModelMetadata as OutputModelMetadata;
use crate::application::services::tenancy_resolver::TenancyResolver;
use crate::domain::services::{
    ModelMetadataService as ModelMetadataDomainService,
    ModelMetadataServiceError as ModelMetadataDomainServiceError
};
use crate::domain::entities;

use thiserror::Error;
use once_cell::sync::Lazy;
use log::error;

#[derive(Debug, Error)]
pub enum ModelMetadataServiceError {
    #[error("Repository error: {0}")]
    ArtifactRepoError(#[from] ArtifactRepositoryError),

    #[error("Repository error: {0}")]
    ModelMetadataRepoError(#[from] ModelMetadataRepositoryError),

    #[error("Artifact not found: {0}")]
    ArtifactNotFound(String),

    #[error("Metadata not found: {0}")]
    MetdataNotFound(String),

    #[error("{0}")]
    DomainServiceError(#[from] ModelMetadataDomainServiceError),

    #[error("Metadata already exists for Artifact '{0}'")]
    DuplicateMetadataError(String),

    #[error("Failed to convert metadata: '{0}'")]
    OutputMetadataConversionError(String),

    #[error("Internal Error: '{0}'")]
    InternalError(String),
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

    pub async fn get_by_author_and_name(&self, input: GetModelMetadataByAuthorAndNameInput) -> Result<ModelMetadataOutput, ModelMetadataServiceError> {
        let tenant_id = TenancyResolver::resolve_from_scope(&input.scope, &input.tenant_id);
        
        let find_metadata = || self.model_metadata_repo.find_by_author_and_name(
            &input.author,
            &input.name,
            &tenant_id,
        );

        let maybe_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY, None)
            .await?;

        let maybe_output = maybe_metadata
            .map(|m| self.build_output_model_from_entity(m))
            .transpose()?;

        Ok(ModelMetadataOutput { model: maybe_output })
    }

    pub async fn list_by_author(&self, input: ListModelMetadataByAuthorInput) -> Result<ModelMetadataListOutput, ModelMetadataServiceError> {
        let find_metadata = || self.model_metadata_repo.find_all_by_author(
            &input.author,
            &input.tenant_id,
        );

        let model_metadata = retry_async(find_metadata, &Self::REPO_RETRY_POLICY, None)
            .await?;

        let output_models = self.build_output_model_list_from_entities(model_metadata)?;

        Ok(ModelMetadataListOutput { models: output_models, count: None, cursor: None })
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

    pub async fn register_model_metadata(&self, input: RegisterModelMetadataInput, ctx: &RequestContext) -> Result<(), ModelMetadataServiceError> {
        let metadata_entity = entities::model_metadata::ModelMetadata::try_from((input.clone(), ctx))
            .map_err(|e| ModelMetadataServiceError::InternalError(e.to_string()))?;
        
        let modified_metadata = self.annotate_with_deployment_strategies(&metadata_entity);

        let upsert_metadata = || self.model_metadata_repo.upsert(&modified_metadata, &ctx);

        retry_async(upsert_metadata, &Self::REPO_RETRY_POLICY, None)
            .await?;

        return Ok(())
    }

    pub async fn discover_models(&self, input: SearchModelsInput, ctx: &RequestContext) -> Result<ModelMetadataListOutput, ModelMetadataServiceError> {
        // By default search in the user's tenant. Search the global tenant if
        // specified
        let mut tenant_ids = vec![ctx.actor_tenant_id().clone()];
        if input.options.include_global_models().unwrap_or(false) {
            tenant_ids.push(String::from(GLOBAL_TENANT))
        }
        
        let search = || self.model_metadata_repo.search(&input, &tenant_ids);

        // Find model metadata by search criteria
        let search_result = retry_async(search, &Self::REPO_RETRY_POLICY, None).await?;

        let annotated_models: Vec<_> = search_result.models
            .iter()
            .map(|m| self.annotate_with_deployment_strategies(m))
            .collect();

        // Build output models
        let output_models = match self.build_output_model_list_from_entities(annotated_models) {
            Ok(o) => o,
            Err(err) => {
                error!("Failed to convert annotated model entities into output models: {}", err.to_string());
                return Err(err)
            }
        };
        
        let output = ModelMetadataListOutput {
            models: output_models,
            count: search_result.count,
            cursor: search_result.cursor,  
        };
        
        Ok(output)
    }

    // Converts ModelMetadata entity into an application output Model Modetadata
    fn build_output_model_list_from_entities(&self, entities: Vec<ModelMetadata>) -> Result<Vec<OutputModelMetadata>, ModelMetadataServiceError> {
        let mut outputs: Vec<OutputModelMetadata> = Vec::with_capacity(entities.len());
        for entity in entities {
            match self.build_output_model_from_entity(entity) {
                Ok(o) => outputs.push(o),
                Err(err) => return Err(err)
            };
        }

        Ok(outputs)
    }

    // Converts ModelMetadata entities into the application output Model Modetadata
    fn build_output_model_from_entity(&self, entity: ModelMetadata) -> Result<OutputModelMetadata, ModelMetadataServiceError> {
        let strategies: Vec<Strategy> = self.client_strategy_sets
            .iter()
            .map(|s| s.strategies().clone())
            .flatten()
            .collect();

        match OutputModelMetadata::try_from((&entity, &strategies)) {
            Ok(o) => Ok(o),
            Err(err) => return Err(ModelMetadataServiceError::OutputMetadataConversionError(err.to_string()))
        }
    }

    fn annotate_with_deployment_strategies(&self, model: &ModelMetadata) -> ModelMetadata {
        let mut deployment_strategy_refs: Vec<DeploymentStrategyReference> = vec![];
        
        for set in self.client_strategy_sets.iter() {
            // Ignore if there is in error resolving strategies
            match resolve_viable_strategies(model, set.strategies()) {
                Ok(viable_strategies) => {
                    for viable_strat in viable_strategies {
                        let strat = viable_strat.into_inner();
                        deployment_strategy_refs.push(
                            DeploymentStrategyReference {
                                name: strat.name,
                                platform: strat.platform,
                            }
                        );
                    }
                }
                Err(err) => {
                    error!("Error resolving viable strategies for model annotation: {}", err.to_string())
                }
            } 
        }

        ModelMetadata {
            deployment_strategy_refs,
            ..model.clone()
        }
    }
}
