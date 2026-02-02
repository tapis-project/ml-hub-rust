use crate::application::errors::ApplicationError;
use crate::application::inputs::deployment::DeployWithStrategyInput;
use crate::application::outputs::deployment::DeployModelWithStrategyOutput;
use crate::application::ports::events::{Event, EventPublisher, ModelDeploymentStateDriftDetectedPayload};
use crate::application::ports::deployment::ModelDeploymentRepository;
use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::application::retries::{
    retry_async, ExponentialBackoff, FixedBackoff, Jitter, Retry, RetryPolicy,
};
use crate::domain::entities::deployment::{
    DeploymentStrategyReference, DesiredState, ModelDeployment, ModelDeploymentProps,
    ModelReference, State,
};
use crate::domain::entities::visibility::Visibility;
use crate::domain::entities::timestamp::TimeStamp;
use log::error;
use once_cell::sync::Lazy;
use std::sync::Arc;
use uuid::Uuid;

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

    pub async fn deploy_model_with_strategy(
        &self,
        input: DeployWithStrategyInput,
    ) -> Result<DeployModelWithStrategyOutput, ApplicationError> {
        // Fetch the metadata for the model of this deployment
        let maybe_model_metadata = retry_async(
            || {
                self.model_metadata_repo
                    .get_by_name_and_author(&input.model_name, &input.model_author)
            },
            &Self::REPO_RETRY_POLICY,
        )
        .await?;

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
        retry_async(
            || self.model_deployment_repo.save(&deployment),
            &Self::REPO_RETRY_POLICY,
        )
            .await?;

        let payload = ModelDeploymentStateDriftDetectedPayload {
            deployment_id: deployment.id,
            deployment_revision: deployment.revision().clone(),
            desired_state: deployment.desired_state.clone(),
            acutal_state: deployment.state.clone(),
            timestamp: TimeStamp::now(),
        };

        let event = Event::ModelDeploymentStateDriftDetected(payload.clone());

        // Closure for publishing model deployment
        let publish_state_drift = || self.event_publisher.publish(&event);

        // Publish the state drift event event
        match retry_async(publish_state_drift, &Self::EVENT_PUBLISHER_RETRY_POLICY,).await {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to publish statue drift event for deployment {}: {}", &deployment.id, err.to_string());
            }
        };

        Ok(DeployModelWithStrategyOutput { deployment })
    }
}
