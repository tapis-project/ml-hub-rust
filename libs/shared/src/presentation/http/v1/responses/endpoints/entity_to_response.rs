use crate::domain::entities::endpoint::NetworkAddressableResource;
use crate::domain::entities::{agent, endpoint};
use crate::presentation::http::v1::responses::endpoints as responses;

impl TryFrom<(endpoint::Endpoint, &agent::Agent)> for responses::Endpoint {
    type Error = responses::EndpointResponseError;

    fn try_from(value: (endpoint::Endpoint, &agent::Agent)) -> Result<Self, Self::Error> {
        let target_base_url = value
            .1
            .resolve_target_url(value.0.target_name())
            .ok_or_else(|| {
                responses::EndpointResponseError::UnresolvableTarget(
                    value.0.target_name().to_owned(),
                )
            })?;

        Ok(Self {
            id: *value.0.id(),
            tenant_id: value.0.tenant_id().to_owned(),
            target_resource_urn: value.0.target_resource_urn().as_str().to_owned(),
            target_name: value.0.target_name().to_owned(),
            slug: value.0.slug().to_owned(),
            target_base_url: target_base_url.to_owned(),
        })
    }
}
