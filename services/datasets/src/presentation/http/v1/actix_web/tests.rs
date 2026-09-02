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

#[test]
fn openapi_documents_dataset_item_count_and_retrieval_limit(
) -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    assert!(document
        .pointer("/components/schemas/Dataset/properties/item_count")
        .is_some());
    assert_eq!(
        document.pointer("/paths/~1datasets-api~1datasets/get/summary"),
        Some(&serde_json::json!(
            "List datasets with at most the first 50 items from each"
        ))
    );
    assert_eq!(
        document.pointer("/paths/~1datasets-api~1datasets~1{dataset_id}/get/summary"),
        Some(&serde_json::json!(
            "Get a dataset with at most its first 50 items"
        ))
    );

    Ok(())
}

#[test]
fn openapi_inlines_the_list_datasets_scope_parameter() -> Result<(), Box<dyn std::error::Error>> {
    let document = serde_json::to_value(ApiDoc::openapi())?;

    let parameters = match document
        .pointer("/paths/~1datasets-api~1datasets/get/parameters")
        .and_then(serde_json::Value::as_array)
    {
        Some(parameters) => parameters,
        None => {
            return Err(
                std::io::Error::other("List datasets operation should define parameters").into(),
            )
        }
    };

    let scope = match parameters
        .iter()
        .find(|parameter| parameter.get("name") == Some(&serde_json::json!("scope")))
    {
        Some(scope) => scope,
        None => {
            return Err(std::io::Error::other("List datasets operation should define scope").into())
        }
    };

    assert_eq!(
        scope.pointer("/schema/enum"),
        Some(&serde_json::json!(["Owned", "Shared", "Global"]))
    );

    Ok(())
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
