use models::presentation::http::v1::actix_web::openapi::ApiDoc;
use utoipa::OpenApi;

// Writes the OpenAPI spec for the Models API to stdout
pub fn main() {
    println!("{}", ApiDoc::openapi().to_json().unwrap());
}