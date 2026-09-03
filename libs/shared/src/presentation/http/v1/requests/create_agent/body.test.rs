#[cfg(test)]
mod create_agent_body_test {
    use validator::Validate;

    use crate::presentation::http::v1::requests::create_agent::body::{
        AgentDeploymentModality, CreateAgentBody, RestHttpAgentEndpoint,
    };
    use crate::presentation::http::v1::requests::create_agent_record::body::Visibility;

    fn body() -> CreateAgentBody {
        CreateAgentBody {
            name: "agent".into(),
            description: "A runnable agent".into(),
            deployment_modality: AgentDeploymentModality::Persistent,
            rest_http_endpoints: vec![RestHttpAgentEndpoint {
                name: Some("rest".into()),
                message_binding: None,
                base_url: Some("https://agent.example.test".into()),
                liveness_probe: None,
            }],
            rpc_endpoints: vec![],
            stdio_endpoints: vec![],
            tags: vec![],
            agent_record_id: None,
            visibility: Visibility::Private,
        }
    }

    #[test]
    fn validates_agent_endpoints() {
        assert!(body().validate().is_ok());
    }

    #[test]
    fn requires_at_least_one_endpoint() {
        let mut body = body();
        body.rest_http_endpoints.clear();
        assert!(body.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_named_endpoints() {
        let request = r#"{"name":"agent","description":"A runnable agent","deployment_modality":"Persistent","rest_http_endpoints":[{"name":"rest"}],"rpc_endpoints":[{"name":"rest"}]}"#;
        let parsed = serde_json::from_str::<CreateAgentBody>(request);
        let body = match parsed {
            Ok(body) => body,
            Err(error) => panic!("Expected request to deserialize: {error}"),
        };
        assert!(body.validate().is_err());
    }

    #[test]
    fn rejects_liveness_probes_for_non_rest_endpoints() {
        let request = r#"{"name":"agent","description":"A runnable agent","deployment_modality":"Persistent","rpc_endpoints":[{"name":"rpc","liveness_probe":{"route":"/health"}}]}"#;
        assert!(serde_json::from_str::<CreateAgentBody>(request).is_err());
    }

    #[test]
    fn validates_bounded_tags() {
        let mut request = body();
        request.tags = vec!["tag".into()];
        assert!(request.validate().is_ok());

        request.tags = vec![String::new()];
        assert!(request.validate().is_err());

        request.tags = (0..17).map(|index| format!("tag-{index}")).collect();
        assert!(request.validate().is_err());
    }
}
