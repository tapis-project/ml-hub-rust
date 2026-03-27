use crate::infra::common::mongo::Index;
use crate::infra::identity::mongo::documents::{FederatedIdentity, FEDERATED_IDENTITY_COLLECTION};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

pub struct IssuerSubjectIndexUnique;

impl Index for IssuerSubjectIndexUnique {
    type Collection = FederatedIdentity;
    const INDEX_NAME: &'static str = "create_issuer_subject_index_unique";
    fn index() -> IndexModel {
        IndexModel::builder()
                    .keys(doc! { "issue": 1, "subject": 1 })
                    .options(
                        Some(IndexOptions::builder()
                            .name(Self::INDEX_NAME.to_string())
                            .unique(true)
                            .build())
                    )
                    .build()
    }

    fn collection() -> &'static str {
        FEDERATED_IDENTITY_COLLECTION
    }
}

pub struct IssuerSubjectPrincipalIdIndexUnique;

impl Index for IssuerSubjectPrincipalIdIndexUnique {
    type Collection = FederatedIdentity;
    const INDEX_NAME: &'static str = "create_issuer_subject_principal_id_index_unique";
    fn index() -> IndexModel {
        IndexModel::builder()
                    .keys(doc! { "issue": 1, "subject": 1, "principal_id": 1 })
                    .options(
                        Some(IndexOptions::builder()
                            .name(Self::INDEX_NAME.to_string())
                            .unique(true)
                            .build())
                    )
                    .build()
    }

    fn collection() -> &'static str {
        FEDERATED_IDENTITY_COLLECTION
    }
}
