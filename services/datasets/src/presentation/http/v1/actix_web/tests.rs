use super::{handlers, openapi::ApiDoc};
use actix_web::{http::StatusCode, test as actix_test, App};
use utoipa::OpenApi;

#[test]
fn openapi_contains_dataset_routes_and_schemas() {
    let document = ApiDoc::openapi();

    assert!(document.paths.paths.contains_key("/datasets-api/datasets"));
    assert!(document
        .paths
        .paths
        .contains_key("/datasets-api/datasets/{dataset_id}"));

    let components = document.components.expect("components");

    assert!(components.schemas.contains_key("Dataset"));
    assert!(components.schemas.contains_key("RegisterDatasetBody"));
}

#[actix_web::test]
async fn routes_have_expected_registration_and_public_statuses() {
    let app = actix_test::init_service(
        App::new()
            .service(handlers::register_dataset::register_dataset)
            .service(handlers::list_datasets::list_datasets)
            .service(handlers::healthcheck::healthcheck)
            .service(handlers::openapi::openapi),
    )
    .await;

    let registration = actix_test::TestRequest::post()
        .uri("/datasets-api/datasets")
        .to_request();
    assert_eq!(
        actix_test::call_service(&app, registration).await.status(),
        StatusCode::BAD_REQUEST
    );

    let healthcheck = actix_test::TestRequest::get()
        .uri("/datasets-api/healthcheck")
        .to_request();
    assert_eq!(
        actix_test::call_service(&app, healthcheck).await.status(),
        StatusCode::OK
    );

    let openapi = actix_test::TestRequest::get()
        .uri("/datasets-api/spec/openapi.json")
        .to_request();
    assert_eq!(
        actix_test::call_service(&app, openapi).await.status(),
        StatusCode::OK
    );
}
