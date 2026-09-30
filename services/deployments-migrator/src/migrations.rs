use async_trait::async_trait;
use mongodb::Database;
use shared::{
    infra::_common::mongo::Index,
    infra::persistence::mongo::documents::hpc_cluster::indexes::{
        HpcClusterDataCenterIndex, HpcClusterIdIndexUnique, HpcClusterQueueIdIndexUnique,
    },
};
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(CreateHpcClusterIndexesMigration)]
}

pub struct CreateHpcClusterIndexesMigration;

#[async_trait]
impl Migration for CreateHpcClusterIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = env.db.as_ref().expect("migration database");

        create_index::<HpcClusterIdIndexUnique>(database).await?;
        create_index::<HpcClusterDataCenterIndex>(database).await?;
        create_index::<HpcClusterQueueIdIndexUnique>(database).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = env.db.as_ref().expect("migration database");

        drop_index::<HpcClusterQueueIdIndexUnique>(database).await?;
        drop_index::<HpcClusterDataCenterIndex>(database).await?;
        drop_index::<HpcClusterIdIndexUnique>(database).await?;

        Ok(())
    }
}

async fn create_index<I>(database: &Database) -> anyhow::Result<()>
where
    I: Index + Send,
    I::Collection: Send + Sync,
{
    I::ensure_collection(database).await?;

    database
        .collection::<I::Collection>(I::collection_name())
        .create_index(I::index())
        .await?;

    Ok(())
}

async fn drop_index<I>(database: &Database) -> anyhow::Result<()>
where
    I: Index,
    I::Collection: Send + Sync,
{
    database
        .collection::<I::Collection>(I::collection_name())
        .drop_index(I::INDEX_NAME)
        .await?;

    Ok(())
}
