#[cfg(test)]
mod agent_records_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::test_fixtures::AgentRecordBuilder;
    use crate::domain::entities::agent_record::{
        AgentArtifactType as DomainAgentArtifactType, AgentInterface as DomainAgentInterface,
        AgentProvider as DomainAgentProvider, AgentRecord as DomainAgentRecord,
        AgentSkill as DomainAgentSkill, ArtifactLocator as DomainArtifactLocator,
        Capabilities as DomainCapabilities, IoMode, LivenessProbeConfiguration, MessageBinding,
        Protocol, ReconstituteAgentRecordProps,
    };
    use crate::presentation::http::v1::responses::agent_records::{
        AgentArtifactType as ResponseAgentArtifactType, AgentRecord,
        MessageBinding as ResponseMessageBinding,
        RestHttpLivenessProbe as ResponseRestHttpLivenessProbe,
    };
    use crate::presentation::http::v1::responses::visibility::Visibility as ResponseVisibility;
    use crate::shared_kernel::enums::Visibility as DomainVisibility;

    #[test]
    fn test_agent_record_entity_to_response() -> Result<(), Box<dyn std::error::Error>> {
        let id = Uuid::now_v7();
        let domain_agent_record = DomainAgentRecord::reconstitute(ReconstituteAgentRecordProps {
            id,
            name: "assistant".into(),
            tenant_id: "tenant-a".into(),
            owner: "owner-a".into(),
            description: "A helpful agent".into(),
            interfaces: vec![
                DomainAgentInterface::new(
                    "rest".into(),
                    Some("REST interface".into()),
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    Some(LivenessProbeConfiguration::RestHttp {
                        route: "/healthcheck".into(),
                        interval_seconds: 30,
                        timeout_seconds: 10,
                        missed_heartbeat_threshold: 3,
                        initial_delay_seconds: 60,
                    }),
                ),
                DomainAgentInterface::new(
                    "rpc".into(),
                    None,
                    Protocol::Rpc,
                    Some(MessageBinding::JsonRpc2_0),
                    None,
                ),
                DomainAgentInterface::new("stdio".into(), None, Protocol::Stdio, None, None),
            ],
            capabilities: DomainCapabilities::new(true, false),
            provider: Some(DomainAgentProvider::new(
                "Example Geo Services Inc.".into(),
                "https://www.examplegeoservices.com".into(),
            )),
            version: "1.2.3".into(),
            artifact_locators: vec![DomainArtifactLocator::new(
                DomainAgentArtifactType::SourceCode,
                "tapis://example-system/path/to/agent-artifact".into(),
            )],
            default_input_modes: vec![IoMode::new("application/json")?],
            default_output_modes: vec![IoMode::new("application/json")?],
            skills: vec![DomainAgentSkill::new(
                "text-analysis".into(),
                "Text analysis".into(),
                "Analyzes text".into(),
                vec!["nlp".into()],
                vec!["Analyze this document".into()],
            )?],
            tags: vec!["tag".into()],
            icon_url: Some("https://example.com/agent-icon.png".into()),
            documentation_url: Some("https://docs.example.com/agents/assistant".into()),
            visibility: DomainVisibility::Public,
        })?;
        let response = AgentRecord::from(domain_agent_record);

        assert_eq!(response.id, id);
        assert_eq!(response.name, "assistant");
        assert_eq!(response.tenant_id, "tenant-a");
        assert_eq!(response.owner, "owner-a");
        assert_eq!(response.description, "A helpful agent");
        assert_eq!(response.version, "1.2.3");
        assert_eq!(response.default_input_modes, vec!["application/json"]);
        assert_eq!(response.default_output_modes, vec!["application/json"]);
        assert!(matches!(response.visibility, ResponseVisibility::Public));
        assert_eq!(
            response.icon_url.as_deref(),
            Some("https://example.com/agent-icon.png")
        );
        assert_eq!(
            response.documentation_url.as_deref(),
            Some("https://docs.example.com/agents/assistant")
        );
        assert_eq!(
            response
                .provider
                .as_ref()
                .map(|provider| provider.organization.as_str()),
            Some("Example Geo Services Inc.")
        );
        assert_eq!(
            response
                .provider
                .as_ref()
                .map(|provider| provider.url.as_str()),
            Some("https://www.examplegeoservices.com")
        );
        assert_eq!(response.rest_http_interfaces.len(), 1);
        assert_eq!(response.rpc_interfaces.len(), 1);
        assert_eq!(response.stdio_interfaces.len(), 1);
        assert_eq!(response.skills.len(), 1);
        assert_eq!(response.skills[0].id, "text-analysis");
        assert_eq!(response.skills[0].name, "Text analysis");
        assert_eq!(response.skills[0].description, "Analyzes text");
        assert_eq!(response.skills[0].tags, vec!["nlp"]);
        assert_eq!(response.skills[0].examples, vec!["Analyze this document"]);
        assert_eq!(response.skills[0].input_modes, None);
        assert_eq!(response.skills[0].output_modes, None);
        assert_eq!(response.tags, vec!["tag"]);
        assert_eq!(response.artifact_locators.len(), 1);
        assert!(matches!(
            response.artifact_locators[0].artifact_type,
            ResponseAgentArtifactType::SourceCode
        ));
        assert_eq!(
            response.artifact_locators[0].url,
            "tapis://example-system/path/to/agent-artifact"
        );
        assert!(response.capabilities.streaming);
        assert!(!response.capabilities.push_notifications);
        assert_eq!(response.rest_http_interfaces[0].name, "rest");
        assert_eq!(
            response.rest_http_interfaces[0].description,
            Some("REST interface".into())
        );
        assert!(matches!(
            response.rest_http_interfaces[0].message_binding,
            Some(ResponseMessageBinding::HttpJson)
        ));
        assert!(matches!(
            response.rest_http_interfaces[0].liveness_probe_config,
            Some(ResponseRestHttpLivenessProbe {
                ref route,
                interval_seconds: 30,
                timeout_seconds: 10,
                missed_heartbeat_threshold: 3,
                initial_delay_seconds: 60,
            }) if route == "/healthcheck"
        ));
        assert!(matches!(
            response.rpc_interfaces[0].message_binding,
            Some(ResponseMessageBinding::JsonRpc2_0)
        ));
        assert_eq!(response.rpc_interfaces[0].name, "rpc");
        assert_eq!(response.stdio_interfaces[0].name, "stdio");
        assert!(response.stdio_interfaces[0].description.is_none());
        assert!(response.stdio_interfaces[0].message_binding.is_none());

        Ok(())
    }

    #[test]
    fn test_agent_record_response_preserves_explicit_skill_io_mode_overrides(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let agent_record = AgentRecordBuilder::new()
            .with_skills(vec![DomainAgentSkill::new_with_io_modes(
                "text-analysis".into(),
                "Text analysis".into(),
                "Analyzes text".into(),
                vec!["nlp".into()],
                vec![],
                Some(vec![IoMode::new("text/plain")?]),
                Some(vec![IoMode::new("application/json")?]),
            )?])
            .build_new()?;

        let response = AgentRecord::from(agent_record);

        assert_eq!(
            response.skills[0].input_modes,
            Some(vec!["text/plain".into()])
        );
        assert_eq!(
            response.skills[0].output_modes,
            Some(vec!["application/json".into()])
        );

        Ok(())
    }
}
