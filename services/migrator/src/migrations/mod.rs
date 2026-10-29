mod deployments;
mod endpoints;
mod federated_identities;
mod models;
mod principals;

use anyhow::Context;
use async_trait::async_trait;
use mongodb::Database;
use tfiala_mongodb_migrator::{migration::Migration, migrator::default::DefaultMigrator};

#[cfg(test)]
#[path = "migrations.test.rs"]
mod tests;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MigrationGroup {
    Principals,
    FederatedIdentities,
    Models,
    Deployments,
    Endpoints,
}

impl MigrationGroup {
    pub const ORDERED: [Self; 5] = [
        Self::Principals,
        Self::FederatedIdentities,
        Self::Models,
        Self::Deployments,
        Self::Endpoints,
    ];

    pub fn migrations(self) -> Vec<Box<dyn Migration>> {
        match self {
            Self::Principals => principals::get_migrations(),
            Self::FederatedIdentities => federated_identities::get_migrations(),
            Self::Models => models::get_migrations(),
            Self::Deployments => deployments::get_migrations(),
            Self::Endpoints => endpoints::get_migrations(),
        }
    }
}

#[async_trait]
pub trait MigrationGroupRunner: Send + Sync {
    async fn run(&self, group: MigrationGroup) -> anyhow::Result<()>;
}

pub struct MongoMigrationGroupRunner {
    database: Database,
}

impl MongoMigrationGroupRunner {
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

#[async_trait]
impl MigrationGroupRunner for MongoMigrationGroupRunner {
    async fn run(&self, group: MigrationGroup) -> anyhow::Result<()> {
        DefaultMigrator::new()
            .with_conn(self.database.clone())
            .with_migrations_vec(group.migrations())
            .up()
            .await?;

        Ok(())
    }
}

pub async fn run_all_migration_groups<R>(runner: &R) -> anyhow::Result<()>
where
    R: MigrationGroupRunner,
{
    for group in MigrationGroup::ORDERED {
        runner
            .run(group)
            .await
            .with_context(|| format!("Migration group '{group:?}' failed"))?;
    }

    Ok(())
}
