use crate::infra::persistence::mongo::documents::visibility as documents;
use crate::domain::entities::visibility as entities;

impl From<entities::Visibility> for documents::Visibility {
    fn from(value: entities::Visibility) -> Self {
        match value {
            entities::Visibility::Private => documents::Visibility::Private,
            entities::Visibility::Public => documents::Visibility::Public,
        }
    }
}