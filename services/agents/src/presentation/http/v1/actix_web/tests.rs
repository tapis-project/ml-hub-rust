#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test};
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
            .service(handlers::list_agents::list_agents)
            .service(handlers::create_agent::create_agent)
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
            StatusCode::INTERNAL_SERVER_ERROR
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

    #[test]
    async fn openapi_inlines_the_list_agents_scope_parameter(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let document = serde_json::to_value(ApiDoc::openapi())?;
        let parameters = match document
            .pointer("/paths/~1agents-api~1agents/get/parameters")
            .and_then(serde_json::Value::as_array)
        {
            Some(parameters) => parameters,
            None => {
                return Err(
                    std::io::Error::other("List agents operation should define parameters").into(),
                )
            }
        };
        let scope = match parameters
            .iter()
            .find(|parameter| parameter.get("name") == Some(&serde_json::json!("scope")))
        {
            Some(scope) => scope,
            None => {
                return Err(std::io::Error::other("List agents operation should define scope").into())
            }
        };

        assert_eq!(
            scope.pointer("/schema/enum"),
            Some(&serde_json::json!(["Owned", "Shared"]))
        );

        Ok(())
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
        assert!(
            document
                .paths
                .paths
                .contains_key("/agents-api/agent-records")
        );
        assert!(document.paths.paths.contains_key("/agents-api/healthcheck"));
        assert!(document.paths.paths.contains_key("/agents-api/agents"));
        assert!(document.components.as_ref().is_some_and(|components| components.schemas.contains_key("Agent")));
        assert!(document.components.as_ref().is_some_and(|components| components.schemas.contains_key("CreateAgentBody")));
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("AgentRecord")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("AgentSkill")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("Capabilities")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("AgentProvider")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("ArtifactLocator")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("Visibility")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("AgentArtifactType")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("CreateAgentRecordBody")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("RestHttpAgentInterface")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("RpcAgentInterface")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("StdioAgentInterface")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("RestHttpLivenessProbe")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("CreateAgentRecordResponse")
        );
        assert!(
            document
                .components
                .as_ref()
                .unwrap()
                .schemas
                .contains_key("ListAgentRecordsResponse")
        );

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
        assert!(
            document
                .pointer("/components/schemas/AgentRecord/required")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| {
                    required.iter().any(|field| field == "artifact_locators")
                })
        );

        let request_artifact_locators = document
            .pointer("/components/schemas/CreateAgentRecordBody/properties/artifact_locators")
            .expect("create request artifact_locators schema should exist");
        assert!(
            request_artifact_locators
                .get("type")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|types| types.iter().any(|value| value == "null"))
        );
        assert!(
            !document
                .pointer("/components/schemas/CreateAgentRecordBody/required")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| {
                    required.iter().any(|field| field == "artifact_locators")
                })
        );

        for request_schema in ["CreateAgentBody", "CreateAgentRecordBody"] {
            assert!(document
                .pointer(&format!("/components/schemas/{request_schema}/properties/tags"))
                .is_some());
            assert!(!document
                .pointer(&format!("/components/schemas/{request_schema}/required"))
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| { required.iter().any(|field| field == "tags") }));
        }

        for response_schema in ["Agent", "AgentRecord"] {
            let tags = match document.pointer(&format!(
                "/components/schemas/{response_schema}/properties/tags"
            )) {
                Some(tags) => tags,
                None => panic!("{response_schema} response tags schema should exist"),
            };

            assert_eq!(tags.get("type"), Some(&serde_json::Value::String("array".into())));
            assert_ne!(tags.get("nullable"), Some(&serde_json::Value::Bool(true)));
            assert!(document
                .pointer(&format!("/components/schemas/{response_schema}/required"))
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| { required.iter().any(|field| field == "tags") }));
        }

        let last_missed_heartbeat = match document
            .pointer("/components/schemas/Agent/properties/last_missed_heartbeat")
        {
            Some(last_missed_heartbeat) => last_missed_heartbeat,
            None => panic!("Agent last_missed_heartbeat schema should exist"),
        };

        assert!(last_missed_heartbeat
            .get("type")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|types| types.iter().any(|value| value == "null")));
        assert!(document
            .pointer("/components/schemas/Agent/properties/consecutive_missed_heartbeats")
            .is_some());
        assert!(document
            .pointer("/components/schemas/Agent/required")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|required| {
                required
                    .iter()
                    .any(|field| field == "consecutive_missed_heartbeats")
            }));

        for interface_collection in [
            "rest_http_interfaces",
            "rpc_interfaces",
            "stdio_interfaces",
        ] {
            assert!(document
                .pointer(&format!(
                    "/components/schemas/CreateAgentRecordBody/properties/{interface_collection}"
                ))
                .is_some());
            assert!(!document
                .pointer("/components/schemas/CreateAgentRecordBody/required")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| { required.iter().any(|field| field == interface_collection) }));
        }
        assert!(document
            .pointer("/components/schemas/CreateAgentRecordBody/properties/interfaces")
            .is_none());

        for interface_collection in [
            "rest_http_interfaces",
            "rpc_interfaces",
            "stdio_interfaces",
        ] {
            assert!(document
                .pointer(&format!(
                    "/components/schemas/AgentRecord/properties/{interface_collection}"
                ))
                .is_some());
            assert!(document
                .pointer("/components/schemas/AgentRecord/required")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| { required.iter().any(|field| field == interface_collection) }));
        }
        assert!(document
            .pointer("/components/schemas/AgentRecord/properties/interfaces")
            .is_none());

        for field in [
            "route",
            "interval_seconds",
            "timeout_seconds",
            "missed_heartbeat_threshold",
            "initial_delay_seconds",
        ] {
            assert!(document
                .pointer(&format!(
                    "/components/schemas/RestHttpLivenessProbe/properties/{field}"
                ))
                .is_some());
            assert!(document
                .pointer("/components/schemas/RestHttpLivenessProbe/required")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| required.iter().any(|required_field| required_field == field)));
        }

        let response_skills = document
            .pointer("/components/schemas/AgentRecord/properties/skills")
            .expect("AgentRecord response skills schema should exist");
        assert_eq!(
            response_skills.get("type"),
            Some(&serde_json::Value::String("array".into()))
        );
        assert!(
            document
                .pointer("/components/schemas/AgentRecord/required")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| { required.iter().any(|field| field == "skills") })
        );
        assert!(
            !document
                .pointer("/components/schemas/CreateAgentRecordBody/required")
                .and_then(serde_json::Value::as_array)
                .is_some_and(|required| { required.iter().any(|field| field == "skills") })
        );
    }
}
