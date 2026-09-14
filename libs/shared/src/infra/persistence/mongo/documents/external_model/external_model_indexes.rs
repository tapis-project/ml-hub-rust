use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::{
    infra::_common::mongo::Index,
    infra::persistence::mongo::{
        database::EXTERNAL_MODEL_COLLECTION,
        documents::external_model::ExternalModel,
    },
};

macro_rules! external_model_index {
    ($name:ident, $index_name:literal, $keys:expr, $unique:expr) => {
        pub struct $name;
        impl Index for $name {
            type Collection = ExternalModel;
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
                EXTERNAL_MODEL_COLLECTION
            }
        }
    };
}

external_model_index!(
    ExternalModelIdIndexUnique,
    "external_model_id_index_unique",
    doc! { "id": 1 },
    true
);
external_model_index!(
    ExternalModelProviderIndex,
    "external_model_provider_index",
    doc! { "provider": 1 },
    false
);
external_model_index!(
    ExternalModelTaskTypesIndex,
    "external_model_task_types_index",
    doc! { "provider": 1, "metadata.derived.task_types": 1 },
    false
);
external_model_index!(
    ExternalModelInferenceRuntimesIndex,
    "external_model_inference_runtimes_index",
    doc! { "provider": 1, "metadata.derived.inference_runtimes": 1 },
    false
);

pub struct ExternalModelHuggingFaceLocatorIndexUnique;
impl Index for ExternalModelHuggingFaceLocatorIndexUnique {
    type Collection = ExternalModel;
    const INDEX_NAME: &'static str = "external_model_huggingface_locator_index_unique";
    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! { "provider": 1, "huggingface_repo_locator.id": 1, "huggingface_repo_locator.sha": 1 })
            .options(IndexOptions::builder().name(Self::INDEX_NAME.to_string()).unique(true).partial_filter_expression(doc! { "provider": "HuggingFace" }).build())
            .build()
    }
    fn collection_name() -> &'static str {
        EXTERNAL_MODEL_COLLECTION
    }
}

pub struct ExternalModelTapisLocatorIndexUnique;
impl Index for ExternalModelTapisLocatorIndexUnique {
    type Collection = ExternalModel;
    const INDEX_NAME: &'static str = "external_model_tapis_locator_index_unique";
    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! { "provider": 1, "tapis_system_locator.site_id": 1, "tapis_system_locator.tenant_id": 1, "tapis_system_locator.system_id": 1, "tapis_system_locator.path": 1 })
            .options(IndexOptions::builder().name(Self::INDEX_NAME.to_string()).unique(true).partial_filter_expression(doc! { "provider": "Tapis" }).build())
            .build()
    }
    fn collection_name() -> &'static str {
        EXTERNAL_MODEL_COLLECTION
    }
}
