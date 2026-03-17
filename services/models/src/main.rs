#[cfg(feature = "actix")]
use actix_web;

#[cfg(feature = "actix")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    models::presentation::http::v1::actix_web::server::run_server().await
}