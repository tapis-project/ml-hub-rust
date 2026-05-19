use crate::infra::persistence::mongo::documents::visibility as documents;
use crate::domain::entities::visibility as entities;

impl From<documents::Visibility> for entities::Visibility {
    fn from(value: documents::Visibility) -> Self {
        match value {
            documents::Visibility::Private => entities::Visibility::Private,
            documents::Visibility::Public => entities::Visibility::Public,
        }
    }
}