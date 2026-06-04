use crate::infra::common::mongo::Index;
use crate::infra::persistence::mongo::database::MODEL_METADATA_COLLECTION;
use crate::infra::persistence::mongo::documents::model_metadata::ModelMetadata;
use mongodb::{bson::doc, options::IndexOptions, IndexModel};

pub struct ModelAuthorNameIndexUnique;

impl Index for ModelAuthorNameIndexUnique {
    type Collection = ModelMetadata;
    const INDEX_NAME: &'static str = "model_author_name_index_unique";
    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! { "author": 1, "name": 1 })
            .options(
                Some(IndexOptions::builder()
                    .name(Self::INDEX_NAME.to_string())
                    .unique(true)
                    .build())
            )
            .build()
    }

    fn collection_name() -> &'static str {
        MODEL_METADATA_COLLECTION
    }
}

pub struct TaskTypesIndex;

impl Index for TaskTypesIndex {
    type Collection = ModelMetadata;
    const INDEX_NAME: &'static str = "task_types_index";
    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! { "task_types": 1 })
            .options(
                Some(IndexOptions::builder()
                    .name(Self::INDEX_NAME.to_string())
                    .build())
            )
            .build()
    }

    fn collection_name() -> &'static str {
        MODEL_METADATA_COLLECTION
    }
}

pub struct ArtifactIdIndex;

impl Index for ArtifactIdIndex {
    type Collection = ModelMetadata;
    const INDEX_NAME: &'static str = "artifact_id_index";
    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! { "artifact_id": 1 })
            .options(
                Some(IndexOptions::builder()
                    .name(Self::INDEX_NAME.to_string())
                    .build())
            )
            .build()
    }

    fn collection_name() -> &'static str {
        MODEL_METADATA_COLLECTION
    }
}
