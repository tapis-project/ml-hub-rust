// pub mod application_mappings;
// pub mod domain_mappings;

use platforms::Platform;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DeployModelWithStrategyPathParams {
    pub platform: Platform,
    pub strategy_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DeployModelWithStrategyBody {
    pub model_name: String,
    pub model_author: String,
    pub params: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StartModelDeploymentPathParams {
    pub deployment_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct StopModelDeploymentPathParams {
    pub deployment_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UndeployModelDeploymentPathParams {
    pub deployment_id: Uuid
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct GetModelDeploymentPathParams {
    pub deployment_id: Uuid
}
