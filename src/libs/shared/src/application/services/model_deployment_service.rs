use std::sync::Arc;
use crate::application::errors::ApplicationError;
use crate::application::inputs::deployment::DeployWithStrategyInput;
use crate::application::outputs::deployment::DeployModelWithStrategyOutput;
use crate::application::ports::deployment::ModelDeploymentRepository;
use crate::application::ports::commands::{DeployModelWithStrategyCommandPayload, Command, CommandPublisher};

use crate::application::ports::model_metadata::ModelMetadataRepository;
use crate::domain::entities::deployment::{ModelDeployment, ModelDeploymentStatus, ModelReference, DeploymentStrategyReference};
use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::visibility::Visibility;
use crate::retry::{retry_async, RetryPolicy, FixedBackoff, Retry, Jitter, ExponentialBackoff};
use once_cell::sync::Lazy;
use uuid::Uuid;
use log::error;

pub struct  ModelDeploymentService {
    model_deployment_repo: Arc<dyn ModelDeploymentRepository>,
    model_metadata_repo: Arc<dyn ModelMetadataRepository>,
    command_publisher: Arc<dyn CommandPublisher>,
}

impl ModelDeploymentService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::FixedBackoff(
        FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        }
    ));

    const COMMAND_PUBLISHER_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::ExponentialBackoff(
        ExponentialBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
            base: Some(2),
            max_delay: 500,
            jitter: Some(Jitter::Full)
        }
    ));

    pub fn new(
        model_deployment_repo: Arc<dyn ModelDeploymentRepository>,
        model_metadata_repo: Arc<dyn ModelMetadataRepository>,
        command_publisher: Arc<dyn CommandPublisher>,
    ) -> Self {
        Self {
            model_deployment_repo,
            model_metadata_repo,
            command_publisher,
        }
    }

    pub async fn deploy_model_with_strategy(&self, input: DeployWithStrategyInput) -> Result<DeployModelWithStrategyOutput, ApplicationError> {
        // Fetch the metadata for the model of this deployment
        let maybe_model_metadata = retry_async(|| self.model_metadata_repo.get_by_name_and_author(&input.model_name, &input.model_author), &Self::REPO_RETRY_POLICY)
            .await?;

        let _ = match maybe_model_metadata {
            Some(mm) => mm,
            None => return Err(ApplicationError::DomainError("Model referenced in model deployment does not exist".into()))
        };

        // Create the model deployment
        let now = TimeStamp::now();
        
        let mut deployment = ModelDeployment {
            id: Uuid::now_v7(),
            owner: input.owner.clone(),
            model: ModelReference { name: input.model_name.clone(), author: input.model_author.clone()},
            status: ModelDeploymentStatus::Submitted,
            last_message: Some("Deployment submitted".into()),
            deployment_strategy: Some(DeploymentStrategyReference {
                name: input.strategy_name.clone(),
                client: input.platform.to_string(),
            }),
            visibility: Visibility::Private,
            created_at: now.clone(),
            last_modified: now.clone(),
            deployment_interface: None,
            parallelism: None
        };

        // Save the deployment 
        retry_async(|| self.model_deployment_repo.save(&deployment), &Self::REPO_RETRY_POLICY).await?;

        let payload = DeployModelWithStrategyCommandPayload {
            model_name: deployment.model.name.clone(),
            model_author: deployment.model.author.clone(),
            owner: deployment.owner.clone(),
            params: input.params.clone(),
            strategy_name: input.strategy_name.clone(),
            platform: input.platform,
        };

        let command = Command::DeployModelWithStrategyCommand(payload.clone());

        // Closure for publishing model deployment
        let publish_model_deployment = || self.command_publisher.publish(
            &command
        );
        
        // Publish the deployment command
        deployment = match retry_async(publish_model_deployment, &Self::COMMAND_PUBLISHER_RETRY_POLICY).await {
            Ok(_) => {
                // Publishing command failed, update the deployment to Failed
                deployment.change_status(ModelDeploymentStatus::Queued, Some("Successfully queued".into()))
                    .map_err(|err| {
                        error!("Error when changing the status of a ModelDeployment: {}", err.to_string());
                        ApplicationError::DomainError(err.to_string())
                    })?;

                deployment
            },
            Err(err) => {
                error!("Failed to publish model deployment command: {}", err.to_string());

                // Publishing of the command failed, mark the deployment as failed
                deployment
                    .change_status(ModelDeploymentStatus::Failed, Some("Internal error: Failed to publish".into()))
                    .map_err(|err| {
                        error!("Error when changing the status of a ModelDeployment: {}", err.to_string());
                        ApplicationError::DomainError(err.to_string())
                    })?;

                deployment
            }
        };

        // Update the status of the deployment
        retry_async(|| self.model_deployment_repo.update_status(&deployment), &Self::REPO_RETRY_POLICY).await?;
        
        Ok(DeployModelWithStrategyOutput { deployment })    
    }
}