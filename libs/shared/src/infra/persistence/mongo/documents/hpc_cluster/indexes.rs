use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::{
    infra::_common::mongo::Index,
    infra::persistence::mongo::{
        database::HPC_CLUSTER_COLLECTION, documents::hpc_cluster::HpcCluster,
    },
};

macro_rules! hpc_cluster_index {
    ($name:ident, $index_name:literal, $keys:expr, $unique:expr) => {
        pub struct $name;

        impl Index for $name {
            type Collection = HpcCluster;
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
                HPC_CLUSTER_COLLECTION
            }
        }
    };
}

hpc_cluster_index!(
    HpcClusterIdIndexUnique,
    "hpc_cluster_id_index_unique",
    doc! { "id": 1 },
    true
);
hpc_cluster_index!(
    HpcClusterDataCenterIndex,
    "hpc_cluster_data_center_index",
    doc! { "data_center": 1 },
    false
);
hpc_cluster_index!(
    HpcClusterQueueIdIndexUnique,
    "hpc_cluster_queue_id_index_unique",
    doc! { "queues.id": 1 },
    true
);
