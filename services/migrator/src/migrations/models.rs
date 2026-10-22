use async_trait::async_trait;
use mongodb::{Database, bson::doc};
use shared::infra::_common::mongo::Index;
use shared::infra::persistence::mongo::database::MODEL_COLLECTION;
use shared::infra::persistence::mongo::documents::deployment_option::indexes::{
    DeploymentOptionExternalModelIdIndex, DeploymentOptionIdIndexUnique,
    DeploymentOptionSearchIndex, DeploymentOptionSemanticIdentityIndexUnique,
};
use shared::infra::persistence::mongo::documents::external_model::external_model_indexes::{
    ExternalModelHuggingFaceLocatorIndexUnique, ExternalModelIdIndexUnique,
    ExternalModelInferenceRuntimesIndex, ExternalModelProviderIndex,
    ExternalModelTapisLocatorIndexUnique, ExternalModelTaskTypesIndex,
};
use shared::infra::persistence::mongo::documents::model::Model;
use shared::infra::persistence::mongo::documents::model::indexes::{
    ModelArtifactIdIndexUnique, ModelIdIndexUnique, ModelOwnerExternalModelIndexUnique,
};
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreateModelAuthorNameIndexMigration),
        Box::new(CreateTaskTypesIndexMigration),
        Box::new(CreateArtifactIdIndexMigration),
        Box::new(RenameModelMetadataKeywordsToTagsMigration),
        Box::new(CreateModelAggregateIndexesMigration),
        Box::new(CreateExternalModelAggregateIndexesMigration),
        Box::new(CreateDeploymentOptionIndexesMigration),
    ]
}

pub struct CreateDeploymentOptionIndexesMigration;

#[async_trait]
impl Migration for CreateDeploymentOptionIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        create_index::<DeploymentOptionIdIndexUnique>(database).await?;
        create_index::<DeploymentOptionSemanticIdentityIndexUnique>(database).await?;
        create_index::<DeploymentOptionExternalModelIdIndex>(database).await?;
        create_index::<DeploymentOptionSearchIndex>(database).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        drop_index::<DeploymentOptionSearchIndex>(database).await?;
        drop_index::<DeploymentOptionExternalModelIdIndex>(database).await?;
        drop_index::<DeploymentOptionSemanticIdentityIndexUnique>(database).await?;
        drop_index::<DeploymentOptionIdIndexUnique>(database).await?;

        Ok(())
    }

    fn get_id(&self) -> &str {
        "CreateDeploymentOptionIndexesMigration"
    }
}

// These identities are historical and must remain stable for already-migrated databases.
pub struct CreateModelAuthorNameIndexMigration;
pub struct CreateTaskTypesIndexMigration;
pub struct CreateArtifactIdIndexMigration;

macro_rules! historical_noop_migration {
    ($migration:ty, $id:literal) => {
        #[async_trait]
        impl Migration for $migration {
            async fn up(&self, _env: Env) -> anyhow::Result<()> {
                Ok(())
            }

            async fn down(&self, _env: Env) -> anyhow::Result<()> {
                Ok(())
            }

            fn get_id(&self) -> &str {
                $id
            }
        }
    };
}

historical_noop_migration!(
    CreateModelAuthorNameIndexMigration,
    "CreateModelAuthorNameIndexMigration"
);
historical_noop_migration!(
    CreateTaskTypesIndexMigration,
    "CreateTaskTypesIndexMigration"
);
historical_noop_migration!(
    CreateArtifactIdIndexMigration,
    "CreateArtifactIdIndexMigration"
);

pub struct RenameModelMetadataKeywordsToTagsMigration;

#[async_trait]
impl Migration for RenameModelMetadataKeywordsToTagsMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        database
            .collection::<Model>(MODEL_COLLECTION)
            .update_many(doc! {}, doc! { "$rename": { "keywords": "tags" } })
            .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        database
            .collection::<Model>(MODEL_COLLECTION)
            .update_many(doc! {}, doc! { "$rename": { "tags": "keywords" } })
            .await?;

        Ok(())
    }

    fn get_id(&self) -> &str {
        "RenameModelMetadataKeywordsToTagsMigration"
    }
}

pub struct CreateModelAggregateIndexesMigration;

#[async_trait]
impl Migration for CreateModelAggregateIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        create_index::<ModelIdIndexUnique>(database).await?;
        create_index::<ModelOwnerExternalModelIndexUnique>(database).await?;
        create_index::<ModelArtifactIdIndexUnique>(database).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        drop_index::<ModelArtifactIdIndexUnique>(database).await?;
        drop_index::<ModelOwnerExternalModelIndexUnique>(database).await?;
        drop_index::<ModelIdIndexUnique>(database).await?;

        Ok(())
    }

    fn get_id(&self) -> &str {
        "CreateModelAggregateIndexesMigration"
    }
}

pub struct CreateExternalModelAggregateIndexesMigration;

#[async_trait]
impl Migration for CreateExternalModelAggregateIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        create_index::<ExternalModelIdIndexUnique>(database).await?;
        create_index::<ExternalModelHuggingFaceLocatorIndexUnique>(database).await?;
        create_index::<ExternalModelTapisLocatorIndexUnique>(database).await?;
        create_index::<ExternalModelProviderIndex>(database).await?;
        create_index::<ExternalModelTaskTypesIndex>(database).await?;
        create_index::<ExternalModelInferenceRuntimesIndex>(database).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        drop_index::<ExternalModelInferenceRuntimesIndex>(database).await?;
        drop_index::<ExternalModelTaskTypesIndex>(database).await?;
        drop_index::<ExternalModelProviderIndex>(database).await?;
        drop_index::<ExternalModelTapisLocatorIndexUnique>(database).await?;
        drop_index::<ExternalModelHuggingFaceLocatorIndexUnique>(database).await?;
        drop_index::<ExternalModelIdIndexUnique>(database).await?;

        Ok(())
    }

    fn get_id(&self) -> &str {
        "CreateExternalModelAggregateIndexesMigration"
    }
}

fn database(env: &Env) -> anyhow::Result<&Database> {
    env.db
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Migration database is not configured"))
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
