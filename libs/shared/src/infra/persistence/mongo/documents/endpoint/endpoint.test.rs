use crate::domain::entities::endpoint::{Endpoint, ReconstituteEndpointProps};
use crate::infra::_common::mongo::Index;
use crate::infra::persistence::mongo::database::ENDPOINT_COLLECTION;
use crate::infra::persistence::mongo::documents::endpoint::indexes::{
    EndpointIdIndexUnique, EndpointOwnerPageIndex, EndpointResourcePageIndex,
    EndpointTargetIndexUnique, EndpointTenantSlugIndexUnique,
};
use crate::shared_kernel::identifiers::urn::Urn;
use mongodb::bson::doc;

use super::Endpoint as EndpointDocument;

#[test]
fn endpoint_document_round_trip_preserves_fields() {
    let id = uuid::Uuid::now_v7();
    let endpoint = Endpoint::reconstitute(ReconstituteEndpointProps {
        id,
        tenant_id: "tenant-a".into(),
        owner: "owner-a".into(),
        target_resource_urn: Urn::new("urn:mlhub:v1:tenant-a:agent:agent-a".into()),
        target_name: "rest".into(),
        slug: "abc123def4".into(),
    });

    let document = EndpointDocument::from(&endpoint);
    let reconstituted = Endpoint::from(document);

    assert_eq!(reconstituted.id(), &id);
    assert_eq!(reconstituted.tenant_id(), "tenant-a");
    assert_eq!(reconstituted.owner(), "owner-a");
    assert_eq!(
        reconstituted.target_resource_urn(),
        endpoint.target_resource_urn()
    );
    assert_eq!(reconstituted.target_name(), "rest");
    assert_eq!(reconstituted.slug(), "abc123def4");
}

#[test]
fn endpoint_indexes_enforce_identity_and_route_uniqueness() {
    let id_index = EndpointIdIndexUnique::index();
    let tenant_slug_index = EndpointTenantSlugIndexUnique::index();
    let target_index = EndpointTargetIndexUnique::index();
    let owner_page_index = EndpointOwnerPageIndex::index();
    let resource_page_index = EndpointResourcePageIndex::index();

    assert_eq!(
        EndpointIdIndexUnique::collection_name(),
        ENDPOINT_COLLECTION
    );
    assert_eq!(id_index.keys, doc! { "id": 1 });
    assert_eq!(tenant_slug_index.keys, doc! { "tenant_id": 1, "slug": 1 });
    assert_eq!(
        target_index.keys,
        doc! { "tenant_id": 1, "target_resource_urn": 1, "target_name": 1 }
    );
    assert_eq!(
        owner_page_index.keys,
        doc! { "tenant_id": 1, "owner": 1, "_id": 1 }
    );
    assert_eq!(
        resource_page_index.keys,
        doc! {
            "tenant_id": 1,
            "owner": 1,
            "target_resource_urn": 1,
            "_id": 1,
        }
    );
    assert_eq!(
        id_index.options.and_then(|options| options.unique),
        Some(true)
    );
    assert_eq!(
        tenant_slug_index.options.and_then(|options| options.unique),
        Some(true)
    );
    assert_eq!(
        target_index.options.and_then(|options| options.unique),
        Some(true)
    );
}
