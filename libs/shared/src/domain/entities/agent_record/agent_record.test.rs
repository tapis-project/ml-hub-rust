#[cfg(test)]
mod agent_record_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::test_fixtures::AgentRecordBuilder;

    #[test]
    fn test_new_agent_record() {
        let agent_record = AgentRecordBuilder::new()
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_owner("owner-a".into())
            .with_description(Some("A helpful agent".into()))
            .build_new();

        assert_eq!(agent_record.id().get_version_num(), 7);
        assert_eq!(agent_record.name(), "assistant");
        assert_eq!(agent_record.tenant_id(), "tenant-a");
        assert_eq!(agent_record.owner(), "owner-a");
        assert_eq!(agent_record.description(), &Some("A helpful agent".into()));
    }

    #[test]
    fn test_reconstitute_agent_record() {
        let id = Uuid::now_v7();
        let agent_record = AgentRecordBuilder::new()
            .with_id(id)
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_owner("owner-a".into())
            .with_description(None)
            .build_reconstituted();

        assert_eq!(agent_record.id(), &id);
        assert_eq!(agent_record.name(), "assistant");
        assert_eq!(agent_record.tenant_id(), "tenant-a");
        assert_eq!(agent_record.owner(), "owner-a");
        assert_eq!(agent_record.description(), &None);
    }
}
