use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::{
    infra::_common::mongo::Index,
    infra::persistence::mongo::{
        database::DEPLOYMENT_OPTION_COLLECTION, documents::deployment_option::DeploymentOption,
    },
};

macro_rules! deployment_option_index {
    ($name:ident, $index_name:literal, $keys:expr, $unique:expr) => {
        pub struct $name;

        impl Index for $name {
            type Collection = DeploymentOption;
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
                DEPLOYMENT_OPTION_COLLECTION
            }
        }
    };
}

deployment_option_index!(
    DeploymentOptionIdIndexUnique,
    "deployment_option_id_index_unique",
    doc! { "id": 1 },
    true
);
deployment_option_index!(
    DeploymentOptionSemanticIdentityIndexUnique,
    "deployment_option_semantic_identity_index_unique",
    doc! {
        "external_model_id": 1,
        "serving_runtime": 1,
        "deployment_target_type": 1,
        "hpc_cluster_queue.hpc_cluster_id": 1,
        "hpc_cluster_queue.batch_scheduler_queue_id": 1,
    },
    true
);
deployment_option_index!(
    DeploymentOptionExternalModelIdIndex,
    "deployment_option_external_model_id_index",
    doc! { "external_model_id": 1 },
    false
);
deployment_option_index!(
    DeploymentOptionSearchIndex,
    "deployment_option_search_index",
    doc! {
        "serving_runtime": 1,
        "supported_deployment_modalities": 1,
        "hpc_cluster_queue.hpc_cluster_id": 1,
        "hpc_cluster_queue.batch_scheduler_queue_id": 1,
        "external_model_id": 1,
    },
    false
);
