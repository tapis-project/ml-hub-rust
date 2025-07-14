#[cfg(test)]
mod download_artifact_test {
    use actix_web::{test, web, App, http::header};
    use bytes::Bytes;
    use std::fs;
    use std::io::Write;
    use shared::common::infra::persistence::mongo::database::{get_db, ClientParams};
    use crate::bootstrap::state::AppState;
    use crate::presentation::http::v1::actix_web::handlers::download_artifact::download_artifact;

    // #[test]
    #[ignore]
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
    async fn test_download_artifact_success() {
        // 1. setup test AppState and service
        let app_state = setup_test_app_state().await;
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(download_artifact)
        ).await;

        let test_artifact_id = ""; // This should be a valid artifact ID in your test database

        let req = test::TestRequest::get()
            .uri(&format!("/models-api/artifacts/{}", test_artifact_id))
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
        let disposition = resp.headers().get(header::CONTENT_DISPOSITION).unwrap();
        assert_eq!(disposition.to_str().unwrap(), "attachment");
        
        let body: Bytes = test::read_body(resp).await;
        
        let download_dir = "./test_downloads";
        fs::create_dir_all(download_dir).expect("failed to create download directory");
        let dest_path = format!("{}/{}", download_dir, test_artifact_id);
        
        let mut dest_file = fs::File::create(&dest_path).expect("failed to create destination file");
        dest_file.write_all(&body).expect("failed to write to destination file");

        println!("✅ completed test_download_artifact_success, file saved to: {}", dest_path);

    }
}