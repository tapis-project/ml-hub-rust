use actix_web::test;
use actix_web::web;
use actix_web::App;
use models::presentation::http::v1::actix_web::handlers::get_model::get_model;
use shared::models::presentation::http::v1::dto::GetModelPath;
use std::env;

#[cfg(test)]
mod tests {
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
        let model = get_public_repo_id();
        // creating application to run test
        // let app = test::init_service(App::new().route(
        //    "models-api/platforms/{huggingface}/models/{model}",
        //    web::get().to(get_model),
        //))
        //.await;

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
        let _response = test::call_service(&app, req).await;
    }

    #[actix_web::test]
    async fn test_get_model_hugging_face_auth_header_with_colon() {
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
                format!("bearer: {}", get_hugging_face_token()).to_string(),
            ))
            .to_request();

        // calling the api
        let respone = test::call_service(&app, req).await;
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
                format!(" bearer {}", get_hugging_face_token().to_string()),
            ))
            .to_request();

        // calling the api
        let _repsonse = test::call_service(&app, req).await;
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
                format!("bearir {}", get_hugging_face_token()).to_string(),
            ))
            .to_request();

        // calling the api
        let _response = test::call_service(&app, req).await;
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
            .insert_header(("Authorization", "bearer "))
            .to_request();

        //calling the api
        let _repsonse = test::call_service(&app, req).await;

        // might be needed later
        //let path = web::Path::from(("huggingface".to_string(), get_public_repo_id()));
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
                format!("bearer {}", get_public_repo_id()).to_string(),
            ))
            .to_request();

        // calling the api
        let _reponse = test::call_service(&app, req).await;
    }
}
