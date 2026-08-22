#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};
    use utoipa::OpenApi;

    use crate::presentation::http::v1::actix_web::{handlers, openapi::ApiDoc};

    fn app() -> App<impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >> {
        App::new()
            .service(handlers::list_agents::list_agents)
            .service(handlers::create_agent::create_agent)
            .service(handlers::healthcheck::healthcheck)
            .service(handlers::openapi::openapi)
    }

    #[actix_web::test]
    async fn routes_have_the_expected_statuses() {
        let app = test::init_service(app()).await;

        let list = test::TestRequest::get().uri("/agents-api/agents").to_request();
        assert_eq!(test::call_service(&app, list).await.status(), StatusCode::NOT_IMPLEMENTED);

        let create = test::TestRequest::post().uri("/agents-api/agents").to_request();
        assert_eq!(test::call_service(&app, create).await.status(), StatusCode::NOT_IMPLEMENTED);

        let healthcheck = test::TestRequest::get().uri("/agents-api/healthcheck").to_request();
        assert_eq!(test::call_service(&app, healthcheck).await.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn openapi_endpoint_and_document_include_requested_routes() {
        let app = test::init_service(app()).await;
        let request = test::TestRequest::get()
            .uri("/agents-api/spec/openapi.json")
            .to_request();
        assert_eq!(test::call_service(&app, request).await.status(), StatusCode::OK);

        let document = ApiDoc::openapi();
        assert!(document.paths.paths.contains_key("/agents-api/agents"));
        assert!(document.paths.paths.contains_key("/agents-api/healthcheck"));
        assert!(document.components.as_ref().unwrap().schemas.contains_key("CreateAgentBody"));
        assert!(document.components.as_ref().unwrap().schemas.contains_key("CreateAgentResponse"));
        assert!(document.components.as_ref().unwrap().schemas.contains_key("ListAgentsResponse"));
    }
}
