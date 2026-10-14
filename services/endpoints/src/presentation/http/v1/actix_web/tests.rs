use utoipa::OpenApi;

use super::openapi::ApiDoc;

#[test]
fn openapi_contains_endpoint_read_routes() {
    let document = ApiDoc::openapi();

    assert!(document
        .paths
        .paths
        .contains_key("/endpoints-api/endpoints"));
    assert!(document
        .paths
        .paths
        .contains_key("/endpoints-api/endpoints/{endpoint_id}"));
    assert!(document
        .paths
        .paths
        .contains_key("/endpoints-api/resources/{resource_urn}/endpoints"));
    assert!(document
        .components
        .as_ref()
        .is_some_and(|components| components.schemas.contains_key("Endpoint")));
}
