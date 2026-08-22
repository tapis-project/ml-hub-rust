use std::env;

use actix_web::{App, HttpServer};

use crate::config::{DEFAULT_HOST, DEFAULT_PORT};
use super::handlers;

pub async fn run_server() -> std::io::Result<()> {
    env_logger::init();

    let address = (
        env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_owned()),
        env::var("PORT")
            .ok()
            .and_then(|port| port.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT),
    );

    HttpServer::new(|| {
        App::new()
            .service(handlers::list_agent_records::list_agent_records)
            .service(handlers::create_agent_record::create_agent_record)
            .service(handlers::healthcheck::healthcheck)
            .service(handlers::openapi::openapi)
    })
        .bind(address)?
        .run()
        .await
}
