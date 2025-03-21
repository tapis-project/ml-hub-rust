mod operations {
    pub mod health_check;
    pub mod get_dataset;
    pub mod list_datasets;
    pub mod download_dataset;
}
mod config;

use config::{DEFAULT_HOST, DEFAULT_PORT};
use std::env;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();
    
    // Set the address from env vars HOST and PORT, fallback to default values
    // if values for these env vars are not defined
    let addrs = (
        env::var("HOST").unwrap_or(DEFAULT_HOST.into()),
        env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT)
    );

    HttpServer::new(|| {
        App::new()
            .service(operations::health_check::health_check)
            .service(operations::get_dataset::get_dataset)
            .service(operations::list_datasets::list_datasets)
            .service(operations::download_dataset::download_dataset)
    })
        .bind(addrs)?
        .run()
        .await
}