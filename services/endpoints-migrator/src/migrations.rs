use async_trait::async_trait;
use mongodb::Database;
use shared::{
    infra::_common::mongo::Index,
    infra::persistence::mongo::documents::endpoint::indexes::{
        EndpointIdIndexUnique, EndpointTargetIndexUnique, EndpointTenantSlugIndexUnique,
    },
};
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(CreateEndpointIndexesMigration)]
}

pub struct CreateEndpointIndexesMigration;

#[async_trait]
impl Migration for CreateEndpointIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = env.db.as_ref().expect("migration database");

        create_index::<EndpointIdIndexUnique>(database).await?;
        create_index::<EndpointTenantSlugIndexUnique>(database).await?;
        create_index::<EndpointTargetIndexUnique>(database).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = env.db.as_ref().expect("migration database");

        drop_index::<EndpointTargetIndexUnique>(database).await?;
        drop_index::<EndpointTenantSlugIndexUnique>(database).await?;
        drop_index::<EndpointIdIndexUnique>(database).await?;

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
