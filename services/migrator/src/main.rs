use std::env;

use migrator::{
    database::{ClientParams, initialize_client},
    migrations::{MongoMigrationGroupRunner, run_all_migration_groups},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_name = env::var("MONGO_DBNAME").expect("MONGO_DBNAME env var not set");
    let client = initialize_client(ClientParams {
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: database_name.clone(),
        replica_set: Some(
            env::var("MONGO_REPLICA_SET").expect("MONGO_REPLICA_SET env var not set"),
        ),
    })
    .await?;

    let runner = MongoMigrationGroupRunner::new(client.database(&database_name));

    run_all_migration_groups(&runner).await
}
