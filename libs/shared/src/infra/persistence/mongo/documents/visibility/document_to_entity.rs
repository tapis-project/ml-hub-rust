use crate::infra::persistence::mongo::documents::visibility as documents;
use crate::shared_kernel::enums::Visibility as DomainVisibility;

impl From<documents::Visibility> for DomainVisibility {
    fn from(value: documents::Visibility) -> Self {
        match value {
            documents::Visibility::Private => DomainVisibility::Private,
            documents::Visibility::Public => DomainVisibility::Public,
        }
    }
}
