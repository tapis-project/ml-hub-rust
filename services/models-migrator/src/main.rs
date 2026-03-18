use models_migrator::migrations::get_migrations;
use models_migrator::database::{ClientParams, get_db};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Database connection
    let db = get_db(ClientParams{
        username: env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var not set"),
        password: env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var not set"),
        host: env::var("MONGO_HOST").expect("MONGO_HOST env var not set"),
        port: env::var("MONGO_PORT").expect("MONGO_PORT env var not set"),
        db: env::var("MONGO_NAME").expect("MONGO_NAME env var not set"),
    })
        .await?;

    tfiala_mongodb_migrator::migrator::default::DefaultMigrator::new()
        .with_conn(db.clone())
        .with_migrations_vec(get_migrations())
        .up()
        .await?;

    Ok(())
}
