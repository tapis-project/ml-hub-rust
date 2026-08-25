#[cfg(test)]
mod agent_records_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::{
        AgentArtifactType as DomainAgentArtifactType, AgentInterface as DomainAgentInterface,
        AgentProvider as DomainAgentProvider, AgentRecord as DomainAgentRecord,
        AgentSkill as DomainAgentSkill, ArtifactLocator as DomainArtifactLocator,
        Capabilities as DomainCapabilities, LivenessProbeConfiguration, MessageBinding, Protocol,
        ReconstituteAgentRecordProps,
    };
    use crate::presentation::http::v1::responses::agent_records::{
        AgentArtifactType as ResponseAgentArtifactType, AgentRecord,
        LivenessProbeConfiguration as ResponseLivenessProbeConfiguration,
        MessageBinding as ResponseMessageBinding, Protocol as ResponseProtocol,
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
                        timeout_seconds: 10,
                    }),
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
            skills: vec![DomainAgentSkill::new(
                "text-analysis".into(),
                "Text analysis".into(),
                "Analyzes text".into(),
                vec!["nlp".into()],
                vec!["Analyze this document".into()],
            )?],
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
        assert_eq!(response.interfaces.len(), 2);
        assert_eq!(response.skills.len(), 1);
        assert_eq!(response.skills[0].id, "text-analysis");
        assert_eq!(response.skills[0].name, "Text analysis");
        assert_eq!(response.skills[0].description, "Analyzes text");
        assert_eq!(response.skills[0].tags, vec!["nlp"]);
        assert_eq!(response.skills[0].examples, vec!["Analyze this document"]);
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
        assert_eq!(response.interfaces[0].name, "rest");
        assert_eq!(
            response.interfaces[0].description,
            Some("REST interface".into())
        );
        assert!(matches!(
            response.interfaces[0].protocol,
            ResponseProtocol::RestHttp
        ));
        assert!(matches!(
            response.interfaces[0].message_binding,
            Some(ResponseMessageBinding::HttpJson)
        ));
        assert!(matches!(
            response.interfaces[0].liveness_probe_config,
            Some(ResponseLivenessProbeConfiguration::RestHttp {
                ref route,
                timeout_seconds: 10,
            }) if route == "/healthcheck"
        ));
        assert!(matches!(
            response.interfaces[1].protocol,
            ResponseProtocol::Stdio
        ));
        assert_eq!(response.interfaces[1].name, "stdio");
        assert!(response.interfaces[1].description.is_none());
        assert!(response.interfaces[1].message_binding.is_none());

        Ok(())
    }
}
