#[cfg(feature = "actix")]
use actix_web;

#[cfg(feature = "actix")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    models::infra::web::http::actix_web::server::run_server().await
}