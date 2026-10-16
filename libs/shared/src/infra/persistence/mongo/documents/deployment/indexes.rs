use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::infra::_common::mongo::Index;
use crate::infra::persistence::mongo::database::MODEL_DEPLOYMENT_COLLECTION;

use super::ModelDeployment;

pub struct ModelDeploymentReconciliationIndex;

impl Index for ModelDeploymentReconciliationIndex {
    type Collection = ModelDeployment;
    const INDEX_NAME: &'static str = "model_deployment_reconciliation_index";

    fn index() -> IndexModel {
        IndexModel::builder()
            .keys(doc! {
                "reconciliation_status": 1,
                "retry_after": 1,
            })
            .options(
                IndexOptions::builder()
                    .name(Self::INDEX_NAME.to_string())
                    .build(),
            )
            .build()
    }

    fn collection_name() -> &'static str {
        MODEL_DEPLOYMENT_COLLECTION
    }
}
