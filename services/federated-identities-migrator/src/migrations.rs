use mongodb::Database;
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};
use async_trait::async_trait;
use shared::infra::_common::mongo::{Index};
use shared::infra::identity::mongo::indexes::{IssuerSubjectIndexUnique, IssuerSubjectPrincipalIdIndexUnique};

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreateIssuerSubjectIndexUniqueMigration),
        Box::new(CreateIssuerSubjectPrincipalIdIndexUniqueMigration),
    ]
}

pub struct CreateIssuerSubjectIndexUniqueMigration;

#[async_trait]
impl Migration for CreateIssuerSubjectIndexUniqueMigration {  
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        IssuerSubjectIndexUnique::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<IssuerSubjectIndexUnique as Index>::Collection>(IssuerSubjectIndexUnique::collection_name())
            .create_index(IssuerSubjectIndexUnique::index())
            .await?;
        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        IssuerSubjectIndexUnique::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<IssuerSubjectIndexUnique as Index>::Collection>(IssuerSubjectIndexUnique::collection_name())
            .drop_index(IssuerSubjectIndexUnique::INDEX_NAME).await?;
        Ok(())
    }
}

pub struct CreateIssuerSubjectPrincipalIdIndexUniqueMigration;

#[async_trait]
impl Migration for CreateIssuerSubjectPrincipalIdIndexUniqueMigration {  
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        IssuerSubjectPrincipalIdIndexUnique::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<IssuerSubjectPrincipalIdIndexUnique as Index>::Collection>(IssuerSubjectPrincipalIdIndexUnique::collection_name())
            .create_index(IssuerSubjectPrincipalIdIndexUnique::index())
            .await?;
        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        IssuerSubjectPrincipalIdIndexUnique::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<IssuerSubjectPrincipalIdIndexUnique as Index>::Collection>(IssuerSubjectPrincipalIdIndexUnique::collection_name())
            .drop_index(IssuerSubjectPrincipalIdIndexUnique::INDEX_NAME).await?;
        Ok(())
    }
}