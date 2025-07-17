use actix_web::http::header::AUTHORIZATION;
use actix_web::test;
use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use models::presentation::http::v1::actix_web::handlers::get_model::get_model;
use shared::models::presentation::http::v1::dto::GetModelPath;
use std::collections::HashMap;
use std::env;

#[cfg(test)]
mod tests {

    use actix_web::{HttpMessage, HttpRequest};

    use super::*;

    //fn get_path_public() -> String {
    //    String::from("https://dev.develop.tapis.io/v3/mlhub/models-api/platforms/models/huggingface/models/NickCliffel/PublicUCATestRepo")
    //}

    fn get_hugging_face_token() -> String {
        let value = env::var("HUGGINGFACE_HUB_TOKEN").expect("HUGGINGFACE_HUB_TOKEN not set");
        value
    }

    fn get_public_repo_id() -> String {
        String::from("nvidia/parakeet-tdt-0.6b-v2")
    }

    #[actix_web::test]
    async fn test_get_model_hugging_face_no_auth_header() {
        // creating application to run test
        let app = test::init_service(App::new().service(get_model)).await;

        // creating the request
        let req = test::TestRequest::get()
            .uri(&format!(
                "/models-api/platforms/{}/models/{}",
                "huggingface",
                get_public_repo_id()
            ))
            .to_request();

        // calling the api
        let response = test::call_service(&app, req).await;

        // ensuring that the api call executed correctly
        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_model_hugging_face_auth_header_with_colon() {
        let _ = env_logger::builder().is_test(true).try_init();
        // creating application to run test
        let app = test::init_service(App::new().service(get_model)).await;

        // creating the request
        let req = test::TestRequest::get()
            .uri(&format!(
                "/models-api/platforms/{}/models/{}",
                "huggingface",
                get_public_repo_id()
            ))
            .insert_header((
                "Authorization",
                format!("Bearer: {}", get_hugging_face_token()).to_string(),
            ))
            .to_request();

        // calling the api
        let response = test::call_service(&app, req).await;

        // ensuring api call executed correctly
        assert!(response.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_get_model_hugging_face_auth_header_space_in_front() {
        // creating application to run test
        let app = test::init_service(App::new().service(get_model)).await;

        // creating the request
        let req = test::TestRequest::get()
            .uri(&format!(
                "/models-api/platforms/{}/models/{}",
                "huggingface",
                get_public_repo_id()
            ))
            .insert_header((
                "Authorization",
                format!(" Bearer {}", get_hugging_face_token().to_string()),
            ))
            .to_request();

        // calling the api
        let response = test::call_service(&app, req).await;

        // ensuring call executed correctly
        assert!(response.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_get_model_hugging_face_auth_header_bearer_spelled_wrong() {
        // creating application to run test
        let app = test::init_service(App::new().service(get_model)).await;

        // creating the request
        let req = test::TestRequest::get()
            .uri(&format!(
                "/models-api/platforms/{}/models/{}",
                "huggingface",
                get_public_repo_id()
            ))
            .insert_header((
                "Authorization",
                format!("Bearir {}", get_hugging_face_token()).to_string(),
            ))
            .to_request();

        // calling the api
        let response = test::call_service(&app, req).await;

        // ensuring call executed correctly
        assert!(response.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_get_model_hugging_face_auth_header_bearer_only() {
        // creating application to run test
        let app = test::init_service(App::new().service(get_model)).await;

        // creating the request
        let req = test::TestRequest::get()
            .uri(&format!(
                "/models-api/platforms/{}/models/{}",
                "huggingface",
                get_public_repo_id()
            ))
            .insert_header(("Authorization", "Bearer "))
            .to_request();

        //calling the api
        let response = test::call_service(&app, req).await;

        // ensuring call executed correctly
        assert!(response.status().is_server_error());
    }

    #[actix_web::test]
    async fn test_get_model_hugging_face_with_auth_header_pass() {
        // creating application to run test
        let app = test::init_service(App::new().service(get_model)).await;

        // creating the request
        let req = test::TestRequest::get()
            .uri(&format!(
                "/models-api/platforms/{}/models/{}",
                "huggingface",
                get_public_repo_id()
            ))
            .insert_header((
                "Authorization",
                format!("Bearer {}", get_hugging_face_token()).to_string(),
            ))
            .to_request();

        // calling the api
        let response = test::call_service(&app, req).await;

        // ensuring that call executed correctly
        assert!(response.status().is_success());
    }
}
