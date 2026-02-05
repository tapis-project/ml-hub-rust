use crate::application::errors::ApplicationError;
use crate::application::inputs::deployment::{DeployWithStrategyInput, FilterInput, FindForReconciliationInput};
use crate::application::outputs::deployment::DeployModelWithStrategyOutput;
use crate::application::ports::events::{Event,EventPayload, Kind, EventEnvelope, EventMetadata, EventPublisher, ModelDeploymentStateDriftDetectedPayload};
use crate::application::ports::deployment::ModelDeploymentRepository;
use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::application::retries::{
    retry_async, ExponentialBackoff, FixedBackoff, Jitter, Retry, RetryPolicy,
};
use crate::domain::entities::deployment::{
    DeploymentStrategyReference, DesiredState, ModelDeployment, ModelDeploymentProps, ModelReference, State
};
use crate::domain::entities::visibility::Visibility;
use log::error;
use once_cell::sync::Lazy;
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelDeploymentServiceError {
    #[error("Revision mismatch: Expected to find revision `{0}` but found `{1}`")]
    RevisionMismatch(String, String),

    #[error("State mismatch: Expected to find state `{0}` but found `{1}`")]
    StateMismatch(String, String),

    #[error("Desired state mismatch: Expected to find desired state `{0}` but found `{1}`")]
    DesiredStateMismatch(String, String),

    #[error("Repository error: {0}")]
    RepoError(#[from] ApplicationError),

    #[error("Model Deployment not found: {0}")]
    DeploymentNotFound(String),
}

pub struct ModelDeploymentService {
    model_deployment_repo: Arc<dyn ModelDeploymentRepository>,
    model_metadata_repo: Arc<dyn ModelMetadataRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl ModelDeploymentService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::FixedBackoff(FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        })
    });

    const EVENT_PUBLISHER_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| {
        RetryPolicy::ExponentialBackoff(ExponentialBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
            base: Some(2),
            max_delay: 500,
            jitter: Some(Jitter::Full),
        })
    });

    pub fn new(
        model_deployment_repo: Arc<dyn ModelDeploymentRepository>,
        model_metadata_repo: Arc<dyn ModelMetadataRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            model_deployment_repo,
            model_metadata_repo,
            event_publisher,
        }
    }

    /// Fetches a model deployment only if its current `revision`, `state`, and
    /// `desired_state` match the expected values provided in `input`.
    ///
    /// Returns an error if the deployment does not exist or if any of the expected
    /// values differ from those stored.
    pub async fn find_for_reconciliation(&self, input: FindForReconciliationInput) -> Result<ModelDeployment, ModelDeploymentServiceError> {
        let filter = FilterInput {
            deployment_id: Some(input.deployment_id),
            state: None,
            revision: None,
        };

        let find_model_deployment = || self.model_deployment_repo.find(&filter);

        // Fetch the deployment
        let maybe_model_deployment = retry_async(find_model_deployment, &Self::REPO_RETRY_POLICY).await?;

        let deployment = match maybe_model_deployment {
            Some(d) => d,
            None => return Err(ModelDeploymentServiceError::DeploymentNotFound(input.deployment_id.to_string()))
        };

        if deployment.revision() != &input.revision {
            return Err(ModelDeploymentServiceError::RevisionMismatch(input.revision.clone().to_string(), deployment.revision().to_string()));
        }

        if deployment.state != input.state {
            return Err(ModelDeploymentServiceError::StateMismatch(String::from(input.state), String::from(deployment.state.clone())));
        }

        if deployment.desired_state != input.desired_state {
            return Err(ModelDeploymentServiceError::DesiredStateMismatch(String::from(input.desired_state), String::from(deployment.desired_state.clone())));
        }

        Ok(deployment)
    }

    pub async fn deploy_model_with_strategy(
        &self,
        input: DeployWithStrategyInput,
    ) -> Result<DeployModelWithStrategyOutput, ApplicationError> {
        let find_model_metadata = || self.model_metadata_repo.get_by_name_and_author(&input.model_name, &input.model_author);
        
        // Fetch the metadata for the model of this deployment
        let maybe_model_metadata = retry_async(find_model_metadata, &Self::REPO_RETRY_POLICY).await?;

        let _ = match maybe_model_metadata {
            Some(mm) => mm,
            None => {
                return Err(ApplicationError::DomainError(
                    "Model referenced in model deployment does not exist".into(),
                ))
            }
        };

        let deployment = ModelDeployment::new(
            ModelDeploymentProps {
                id: Uuid::now_v7(),
                platform: input.platform.clone(),
                owner: input.owner.clone(),
                model: ModelReference {
                    name: input.model_name.clone(),
                    author: input.model_author.clone(),
                },
                state: State::NotDeployed,
                desired_state: DesiredState::Running,
                last_message: Some("Deployment submitted".into()),
                deployment_strategy: Some(DeploymentStrategyReference {
                    name: input.strategy_name.clone(),
                    client: input.platform.to_string(),
                }),
                visibility: Visibility::Private,
                deployment_interface: None,
                parallelism: None,
            }
        );

        // Save the deployment
        retry_async(|| self.model_deployment_repo.save(&deployment), &Self::REPO_RETRY_POLICY).await?;

        let payload = EventPayload::ModelDeploymentStateDriftDetectedPayload(
            ModelDeploymentStateDriftDetectedPayload {
                deployment_id: deployment.id,
                message: Some("Model deployment initiated with StateDriftDetected event".into()),
                deployment_revision: deployment.revision().clone(),
                desired_state: deployment.desired_state.clone(),
                actual_state: deployment.state.clone(),
            }
        );

        let event = Event::from_payload(&payload, None);
         
        // Closure for publishing model deployment
        let publish_state_drift_event = || self.event_publisher.publish(&event);

        // Publish the state drift event event
        match retry_async(publish_state_drift_event, &Self::EVENT_PUBLISHER_RETRY_POLICY,).await {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to publish state drift event for deployment {}: {}", &deployment.id, err.to_string());
            }
        };

        Ok(DeployModelWithStrategyOutput { deployment })
    }
}
