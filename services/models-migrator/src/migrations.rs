use mongodb::{Database, bson::doc};
use shared::domain::entities::model_metadata::ModelMetadata;
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};
use async_trait::async_trait;
use shared::infra::persistence::mongo::documents::model_metadata::indexes::{TaskTypesIndex, ArtifactIdIndex, ModelAuthorNameIndexUnique};
use shared::infra::persistence::mongo::database::MODEL_METADATA_COLLECTION;
use shared::infra::_common::mongo::Index;

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreateModelAuthorNameIndexMigration),
        Box::new(CreateTaskTypesIndexMigration),
        Box::new(CreateArtifactIdIndexMigration),
        Box::new(RenameModelMetadataKeywordsToTagsMigration),
    ]
}

pub struct CreateModelAuthorNameIndexMigration;

#[async_trait]
impl Migration for CreateModelAuthorNameIndexMigration {  
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();
        
        ModelAuthorNameIndexUnique::ensure_collection(db)
            .await?;

        db.collection::<<ModelAuthorNameIndexUnique as Index>::Collection>(ModelAuthorNameIndexUnique::collection_name())
            .create_index(ModelAuthorNameIndexUnique::index())
            .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        ModelAuthorNameIndexUnique::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<ModelAuthorNameIndexUnique as Index>::Collection>(ModelAuthorNameIndexUnique::collection_name())
            .drop_index(ModelAuthorNameIndexUnique::INDEX_NAME).await?;
        Ok(())
    }
}

pub struct CreateTaskTypesIndexMigration;

#[async_trait]
impl Migration for CreateTaskTypesIndexMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        TaskTypesIndex::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<TaskTypesIndex as Index>::Collection>(TaskTypesIndex::collection_name())
            .create_index(TaskTypesIndex::index())
            .await?;
        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        TaskTypesIndex::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<TaskTypesIndex as Index>::Collection>(TaskTypesIndex::collection_name())
            .drop_index(TaskTypesIndex::INDEX_NAME).await?;
        Ok(())
    }
}

pub struct CreateArtifactIdIndexMigration;

#[async_trait]
impl Migration for CreateArtifactIdIndexMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        ArtifactIdIndex::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<ArtifactIdIndex as Index>::Collection>(ArtifactIdIndex::collection_name())
            .create_index(ArtifactIdIndex::index())
            .await?;
        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        ArtifactIdIndex::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<ArtifactIdIndex as Index>::Collection>(ArtifactIdIndex::collection_name()).drop_index(ArtifactIdIndex::INDEX_NAME).await?;
        Ok(())
    }
}

pub struct RenameModelMetadataKeywordsToTagsMigration;

#[async_trait]
impl Migration for RenameModelMetadataKeywordsToTagsMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = env.db.as_ref().unwrap();

        db.collection::<ModelMetadata>(MODEL_METADATA_COLLECTION)
            .update_many(doc! {}, doc! { "$rename": { "keywords": "tags" } })
            .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = env.db.as_ref().unwrap();

        db.collection::<ModelMetadata>(MODEL_METADATA_COLLECTION)
            .update_many(doc! {}, doc! { "$rename": { "tags": "keywords" } })
            .await?;
        
        Ok(())
    }
}