use super::Model as ModelDocument;
use crate::domain::entities::model::{fixtures::full_model, Model as DomainModel};

#[test]
fn model_document_round_trip_preserves_external_reference() {
    let original = full_model();

    let document = ModelDocument::from(&original);

    let restored = match DomainModel::try_from(document) {
        Ok(model) => model,
        Err(error) => panic!("document should reconstitute: {error}"),
    };

    assert_eq!(restored.id(), original.id());
    assert_eq!(restored.external_model_id(), original.external_model_id());
    assert_eq!(restored.owner(), original.owner());
}
