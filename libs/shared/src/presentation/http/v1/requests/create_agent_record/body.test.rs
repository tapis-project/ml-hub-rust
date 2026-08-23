#[cfg(test)]
mod create_agent_record_body_test {
    use validator::Validate;

    use crate::presentation::http::v1::requests::create_agent_record::body::{
        AgentInterface, CreateAgentRecordBody, MessageBinding, Protocol,
    };

    fn supported_interfaces() -> Vec<AgentInterface> {
        vec![AgentInterface {
            protocol: Protocol::RestHttp,
            message_binding: Some(MessageBinding::HttpJson),
        }]
    }

    #[test]
    fn test_valid_create_agent_record_body() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            supported_interfaces: supported_interfaces(),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_name() {
        let body = CreateAgentRecordBody {
            name: String::new(),
            description: "A helpful agent".into(),
            supported_interfaces: supported_interfaces(),
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_unknown_fields() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","supported_interfaces":[{"protocol":"RestHttp"}],"unexpected":true}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_supported_interfaces() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            supported_interfaces: vec![],
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_supported_interfaces() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent"}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_accepts_missing_message_binding() {
        let body = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","supported_interfaces":[{"protocol":"Stdio"}]}"#,
        );

        let body = match body {
            Ok(body) => body,
            Err(error) => panic!("Expected valid create agent record body: {error}"),
        };

        assert!(body.validate().is_ok());
        assert!(body.supported_interfaces[0].message_binding.is_none());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_interface_enum() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","supported_interfaces":[{"protocol":"Unknown"}]}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_description() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","supported_interfaces":[{"protocol":"RestHttp"}]}"#,
        );

        assert!(result.is_err());
    }
}
