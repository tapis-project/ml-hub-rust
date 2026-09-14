use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::application::inputs::model::ListModelsInput;

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetModelPath {
    #[param(value_type = String, format = "uuid")]
    pub model_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetExternalModelPath {
    #[param(value_type = String, format = "uuid")]
    pub external_model_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListModelsQueryParams {
    #[serde(default)]
    #[param(inline)]
    pub scope: ModelScope,
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, ToSchema)]
pub enum ModelScope {
    #[default]
    Owned,
    Shared,
}

impl From<&ListModelsQueryParams> for ListModelsInput {
    fn from(value: &ListModelsQueryParams) -> Self {
        Self::new(value.limit, value.cursor.clone(), value.include_count)
    }
}
