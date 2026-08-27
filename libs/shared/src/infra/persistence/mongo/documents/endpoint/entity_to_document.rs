use crate::domain::entities::endpoint as entities;
use crate::infra::persistence::mongo::documents::endpoint as documents;

impl From<&entities::Endpoint> for documents::Endpoint {
    fn from(value: &entities::Endpoint) -> Self {
        Self {
            _id: None,
            id: mongodb::bson::Uuid::from_bytes(*value.id().as_bytes()),
            tenant_id: value.tenant_id().to_owned(),
            target_resource_urn: value.target_resource_urn().as_str().to_owned(),
            target_url: value.target_url().to_owned(),
            slug: value.slug().to_owned(),
        }
    }
}
