use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::infra::_common::mongo::Index;
use crate::infra::persistence::mongo::database::ENDPOINT_COLLECTION;
use crate::infra::persistence::mongo::documents::endpoint::Endpoint;

macro_rules! endpoint_index {
    ($name:ident, $index_name:literal, $keys:expr) => {
        pub struct $name;

        impl Index for $name {
            type Collection = Endpoint;
            const INDEX_NAME: &'static str = $index_name;

            fn index() -> IndexModel {
                IndexModel::builder()
                    .keys($keys)
                    .options(
                        IndexOptions::builder()
                            .name(Self::INDEX_NAME.to_string())
                            .unique(true)
                            .build(),
                    )
                    .build()
            }

            fn collection_name() -> &'static str {
                ENDPOINT_COLLECTION
            }
        }
    };
}

endpoint_index!(
    EndpointIdIndexUnique,
    "endpoint_id_index_unique",
    doc! { "id": 1 }
);
endpoint_index!(
    EndpointTenantSlugIndexUnique,
    "endpoint_tenant_slug_index_unique",
    doc! { "tenant_id": 1, "slug": 1 }
);
endpoint_index!(
    EndpointTargetIndexUnique,
    "endpoint_target_index_unique",
    doc! { "tenant_id": 1, "target_resource_urn": 1, "target_name": 1 }
);
