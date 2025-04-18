mod operations {
    pub mod get_inference_server;
    pub mod list_inference_servers;
    pub mod get_inference_server_deployment;
    pub mod list_inference_server_deployments;
    pub mod get_inference_server_docs;
    pub mod create_inference_server;
}
mod repositories {
    pub mod inference_server_repository;
}
mod config;
mod database;
mod state;

use config::{DEFAULT_HOST, DEFAULT_PORT};
use database::{get_db, ClientParams};
use shared::system::Env;
use std::env;
use actix_web::{web, App, HttpServer};
use state::AppState;
use log::error;

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

    let env = Env::new()
        .map_err(|err| {
            error!("Shared environment initialization error: {}", err.to_string().as_str());
            err 
        })
        .expect("Shared environment initialization error");

    // Initialize AppState
    let state = AppState {
        db: get_db(ClientParams{
            username: env.inference_db_user,
            password: env.inference_db_password,
            host: env.inference_db_host,
            port: env.inference_db_port,
            db: env.inference_db
        })
            .await
            .map_err(|err| {
                error!("Database initialization error: {}", err.to_string().as_str());
                err 
            })
            .expect("Datbase initialization error")
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(operations::get_inference_server::get_inference_server)
            .service(operations::list_inference_servers::list_inference_servers)
            .service(operations::get_inference_server_docs::get_inference_server_docs)
            .service(operations::list_inference_servers::list_inference_servers)
            .service(operations::create_inference_server::create_inference_server)
    })
        .bind(addrs)?
        .run()
        .await
}