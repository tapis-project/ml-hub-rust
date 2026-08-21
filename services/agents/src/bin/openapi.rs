use agents::presentation::http::v1::actix_web::openapi::ApiDoc;
use utoipa::OpenApi;

fn main() {
    println!("{}", ApiDoc::openapi().to_json().expect("OpenAPI document serializes"));
}
