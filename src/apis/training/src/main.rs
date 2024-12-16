mod operations {
    pub mod get_training;
    pub mod list_trainings;
}
mod dtos { 
    pub mod training_dto;
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
            .service(operations::get_training::get_training)
            .service(operations::list_trainings::list_trainings)
    })
        .bind(addrs)?
        .run()
        .await
        
}