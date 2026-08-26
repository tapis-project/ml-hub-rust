#[cfg(test)]
mod agent_record_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::{
        AgentArtifactType, AgentInterface, AgentProvider, AgentRecordError, AgentSkill,
        AgentSkillError, ArtifactLocator, Capabilities, LivenessProbeConfiguration, MessageBinding,
        Protocol, ReconstituteAgentSkillProps, test_fixtures::AgentRecordBuilder,
    };
    use crate::shared_kernel::enums::Visibility;
    use crate::shared_kernel::identifiers::traits::UrnGenerator;

    #[test]
    fn test_new_agent_record() -> Result<(), AgentRecordError> {
        let agent_record = AgentRecordBuilder::new()
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_owner("owner-a".into())
            .with_description("A helpful agent".into())
            .build_new()?;

        assert_eq!(agent_record.id().get_version_num(), 7);
        assert_eq!(agent_record.name(), "assistant");
        assert_eq!(agent_record.tenant_id(), "tenant-a");
        assert_eq!(agent_record.owner(), "owner-a");
        assert_eq!(agent_record.description(), "A helpful agent");
        assert_eq!(agent_record.version(), "0.1.0");
        assert_eq!(agent_record.provider_organization(), None);
        assert_eq!(agent_record.provider_url(), None);
        assert!(!agent_record.supports_streaming());
        assert!(!agent_record.supports_push_notifications());
        assert!(agent_record.artifact_locators().is_empty());
        assert!(agent_record.skills().is_empty());
        assert!(agent_record.tags().is_empty());
        assert_eq!(agent_record.icon_url(), None);
        assert_eq!(agent_record.documentation_url(), None);
        assert!(!agent_record.is_public());
        assert_eq!(agent_record.interfaces().first().name(), "default");
        assert_eq!(
            agent_record.interfaces().first().description(),
            &Some("Default test interface".into())
        );
        assert!(matches!(
            agent_record.interfaces().first().protocol(),
            Protocol::RestHttp
        ));
        assert!(matches!(
            agent_record.interfaces().first().message_binding(),
            Some(MessageBinding::HttpJson)
        ));

        Ok(())
    }

    #[test]
    fn test_agent_record_generates_urn() -> Result<(), AgentRecordError> {
        let agent_record = AgentRecordBuilder::new()
            .with_tenant_id("tenant-a".into())
            .build_new()?;

        assert_eq!(
            agent_record.urn().as_str(),
            format!("urn:mlhub:v1:tenant-a:agent_record:{}", agent_record.id())
        );

        Ok(())
    }

    #[test]
    fn test_reconstitute_agent_record() -> Result<(), Box<dyn std::error::Error>> {
        let id = Uuid::now_v7();
        let agent_record = AgentRecordBuilder::new()
            .with_id(id)
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_owner("owner-a".into())
            .with_description("A helpful agent".into())
            .with_interfaces(vec![AgentInterface::new(
                "stdio".into(),
                None,
                Protocol::Stdio,
                None,
                None,
            )])
            .with_capabilities(Capabilities::new(true, true))
            .with_provider(AgentProvider::new(
                "Example Geo Services Inc.".into(),
                "https://www.examplegeoservices.com".into(),
            ))
            .with_version("1.2.3-rc.1+build.42".into())
            .with_artifact_locators(vec![ArtifactLocator::new(
                AgentArtifactType::DockerImage,
                "registry.example.com/agents/assistant:1.2.3".into(),
            )])
            .with_skills(vec![AgentSkill::new(
                "text-analysis".into(),
                "Text analysis".into(),
                "Analyzes text".into(),
                vec!["nlp".into()],
                vec!["Analyze this document".into()],
            )?])
            .with_icon_url("https://example.com/agent-icon.png".into())
            .with_documentation_url("https://docs.example.com/agents/assistant".into())
            .with_visibility(Visibility::Public)
            .build_reconstituted()?;

        assert_eq!(agent_record.id(), &id);
        assert_eq!(agent_record.name(), "assistant");
        assert_eq!(agent_record.tenant_id(), "tenant-a");
        assert_eq!(agent_record.owner(), "owner-a");
        assert_eq!(agent_record.description(), "A helpful agent");
        assert_eq!(agent_record.version(), "1.2.3-rc.1+build.42");
        assert_eq!(
            agent_record.provider_organization(),
            Some("Example Geo Services Inc.")
        );
        assert_eq!(
            agent_record.provider_url(),
            Some("https://www.examplegeoservices.com")
        );
        assert!(agent_record.supports_streaming());
        assert!(agent_record.supports_push_notifications());
        assert!(agent_record.is_public());
        assert_eq!(agent_record.artifact_locators().len(), 1);
        assert_eq!(agent_record.skills().len(), 1);
        assert_eq!(agent_record.skills()[0].id(), "text-analysis");
        assert_eq!(agent_record.skills()[0].tags().first(), "nlp");
        assert!(matches!(
            agent_record.artifact_locators()[0].artifact_type(),
            AgentArtifactType::DockerImage
        ));
        assert_eq!(
            agent_record.artifact_locators()[0].url(),
            "registry.example.com/agents/assistant:1.2.3"
        );
        assert_eq!(
            agent_record.icon_url(),
            Some("https://example.com/agent-icon.png")
        );
        assert_eq!(
            agent_record.documentation_url(),
            Some("https://docs.example.com/agents/assistant")
        );
        assert_eq!(agent_record.interfaces().first().name(), "stdio");
        assert!(matches!(
            agent_record.interfaces().first().protocol(),
            Protocol::Stdio
        ));
        assert!(
            agent_record
                .interfaces()
                .first()
                .message_binding()
                .is_none()
        );

        Ok(())
    }

    #[test]
    fn test_new_agent_record_requires_supported_interface() {
        let result = AgentRecordBuilder::new()
            .with_interfaces(vec![])
            .build_new();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected agent record construction to reject empty interfaces"),
        };

        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));
    }

    #[test]
    fn test_reconstitute_agent_record_requires_supported_interface() {
        let result = AgentRecordBuilder::new()
            .with_interfaces(vec![])
            .build_reconstituted();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected agent record reconstitution to reject empty interfaces"),
        };

        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));
    }

    #[test]
    fn test_new_agent_record_rejects_duplicate_interface_names() {
        let result = AgentRecordBuilder::new()
            .with_interfaces(vec![
                AgentInterface::new("rest".into(), None, Protocol::RestHttp, None, None),
                AgentInterface::new("rest".into(), None, Protocol::Stdio, None, None),
            ])
            .build_new();

        let error = match result {
            Err(error) => error,
            Ok(_) => {
                panic!("Expected agent record construction to reject duplicate interface names")
            }
        };

        assert!(matches!(
            error,
            AgentRecordError::DuplicateAgentInterfaceIdentifier(identifier) if identifier == "rest"
        ));
    }

    #[test]
    fn test_reconstitute_agent_record_rejects_duplicate_interface_names() {
        let result = AgentRecordBuilder::new()
            .with_interfaces(vec![
                AgentInterface::new("rest".into(), None, Protocol::RestHttp, None, None),
                AgentInterface::new("rest".into(), None, Protocol::Stdio, None, None),
            ])
            .build_reconstituted();

        let error = match result {
            Err(error) => error,
            Ok(_) => {
                panic!("Expected agent record reconstitution to reject duplicate interface names")
            }
        };

        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));
    }

    #[test]
    fn test_agent_skill_creation() -> Result<(), AgentSkillError> {
        let skill = AgentSkill::new(
            "text-analysis-v2".into(),
            "Text analysis".into(),
            "Analyzes text".into(),
            vec!["nlp".into()],
            vec!["Analyze this document".into()],
        )?;

        assert_eq!(skill.id(), "text-analysis-v2");
        assert_eq!(skill.name(), "Text analysis");
        assert_eq!(skill.description(), "Analyzes text");
        assert_eq!(skill.tags().first(), "nlp");
        assert_eq!(skill.examples(), ["Analyze this document"]);

        Ok(())
    }

    #[test]
    fn test_agent_skill_rejects_invalid_identifier_and_empty_tags() {
        let invalid_identifier = AgentSkill::new(
            "Text_Analysis".into(),
            "Text analysis".into(),
            "Analyzes text".into(),
            vec!["nlp".into()],
            vec![],
        );
        let invalid_identifier = match invalid_identifier {
            Err(error) => error,
            Ok(_) => panic!("Expected invalid skill identifier to be rejected"),
        };
        assert!(matches!(
            invalid_identifier,
            AgentSkillError::InvalidIdentifier(identifier) if identifier == "Text_Analysis"
        ));

        let empty_tags = AgentSkill::new(
            "text-analysis".into(),
            "Text analysis".into(),
            "Analyzes text".into(),
            vec![],
            vec![],
        );
        let empty_tags = match empty_tags {
            Err(error) => error,
            Ok(_) => panic!("Expected empty skill tags to be rejected"),
        };
        assert!(matches!(empty_tags, AgentSkillError::EmptyTags));
    }

    #[test]
    fn test_reconstitute_agent_skill_rejects_invalid_data() {
        let result = AgentSkill::reconstitute(ReconstituteAgentSkillProps {
            id: "Text_Analysis".into(),
            name: "Text analysis".into(),
            description: "Analyzes text".into(),
            tags: vec!["nlp".into()],
            examples: vec![],
        });

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected invalid persisted skill data to be rejected"),
        };
        assert!(matches!(error, AgentSkillError::DataIntegrityError(..)));
    }

    #[test]
    fn test_new_agent_record_rejects_duplicate_skill_ids() -> Result<(), Box<dyn std::error::Error>>
    {
        let result = AgentRecordBuilder::new()
            .with_skills(vec![
                AgentSkill::new(
                    "text-analysis".into(),
                    "Text analysis".into(),
                    "Analyzes text".into(),
                    vec!["nlp".into()],
                    vec![],
                )?,
                AgentSkill::new(
                    "text-analysis".into(),
                    "Other analysis".into(),
                    "Analyzes other text".into(),
                    vec!["nlp".into()],
                    vec![],
                )?,
            ])
            .build_new();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected duplicate skill IDs to be rejected"),
        };
        assert!(matches!(
            error,
            AgentRecordError::DuplicateAgentSkillIdentifier(identifier) if identifier == "text-analysis"
        ));

        Ok(())
    }

    #[test]
    fn test_reconstitute_agent_record_rejects_duplicate_skill_ids()
    -> Result<(), Box<dyn std::error::Error>> {
        let result = AgentRecordBuilder::new()
            .with_skills(vec![
                AgentSkill::new(
                    "text-analysis".into(),
                    "Text analysis".into(),
                    "Analyzes text".into(),
                    vec!["nlp".into()],
                    vec![],
                )?,
                AgentSkill::new(
                    "text-analysis".into(),
                    "Other analysis".into(),
                    "Analyzes other text".into(),
                    vec!["nlp".into()],
                    vec![],
                )?,
            ])
            .build_reconstituted();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected duplicate persisted skill IDs to be rejected"),
        };
        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));

        Ok(())
    }

    #[test]
    fn test_new_agent_record_rejects_invalid_semver_version() {
        let result = AgentRecordBuilder::new()
            .with_version("v1.2.3".into())
            .build_new();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected agent record construction to reject invalid SemVer"),
        };

        assert!(matches!(
            error,
            AgentRecordError::InvalidVersion(version) if version == "v1.2.3"
        ));
    }

    #[test]
    fn test_reconstitute_agent_record_rejects_invalid_semver_version() {
        let result = AgentRecordBuilder::new()
            .with_version("1.2".into())
            .build_reconstituted();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected agent record reconstitution to reject invalid SemVer"),
        };

        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));
    }

    #[test]
    fn test_new_agent_record_rejects_invalid_tags() {
        let result = AgentRecordBuilder::new()
            .with_tags(vec![String::new()])
            .build_new();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected agent record construction to reject invalid tags"),
        };

        assert!(matches!(error, AgentRecordError::InvalidTags(_)));
    }

    #[test]
    fn test_reconstitute_agent_record_rejects_invalid_tags() {
        let result = AgentRecordBuilder::new()
            .with_tags(vec![String::new()])
            .build_reconstituted();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected agent record reconstitution to reject invalid tags"),
        };

        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));
    }

    #[test]
    fn test_new_agent_record_accepts_rest_http_liveness_probe() -> Result<(), AgentRecordError> {
        let agent_record = AgentRecordBuilder::new()
            .with_interfaces(vec![AgentInterface::new(
                "rest".into(),
                None,
                Protocol::RestHttp,
                None,
                Some(LivenessProbeConfiguration::RestHttp {
                    route: "/healthcheck".into(),
                    interval_seconds: 30,
                    timeout_seconds: 10,
                    missed_heartbeat_threshold: 3,
                    initial_delay_seconds: 60,
                }),
            )])
            .build_new()?;

        assert!(matches!(
            agent_record.interfaces().first().liveness_probe_config(),
            Some(LivenessProbeConfiguration::RestHttp {
                route,
                interval_seconds: 30,
                timeout_seconds: 10,
                missed_heartbeat_threshold: 3,
                initial_delay_seconds: 60,
            }) if route == "/healthcheck"
        ));

        Ok(())
    }

    #[test]
    fn test_new_agent_record_rejects_incompatible_liveness_probe() {
        let result = AgentRecordBuilder::new()
            .with_interfaces(vec![AgentInterface::new(
                "rpc".into(),
                None,
                Protocol::Rpc,
                None,
                Some(LivenessProbeConfiguration::RestHttp {
                    route: "/healthcheck".into(),
                    interval_seconds: 30,
                    timeout_seconds: 10,
                    missed_heartbeat_threshold: 3,
                    initial_delay_seconds: 60,
                }),
            )])
            .build_new();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected incompatible liveness probe to be rejected"),
        };

        assert!(matches!(
            error,
            AgentRecordError::IncompatibleLivenessProbeConfiguration(interface_name)
                if interface_name == "rpc"
        ));
    }

    #[test]
    fn test_reconstitute_agent_record_rejects_incompatible_liveness_probe() {
        let result = AgentRecordBuilder::new()
            .with_interfaces(vec![AgentInterface::new(
                "stdio".into(),
                None,
                Protocol::Stdio,
                None,
                Some(LivenessProbeConfiguration::RestHttp {
                    route: "/healthcheck".into(),
                    interval_seconds: 30,
                    timeout_seconds: 10,
                    missed_heartbeat_threshold: 3,
                    initial_delay_seconds: 60,
                }),
            )])
            .build_reconstituted();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected incompatible persisted liveness probe to be rejected"),
        };

        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));
    }
}
