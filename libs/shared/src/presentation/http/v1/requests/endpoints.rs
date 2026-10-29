use serde::Deserialize;
use utoipa::IntoParams;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListEndpointsQuery {
    #[param(minimum = 1, maximum = 100)]
    pub limit: Option<u16>,
    pub cursor: Option<String>,
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetEndpointPath {
    #[param(value_type = String, format = "uuid")]
    pub endpoint_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct ListResourceEndpointsPath {
    pub resource_urn: String,
}
