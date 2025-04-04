use std::env;
use config::{DEFAULT_HOST, DEFAULT_PORT};
use actix_web::{
    App,
    HttpServer,
    middleware::Logger
};

mod operations {
    pub mod get_model;
    pub mod list_models;
    pub mod download_model;
    pub mod publish_model;
    pub mod discover_models;
    pub mod index;
    pub mod health_check;
}
mod config;

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
            .wrap(Logger::default())
            .service(operations::index::index)
            .service(operations::health_check::health_check)
            .service(operations::get_model::get_model)
            .service(operations::list_models::list_models)
            .service(operations::download_model::download_model)
            .service(operations::discover_models::discover_models)
            .service(operations::publish_model::publish_model)
    })
        .bind(addrs)?
        .run()
        .await
}