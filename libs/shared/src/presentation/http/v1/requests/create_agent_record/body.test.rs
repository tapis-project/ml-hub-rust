#[cfg(test)]
mod create_agent_record_body_test {
    use validator::Validate;

    use crate::presentation::http::v1::requests::create_agent_record::body::{
        AgentInterface, CreateAgentRecordBody, MessageBinding, Protocol,
    };

    fn interfaces() -> Vec<AgentInterface> {
        vec![AgentInterface {
            name: "rest".into(),
            description: Some("REST interface".into()),
            protocol: Protocol::RestHttp,
            message_binding: Some(MessageBinding::HttpJson),
        }]
    }

    #[test]
    fn test_valid_create_agent_record_body() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_name() {
        let body = CreateAgentRecordBody {
            name: String::new(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_unknown_fields() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"unexpected":true}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_interfaces() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: vec![],
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_interfaces() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent"}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_accepts_missing_message_binding() {
        let body = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"stdio","protocol":"Stdio"}]}"#,
        );

        let body = match body {
            Ok(body) => body,
            Err(error) => panic!("Expected valid create agent record body: {error}"),
        };

        assert!(body.validate().is_ok());
        assert_eq!(body.interfaces[0].name, "stdio");
        assert!(body.interfaces[0].description.is_none());
        assert!(body.interfaces[0].message_binding.is_none());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_interface_enum() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"Unknown"}]}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_description() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","interfaces":[{"name":"rest","protocol":"RestHttp"}]}"#,
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
        };

        assert!(body.validate().is_err());
    }
}
