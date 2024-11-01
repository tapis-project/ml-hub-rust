mod operations {
    pub mod get_model;
    pub mod list_models;
}
mod models { 
    pub mod models;
    pub mod requests;
    pub mod responses;
}

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(operations::get_model::get_model)
            .service(operations::list_models::list_models)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}