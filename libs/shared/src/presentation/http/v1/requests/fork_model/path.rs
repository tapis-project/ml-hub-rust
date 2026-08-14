use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

#[derive(Clone, Debug, Serialize, Deserialize, IntoParams)]
pub struct ForkModelPath {
    pub author: String,
    pub name: String,
}