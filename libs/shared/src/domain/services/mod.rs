use crate::domain::entities::artifact::Artifact;
use crate::domain::entities::artifact_ingestion::{ArtifactIngestion, ArtifactIngestionStatus};
use thiserror::Error;

use crate::domain::entities::artifact::ArtifactType;
use crate::domain::entities::deployment::{
    CreateFromOptionProps, DeploymentOptionSnapshot, ModelDeployment, ModelDeploymentError,
};
use crate::domain::entities::deployment_option::{
    deployment_parameters::{
        DeploymentParameterError, DeploymentParameters, ResolvedDeploymentParameter,
    },
    traits::ProvideDeploymentParameters,
    DeploymentOption, DeploymentTarget,
};
use crate::domain::entities::hpc_cluster::{BatchSchedulerQueue, HpcCluster};
use crate::domain::entities::model::Model;

pub mod endpoint_issuance_service;

#[derive(Debug, Error)]
pub enum ArtifactServiceError {
    #[error("{0}")]
    InvalidIngestionState(String),
}

pub struct ArtifactService {}

impl ArtifactService {
    /// Adds the final path of the ingestion to the artifact
    pub fn finish_artifact_ingestion<'a>(
        artifact: &'a mut Artifact,
        ingestion: &ArtifactIngestion,
    ) -> Result<&'a mut Artifact, ArtifactServiceError> {
        if ingestion.status != ArtifactIngestionStatus::Finished {
            return Err(ArtifactServiceError::InvalidIngestionState("Artifact ingestion must be Finished before setting the download url of an artifact".into()));
        }

        match &ingestion.artifact_path {
            Some(path) => {
                artifact.set_path(path.clone());
                Ok(artifact)
            },
            None => {
                Err(ArtifactServiceError::InvalidIngestionState("Cannot set the artifact's path because the ingestion is missing a value for field artifact_path".into()))
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum ModelServiceError {
    #[error("Cannot associate a model with an artifact that is not fully ingested")]
    ArtifactNotReady,

    #[error("Invalid artifact type. Artifact must be of type 'Model'")]
    InvalidArtifactType,
}

pub struct ModelService {}

impl ModelService {
    /// Verifies the the artifact exists and that the artifact has is fully
    /// ingested or uploaded
    pub fn associate_model_with_artifact(
        artifact: &Artifact,
        _model: &Model,
    ) -> Result<(), ModelServiceError> {
        if !artifact.is_fully_ingested() {
            return Err(ModelServiceError::ArtifactNotReady);
        }

        if artifact.artifact_type != ArtifactType::Model {
            return Err(ModelServiceError::InvalidArtifactType);
        }

        return Ok(());
    }
}

#[derive(Debug, Error)]
pub enum ModelDeploymentDomainServiceError {
    #[error("Cannot create model deployment for model {0}/{1}. Artifact for the selected model must be fully ingested")]
    ArtifactIngestionRequired(String, String),

    #[error(
        "Provided Model and Artifact have different ids: Model artifact id: {0}. Artifact id {1}"
    )]
    MismatchedArtifactIds(String, String),

    #[error("The artifact associated with this deployment's model is not a Model artifact")]
    InvalidArtifactType,

    #[error(transparent)]
    DomainError(#[from] ModelDeploymentError),

    #[error("Deployment option target does not match the supplied HPC cluster and queue")]
    InvalidDeploymentTarget,

    #[error(transparent)]
    DeploymentParameterError(#[from] DeploymentParameterError),
}

pub struct ModelDeploymentService;

pub struct DeployWithOptionResult {
    pub deployment: ModelDeployment,
    pub resolved_parameters: Vec<ResolvedDeploymentParameter>,
}

impl ModelDeploymentService {
    pub fn deployment_parameters(
        option: &DeploymentOption,
        cluster: &HpcCluster,
        queue: &BatchSchedulerQueue,
    ) -> Result<DeploymentParameters, ModelDeploymentDomainServiceError> {
        let target = match option.deployment_target() {
            DeploymentTarget::HpcClusterQueue(target) => target,
        };

        if target.hpc_cluster_id() != cluster.id()
            || target.batch_scheduler_queue_id() != queue.id()
            || queue.cluster_id() != cluster.id()
        {
            return Err(ModelDeploymentDomainServiceError::InvalidDeploymentTarget);
        }

        let parameters = option
            .serving_runtime()
            .provide_parameters()
            .into_iter()
            .chain(cluster.provide_parameters())
            .chain(queue.provide_parameters())
            .collect();

        Ok(DeploymentParameters::new(parameters)?)
    }

    pub fn deploy_with_option(
        props: CreateFromOptionProps,
        option: &DeploymentOption,
        snapshot: DeploymentOptionSnapshot,
        cluster: &HpcCluster,
        queue: &BatchSchedulerQueue,
        supplied_parameters: &[(String, String)],
    ) -> Result<DeployWithOptionResult, ModelDeploymentDomainServiceError> {
        let parameters = Self::deployment_parameters(option, cluster, queue)?;

        let resolved_parameters = parameters.resolve(supplied_parameters)?;

        let deployment = ModelDeployment::create_from_option(props, option, snapshot)?;

        Ok(DeployWithOptionResult {
            deployment,
            resolved_parameters,
        })
    }
}
