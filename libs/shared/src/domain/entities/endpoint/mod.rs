use nanoid::nanoid;
use thiserror::Error;
use url::Url;
use uuid::Uuid;

use crate::domain::entities::tenancy::TenantScopedResource;
use crate::impl_urn_generator;
use crate::shared_kernel::identifiers::{traits::UrnGenerator, urn::Urn};

#[derive(Clone, Debug)]
pub struct Endpoint {
    id: Uuid,
    tenant_id: String,
    owner: String,
    target_resource_urn: Urn,
    target_name: String,
    slug: String,
}

impl_urn_generator!(Endpoint, tenant_id, "endpoint", id);

impl Endpoint {
    pub fn new_from_resource(
        resource: &impl NetworkAddressableResource,
        target_name: String,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            tenant_id: resource.tenant_id(),
            owner: resource.owner().to_owned(),
            target_resource_urn: resource.urn(),
            target_name,
            slug: Self::generate_slug(),
        }
    }

    pub fn reconstitute(props: ReconstituteEndpointProps) -> Self {
        Self {
            id: props.id,
            tenant_id: props.tenant_id,
            owner: props.owner,
            target_resource_urn: props.target_resource_urn,
            target_name: props.target_name,
            slug: props.slug,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn tenant_id(&self) -> &str {
        &self.tenant_id
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn target_resource_urn(&self) -> &Urn {
        &self.target_resource_urn
    }

    pub fn target_name(&self) -> &str {
        &self.target_name
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
    pub owner: String,
    pub target_resource_urn: Urn,
    pub target_name: String,
    pub slug: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkProtocol {
    RestHttp,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedNetworkTarget {
    protocol: NetworkProtocol,
    base_url: Url,
}

impl ResolvedNetworkTarget {
    pub fn new(
        protocol: NetworkProtocol,
        base_url: impl AsRef<str>,
    ) -> Result<Self, NetworkTargetError> {
        let base_url = Url::parse(base_url.as_ref())
            .map_err(|_| NetworkTargetError::InvalidBaseUrl(base_url.as_ref().into()))?;

        if !matches!(base_url.scheme(), "http" | "https") || base_url.host_str().is_none() {
            return Err(NetworkTargetError::InvalidBaseUrl(base_url.to_string()));
        }

        Ok(Self { protocol, base_url })
    }

    pub fn protocol(&self) -> &NetworkProtocol {
        &self.protocol
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum NetworkTargetError {
    #[error("Invalid network target base URL: {0}")]
    InvalidBaseUrl(String),

    #[error("Network target protocol is not supported by the endpoint proxy")]
    UnsupportedProtocol,
}

pub trait NetworkAddressableResource: TenantScopedResource + UrnGenerator {
    fn owner(&self) -> &str;

    fn network_target_names(&self) -> Vec<String>;

    fn resolve_target_url(&self, target_name: &str) -> Option<&str>;

    fn resolve_network_target(
        &self,
        target_name: &str,
    ) -> Result<Option<ResolvedNetworkTarget>, NetworkTargetError> {
        self.resolve_target_url(target_name)
            .map(|base_url| ResolvedNetworkTarget::new(NetworkProtocol::RestHttp, base_url))
            .transpose()
    }
}

#[cfg(test)]
#[path = "endpoint.test.rs"]
mod endpoint_test;
