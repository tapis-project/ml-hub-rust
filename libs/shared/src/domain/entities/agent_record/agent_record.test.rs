#[cfg(test)]
mod agent_record_test {
    use uuid::Uuid;

    use crate::domain::entities::agent_record::{
        test_fixtures::AgentRecordBuilder, AgentInterface, AgentRecordError, MessageBinding,
        Protocol,
    };

    #[test]
    fn test_new_agent_record() -> Result<(), AgentRecordError> {
        let agent_record = AgentRecordBuilder::new()
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_owner("owner-a".into())
            .with_description(Some("A helpful agent".into()))
            .build_new()?;

        assert_eq!(agent_record.id().get_version_num(), 7);
        assert_eq!(agent_record.name(), "assistant");
        assert_eq!(agent_record.tenant_id(), "tenant-a");
        assert_eq!(agent_record.owner(), "owner-a");
        assert_eq!(agent_record.description(), &Some("A helpful agent".into()));
        assert!(matches!(
            agent_record.supported_interfaces().first().protocol(),
            Protocol::RestHttp
        ));
        assert!(matches!(
            agent_record
                .supported_interfaces()
                .first()
                .message_binding(),
            Some(MessageBinding::HttpJson)
        ));

        Ok(())
    }

    #[test]
    fn test_reconstitute_agent_record() -> Result<(), AgentRecordError> {
        let id = Uuid::now_v7();
        let agent_record = AgentRecordBuilder::new()
            .with_id(id)
            .with_name("assistant".into())
            .with_tenant_id("tenant-a".into())
            .with_owner("owner-a".into())
            .with_description(None)
            .with_supported_interfaces(vec![AgentInterface::new(Protocol::Stdio, None)])
            .build_reconstituted()?;

        assert_eq!(agent_record.id(), &id);
        assert_eq!(agent_record.name(), "assistant");
        assert_eq!(agent_record.tenant_id(), "tenant-a");
        assert_eq!(agent_record.owner(), "owner-a");
        assert_eq!(agent_record.description(), &None);
        assert!(matches!(
            agent_record.supported_interfaces().first().protocol(),
            Protocol::Stdio
        ));
        assert!(agent_record
            .supported_interfaces()
            .first()
            .message_binding()
            .is_none());

        Ok(())
    }

    #[test]
    fn test_new_agent_record_requires_supported_interface() {
        let result = AgentRecordBuilder::new()
            .with_supported_interfaces(vec![])
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
            .with_supported_interfaces(vec![])
            .build_reconstituted();

        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("Expected agent record reconstitution to reject empty interfaces"),
        };

        assert!(matches!(error, AgentRecordError::DataIntegrityError(..)));
    }
}
