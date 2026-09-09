use std::sync::Arc;

use crate::application::inputs::discover_models::SearchModelsInput;
use crate::application::inputs::model::{
    AssociateModel, GetModelByAuthorAndNameInput, ListModelsByAuthorInput, RegisterModelInput,
    UpdateModelArtifactId,
};
use crate::application::outputs::model::Model as OutputModel;
use crate::application::outputs::model::{ModelListOutput, ModelOutput};
use crate::application::ports::artifacts::{ArtifactRepository, ArtifactRepositoryError};
use crate::application::ports::model::{ModelRepository, ModelRepositoryError};
use crate::application::services::tenancy_resolver::TenancyResolver;
use crate::domain::entities;
use crate::domain::entities::deployment_strategy::client_strategy_set::ClientStrategySet;
use crate::domain::entities::deployment_strategy::strategy::Strategy;
use crate::domain::entities::model::{DeploymentStrategyReference, Model};
use crate::domain::services::deployment_strategy::resolve_viable_strategies;
use crate::domain::services::{
    ModelService as ModelDomainService, ModelServiceError as ModelDomainServiceError,
};
use crate::shared_kernel::constants::GLOBAL_TENANT;
use crate::shared_kernel::context::RequestContext;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};

use log::error;
use once_cell::sync::Lazy;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelServiceError {
    #[error("Repository error: {0}")]
    ArtifactRepoError(#[from] ArtifactRepositoryError),

    #[error("Repository error: {0}")]
    ModelRepoError(#[from] ModelRepositoryError),

    #[error("Artifact not found: {0}")]
    ArtifactNotFound(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("{0}")]
    DomainServiceError(#[from] ModelDomainServiceError),

    #[error("Model already exists for Artifact '{0}'")]
    DuplicateModelError(String),

    #[error("Failed to convert model: '{0}'")]
    OutputModelConversionError(String),

    #[error("Internal Error: '{0}'")]
    InternalError(String),
}

pub struct ModelService {
    model_repo: Arc<dyn ModelRepository>,
    artifact_repo: Arc<dyn ArtifactRepository>,
    client_strategy_sets: Arc<Vec<ClientStrategySet>>,
}

impl ModelService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        model_repo: Arc<dyn ModelRepository>,
        artifact_repo: Arc<dyn ArtifactRepository>,
        client_strategy_sets: Arc<Vec<ClientStrategySet>>,
    ) -> Self {
        Self {
            model_repo,
            artifact_repo,
            client_strategy_sets,
        }
    }

    pub async fn get_by_author_and_name(
        &self,
        input: GetModelByAuthorAndNameInput,
    ) -> Result<ModelOutput, ModelServiceError> {
        let tenant_id = TenancyResolver::resolve_from_scope(&input.scope, &input.tenant_id);

        let find_model = || {
            self.model_repo
                .find_by_author_and_name(&input.author, &input.name, &tenant_id)
        };

        let maybe_model = retry_async(find_model, &Self::REPO_RETRY_POLICY, None).await?;

        let maybe_output = maybe_model
            .map(|m| self.build_output_model_from_entity(m))
            .transpose()?;

        Ok(ModelOutput {
            model: maybe_output,
        })
    }

    pub async fn list_by_author(
        &self,
        input: ListModelsByAuthorInput,
    ) -> Result<ModelListOutput, ModelServiceError> {
        let find_model = || {
            self.model_repo
                .find_all_by_author(&input.author, &input.tenant_id)
        };

        let model = retry_async(find_model, &Self::REPO_RETRY_POLICY, None).await?;

        let output_models = self.build_output_model_list_from_entities(model)?;

        Ok(ModelListOutput {
            models: output_models,
            count: None,
            cursor: None,
        })
    }

    pub async fn associate_model_with_artifact(
        &self,
        input: AssociateModel,
    ) -> Result<(), ModelServiceError> {
        // Get the artifact_id from the input
        let artifact_id = input.artifact_id.clone();

        let find_artifact = || self.artifact_repo.get_by_id(&artifact_id);

        // Find the artifact by id
        let artifact = retry_async(find_artifact, &Self::REPO_RETRY_POLICY, None)
            .await?
            .ok_or_else(|| {
                ModelServiceError::ArtifactNotFound(format!(
                    "Artifact with id {} does not exist",
                    &artifact_id
                ))
            })?;

        // Ensure no model already exists for this artifact
        let find_model = || self.model_repo.find_by_artifact_id(&artifact_id);

        let maybe_model = retry_async(find_model, &Self::REPO_RETRY_POLICY, None).await?;

        let model = match maybe_model {
            Some(m) => m,
            None => {
                return Err(ModelServiceError::ModelNotFound(format!(
                    "No model found with author {} and name {}",
                    input.author, input.name
                )))
            }
        };

        // Determine if we are allowed to create the model for this artifact
        ModelDomainService::associate_model_with_artifact(&artifact, model)?;

        let update_input = UpdateModelArtifactId::from(input);

        let update_model = || self.model_repo.update_artifact_id(&update_input);

        retry_async(update_model, &Self::REPO_RETRY_POLICY, None).await?;

        return Ok(());
    }

    pub async fn register_model(
        &self,
        input: RegisterModelInput,
        ctx: &RequestContext,
    ) -> Result<(), ModelServiceError> {
        let model_entity = entities::model::Model::try_from((input.clone(), ctx))
            .map_err(|e| ModelServiceError::InternalError(e.to_string()))?;

        let modified_model = self.annotate_with_deployment_strategies(&model_entity);

        let upsert_model = || self.model_repo.upsert(&modified_model, &ctx);

        retry_async(upsert_model, &Self::REPO_RETRY_POLICY, None).await?;

        return Ok(());
    }

    pub async fn discover_models(
        &self,
        input: SearchModelsInput,
        ctx: &RequestContext,
    ) -> Result<ModelListOutput, ModelServiceError> {
        // By default search in the user's tenant. Search the global tenant if
        // specified
        let mut tenant_ids = vec![ctx.actor_tenant_id().clone()];
        if input.options.include_global_models().unwrap_or(false) {
            tenant_ids.push(String::from(GLOBAL_TENANT))
        }

        let search = || self.model_repo.search(&input, &tenant_ids);

        // Find model by search criteria
        let search_result = retry_async(search, &Self::REPO_RETRY_POLICY, None).await?;

        let annotated_models: Vec<_> = search_result
            .models
            .iter()
            .map(|m| self.annotate_with_deployment_strategies(m))
            .collect();

        // Build output models
        let output_models = match self.build_output_model_list_from_entities(annotated_models) {
            Ok(o) => o,
            Err(err) => {
                error!(
                    "Failed to convert annotated model entities into output models: {}",
                    err.to_string()
                );
                return Err(err);
            }
        };

        let output = ModelListOutput {
            models: output_models,
            count: search_result.count,
            cursor: search_result.cursor,
        };

        Ok(output)
    }

    // Converts Model entities into application output models.
    fn build_output_model_list_from_entities(
        &self,
        entities: Vec<Model>,
    ) -> Result<Vec<OutputModel>, ModelServiceError> {
        let mut outputs: Vec<OutputModel> = Vec::with_capacity(entities.len());
        for entity in entities {
            match self.build_output_model_from_entity(entity) {
                Ok(o) => outputs.push(o),
                Err(err) => return Err(err),
            };
        }

        Ok(outputs)
    }

    // Converts a Model entity into an application output model.
    fn build_output_model_from_entity(
        &self,
        entity: Model,
    ) -> Result<OutputModel, ModelServiceError> {
        let strategies: Vec<Strategy> = self
            .client_strategy_sets
            .iter()
            .map(|s| s.strategies().clone())
            .flatten()
            .collect();

        match OutputModel::try_from((&entity, &strategies)) {
            Ok(o) => Ok(o),
            Err(err) => {
                return Err(ModelServiceError::OutputModelConversionError(
                    err.to_string(),
                ))
            }
        }
    }

    fn annotate_with_deployment_strategies(&self, model: &Model) -> Model {
        let mut deployment_strategy_refs: Vec<DeploymentStrategyReference> = vec![];

        for set in self.client_strategy_sets.iter() {
            // Ignore if there is in error resolving strategies
            match resolve_viable_strategies(model, set.strategies()) {
                Ok(viable_strategies) => {
                    for viable_strat in viable_strategies {
                        let strat = viable_strat.into_inner();
                        deployment_strategy_refs.push(DeploymentStrategyReference {
                            name: strat.name,
                            platform: strat.platform,
                        });
                    }
                }
                Err(err) => {
                    error!(
                        "Error resolving viable strategies for model annotation: {}",
                        err.to_string()
                    )
                }
            }
        }

        Model {
            deployment_strategy_refs,
            ..model.clone()
        }
    }
}
