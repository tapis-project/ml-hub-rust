#[cfg(test)]
mod agent_records_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::{AgentRecord as DomainAgentRecord, ReconstituteAgentRecordProps};
    use crate::presentation::http::v1::responses::agent_records::AgentRecord;

    #[test]
    fn test_agent_record_entity_to_response() {
        let id = Uuid::now_v7();
        let response = AgentRecord::from(DomainAgentRecord::reconstitute(ReconstituteAgentRecordProps {
            id,
            name: "assistant".into(),
            tenant_id: "tenant-a".into(),
            owner: "owner-a".into(),
            description: Some("A helpful agent".into()),
        }));

        assert_eq!(response.id, id);
        assert_eq!(response.name, "assistant");
        assert_eq!(response.tenant_id, "tenant-a");
        assert_eq!(response.owner, "owner-a");
        assert_eq!(response.description, Some("A helpful agent".into()));
    }
}
