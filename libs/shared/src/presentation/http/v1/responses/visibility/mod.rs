use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::entities;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum Visibility {
    Public,
    Private,
}

impl From<entities::visibility::Visibility> for Visibility {
    fn from(value: entities::visibility::Visibility) -> Self {
        match value {
            entities::visibility::Visibility::Private => Visibility::Private,
            entities::visibility::Visibility::Public => Visibility::Public,
        }
    }
}