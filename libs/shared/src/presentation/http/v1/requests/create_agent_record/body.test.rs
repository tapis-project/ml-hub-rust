#[cfg(test)]
mod create_agent_record_body_test {
    use validator::Validate;

    use crate::presentation::http::v1::requests::create_agent_record::body::{
        AgentArtifactType, AgentProvider, AgentSkill, ArtifactLocator, Capabilities,
        CreateAgentRecordBody, MessageBinding, RestHttpAgentInterface, RestHttpLivenessProbe,
        RpcAgentInterface, StdioAgentInterface, Visibility,
    };

    fn rest_http_interfaces() -> Vec<RestHttpAgentInterface> {
        vec![RestHttpAgentInterface {
            name: "rest".into(),
            description: Some("REST interface".into()),
            message_binding: Some(MessageBinding::HttpJson),
            liveness_probe_config: None,
        }]
    }

    fn capabilities() -> Capabilities {
        Capabilities {
            streaming: true,
            push_notifications: false,
        }
    }

    fn body() -> CreateAgentRecordBody {
        CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            rest_http_interfaces: rest_http_interfaces(),
            rpc_interfaces: vec![],
            stdio_interfaces: vec![],
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: vec![],
            skills: vec![],
            icon_url: None,
            documentation_url: None,
            visibility: Visibility::Private,
        }
    }

    #[test]
    fn test_valid_create_agent_record_body() {
        assert!(body().validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_accepts_each_concrete_interface_type() {
        let mut body = body();
        body.rpc_interfaces = vec![RpcAgentInterface {
            name: "rpc".into(),
            description: None,
            message_binding: Some(MessageBinding::JsonRpc2_0),
        }];
        body.stdio_interfaces = vec![StdioAgentInterface {
            name: "stdio".into(),
            description: None,
            message_binding: None,
        }];

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_defaults_omitted_interface_collections_to_empty() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0","rest_http_interfaces":[{"name":"rest"}]}"#,
        );
        let body = match result {
            Ok(body) => body,
            Err(error) => panic!("Expected valid create agent record body: {error}"),
        };

        assert!(body.rpc_interfaces.is_empty());
        assert!(body.stdio_interfaces.is_empty());
        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_rejects_no_interfaces() {
        let mut body = body();
        body.rest_http_interfaces.clear();

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_duplicate_interface_names_across_collections() {
        let mut body = body();
        body.rpc_interfaces = vec![RpcAgentInterface {
            name: "rest".into(),
            description: None,
            message_binding: None,
        }];

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_accepts_rest_http_liveness_probe() {
        let mut body = body();
        body.rest_http_interfaces[0].liveness_probe_config = Some(RestHttpLivenessProbe {
            route: "/healthcheck".into(),
            timeout_seconds: 10,
        });

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_non_rest_interfaces_reject_liveness_probe_field() {
        for field in ["rpc_interfaces", "stdio_interfaces"] {
            let request = format!(
                r#"{{"name":"assistant","description":"A helpful agent","{field}":[{{"name":"interface","liveness_probe_config":{{"route":"/healthcheck","timeout_seconds":10}}}}],"capabilities":{{"streaming":false,"push_notifications":false}},"version":"1.0.0"}}"#
            );

            assert!(serde_json::from_str::<CreateAgentRecordBody>(&request).is_err());
        }
    }

    #[test]
    fn test_create_agent_record_body_rejects_legacy_interface_property() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0"}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_unknown_fields() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","rest_http_interfaces":[{"name":"rest"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0","unexpected":true}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_description_capabilities_and_version() {
        for request in [
            r#"{"name":"assistant","rest_http_interfaces":[{"name":"rest"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0"}"#,
            r#"{"name":"assistant","description":"A helpful agent","rest_http_interfaces":[{"name":"rest"}],"version":"1.0.0"}"#,
            r#"{"name":"assistant","description":"A helpful agent","rest_http_interfaces":[{"name":"rest"}],"capabilities":{"streaming":false,"push_notifications":false}}"#,
        ] {
            assert!(serde_json::from_str::<CreateAgentRecordBody>(request).is_err());
        }
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_semver_and_empty_interface_name() {
        let mut invalid_version = body();
        invalid_version.version = "v1.0.0".into();
        assert!(invalid_version.validate().is_err());

        let mut empty_name = body();
        empty_name.rest_http_interfaces[0].name = String::new();
        assert!(empty_name.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_validates_existing_nested_properties() {
        let mut body = body();
        body.provider = Some(AgentProvider {
            organization: String::new(),
            url: "not-a-url".into(),
        });
        body.artifact_locators = vec![ArtifactLocator {
            artifact_type: AgentArtifactType::SourceCode,
            url: "not-a-url".into(),
        }];
        body.skills = vec![AgentSkill {
            id: "Text Analysis".into(),
            name: "Text analysis".into(),
            description: "Analyzes text".into(),
            tags: vec![],
            examples: vec![],
        }];
        body.icon_url = Some("not-a-url".into());
        body.documentation_url = Some("also-not-a-url".into());

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_accepts_valid_provider_and_artifact_locator() {
        let mut body = body();
        body.provider = Some(AgentProvider {
            organization: "Example Geo Services Inc.".into(),
            url: "https://www.examplegeoservices.com".into(),
        });
        body.artifact_locators = vec![ArtifactLocator {
            artifact_type: AgentArtifactType::SourceCode,
            url: "tapis://example-system/path/to/agent-artifact".into(),
        }];

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_rejects_duplicate_skill_ids() {
        let mut body = body();
        body.skills = vec![
            AgentSkill {
                id: "text-analysis".into(),
                name: "Text analysis".into(),
                description: "Analyzes text".into(),
                tags: vec!["nlp".into()],
                examples: vec![],
            },
            AgentSkill {
                id: "text-analysis".into(),
                name: "Other analysis".into(),
                description: "Analyzes other text".into(),
                tags: vec!["nlp".into()],
                examples: vec![],
            },
        ];

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_defaults_artifact_locators_and_visibility() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","rest_http_interfaces":[{"name":"rest"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0","artifact_locators":null}"#,
        );
        let body = match result {
            Ok(body) => body,
            Err(error) => panic!("Expected valid create agent record body: {error}"),
        };

        assert!(body.artifact_locators.is_empty());
        assert!(matches!(body.visibility, Visibility::Private));
    }
}
