use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::shared_kernel::enums::Visibility as DomainVisibility;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum Visibility {
    Public,
    Private,
}

impl From<DomainVisibility> for Visibility {
    fn from(value: DomainVisibility) -> Self {
        match value {
            DomainVisibility::Private => Visibility::Private,
            DomainVisibility::Public => Visibility::Public,
        }
    }
}
