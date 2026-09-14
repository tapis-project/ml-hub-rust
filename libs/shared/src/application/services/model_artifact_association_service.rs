use std::sync::Arc;

use once_cell::sync::Lazy;
use retry_utils::{retry_async, FixedBackoff, Retry, RetryPolicy};
use thiserror::Error;

use crate::{
    application::{
        inputs::model::AssociateModelWithArtifactInput,
        ports::{
            artifacts::{ArtifactRepository, ArtifactRepositoryError},
            model::{ModelRepository, ModelRepositoryError},
        },
    },
    domain::services::{ModelService as ModelDomainService, ModelServiceError},
    shared_kernel::context::RequestContext,
};

#[derive(Debug, Error)]
pub enum ModelArtifactAssociationServiceError {
    #[error("Artifact not found")]
    ArtifactNotFound,

    #[error("Model not found")]
    ModelNotFound,

    #[error(transparent)]
    ArtifactRepository(#[from] ArtifactRepositoryError),

    #[error(transparent)]
    ModelRepository(#[from] ModelRepositoryError),

    #[error(transparent)]
    Domain(#[from] ModelServiceError),
}

pub struct ModelArtifactAssociationService {
    model_repository: Arc<dyn ModelRepository>,
    artifact_repository: Arc<dyn ArtifactRepository>,
}

impl ModelArtifactAssociationService {
    const RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    pub fn new(
        model_repository: Arc<dyn ModelRepository>,
        artifact_repository: Arc<dyn ArtifactRepository>,
    ) -> Self {
        Self {
            model_repository,
            artifact_repository,
        }
    }

    pub async fn associate(
        &self,
        ctx: &RequestContext,
        input: AssociateModelWithArtifactInput,
    ) -> Result<(), ModelArtifactAssociationServiceError> {
        let artifact = retry_async(
            || self.artifact_repository.get_by_id(&input.artifact_id),
            &Self::RETRY_POLICY,
            None,
        )
        .await?
        .ok_or(ModelArtifactAssociationServiceError::ArtifactNotFound)?;

        let mut model = retry_async(
            || {
                self.model_repository
                    .find_by_id(ctx.actor_tenant_id(), input.model_id)
            },
            &Self::RETRY_POLICY,
            None,
        )
        .await?
        .filter(|model| model.owner() == ctx.actor_principal_id())
        .ok_or(ModelArtifactAssociationServiceError::ModelNotFound)?;

        ModelDomainService::associate_model_with_artifact(&artifact, &model)?;

        model.associate_artifact(input.artifact_id);

        retry_async(
            || self.model_repository.update(&model),
            &Self::RETRY_POLICY,
            None,
        )
        .await?;

        Ok(())
    }
}
