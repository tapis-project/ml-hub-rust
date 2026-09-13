use async_trait::async_trait;
use mongodb::{bson::doc, Database};
use shared::infra::_common::mongo::Index;
use shared::infra::persistence::mongo::database::MODEL_COLLECTION;
use shared::infra::persistence::mongo::documents::external_model::external_model_indexes::{
    ExternalModelHuggingFaceLocatorIndexUnique, ExternalModelIdIndexUnique,
    ExternalModelInferenceRuntimesIndex, ExternalModelProviderIndex,
    ExternalModelTapisLocatorIndexUnique, ExternalModelTaskTypesIndex,
};
use shared::infra::persistence::mongo::documents::model::indexes::{
    ModelArtifactIdIndexUnique, ModelIdIndexUnique, ModelOwnerExternalModelIndexUnique,
};
use shared::infra::persistence::mongo::documents::model::Model;
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreateModelAuthorNameIndexMigration),
        Box::new(CreateTaskTypesIndexMigration),
        Box::new(CreateArtifactIdIndexMigration),
        Box::new(RenameModelMetadataKeywordsToTagsMigration),
        Box::new(CreateModelAggregateIndexesMigration),
        Box::new(CreateExternalModelAggregateIndexesMigration),
    ]
}

// These identities are historical and must remain stable for already-migrated databases.
pub struct CreateModelAuthorNameIndexMigration;
pub struct CreateTaskTypesIndexMigration;
pub struct CreateArtifactIdIndexMigration;

macro_rules! historical_noop_migration {
    ($migration:ty) => {
        #[async_trait]
        impl Migration for $migration {
            async fn up(&self, _env: Env) -> anyhow::Result<()> {
                Ok(())
            }
            async fn down(&self, _env: Env) -> anyhow::Result<()> {
                Ok(())
            }
        }
    };
}

historical_noop_migration!(CreateModelAuthorNameIndexMigration);
historical_noop_migration!(CreateTaskTypesIndexMigration);
historical_noop_migration!(CreateArtifactIdIndexMigration);

pub struct RenameModelMetadataKeywordsToTagsMigration;

#[async_trait]
impl Migration for RenameModelMetadataKeywordsToTagsMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = env.db.as_ref().expect("migration database");

        db.collection::<Model>(MODEL_COLLECTION)
            .update_many(doc! {}, doc! { "$rename": { "keywords": "tags" } })
            .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = env.db.as_ref().expect("migration database");

        db.collection::<Model>(MODEL_COLLECTION)
            .update_many(doc! {}, doc! { "$rename": { "tags": "keywords" } })
            .await?;

        Ok(())
    }
}

pub struct CreateModelAggregateIndexesMigration;

#[async_trait]
impl Migration for CreateModelAggregateIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db = env.db.as_ref().expect("migration database");

        create_index::<ModelIdIndexUnique>(db).await?;
        create_index::<ModelOwnerExternalModelIndexUnique>(db).await?;
        create_index::<ModelArtifactIdIndexUnique>(db).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db = env.db.as_ref().expect("migration database");

        drop_index::<ModelArtifactIdIndexUnique>(db).await?;
        drop_index::<ModelOwnerExternalModelIndexUnique>(db).await?;
        drop_index::<ModelIdIndexUnique>(db).await?;

        Ok(())
    }
}

pub struct CreateExternalModelAggregateIndexesMigration;

#[async_trait]
impl Migration for CreateExternalModelAggregateIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db = env.db.as_ref().expect("migration database");

        create_index::<ExternalModelIdIndexUnique>(db).await?;
        create_index::<ExternalModelHuggingFaceLocatorIndexUnique>(db).await?;
        create_index::<ExternalModelTapisLocatorIndexUnique>(db).await?;
        create_index::<ExternalModelProviderIndex>(db).await?;
        create_index::<ExternalModelTaskTypesIndex>(db).await?;
        create_index::<ExternalModelInferenceRuntimesIndex>(db).await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db = env.db.as_ref().expect("migration database");

        drop_index::<ExternalModelInferenceRuntimesIndex>(db).await?;
        drop_index::<ExternalModelTaskTypesIndex>(db).await?;
        drop_index::<ExternalModelProviderIndex>(db).await?;
        drop_index::<ExternalModelTapisLocatorIndexUnique>(db).await?;
        drop_index::<ExternalModelHuggingFaceLocatorIndexUnique>(db).await?;
        drop_index::<ExternalModelIdIndexUnique>(db).await?;

        Ok(())
    }
}

async fn create_index<I>(db: &Database) -> anyhow::Result<()>
where
    I: Index + Send,
    I::Collection: Send + Sync,
{
    I::ensure_collection(db).await?;
    db.collection::<I::Collection>(I::collection_name())
        .create_index(I::index())
        .await?;

    Ok(())
}

async fn drop_index<I>(db: &Database) -> anyhow::Result<()>
where
    I: Index,
    I::Collection: Send + Sync,
{
    db.collection::<I::Collection>(I::collection_name())
        .drop_index(I::INDEX_NAME)
        .await?;

    Ok(())
}
