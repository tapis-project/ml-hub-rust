use std::{env, sync::Arc};

use hpc_cluster_seeder::{
    application::services::hpc_cluster_seed_service::{
        HpcClusterSeedOutcome, HpcClusterSeedService,
    },
    infra::{
        file_seed_source::FileHpcClusterSeedSource,
        mongo_seed_repository::MongoHpcClusterSeedRepository,
    },
};
use shared::infra::_common::mongo::{ClientParams, initialize_client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let database_name = required_env("MONGO_DBNAME")?;

    let client = initialize_client(ClientParams {
        username: required_env("MONGO_USERNAME")?,
        password: required_env("MONGO_PASSWORD")?,
        host: required_env("MONGO_HOST")?,
        port: required_env("MONGO_PORT")?,
        db: database_name.clone(),
        replica_set: Some(required_env("MONGO_REPLICA_SET")?),
    })
    .await?;

    let service = HpcClusterSeedService::new(
        Arc::new(FileHpcClusterSeedSource::new(required_env(
            "HPC_CLUSTER_SEED_PATH",
        )?)),
        Arc::new(MongoHpcClusterSeedRepository::new(client, database_name)),
    );

    match service.seed().await? {
        HpcClusterSeedOutcome::SkippedCollectionExists => {
            log::info!("HPC_CLUSTERS already exists; seed data was not loaded");
        }
        HpcClusterSeedOutcome::Seeded {
            hpc_cluster_count,
            queue_count,
        } => {
            log::info!("Seeded {hpc_cluster_count} HPC clusters with {queue_count} queues");
        }
    }

    Ok(())
}

fn required_env(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    env::var(name).map_err(|_| format!("{name} env var not set").into())
}
