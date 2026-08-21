#[cfg(test)]
mod body_test {
    use validator::Validate;

    use crate::presentation::http::v1::requests::create_agent::body::CreateAgentBody;

    #[test]
    fn test_valid_create_agent_body() {
        let body = CreateAgentBody {
            name: "assistant".into(),
            description: Some("A helpful agent".into()),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_body_rejects_empty_name() {
        let body = CreateAgentBody {
            name: String::new(),
            description: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_body_rejects_unknown_fields() {
        let result = serde_json::from_str::<CreateAgentBody>(
            r#"{"name":"assistant","unexpected":true}"#,
        );

        assert!(result.is_err());
    }
}
