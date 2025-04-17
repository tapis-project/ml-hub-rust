mod operations {
    pub mod get_inference_server;
    pub mod list_inference_servers;
    pub mod get_inference_server_deployment;
    pub mod list_inference_server_deployments;
    pub mod get_inference_server_docs;
}
mod dtos { 
    pub mod inference_dto;
    pub mod responses;
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
            .service(operations::get_inference_server::get_inference_server)
            .service(operations::list_inference_servers::list_inference_servers)
            .service(operations::get_inference_server::get_inference_server)
            .service(operations::list_inference_servers::list_inference_servers)
    })
        .bind(addrs)?
        .run()
        .await
}