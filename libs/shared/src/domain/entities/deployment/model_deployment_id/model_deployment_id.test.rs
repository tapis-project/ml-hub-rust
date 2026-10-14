use uuid::{Uuid, Version};

use super::ModelDeploymentId;

#[test]
fn new_generates_uuid_v7_identity() {
    let id = ModelDeploymentId::new();

    assert_eq!(id.as_uuid().get_version(), Some(Version::SortRand));
}

#[test]
fn reconstitute_preserves_uuid_identity() {
    let uuid = Uuid::now_v7();

    let id = ModelDeploymentId::reconstitute(uuid);

    assert_eq!(id.as_uuid(), &uuid);
    assert_eq!(id.into_uuid(), uuid);
}

#[test]
fn display_uses_uuid_representation() {
    let uuid = Uuid::now_v7();

    let id = ModelDeploymentId::reconstitute(uuid);

    assert_eq!(id.to_string(), uuid.to_string());
}
