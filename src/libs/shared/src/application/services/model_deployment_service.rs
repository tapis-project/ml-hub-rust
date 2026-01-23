use uuid::Uuid;
use crate::application::inputs::deployment::DeployWithStrategyInput;
use crate::application::outputs::deployment::DeployModelWithStrategyOutput;
use crate::domain::entities::deployment::{ModelDeployment, ModelDeploymentStatus, ModelReference, DeploymentStrategyReference};
use crate::domain::entities::timestamp::TimeStamp;
use crate::domain::entities::visibility::Visibility;

pub struct  ModelDeploymentService {

}

impl ModelDeploymentService {
    pub async fn deploy_model_with_strategy(&self, input: DeployWithStrategyInput) -> DeployModelWithStrategyOutput {
        let now = TimeStamp::now();
        let deployment = DeployModelWithStrategyOutput {
            deployment: ModelDeployment {
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
            }
        };

        deployment
    }
}