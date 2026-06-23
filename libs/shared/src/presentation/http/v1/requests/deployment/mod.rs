// pub mod application_mappings;
// pub mod domain_mappings;

use platforms::Platform;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::presentation::http::v1::requests::common::Scope;

#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct DeployModelWithStrategyPathParams {
    pub platform: Platform,
    pub strategy_name: String,
}

#[derive(Debug, Deserialize, Clone, ToSchema)]
pub struct DeployModelWithStrategyBody {
    pub model_name: String,
    pub model_author: String,
    pub params: Value,
}

#[derive(Deserialize, Debug, Clone, ToSchema)]
pub struct DeployModelWithStrategyQueryParams {
    pub model_scope: Scope
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