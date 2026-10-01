use mongodb::bson::{doc, DateTime};

use super::*;

fn federated_identity() -> FederatedIdentity {
    let now = DateTime::now();

    FederatedIdentity {
        _id: None,
        principal_id: "cgarcia".into(),
        issuer: "https://tacc.tapis.io/v3/tokens".into(),
        subject: "cgarcia@tacc".into(),
        metadata: None,
        tenant_id: "tacc".into(),
        created_at: now,
        last_modified: now,
        last_seen: now,
    }
}

#[test]
fn federated_identity_upsert_filter_is_tenant_scoped() {
    let identity = federated_identity();

    let filter = federated_identity_filter(&identity);

    assert_eq!(
        filter,
        doc! {
            "issuer": "https://tacc.tapis.io/v3/tokens",
            "subject": "cgarcia@tacc",
            "principal_id": "cgarcia",
            "tenant_id": "tacc",
        }
    );
}

#[test]
fn principal_lookup_filter_is_tenant_scoped() {
    let filter = principal_filter("cgarcia", "tacc");

    assert_eq!(
        filter,
        doc! {
            "id": "cgarcia",
            "tenant_id": "tacc",
        }
    );
}
