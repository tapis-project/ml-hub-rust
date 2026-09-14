use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::{
    application::inputs::model::CreateModelInput,
    presentation::http::v1::requests::datasets::Visibility,
    shared_kernel::{enums::Visibility as DomainVisibility, identifiers::ExternalModelId},
};

#[derive(Deserialize, Serialize, Validate, Debug, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateModelBody {
    #[validate(length(min = 1))]
    pub name: String,
    pub description: Option<String>,
    pub external_model_id: Uuid,
    #[serde(default)]
    pub visibility: Visibility,
}

impl From<CreateModelBody> for CreateModelInput {
    fn from(value: CreateModelBody) -> Self {
        Self {
            name: value.name,
            description: value.description,
            external_model_id: ExternalModelId::reconstitute(value.external_model_id),
            visibility: match value.visibility {
                Visibility::Public => DomainVisibility::Public,
                Visibility::Private => DomainVisibility::Private,
            },
        }
    }
}
