use crate::infra::_common::mongo::Index;
use crate::infra::identity::mongo::documents::{FederatedIdentity, FEDERATED_IDENTITY_COLLECTION};
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

macro_rules! federated_identity_index {
    ($name:ident, $index_name:literal, $keys:expr) => {
        pub struct $name;

        impl Index for $name {
            type Collection = FederatedIdentity;
            const INDEX_NAME: &'static str = $index_name;

            fn index() -> IndexModel {
                IndexModel::builder()
                    .keys($keys)
                    .options(Some(
                        IndexOptions::builder()
                            .name(Self::INDEX_NAME.to_string())
                            .unique(true)
                            .build(),
                    ))
                    .build()
            }

            fn collection_name() -> &'static str {
                FEDERATED_IDENTITY_COLLECTION
            }
        }
    };
}

federated_identity_index!(
    LegacyIssuerSubjectIndexUnique,
    "create_issuer_subject_index_unique",
    doc! { "issue": 1, "subject": 1 }
);

federated_identity_index!(
    LegacyIssuerSubjectPrincipalIdIndexUnique,
    "create_issuer_subject_principal_id_index_unique",
    doc! { "issue": 1, "subject": 1, "principal_id": 1 }
);

federated_identity_index!(
    IssuerSubjectIndexUnique,
    "issuer_subject_unique",
    doc! { "issuer": 1, "subject": 1 }
);

federated_identity_index!(
    IssuerSubjectPrincipalIdIndexUnique,
    "issuer_subject_principal_id_unique",
    doc! { "issuer": 1, "subject": 1, "principal_id": 1 }
);

#[cfg(test)]
#[path = "indexes.test.rs"]
mod indexes_test;
