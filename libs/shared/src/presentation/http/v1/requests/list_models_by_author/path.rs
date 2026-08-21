use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

#[derive(Deserialize, Serialize, Debug, IntoParams)]
pub struct ListModelsByAuthorPath {
    /// The author of the model
    pub author: String,
}