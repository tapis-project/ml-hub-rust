use mongodb::Database;
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};
use async_trait::async_trait;
use shared::infra::principal::mongo::indexes::PrincipalIdTenantIdIndexUnique;
use shared::infra::common::mongo::Index;

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreatePrincipalIdTenantIdIndexUniqueMigration),
    ]
}

pub struct CreatePrincipalIdTenantIdIndexUniqueMigration;

#[async_trait]
impl Migration for CreatePrincipalIdTenantIdIndexUniqueMigration {  
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        PrincipalIdTenantIdIndexUnique::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<PrincipalIdTenantIdIndexUnique as Index>::Collection>(PrincipalIdTenantIdIndexUnique::collection_name())
            .create_index(PrincipalIdTenantIdIndexUnique::index())
            .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let db: &Database = &env.db.unwrap();

        PrincipalIdTenantIdIndexUnique::ensure_collection(db)
            .await
            .expect("Collection to be created");

        db.collection::<<PrincipalIdTenantIdIndexUnique as Index>::Collection>(PrincipalIdTenantIdIndexUnique::collection_name()).drop_index(PrincipalIdTenantIdIndexUnique::INDEX_NAME).await?;
        
        Ok(())
    }
}