use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::application::inputs::deployment_option::ListDeploymentOptionsInput;

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct ListExternalModelDeploymentOptionsPath {
    #[param(value_type = String, format = "uuid")]
    pub external_model_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListDeploymentOptionsQuery {
    /// Maximum number of deployment options to return. Values are capped at 100.
    #[param(minimum = 1, maximum = 100)]
    pub limit: Option<u16>,
    /// Opaque cursor returned by the previous page.
    pub cursor: Option<String>,
    /// Include the number of deployment options for the ExternalModel.
    pub include_count: Option<bool>,
}

impl From<ListDeploymentOptionsQuery> for ListDeploymentOptionsInput {
    fn from(value: ListDeploymentOptionsQuery) -> Self {
        Self::new(value.limit, value.cursor, value.include_count)
    }
}

#[cfg(test)]
#[path = "deployment_options.test.rs"]
mod deployment_options_test;
