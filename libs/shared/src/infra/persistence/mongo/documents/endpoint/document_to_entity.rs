use crate::domain::entities::endpoint as entities;
use crate::infra::persistence::mongo::documents::endpoint as documents;
use crate::shared_kernel::identifiers::urn::Urn;

impl From<documents::Endpoint> for entities::Endpoint {
    fn from(value: documents::Endpoint) -> Self {
        entities::Endpoint::reconstitute(entities::ReconstituteEndpointProps {
            id: uuid::Uuid::from_bytes(value.id.bytes()),
            tenant_id: value.tenant_id,
            target_resource_urn: Urn::new(value.target_resource_urn),
            target_url: value.target_url,
            slug: value.slug,
        })
    }
}
