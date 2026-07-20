use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum DeploymentModality {
    Batch,
    Service,
}