use uuid::Uuid;

use super::{Model, ModelError, ReconstituteModelProps};
use crate::shared_kernel::{
    enums::Visibility,
    identifiers::{traits::UrnGenerator, ExternalModelId},
    value_objects::TimeStamp,
};

#[test]
fn creates_owned_model_with_dormant_artifact() {
    let external_model_id = ExternalModelId::new();

    let model = match Model::create(
        "tenant-a".into(),
        "user-a".into(),
        "My model".into(),
        None,
        external_model_id,
        Visibility::Private,
    ) {
        Ok(model) => model,
        Err(error) => panic!("model should be valid: {error}"),
    };

    assert_eq!(model.external_model_id(), &external_model_id);
    assert!(model.artifact_id().is_none());
    assert_eq!(model.updated_at(), model.created_at());
    assert_eq!(
        model.urn().to_string(),
        format!("urn:mlhub:v1:tenant-a:model:{}", model.id())
    );
}

#[test]
fn rejects_empty_name_during_creation() {
    let result = Model::create(
        "tenant-a".into(),
        "user-a".into(),
        String::new(),
        None,
        ExternalModelId::new(),
        Visibility::Private,
    );

    assert!(matches!(result, Err(ModelError::EmptyName)));
}

#[test]
fn reports_invalid_persisted_name_as_data_integrity_error() {
    let now = TimeStamp::now();

    let result = Model::reconstitute(ReconstituteModelProps {
        id: Uuid::now_v7(),
        name: String::new(),
        description: None,
        tenant_id: "tenant-a".into(),
        owner: "user-a".into(),
        artifact_id: None,
        external_model_id: ExternalModelId::new(),
        visibility: Visibility::Private,
        updated_at: now.clone(),
        created_at: now,
    });

    assert!(matches!(result, Err(ModelError::DataIntegrityError(_))));
}
