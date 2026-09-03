use crate::domain::entities::endpoint::{Endpoint, ReconstituteEndpointProps};
use crate::shared_kernel::identifiers::urn::Urn;

use super::Endpoint as EndpointDocument;

#[test]
fn endpoint_document_round_trip_preserves_fields() {
    let id = uuid::Uuid::now_v7();
    let endpoint = Endpoint::reconstitute(ReconstituteEndpointProps {
        id,
        tenant_id: "tenant-a".into(),
        target_resource_urn: Urn::new("urn:mlhub:v1:tenant-a:agent:agent-a".into()),
        target_name: "rest".into(),
        slug: "abc123def4".into(),
    });

    let document = EndpointDocument::from(&endpoint);
    let reconstituted = Endpoint::from(document);

    assert_eq!(reconstituted.id(), &id);
    assert_eq!(reconstituted.tenant_id(), "tenant-a");
    assert_eq!(
        reconstituted.target_resource_urn(),
        endpoint.target_resource_urn()
    );
    assert_eq!(reconstituted.target_name(), "rest");
    assert_eq!(reconstituted.slug(), "abc123def4");
}
