#[cfg(test)]
mod create_agent_record_body_test {
    use validator::Validate;

    use crate::presentation::http::v1::requests::create_agent_record::body::{
        AgentInterface, Capabilities, CreateAgentRecordBody, MessageBinding, Protocol,
    };

    fn interfaces() -> Vec<AgentInterface> {
        vec![AgentInterface {
            name: "rest".into(),
            description: Some("REST interface".into()),
            protocol: Protocol::RestHttp,
            message_binding: Some(MessageBinding::HttpJson),
        }]
    }

    fn capabilities() -> Capabilities {
        Capabilities {
            streaming: true,
            push_notifications: false,
        }
    }

    #[test]
    fn test_valid_create_agent_record_body() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_name() {
        let body = CreateAgentRecordBody {
            name: String::new(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_unknown_fields() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false},"unexpected":true}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_interfaces() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: vec![],
            capabilities: capabilities(),
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_interfaces() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","capabilities":{"streaming":false,"push_notifications":false}}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_accepts_missing_message_binding() {
        let body = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"stdio","protocol":"Stdio"}],"capabilities":{"streaming":true,"push_notifications":false}}"#,
        );

        let body = match body {
            Ok(body) => body,
            Err(error) => panic!("Expected valid create agent record body: {error}"),
        };

        assert!(body.validate().is_ok());
        assert_eq!(body.interfaces[0].name, "stdio");
        assert!(body.capabilities.streaming);
        assert!(!body.capabilities.push_notifications);
        assert!(body.interfaces[0].description.is_none());
        assert!(body.interfaces[0].message_binding.is_none());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_interface_enum() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"Unknown"}],"capabilities":{"streaming":false,"push_notifications":false}}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_description() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false}}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_interface_name() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: vec![AgentInterface {
                name: String::new(),
                description: None,
                protocol: Protocol::RestHttp,
                message_binding: None,
            }],
            capabilities: capabilities(),
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_duplicate_interface_names() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: vec![
                AgentInterface {
                    name: "rest".into(),
                    description: None,
                    protocol: Protocol::RestHttp,
                    message_binding: None,
                },
                AgentInterface {
                    name: "rest".into(),
                    description: None,
                    protocol: Protocol::Stdio,
                    message_binding: None,
                },
            ],
            capabilities: capabilities(),
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_capabilities() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}]}"#,
        );

        assert!(result.is_err());
    }
}
