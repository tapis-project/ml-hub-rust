// pub mod application_mappings;
// pub mod domain_mappings;

use platforms::Platform;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DeployModelWithStrategyPathParams {
    pub platform: Platform
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct DeployModelWithStrategyBody {
    pub model_id: String,
    pub model_author: String,
    pub strategy_name: String,
    pub params: Value,
}