use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AssociateModelBody {
    pub model_id: Uuid,
}
