use crate::shared_kernel::enums;
use crate::presentation::http::v1::requests::deployment::DeploymentModality;

impl From<DeploymentModality> for enums::DeploymentModality {
    fn from(value: DeploymentModality) -> Self {
        match value {
            DeploymentModality::Batch => enums::DeploymentModality::Batch,
            DeploymentModality::Service => enums::DeploymentModality::Service,
        }
    }
}