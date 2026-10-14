use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    Database,
    bson::{Document, Uuid, doc},
};
use shared::{
    infra::_common::mongo::Index,
    infra::persistence::mongo::database::{
        AGENT_COLLECTION, ENDPOINT_COLLECTION, MODEL_DEPLOYMENT_COLLECTION,
    },
    infra::persistence::mongo::documents::endpoint::indexes::{
        EndpointIdIndexUnique, EndpointOwnerPageIndex, EndpointResourcePageIndex,
        EndpointTargetIndexUnique, EndpointTenantSlugIndexUnique,
    },
    shared_kernel::identifiers::urn::{NetworkResourceReference, Urn},
};
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreateEndpointIndexesMigration),
        Box::new(AddEndpointOwnerMigration),
    ]
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

pub struct AddEndpointOwnerMigration;

#[async_trait]
impl Migration for AddEndpointOwnerMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = env.db.as_ref().expect("migration database");
        let endpoints = database.collection::<Document>(ENDPOINT_COLLECTION);
        let agents = database.collection::<Document>(AGENT_COLLECTION);
        let deployments = database.collection::<Document>(MODEL_DEPLOYMENT_COLLECTION);

        let mut cursor = endpoints
            .find(doc! { "owner": { "$exists": false } })
            .await?;

        while let Some(endpoint) = cursor.try_next().await? {
            let endpoint_id = endpoint
                .get_object_id("_id")
                .map_err(|error| anyhow::anyhow!(error.to_string()))?;
            let tenant_id = endpoint
                .get_str("tenant_id")
                .map_err(|error| anyhow::anyhow!(error.to_string()))?;
            let resource_urn = endpoint
                .get_str("target_resource_urn")
                .map_err(|error| anyhow::anyhow!(error.to_string()))?;
            let reference = Urn::new(resource_urn.to_owned()).parse_network_resource()?;

            if reference.tenant_id() != tenant_id {
                anyhow::bail!("Endpoint resource tenant does not match Endpoint tenant");
            }

            let id = Uuid::from_bytes(*reference.id().as_bytes());
            let resource = match reference {
                NetworkResourceReference::Agent { .. } => {
                    agents
                        .find_one(doc! { "tenant_id": tenant_id, "id": id })
                        .await?
                }
                NetworkResourceReference::ModelDeployment { .. } => {
                    deployments
                        .find_one(doc! { "tenant_id": tenant_id, "id": id })
                        .await?
                }
            }
            .ok_or_else(|| anyhow::anyhow!("Endpoint references a missing resource"))?;

            let owner = resource
                .get_str("owner")
                .map_err(|error| anyhow::anyhow!(error.to_string()))?;

            endpoints
                .update_one(
                    doc! { "_id": endpoint_id },
                    doc! { "$set": { "owner": owner } },
                )
                .await?;
        }

        create_index::<EndpointOwnerPageIndex>(database).await?;
        create_index::<EndpointResourcePageIndex>(database).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = env.db.as_ref().expect("migration database");

        drop_index::<EndpointResourcePageIndex>(database).await?;
        drop_index::<EndpointOwnerPageIndex>(database).await?;

        database
            .collection::<Document>(ENDPOINT_COLLECTION)
            .update_many(doc! {}, doc! { "$unset": { "owner": "" } })
            .await?;

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
