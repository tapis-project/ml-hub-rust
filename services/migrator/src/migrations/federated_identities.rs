use async_trait::async_trait;
use mongodb::{Collection, Database, IndexModel};
use shared::infra::_common::mongo::Index;
use shared::infra::identity::mongo::{
    documents::FederatedIdentity,
    indexes::{
        IssuerSubjectIndexUnique, IssuerSubjectPrincipalIdIndexUnique,
        LegacyIssuerSubjectIndexUnique, LegacyIssuerSubjectPrincipalIdIndexUnique,
    },
};
use tfiala_mongodb_migrator::{migration::Migration, migrator::Env};

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(CreateIssuerSubjectIndexUniqueMigration),
        Box::new(CreateIssuerSubjectPrincipalIdIndexUniqueMigration),
        Box::new(CorrectFederatedIdentityIssuerIndexesMigration),
    ]
}

pub struct CreateIssuerSubjectIndexUniqueMigration;

#[async_trait]
impl Migration for CreateIssuerSubjectIndexUniqueMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        LegacyIssuerSubjectIndexUnique::ensure_collection(database).await?;

        collection(database)
            .create_index(LegacyIssuerSubjectIndexUnique::index())
            .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        LegacyIssuerSubjectIndexUnique::ensure_collection(database).await?;

        drop_index_if_exists(
            &collection(database),
            LegacyIssuerSubjectIndexUnique::INDEX_NAME,
        )
        .await?;

        Ok(())
    }

    fn get_id(&self) -> &str {
        "CreateIssuerSubjectIndexUniqueMigration"
    }
}

pub struct CreateIssuerSubjectPrincipalIdIndexUniqueMigration;

#[async_trait]
impl Migration for CreateIssuerSubjectPrincipalIdIndexUniqueMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        LegacyIssuerSubjectPrincipalIdIndexUnique::ensure_collection(database).await?;

        collection(database)
            .create_index(LegacyIssuerSubjectPrincipalIdIndexUnique::index())
            .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        LegacyIssuerSubjectPrincipalIdIndexUnique::ensure_collection(database).await?;

        drop_index_if_exists(
            &collection(database),
            LegacyIssuerSubjectPrincipalIdIndexUnique::INDEX_NAME,
        )
        .await?;

        Ok(())
    }

    fn get_id(&self) -> &str {
        "CreateIssuerSubjectPrincipalIdIndexUniqueMigration"
    }
}

pub struct CorrectFederatedIdentityIssuerIndexesMigration;

#[async_trait]
impl Migration for CorrectFederatedIdentityIssuerIndexesMigration {
    async fn up(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        IssuerSubjectIndexUnique::ensure_collection(database).await?;

        let collection = collection(database);

        create_index_if_missing(&collection, IssuerSubjectIndexUnique::index()).await?;
        create_index_if_missing(&collection, IssuerSubjectPrincipalIdIndexUnique::index()).await?;

        drop_index_if_exists(&collection, LegacyIssuerSubjectIndexUnique::INDEX_NAME).await?;
        drop_index_if_exists(
            &collection,
            LegacyIssuerSubjectPrincipalIdIndexUnique::INDEX_NAME,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, env: Env) -> anyhow::Result<()> {
        let database = database(&env)?;

        LegacyIssuerSubjectIndexUnique::ensure_collection(database).await?;

        let collection = collection(database);

        create_index_if_missing(&collection, LegacyIssuerSubjectIndexUnique::index()).await?;
        create_index_if_missing(
            &collection,
            LegacyIssuerSubjectPrincipalIdIndexUnique::index(),
        )
        .await?;

        drop_index_if_exists(&collection, IssuerSubjectIndexUnique::INDEX_NAME).await?;
        drop_index_if_exists(&collection, IssuerSubjectPrincipalIdIndexUnique::INDEX_NAME).await?;

        Ok(())
    }

    fn get_id(&self) -> &str {
        "CorrectFederatedIdentityIssuerIndexesMigration"
    }
}

fn database(env: &Env) -> anyhow::Result<&Database> {
    env.db
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Migration database is not configured"))
}

fn collection(database: &Database) -> Collection<FederatedIdentity> {
    database.collection(IssuerSubjectIndexUnique::collection_name())
}

async fn drop_index_if_exists(
    collection: &Collection<FederatedIdentity>,
    index_name: &str,
) -> anyhow::Result<()> {
    let index_names = collection.list_index_names().await?;

    if index_names.iter().any(|name| name == index_name) {
        collection.drop_index(index_name).await?;
    }

    Ok(())
}

async fn create_index_if_missing(
    collection: &Collection<FederatedIdentity>,
    index: IndexModel,
) -> anyhow::Result<()> {
    let index_name = index
        .options
        .as_ref()
        .and_then(|options| options.name.as_deref())
        .ok_or_else(|| anyhow::anyhow!("Federated identity index is missing its name"))?;

    let index_names = collection.list_index_names().await?;

    if !index_names.iter().any(|name| name == index_name) {
        collection.create_index(index).await?;
    }

    Ok(())
}
