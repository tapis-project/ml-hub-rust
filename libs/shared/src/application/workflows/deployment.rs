use crate::application::inputs::deployment::FilterInput;
use crate::application::ports::deployment::ModelDeploymentRepository;
use crate::application::ports::events::payloads::{
    ModelDeploymentDeletedPayload, ModelDeploymentStartedPayload, ModelDeploymentStoppedPayload,
};
use crate::application::ports::events::{Event, EventPublisher, Payload};
use crate::application::services::model_deployment_service::ModelDeploymentServiceError;
use crate::application::workflows::Workflow;
use crate::domain::entities::deployment::{DesiredState, ModelDeployment, ModelDeploymentId};
use crate::shared_kernel::value_objects::TimeStamp;
use log::error;
use once_cell::sync::Lazy;
use retry_utils::{retry_async, ExponentialBackoff, FixedBackoff, Jitter, Retry, RetryPolicy};
use std::sync::Arc;

impl UpdateDesiredStateWorkflow {
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
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            model_deployment_repo,
            event_publisher,
        }
    }
}

#[async_trait::async_trait]
impl Workflow<UpdateDesiredStateWorkflowInput, ModelDeployment, ModelDeploymentServiceError>
    for UpdateDesiredStateWorkflow
{
    async fn run(
        &self,
        input: UpdateDesiredStateWorkflowInput,
    ) -> Result<ModelDeployment, ModelDeploymentServiceError> {
        let filter = FilterInput {
            deployment_id: Some(input.deployment_id),
            state: None,
            revision: None,
        };

        let mut deployment = match retry_async(
            || self.model_deployment_repo.find(&filter),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?
        {
            Some(d) => d,
            None => {
                return Err(ModelDeploymentServiceError::DeploymentNotFound(
                    input.deployment_id.to_string(),
                ))
            }
        };

        if deployment.is_expired(&TimeStamp::now()) {
            return Err(ModelDeploymentServiceError::DeploymentExpired);
        }

        // If the user is request that the deployment move to a state that it is already in
        // we noop and return the same deployment without revision
        if deployment.desired_state == input.desired_state {
            return Ok(deployment);
        }

        let modified_deployment = deployment
            .revise()
            .transition_to_desired(input.desired_state, input.last_message)?
            .finish();

        // Update the deployment
        retry_async(
            || self.model_deployment_repo.update(&modified_deployment),
            &Self::REPO_RETRY_POLICY,
            None,
        )
        .await?;

        let event = match modified_deployment.desired_state {
            DesiredState::Running => {
                let payload = ModelDeploymentStartedPayload {
                    deployment_id: *modified_deployment.id(),
                    message: Some("Model deployment start requested".into()),
                    deployment_revision: modified_deployment.revision().clone(),
                };

                Event::from_payload(&Payload::ModelDeploymentStartedPayload(payload), None, None)
            }
            DesiredState::Stopped => {
                let payload = ModelDeploymentStoppedPayload {
                    deployment_id: *modified_deployment.id(),
                    message: Some("Model deployment stop requested".into()),
                    deployment_revision: modified_deployment.revision().clone(),
                };

                Event::from_payload(&Payload::ModelDeploymentStoppedPayload(payload), None, None)
            }
            DesiredState::NotDeployed => {
                let payload = ModelDeploymentDeletedPayload {
                    deployment_id: *modified_deployment.id(),
                    message: Some("Model deployment deletion requested".into()),
                    deployment_revision: modified_deployment.revision().clone(),
                };

                Event::from_payload(&Payload::ModelDeploymentDeletedPayload(payload), None, None)
            }
        };

        // Closure for publishing model deployment
        let publish_event = || self.event_publisher.publish(&event);

        // Publish the state drift event event
        match retry_async(publish_event, &Self::EVENT_PUBLISHER_RETRY_POLICY, None).await {
            Ok(_) => (),
            Err(err) => {
                error!(
                    "Failed to publish state drift event for deployment {}: {}",
                    deployment.id(),
                    err.to_string()
                );
            }
        };

        Ok(modified_deployment)
    }
}

pub struct UpdateDesiredStateWorkflow {
    model_deployment_repo: Arc<dyn ModelDeploymentRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

pub struct UpdateDesiredStateWorkflowInput {
    pub deployment_id: ModelDeploymentId,
    pub desired_state: DesiredState,
    pub last_message: Option<String>,
}
