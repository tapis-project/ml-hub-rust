use crate::infra::common::mongo::Index;
use crate::infra::principal::mongo::documents::{Principal, PRINCIPAL_COLLECTION};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

pub struct PrincipalIdTenantIdIndexUnique;

impl Index for PrincipalIdTenantIdIndexUnique {
    type Collection = Principal;
    const INDEX_NAME: &'static str = "create_principal_id_tenant_id_unique";
    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! { "id": 1, "tenant_id": 1 })
            .options(
                Some(IndexOptions::builder()
                    .name(Self::INDEX_NAME.to_string())
                    .unique(true)
                    .build())
            )
            .build()
    }

    fn collection() -> &'static str {
        PRINCIPAL_COLLECTION
    }
}


