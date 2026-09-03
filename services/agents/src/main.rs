#[actix_web::main]
async fn main() -> std::io::Result<()> {
    agents::presentation::http::v1::actix_web::server::run_server().await
}
