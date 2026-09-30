use crate::infra::persistence::mongo::documents::visibility as documents;
use crate::shared_kernel::enums::Visibility as DomainVisibility;

impl From<DomainVisibility> for documents::Visibility {
    fn from(value: DomainVisibility) -> Self {
        match value {
            DomainVisibility::Private => documents::Visibility::Private,
            DomainVisibility::Public => documents::Visibility::Public,
        }
    }
}
