#[cfg(test)]
mod test {
    use actix_web::{test, App};
    use crate::presentation::http::v1::actix_web::handlers::health_check::health_check;

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new()
            .service(health_check)
        ).await;
        let req = test::TestRequest::get()
            .uri("/models-api/health-check")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("success"));
    }
}