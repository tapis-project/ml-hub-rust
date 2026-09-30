use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub enum ServingRuntime {
    FlexServ,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub enum DeploymentModality {
    Batch,
    Service,
}
