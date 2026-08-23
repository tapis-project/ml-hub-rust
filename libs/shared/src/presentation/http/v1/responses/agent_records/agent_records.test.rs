#[cfg(test)]
mod agent_records_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::{
        AgentInterface as DomainAgentInterface, AgentRecord as DomainAgentRecord, MessageBinding,
        Protocol, ReconstituteAgentRecordProps,
    };
    use crate::presentation::http::v1::responses::agent_records::{
        AgentRecord, MessageBinding as ResponseMessageBinding, Protocol as ResponseProtocol,
    };

    #[test]
    fn test_agent_record_entity_to_response() {
        let id = Uuid::now_v7();
        let response = AgentRecord::from(
            DomainAgentRecord::reconstitute(ReconstituteAgentRecordProps {
                id,
                name: "assistant".into(),
                tenant_id: "tenant-a".into(),
                owner: "owner-a".into(),
                description: Some("A helpful agent".into()),
                supported_interfaces: vec![
                    DomainAgentInterface::new(Protocol::RestHttp, Some(MessageBinding::HttpJson)),
                    DomainAgentInterface::new(Protocol::Stdio, None),
                ],
            })
            .unwrap(),
        );

        assert_eq!(response.id, id);
        assert_eq!(response.name, "assistant");
        assert_eq!(response.tenant_id, "tenant-a");
        assert_eq!(response.owner, "owner-a");
        assert_eq!(response.description, Some("A helpful agent".into()));
        assert_eq!(response.supported_interfaces.len(), 2);
        assert!(matches!(
            response.supported_interfaces[0].protocol,
            ResponseProtocol::RestHttp
        ));
        assert!(matches!(
            response.supported_interfaces[0].message_binding,
            Some(ResponseMessageBinding::HttpJson)
        ));
        assert!(matches!(
            response.supported_interfaces[1].protocol,
            ResponseProtocol::Stdio
        ));
        assert!(response.supported_interfaces[1].message_binding.is_none());
    }
}
