#[cfg(test)]
mod agent_records_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::{
        AgentInterface as DomainAgentInterface, AgentProvider as DomainAgentProvider,
        AgentRecord as DomainAgentRecord, Capabilities as DomainCapabilities, MessageBinding,
        Protocol, ReconstituteAgentRecordProps,
    };
    use crate::presentation::http::v1::responses::agent_records::{
        AgentRecord, MessageBinding as ResponseMessageBinding, Protocol as ResponseProtocol,
    };

    #[test]
    fn test_agent_record_entity_to_response(
    ) -> Result<(), crate::domain::entities::agent_record::AgentRecordError> {
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
                ),
                DomainAgentInterface::new("stdio".into(), None, Protocol::Stdio, None),
            ],
            capabilities: DomainCapabilities::new(true, false),
            provider: Some(DomainAgentProvider::new(
                "Example Geo Services Inc.".into(),
                "https://www.examplegeoservices.com".into(),
            )),
            version: "1.2.3".into(),
        })?;
        let response = AgentRecord::from(domain_agent_record);

        assert_eq!(response.id, id);
        assert_eq!(response.name, "assistant");
        assert_eq!(response.tenant_id, "tenant-a");
        assert_eq!(response.owner, "owner-a");
        assert_eq!(response.description, "A helpful agent");
        assert_eq!(response.version, "1.2.3");
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
            response.interfaces[1].protocol,
            ResponseProtocol::Stdio
        ));
        assert_eq!(response.interfaces[1].name, "stdio");
        assert!(response.interfaces[1].description.is_none());
        assert!(response.interfaces[1].message_binding.is_none());

        Ok(())
    }
}
