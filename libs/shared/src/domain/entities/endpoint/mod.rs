use nanoid::nanoid;
use uuid::Uuid;

use crate::domain::entities::tenancy::TenantScopedResource;
use crate::impl_urn_generator;
use crate::shared_kernel::identifiers::{traits::UrnGenerator, urn::Urn};

#[derive(Clone, Debug)]
pub struct Endpoint {
    id: Uuid,
    tenant_id: String,
    target_resource_urn: Urn,
    target_url: String,
    slug: String,
}

impl_urn_generator!(Endpoint, tenant_id, "endpoint", id);

impl Endpoint {
    pub fn new_from_resource(
        resource: &impl NetworkAddressableResource,
        target_url: String,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            tenant_id: resource.tenant_id(),
            target_resource_urn: resource.urn(),
            target_url,
            slug: Self::generate_slug(),
        }
    }

    pub fn reconstitute(props: ReconstituteEndpointProps) -> Self {
        Self {
            id: props.id,
            tenant_id: props.tenant_id,
            target_resource_urn: props.target_resource_urn,
            target_url: props.target_url,
            slug: props.slug,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn target_resource_urn(&self) -> &Urn {
        &self.target_resource_urn
    }

    pub fn target_url(&self) -> &str {
        &self.target_url
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }

    fn generate_slug() -> String {
        let alphabet: [char; 36] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7',
            '8', '9',
        ];

        nanoid!(10, &alphabet)
    }
}

#[derive(Clone, Debug)]
pub struct ReconstituteEndpointProps {
    pub id: Uuid,
    pub tenant_id: String,
    pub target_resource_urn: Urn,
    pub target_url: String,
    pub slug: String,
}

pub trait NetworkAddressableResource: TenantScopedResource + UrnGenerator {
    fn target_urls(&self) -> Vec<String>;
}

#[cfg(test)]
#[path = "endpoint.test.rs"]
mod endpoint_test;
