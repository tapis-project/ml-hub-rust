use std::time::Duration;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, from_document, oid::ObjectId, to_bson, Document, Uuid},
    Client, Collection,
};

use crate::{
    application::{
        inputs::hpc_cluster::ListHpcClustersInput,
        outputs::hpc_cluster::{HpcClusterListOutput, HpcClusterSummaryOutput},
        ports::{
            errors::InfrastructureError,
            hpc_cluster::{
                HpcClusterRepository as HpcClusterRepositoryPort, HpcClusterRepositoryError,
            },
        },
    },
    domain::entities::hpc_cluster::{
        DataCenter as DomainDataCenter, HpcCluster as DomainHpcCluster, HpcClusterId,
    },
    infra::persistence::mongo::{
        database::HPC_CLUSTER_COLLECTION,
        documents::hpc_cluster::{DataCenter, HpcCluster, HpcClusterSummary},
    },
};

pub struct HpcClusterRepository {
    collection: Collection<HpcCluster>,
}

impl HpcClusterRepository {
    pub fn new(client: &Client, db_name: String) -> Self {
        Self {
            collection: client.database(&db_name).collection(HPC_CLUSTER_COLLECTION),
        }
    }
}

#[async_trait]
impl HpcClusterRepositoryPort for HpcClusterRepository {
    async fn find_by_id(
        &self,
        data_center: &DomainDataCenter,
        id: &HpcClusterId,
    ) -> Result<Option<DomainHpcCluster>, HpcClusterRepositoryError> {
        let data_center = to_bson(&DataCenter::from(data_center)).map_err(map_error)?;
        let id = Uuid::from_bytes(*id.as_uuid().as_bytes());

        self.collection
            .find_one(doc! { "data_center": data_center, "id": id })
            .await
            .map_err(map_error)?
            .map(TryInto::try_into)
            .transpose()
            .map_err(map_conversion_error)
    }

    async fn list(
        &self,
        input: &ListHpcClustersInput,
    ) -> Result<HpcClusterListOutput, HpcClusterRepositoryError> {
        let filter = list_filter(input)?;
        let count_filter = filter.clone();
        let pipeline = list_pipeline(filter, input)?;

        let mut cursor = self
            .collection
            .aggregate(pipeline)
            .await
            .map_err(map_error)?;

        let mut documents = Vec::with_capacity(usize::from(input.limit()) + 1);

        while let Some(document) = cursor.try_next().await.map_err(map_error)? {
            documents.push(from_document::<HpcClusterSummary>(document).map_err(map_error)?);
        }

        let (hpc_clusters, cursor) = summaries_to_page(documents, input.limit());

        let count = if input.include_count() {
            Some(
                self.collection
                    .count_documents(count_filter)
                    .max_time(Duration::from_secs(2))
                    .await
                    .map_err(map_error)?,
            )
        } else {
            None
        };

        Ok(HpcClusterListOutput {
            hpc_clusters,
            cursor,
            count,
        })
    }
}

fn list_filter(input: &ListHpcClustersInput) -> Result<Document, HpcClusterRepositoryError> {
    Ok(doc! {
        "data_center": to_bson(&DataCenter::from(input.data_center())).map_err(map_error)?,
    })
}

fn list_pipeline(
    mut filter: Document,
    input: &ListHpcClustersInput,
) -> Result<Vec<Document>, HpcClusterRepositoryError> {
    if let Some(cursor) = input.cursor() {
        let id = ObjectId::parse_str(cursor).map_err(map_error)?;

        filter.insert("_id", doc! { "$gt": id });
    }

    Ok(vec![
        doc! { "$match": filter },
        doc! { "$sort": { "_id": 1 } },
        doc! { "$limit": i64::from(input.limit()) + 1 },
        doc! {
            "$project": {
                "_id": 1,
                "id": 1,
                "enabled": 1,
                "name": 1,
                "data_center": 1,
            }
        },
    ])
}

fn summaries_to_page(
    documents: Vec<HpcClusterSummary>,
    limit: u16,
) -> (Vec<HpcClusterSummaryOutput>, Option<String>) {
    let limit = usize::from(limit);
    let has_next_page = documents.len() > limit;
    let mut last_id = None;

    let hpc_clusters = documents
        .into_iter()
        .take(limit)
        .map(|document| {
            last_id = Some(document._id);

            HpcClusterSummaryOutput {
                id: uuid::Uuid::from_bytes(document.id.bytes()),
                enabled: document.enabled,
                name: document.name,
                data_center: document.data_center.into(),
            }
        })
        .collect();

    let cursor = if has_next_page {
        last_id.map(|id| id.to_hex())
    } else {
        None
    };

    (hpc_clusters, cursor)
}

fn map_error(error: impl std::fmt::Display) -> HpcClusterRepositoryError {
    let infrastructure_error = InfrastructureError::new_internal();

    log::error!(
        "[{}] HPC cluster persistence error: {}",
        infrastructure_error.error_id(),
        error
    );

    infrastructure_error.into()
}

fn map_conversion_error(error: impl std::fmt::Display) -> HpcClusterRepositoryError {
    map_error(error)
}

#[cfg(test)]
#[path = "hpc_cluster_repository.test.rs"]
mod hpc_cluster_repository_test;
