#[cfg(test)]
mod urn_test {
    use super::super::{NetworkResourceReference, Urn, UrnParseError};

    #[test]
    fn parses_supported_network_resource_urns() -> Result<(), UrnParseError> {
        let id = uuid::Uuid::now_v7();
        let agent = Urn::new(format!("urn:mlhub:v1:tenant-a:agent:{id}"));
        let deployment = Urn::new(format!("urn:mlhub:v1:tenant-a:deployment:{id}"));

        assert_eq!(
            agent.parse_network_resource()?,
            NetworkResourceReference::Agent {
                tenant_id: "tenant-a".into(),
                id,
            }
        );
        assert_eq!(
            deployment.parse_network_resource()?,
            NetworkResourceReference::ModelDeployment {
                tenant_id: "tenant-a".into(),
                id,
            }
        );

        Ok(())
    }

    #[test]
    fn rejects_malformed_and_unsupported_urns() {
        let id = uuid::Uuid::now_v7();

        assert!(matches!(
            Urn::new("not-a-urn".into()).parse_network_resource(),
            Err(UrnParseError::Malformed(_))
        ));
        assert!(matches!(
            Urn::new(format!("urn:mlhub:v1:tenant-a:model:{id}")).parse_network_resource(),
            Err(UrnParseError::UnsupportedResourceType(_))
        ));
    }
}
