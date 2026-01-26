use std::sync::Arc;
use crate::application::errors::ApplicationError;
use crate::application::inputs::deployment::DeployWithStrategyInput;
use crate::application::outputs::deployment::DeployModelWithStrategyOutput;
use crate::application::ports::deployment::ModelDeploymentRepository;
use crate::domain::entities::deployment::{ModelDeployment, ModelDeploymentStatus, ModelReference, DeploymentStrategyReference};
use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::visibility::Visibility;
use crate::retry::{retry_async, RetryPolicy, FixedBackoff, Retry};
use once_cell::sync::Lazy;
use uuid::Uuid;
use log::error;

pub struct  ModelDeploymentService {
    model_deployment_repo: Arc<dyn ModelDeploymentRepository>
}

impl ModelDeploymentService {
    const REPO_RETRY_POLICY: Lazy<RetryPolicy> = Lazy::new(|| RetryPolicy::FixedBackoff(
        FixedBackoff {
            retries: Retry::NTimes(3),
            delay: 50,
        }
    ));

    pub fn new(model_deployment_repo: Arc<dyn ModelDeploymentRepository>) -> Self {
        Self {
            model_deployment_repo
        }
    }

    pub async fn deploy_model_with_strategy(&self, input: DeployWithStrategyInput) -> Result<DeployModelWithStrategyOutput, ApplicationError> {
        let now = TimeStamp::now();
        
        let mut deployment = ModelDeployment {
            id: Uuid::now_v7(),
            owner: input.owner,
            model: ModelReference { name: input.model_name, author: input.model_author },
            status: ModelDeploymentStatus::Submitted,
            last_message: Some("Deployment submitted".into()),
            deployment_strategy: Some(DeploymentStrategyReference {
                name: input.strategy_name,
                client: input.platform.to_string(),
            }),
            visibility: Visibility::Private,
            created_at: now.clone(),
            last_modified: now.clone(),
            deployment_interface: None,
            parallelism: None
        };

        // Save the deployment 
        retry_async(|| self.model_deployment_repo.save(&deployment), &Self::REPO_RETRY_POLICY)
            .await?;
            // .map_err(|err| error!("Failed to save deployment: {}", err.to_string()));

        deployment.change_status(ModelDeploymentStatus::Queued)
            .map_err(|err| {
                let message = format!("Error when changing the status of a ModelDeployment: {}", err.to_string());
                error!("{}", message);
                ApplicationError::DomainError(message)
            })?;

        // Save the deployment 
        retry_async(|| self.model_deployment_repo.update_status(&deployment), &Self::REPO_RETRY_POLICY)
            .await?;
            // .map_err(|err| error!("Failed to update status of model deployment: {}", err.to_string()));
        
        Ok(DeployModelWithStrategyOutput { deployment })    
    }
}