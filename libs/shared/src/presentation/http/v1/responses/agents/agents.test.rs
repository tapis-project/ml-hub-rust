#[cfg(test)]
mod agents_test {
    use uuid::Uuid;

    use crate::domain::entities::agent::{Agent as DomainAgent, ReconstituteAgentProps};
    use crate::presentation::http::v1::responses::agents::Agent;

    #[test]
    fn test_agent_entity_to_response() {
        let id = Uuid::now_v7();
        let response = Agent::from(DomainAgent::reconstitute(ReconstituteAgentProps {
            id,
            name: "assistant".into(),
            tenant_id: "tenant-a".into(),
            description: Some("A helpful agent".into()),
        }));

        assert_eq!(response.id, id);
        assert_eq!(response.name, "assistant");
        assert_eq!(response.tenant_id, "tenant-a");
        assert_eq!(response.description, Some("A helpful agent".into()));
    }
}
