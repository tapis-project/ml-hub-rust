use std::{
    env,
    sync::{Arc, Mutex}
};

mod operations {
    pub mod get_model;
    pub mod list_models;
}
mod dtos { 
    pub mod model_dto;
}
mod config;

use config::{DEFAULT_HOST, DEFAULT_PORT};
use huggingface_client::client::HuggingFaceClient;
use actix_web::{
    App,
    HttpServer,
    HttpMessage,
    dev::{
        ServiceRequest,
        Service
    }
};
use shared::clients::Client;

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

    // Initialize the platform client registrar and register the available clients
    // let mut registrar = PlatformClientRegistrar::new();
    // registrar
    //     .register(String::from("huggingface"), ClientType::Model, Arc::new(Mutex::new(Box::new(HuggingFaceClient::new()) as Box<dyn Client>)))
    //     .register(String::from("huggingface"), ClientType::Dataset, Arc::new(Mutex::new(Box::new(HuggingFaceClient::new()) as Box<dyn Client>)))
    //     .register(String::from("huggingface"), ClientType::Inference, Arc::new(Mutex::new(Box::new(HuggingFaceClient::new()) as Box<dyn Client>)))
    //     .register(String::from("huggingface"), ClientType::Training, Arc::new(Mutex::new(Box::new(HuggingFaceClient::new()) as Box<dyn Client>)));

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req: ServiceRequest, srv| {
                // Add the platform client registrar to the mutable extensions
                req.extensions_mut().insert(Arc::new(Mutex::new(Box::new(HuggingFaceClient::new()) as Box<dyn Client>)));
                
                // Continue processing the request
                srv.call(req)
            })
            .service(operations::get_model::get_model)
            .service(operations::list_models::list_models)
    })
        .bind(addrs)?
        .run()
        .await
}