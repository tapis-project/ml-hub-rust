#[cfg(test)]
mod agent_test {
    use crate::domain::entities::agent::test_fixtures::AgentBuilder;
    use crate::domain::entities::agent::{Agent, AgentEndpoint, AgentError, RegisterAgentProps};
    use crate::domain::entities::agent_record::test_fixtures::AgentRecordBuilder;
    use crate::domain::entities::agent_record::{MessageBinding, Protocol};
    use crate::domain::entities::endpoint::NetworkAddressableResource;
    use crate::shared_kernel::enums::Visibility;

    #[test]
    fn register_agent_generates_v7_id_and_starts_dead() -> Result<(), AgentError> {
        let agent = AgentBuilder::new().build_registered()?;
        assert_eq!(agent.id().get_version_num(), 7);
        assert!(matches!(
            agent.liveness(),
            crate::domain::entities::agent::AgentLiveness::Dead
        ));
        assert_eq!(agent.target_endpoints().len(), 1);
        assert!(agent.last_missed_heartbeat().is_none());
        assert_eq!(agent.consecutive_missed_heartbeats(), 0);
        Ok(())
    }

    #[test]
    fn network_addressable_endpoint_requires_a_name() {
        let result = Agent::register(
            RegisterAgentProps {
                name: "Agent".into(),
                description: "Description".into(),
                owner: "owner".into(),
                tenant_id: "tenant".into(),
                deployment_modality:
                    crate::domain::entities::agent::AgentDeploymentModality::Persistent,
                endpoints: vec![AgentEndpoint::new(
                    None,
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    Some("https://example.test".into()),
                    None,
                )],
                tags: vec![],
                visibility: Visibility::Private,
            },
            None,
        );

        assert!(matches!(
            result,
            Err(AgentError::MissingNetworkAddressableEndpointIdentifier)
        ));
    }

    #[test]
    fn resolves_network_target_url_by_name() -> Result<(), AgentError> {
        let agent = AgentBuilder::new().build_registered()?;

        assert_eq!(agent.network_target_names(), vec!["default"]);
        assert_eq!(
            agent.resolve_target_url("default"),
            Some("https://example.test")
        );
        assert!(agent.resolve_target_url("missing").is_none());

        Ok(())
    }

    #[test]
    fn reconstitute_agent_preserves_missed_heartbeat_state() -> Result<(), AgentError> {
        let last_missed_heartbeat = crate::shared_kernel::value_objects::TimeStamp::parse_string(
            "2026-08-26T12:00:00Z",
        )
        .map_err(|error| AgentError::DataIntegrityError(error.to_string()))?;
        let agent = AgentBuilder::new()
            .with_last_missed_heartbeat(last_missed_heartbeat.clone())
            .with_consecutive_missed_heartbeats(3)
            .build_reconstituted()?;

        assert_eq!(agent.last_missed_heartbeat(), Some(&last_missed_heartbeat));
        assert_eq!(agent.consecutive_missed_heartbeats(), 3);
        Ok(())
    }

    #[test]
    fn register_agent_validates_associated_agent_record() -> Result<(), Box<dyn std::error::Error>>
    {
        let record = AgentRecordBuilder::new().build_new()?;
        let agent = Agent::register(
            RegisterAgentProps {
                name: "Agent".into(),
                description: "Description".into(),
                owner: "owner".into(),
                tenant_id: "tenant".into(),
                deployment_modality:
                    crate::domain::entities::agent::AgentDeploymentModality::Persistent,
                endpoints: vec![AgentEndpoint::new(
                    Some("default".into()),
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    Some("https://example.test".into()),
                    None,
                )],
                tags: vec![],
                visibility: Visibility::Private,
            },
            Some(&record),
        )?;
        assert_eq!(agent.agent_record_id(), Some(record.id()));
        Ok(())
    }

    #[test]
    fn register_agent_rejects_mismatched_agent_record_interface(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let record = AgentRecordBuilder::new().build_new()?;
        let result = Agent::register(
            RegisterAgentProps {
                name: "Agent".into(),
                description: "Description".into(),
                owner: "owner".into(),
                tenant_id: "tenant".into(),
                deployment_modality:
                    crate::domain::entities::agent::AgentDeploymentModality::Persistent,
                endpoints: vec![AgentEndpoint::new(
                    Some("default".into()),
                    Protocol::Rpc,
                    Some(MessageBinding::HttpJson),
                    None,
                    None,
                )],
                tags: vec![],
                visibility: Visibility::Private,
            },
            Some(&record),
        );
        assert!(matches!(
            result,
            Err(AgentError::MismatchedAgentInterfaceDetails(_))
        ));
        Ok(())
    }

    #[test]
    fn register_agent_inherits_agent_record_tags() -> Result<(), Box<dyn std::error::Error>> {
        let record = AgentRecordBuilder::new()
            .with_tags(vec!["inherited".into()])
            .build_new()?;
        let agent = Agent::register(
            RegisterAgentProps {
                name: "Agent".into(),
                description: "Description".into(),
                owner: "owner".into(),
                tenant_id: "tenant".into(),
                deployment_modality:
                    crate::domain::entities::agent::AgentDeploymentModality::Persistent,
                endpoints: vec![AgentEndpoint::new(
                    Some("default".into()),
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    Some("https://example.test".into()),
                    None,
                )],
                tags: vec![],
                visibility: Visibility::Private,
            },
            Some(&record),
        )?;

        assert_eq!(
            agent.tags().iter().next().map(|tag| tag.as_str()),
            Some("inherited")
        );
        Ok(())
    }

    #[test]
    fn register_agent_prefers_explicit_tags_over_agent_record_tags(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let record = AgentRecordBuilder::new()
            .with_tags(vec!["inherited".into()])
            .build_new()?;
        let agent = Agent::register(
            RegisterAgentProps {
                name: "Agent".into(),
                description: "Description".into(),
                owner: "owner".into(),
                tenant_id: "tenant".into(),
                deployment_modality:
                    crate::domain::entities::agent::AgentDeploymentModality::Persistent,
                endpoints: vec![AgentEndpoint::new(
                    Some("default".into()),
                    Protocol::RestHttp,
                    Some(MessageBinding::HttpJson),
                    Some("https://example.test".into()),
                    None,
                )],
                tags: vec!["explicit".into()],
                visibility: Visibility::Private,
            },
            Some(&record),
        )?;

        assert_eq!(
            agent.tags().iter().next().map(|tag| tag.as_str()),
            Some("explicit")
        );
        Ok(())
    }
}
