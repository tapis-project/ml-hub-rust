#[cfg(test)]
mod agent_test {
    use uuid::Uuid;

    use crate::domain::entities::agent::test_fixtures::AgentBuilder;

    #[test]
    fn test_new_agent() {
        let agent = AgentBuilder::new()
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_description(Some("A helpful agent".into()))
            .build_new();

        assert_eq!(agent.id().get_version_num(), 7);
        assert_eq!(agent.name(), "assistant");
        assert_eq!(agent.tenant_id(), "tenant-a");
        assert_eq!(agent.description(), &Some("A helpful agent".into()));
    }

    #[test]
    fn test_reconstitute_agent() {
        let id = Uuid::now_v7();
        let agent = AgentBuilder::new()
            .with_id(id)
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_description(None)
            .build_reconstituted();

        assert_eq!(agent.id(), &id);
        assert_eq!(agent.name(), "assistant");
        assert_eq!(agent.tenant_id(), "tenant-a");
        assert_eq!(agent.description(), &None);
    }
}
