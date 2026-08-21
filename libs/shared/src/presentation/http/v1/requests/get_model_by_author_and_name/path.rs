use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

#[derive(Deserialize, Serialize, Debug, IntoParams)]
pub struct GetModelByAuthorAndNamePath {
    /// The name of the model
    pub name: String,
    /// The author of the model
    pub author: String,
}