use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::{
    infra::_common::mongo::Index,
    infra::persistence::mongo::{database::MODEL_COLLECTION, documents::model::Model},
};

macro_rules! model_index {
    ($name:ident, $index_name:literal, $keys:expr, $unique:expr) => {
        pub struct $name;
        impl Index for $name {
            type Collection = Model;
            const INDEX_NAME: &'static str = $index_name;
            fn index() -> IndexModel {
                IndexModel::builder()
                    .keys($keys)
                    .options(
                        IndexOptions::builder()
                            .name(Self::INDEX_NAME.to_string())
                            .unique($unique)
                            .build(),
                    )
                    .build()
            }
            fn collection_name() -> &'static str {
                MODEL_COLLECTION
            }
        }
    };
}

model_index!(
    ModelIdIndexUnique,
    "model_id_index_unique",
    doc! { "id": 1 },
    true
);
model_index!(
    ModelOwnerExternalModelIndexUnique,
    "model_owner_external_model_index_unique",
    doc! { "tenant_id": 1, "owner": 1, "external_model_id": 1 },
    true
);

pub struct ModelArtifactIdIndexUnique;

impl Index for ModelArtifactIdIndexUnique {
    type Collection = Model;
    const INDEX_NAME: &'static str = "model_artifact_id_index_unique";

    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! { "artifact_id": 1 })
            .options(
                IndexOptions::builder()
                    .name(Self::INDEX_NAME.to_string())
                    .unique(true)
                    .partial_filter_expression(doc! { "artifact_id": { "$type": "binData" } })
                    .build(),
            )
            .build()
    }

    fn collection_name() -> &'static str {
        MODEL_COLLECTION
    }
}
