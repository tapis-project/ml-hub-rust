#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, App};
    use utoipa::OpenApi;

    use crate::presentation::http::v1::actix_web::{handlers, openapi::ApiDoc};

    fn app() -> App<
        impl actix_web::dev::ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        >,
    > {
        App::new()
            .service(handlers::list_agent_records::list_agent_records)
            .service(handlers::create_agent_record::create_agent_record)
            .service(handlers::healthcheck::healthcheck)
            .service(handlers::openapi::openapi)
    }

    #[actix_web::test]
    async fn routes_have_the_expected_statuses() {
        let app = test::init_service(app()).await;

        let list = test::TestRequest::get()
            .uri("/agents-api/agent-records")
            .to_request();
        assert_eq!(
            test::call_service(&app, list).await.status(),
            StatusCode::NOT_IMPLEMENTED
        );

        let create = test::TestRequest::post()
            .uri("/agents-api/agent-records")
            .to_request();
        assert_eq!(
            test::call_service(&app, create).await.status(),
            StatusCode::BAD_REQUEST
        );

        let healthcheck = test::TestRequest::get()
            .uri("/agents-api/healthcheck")
            .to_request();
        assert_eq!(
            test::call_service(&app, healthcheck).await.status(),
            StatusCode::OK
        );
    }

    #[actix_web::test]
    async fn openapi_endpoint_and_document_include_requested_routes() {
        let app = test::init_service(app()).await;
        let request = test::TestRequest::get()
            .uri("/agents-api/spec/openapi.json")
            .to_request();
        assert_eq!(
            test::call_service(&app, request).await.status(),
            StatusCode::OK
        );

        let document = ApiDoc::openapi();
        assert!(document
            .paths
            .paths
            .contains_key("/agents-api/agent-records"));
        assert!(document.paths.paths.contains_key("/agents-api/healthcheck"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("AgentRecord"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("AgentInterface"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("AgentSkill"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("Capabilities"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("AgentProvider"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("ArtifactLocator"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("AgentArtifactType"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("CreateAgentRecordBody"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("CreateAgentRecordResponse"));
        assert!(document
            .components
            .as_ref()
            .unwrap()
            .schemas
            .contains_key("ListAgentRecordsResponse"));

        let document = serde_json::to_value(ApiDoc::openapi())
            .expect("OpenAPI document should serialize to JSON");
        let response_artifact_locators = document
            .pointer("/components/schemas/AgentRecord/properties/artifact_locators")
            .expect("AgentRecord response artifact_locators schema should exist");
        assert_eq!(
            response_artifact_locators.get("type"),
            Some(&serde_json::Value::String("array".into()))
        );
        assert_ne!(
            response_artifact_locators.get("nullable"),
            Some(&serde_json::Value::Bool(true))
        );
        assert!(document
            .pointer("/components/schemas/AgentRecord/required")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|required| { required.iter().any(|field| field == "artifact_locators") }));

        let request_artifact_locators = document
            .pointer("/components/schemas/CreateAgentRecordBody/properties/artifact_locators")
            .expect("create request artifact_locators schema should exist");
        assert!(request_artifact_locators
            .get("type")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|types| types.iter().any(|value| value == "null")));
        assert!(!document
            .pointer("/components/schemas/CreateAgentRecordBody/required")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|required| { required.iter().any(|field| field == "artifact_locators") }));

        let response_skills = document
            .pointer("/components/schemas/AgentRecord/properties/skills")
            .expect("AgentRecord response skills schema should exist");
        assert_eq!(
            response_skills.get("type"),
            Some(&serde_json::Value::String("array".into()))
        );
        assert!(document
            .pointer("/components/schemas/AgentRecord/required")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|required| { required.iter().any(|field| field == "skills") }));
        assert!(!document
            .pointer("/components/schemas/CreateAgentRecordBody/required")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|required| { required.iter().any(|field| field == "skills") }));
    }
}
