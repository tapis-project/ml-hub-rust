// pub mod application_mappings;
// pub mod domain_mappings;

use platforms::Platform;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::presentation::http::v1::requests::common::Scope;

#[derive(Debug, Deserialize, Clone, IntoParams, ToSchema)]
pub struct DeployModelWithStrategyPathParams {
    /// The target platform for the Model Deployment
    pub platform: Platform,
    /// The name of the deployment strategy
    pub strategy_name: String,
}

#[derive(Debug, Deserialize, Clone, IntoParams, ToSchema)]
pub struct DeployModelWithStrategyBody {
    pub name: String,
    pub description: Option<String>,
    pub model_name: String,
    pub model_author: String,
    pub params: Value,
    #[serde(default = "default_scope")]
    #[param(value_type = Scope, inline, required)]
    /// Selector for global vs tenant-scoped models
    pub scope: Scope
}

fn default_scope () -> Scope { Scope::Tenant }

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