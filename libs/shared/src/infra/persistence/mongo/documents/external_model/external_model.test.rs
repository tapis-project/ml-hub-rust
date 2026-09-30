use super::ExternalModel as ExternalModelDocument;
use crate::domain::entities::model::{
    external_model::ExternalModel, fixtures::full_external_model,
};

#[test]
fn external_model_document_round_trip_preserves_provider_locator_and_canonical_metadata() {
    let original = full_external_model();

    let document = match ExternalModelDocument::try_from(&original) {
        Ok(document) => document,
        Err(error) => panic!("entity should map to a document: {error}"),
    };

    let restored = match ExternalModel::try_from(document) {
        Ok(model) => model,
        Err(error) => panic!("document should reconstitute: {error}"),
    };

    assert_eq!(restored.id(), original.id());
    assert_eq!(restored.provider(), original.provider());
    assert_eq!(
        restored.metadata().canonical(),
        original.metadata().canonical()
    );
}
