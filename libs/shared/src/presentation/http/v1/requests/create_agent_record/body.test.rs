#[cfg(test)]
mod create_agent_record_body_test {
    use validator::Validate;

    use crate::presentation::http::v1::requests::create_agent_record::body::{
        AgentArtifactType, AgentInterface, AgentProvider, AgentSkill, ArtifactLocator,
        Capabilities, CreateAgentRecordBody, MessageBinding, Protocol,
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

    fn artifact_locators() -> Vec<ArtifactLocator> {
        vec![ArtifactLocator {
            artifact_type: AgentArtifactType::SourceCode,
            url: "tapis://example-system/path/to/agent-artifact".into(),
        }]
    }

    fn skills() -> Vec<AgentSkill> {
        vec![AgentSkill {
            id: "text-analysis".into(),
            name: "Text analysis".into(),
            description: "Analyzes text".into(),
            tags: vec!["nlp".into()],
            examples: vec!["Analyze this document".into()],
        }]
    }

    #[test]
    fn test_valid_create_agent_record_body() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0-beta.1+build.42".into(),
            artifact_locators: artifact_locators(),
            skills: skills(),
            icon_url: None,
            documentation_url: None,
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_semver_versions() {
        for version in ["v1.0.0", "1.0", "not-a-version"] {
            let body = CreateAgentRecordBody {
                name: "assistant".into(),
                description: "A helpful agent".into(),
                interfaces: interfaces(),
                capabilities: capabilities(),
                provider: None,
                version: version.into(),
                artifact_locators: artifact_locators(),
                skills: vec![],
                icon_url: None,
                documentation_url: None,
            };

            assert!(body.validate().is_err(), "Expected {version} to be invalid");
        }
    }

    #[test]
    fn test_create_agent_record_body_rejects_empty_name() {
        let body = CreateAgentRecordBody {
            name: String::new(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![],
            icon_url: None,
            documentation_url: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_unknown_fields() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0","unexpected":true}"#,
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
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![],
            icon_url: None,
            documentation_url: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_interfaces() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0"}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_accepts_missing_message_binding() {
        let body = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"stdio","protocol":"Stdio"}],"capabilities":{"streaming":true,"push_notifications":false},"version":"1.0.0"}"#,
        );

        let body = match body {
            Ok(body) => body,
            Err(error) => panic!("Expected valid create agent record body: {error}"),
        };

        assert!(body.validate().is_ok());
        assert_eq!(body.interfaces[0].name, "stdio");
        assert!(body.capabilities.streaming);
        assert!(!body.capabilities.push_notifications);
        assert!(body.provider.is_none());
        assert_eq!(body.version, "1.0.0");
        assert!(body.skills.is_empty());
        assert!(body.icon_url.is_none());
        assert!(body.documentation_url.is_none());
        assert!(body.interfaces[0].description.is_none());
        assert!(body.interfaces[0].message_binding.is_none());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_interface_enum() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"Unknown"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0"}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_description() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0"}"#,
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
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![],
            icon_url: None,
            documentation_url: None,
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
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![],
            icon_url: None,
            documentation_url: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_capabilities() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"version":"1.0.0"}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_requires_version() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false}}"#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_create_agent_record_body_accepts_provider() {
        let body = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false},"provider":{"organization":"Example Geo Services Inc.","url":"https://www.examplegeoservices.com"},"version":"1.0.0"}"#,
        );

        let body = match body {
            Ok(body) => body,
            Err(error) => panic!("Expected valid create agent record body: {error}"),
        };

        assert!(body.validate().is_ok());
        assert_eq!(
            body.provider
                .as_ref()
                .map(|provider| provider.organization.as_str()),
            Some("Example Geo Services Inc.")
        );
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_provider() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: Some(AgentProvider {
                organization: String::new(),
                url: "not-a-url".into(),
            }),
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![],
            icon_url: None,
            documentation_url: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_optional_urls() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![],
            icon_url: Some("not-a-url".into()),
            documentation_url: Some("also-not-a-url".into()),
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_skills() {
        let invalid_identifier = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![AgentSkill {
                id: "Text_Analysis".into(),
                name: "Text analysis".into(),
                description: "Analyzes text".into(),
                tags: vec!["nlp".into()],
                examples: vec![],
            }],
            icon_url: None,
            documentation_url: None,
        };
        assert!(invalid_identifier.validate().is_err());

        let empty_tags = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![AgentSkill {
                id: "text-analysis".into(),
                name: "Text analysis".into(),
                description: "Analyzes text".into(),
                tags: vec![],
                examples: vec![],
            }],
            icon_url: None,
            documentation_url: None,
        };
        assert!(empty_tags.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_duplicate_skill_ids() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![
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
            ],
            icon_url: None,
            documentation_url: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_defaults_omitted_artifact_locators_to_empty() {
        let body = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0"}"#,
        );

        let body = match body {
            Ok(body) => body,
            Err(error) => panic!("Expected artifact locators to default when omitted: {error}"),
        };

        assert!(body.artifact_locators.is_empty());
    }

    #[test]
    fn test_create_agent_record_body_defaults_null_artifact_locators_to_empty() {
        let body = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0","artifact_locators":null}"#,
        );

        let body = match body {
            Ok(body) => body,
            Err(error) => panic!("Expected null artifact locators to default: {error}"),
        };

        assert!(body.artifact_locators.is_empty());
    }

    #[test]
    fn test_create_agent_record_body_accepts_tapis_artifact_locator_url() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: artifact_locators(),
            skills: vec![],
            icon_url: Some("https://example.com/agent-icon.png".into()),
            documentation_url: Some("https://docs.example.com/agents/assistant".into()),
        };

        assert!(body.validate().is_ok());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_artifact_locator_url() {
        let body = CreateAgentRecordBody {
            name: "assistant".into(),
            description: "A helpful agent".into(),
            interfaces: interfaces(),
            capabilities: capabilities(),
            provider: None,
            version: "1.0.0".into(),
            artifact_locators: vec![ArtifactLocator {
                artifact_type: AgentArtifactType::SourceCode,
                url: "not-a-url".into(),
            }],
            skills: vec![],
            icon_url: None,
            documentation_url: None,
        };

        assert!(body.validate().is_err());
    }

    #[test]
    fn test_create_agent_record_body_rejects_invalid_artifact_locator_type() {
        let result = serde_json::from_str::<CreateAgentRecordBody>(
            r#"{"name":"assistant","description":"A helpful agent","interfaces":[{"name":"rest","protocol":"RestHttp"}],"capabilities":{"streaming":false,"push_notifications":false},"version":"1.0.0","artifact_locators":[{"artifact_type":"Unknown","url":"tapis://example-system/path/to/agent-artifact"}]}"#,
        );

        assert!(result.is_err());
    }
}
