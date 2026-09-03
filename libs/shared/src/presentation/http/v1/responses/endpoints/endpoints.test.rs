use crate::domain::entities::agent::test_fixtures::AgentBuilder;
use crate::domain::entities::endpoint::{Endpoint as DomainEndpoint, ReconstituteEndpointProps};
use crate::presentation::http::v1::responses::endpoints::Endpoint;
use crate::shared_kernel::identifiers::traits::UrnGenerator;
use crate::shared_kernel::identifiers::urn::Urn;

#[test]
fn endpoint_response_derives_the_target_base_url_from_the_agent(
) -> Result<(), Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new().build_registered()?;
    let endpoint = DomainEndpoint::reconstitute(ReconstituteEndpointProps {
        id: uuid::Uuid::now_v7(),
        tenant_id: "test-tenant".into(),
        target_resource_urn: agent.urn(),
        target_name: "default".into(),
        slug: "abc123def4".into(),
    });

    let response = Endpoint::try_from((endpoint, &agent))?;

    assert_eq!(response.target_name, "default");
    assert_eq!(response.target_base_url, "https://example.test");

    Ok(())
}

#[test]
fn endpoint_response_rejects_an_unresolvable_target() -> Result<(), Box<dyn std::error::Error>> {
    let agent = AgentBuilder::new().build_registered()?;
    let endpoint = DomainEndpoint::reconstitute(ReconstituteEndpointProps {
        id: uuid::Uuid::now_v7(),
        tenant_id: "test-tenant".into(),
        target_resource_urn: Urn::new("urn:mlhub:v1:test-tenant:agent:agent-a".into()),
        target_name: "missing".into(),
        slug: "abc123def4".into(),
    });

    let result = Endpoint::try_from((endpoint, &agent));

    assert!(matches!(
        result,
        Err(crate::presentation::http::v1::responses::endpoints::EndpointResponseError::UnresolvableTarget(name)) if name == "missing"
    ));

    Ok(())
}
