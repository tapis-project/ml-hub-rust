// pub mod entity_to_dto;
// pub mod dto_to_input;

use crate::presentation::http::v1::requests::common::tasks::Task;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateModelMetadataBody {
    // General fields
    #[validate(length(min=1))]
    pub name: String,
    #[validate(length(min=1, max=255))]
    pub description: Option<String>,
    pub model_type: Option<String>,
    pub libraries: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub task_types: Option<Vec<Task>>,
    pub regulatory: Option<Vec<String>>,
    pub license: Option<String>,
}
