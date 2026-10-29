use actix_web::{http::StatusCode, test, App};

use super::healthcheck;

#[actix_web::test]
async fn returns_ok() {
    let app = test::init_service(App::new().service(healthcheck)).await;

    let request = test::TestRequest::get()
        .uri("/deployments-api/healthcheck")
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), StatusCode::OK);
}
