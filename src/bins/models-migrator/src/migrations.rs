use mongodb::{Database, bson::doc};
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};
use async_trait::async_trait;
use shared::infra::persistence::mongo::database::MODEL_METADATA_COLLECTION;
use shared::infra::persistence::mongo::documents::model_metadata::ModelMetadata;

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreateModelAuthorNameIndex {}),
        Box::new(CreateTaskTypesIndex {}),
    ]
}

pub struct CreateModelAuthorNameIndex;

#[async_trait]
impl Migration for CreateModelAuthorNameIndex {  
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();
        db.collection::<ModelMetadata>(MODEL_METADATA_COLLECTION)
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "author": 1, "name": 1 })
                    .options(
                        Some(mongodb::options::IndexOptions::builder()
                            .name("create_model_author_name_index_unique".to_string())
                            .unique(true)
                            .build())
                    )
                    .build()
            )
            .await?;
        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();
        db.collection::<ModelMetadata>(MODEL_METADATA_COLLECTION).drop_index("create_model_author_name_index_unique").await?;
        Ok(())
    }
}

pub struct CreateTaskTypesIndex;

#[async_trait]
impl Migration for CreateTaskTypesIndex {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        println!("INSIDE");
        let db: &Database = &env.db.unwrap();
        db.collection::<ModelMetadata>(MODEL_METADATA_COLLECTION)
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { "task_types": 1 })
                    .options(
                        Some(mongodb::options::IndexOptions::builder()
                            .name("create_task_types_index".to_string())
                            .build())
                    )
                    .build()
            )
            .await?;
        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();
        db.collection::<ModelMetadata>(MODEL_METADATA_COLLECTION).drop_index("create_task_types_index").await?;
        Ok(())
    }
}