use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, Display, Deserialize, Serialize, Eq, PartialEq)]
pub enum DeploymentModality {
    Batch,
    Service,
}