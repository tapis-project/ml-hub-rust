use serde::Serialize;
use strum_macros::Display;

#[derive(Clone, Debug, Display, Serialize, Eq, PartialEq)]
pub enum DeploymentModality {
    Batch,
    Service,
}