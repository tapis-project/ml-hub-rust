#[cfg(test)]
mod upload_artifacts_test {
    use std::env;
    use actix_web::{test, web, App, http::header};
    use bytes::Bytes;
    use shared::common::infra::persistence::mongo::database::{get_db, ClientParams};
    use crate::bootstrap::state::AppState;
    use crate::presentation::http::v1::actix_web::handlers::upload_artifacts::upload_artifacts;

    // #[test]
    async fn setup_test_app_state() -> web::Data<AppState> {
        web::Data::new(AppState {
            db: get_db(ClientParams{
                username: String::from("myuser"),
                password: String::from("mypassword"),
                host:     String::from("127.0.0.1"),
                port:     String::from("27017"),
                db:       String::from("testdb"),
            })
                .await
                .map_err(|err| {
                    panic!("Database initialization error: {}", err.to_string().as_str());
                })
                .expect("Datbase initialization error")
        })
    }

    #[actix_web::test]#[ignore]
    async fn test_upload_artifacts_success() {
        // 1. setup test AppState and service
        let app_state = setup_test_app_state().await;
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(upload_artifacts)
        ).await;

        // 2. Multipart/form-data
        let boundary = "----TestBoundary12345";
        let file_content = b"This is the content of the zip file.";
        let field_name = "artifact"; //
        let file_name = "test_artifact.zip";

        // Multipart structure
        let mut body = Vec::new();
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(
            format!(
                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                field_name, file_name
            )
                .as_bytes(),
        );
        body.extend_from_slice(b"Content-Type: application/zip\r\n\r\n");
        body.extend_from_slice(file_content);
        body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

        let payload = Bytes::from(body);

        // 3. HTTP request
        let req = test::TestRequest::post()
            .uri("/models-api/artifacts")
            .insert_header((
                header::CONTENT_TYPE,
                format!("multipart/form-data; boundary={}", boundary),
            ))
            .set_payload(payload)
            .to_request();

        // 4. service call
        let resp = test::call_service(&app, req).await;

        // 5. check response
        assert!(resp.status().is_success(), "Expected a success status");

        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("success"));
    }
}