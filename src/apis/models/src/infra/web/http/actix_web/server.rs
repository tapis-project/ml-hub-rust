use crate::presentation;
// use shared::common::infra::system::Env;
// use log::error;
use actix_web::{App, HttpServer};
use std::env;
use actix_web::middleware::Logger;

pub async fn run_server() -> std::io::Result<()> {
    pub const DEFAULT_PORT: u16 = 8000;
    pub const DEFAULT_HOST: &str = "0.0.0.0";
    
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
            .service(presentation::http::v1::actix_web::handlers::index::index)
            .service(presentation::http::v1::actix_web::handlers::health_check::health_check)
            .service(presentation::http::v1::actix_web::handlers::get_model::get_model)
            .service(presentation::http::v1::actix_web::handlers::list_models::list_models)
            .service(presentation::http::v1::actix_web::handlers::stage_artifact::stage_artifact)
            .service(presentation::http::v1::actix_web::handlers::discover_models::discover_models)
            .service(presentation::http::v1::actix_web::handlers::publish_model::publish_model)
            .service(presentation::http::v1::actix_web::handlers::list_platforms::list_platforms)
    })
        .bind(addrs)?
        .run()
        .await
}