use async_trait::async_trait;
use mongodb::Client;
use shared::{
    domain::entities::hpc_cluster::HpcCluster as DomainHpcCluster,
    infra::persistence::mongo::{
        database::HPC_CLUSTER_COLLECTION, documents::hpc_cluster::HpcCluster,
    },
};

use crate::application::ports::{HpcClusterSeedError, HpcClusterSeedRepository};

pub struct MongoHpcClusterSeedRepository {
    client: Client,
    database_name: String,
}

impl MongoHpcClusterSeedRepository {
    pub fn new(client: Client, database_name: String) -> Self {
        Self {
            client,
            database_name,
        }
    }
}

#[async_trait]
impl HpcClusterSeedRepository for MongoHpcClusterSeedRepository {
    async fn collection_exists(&self) -> Result<bool, HpcClusterSeedError> {
        let collection_names = self
            .client
            .database(&self.database_name)
            .list_collection_names()
            .await
            .map_err(map_error)?;

        Ok(collection_names
            .iter()
            .any(|name| name == HPC_CLUSTER_COLLECTION))
    }

    async fn seed(&self, hpc_clusters: &[DomainHpcCluster]) -> Result<(), HpcClusterSeedError> {
        let documents = hpc_clusters
            .iter()
            .map(HpcCluster::from)
            .collect::<Vec<_>>();

        let collection = self
            .client
            .database(&self.database_name)
            .collection::<HpcCluster>(HPC_CLUSTER_COLLECTION);

        let mut session = self.client.start_session().await.map_err(map_error)?;

        session
            .start_transaction()
            .and_run2(async move |session| {
                collection
                    .insert_many(documents.clone())
                    .session(session)
                    .await
                    .map(|_| ())
            })
            .await
            .map_err(map_error)?;

        Ok(())
    }
}

fn map_error(error: impl std::fmt::Display) -> HpcClusterSeedError {
    HpcClusterSeedError::Persistence(error.to_string())
}
