#[cfg(test)]
mod endpoint_test {
    use crate::domain::entities::endpoint::{
        Endpoint, NetworkAddressableResource, ReconstituteEndpointProps,
    };
    use crate::domain::entities::tenancy::TenantScopedResource;
    use crate::shared_kernel::identifiers::{traits::UrnGenerator, urn::Urn};

    struct TestResource;

    impl TenantScopedResource for TestResource {
        fn tenant_id(&self) -> String {
            "tenant-a".into()
        }
    }

    impl UrnGenerator for TestResource {
        fn urn(&self) -> Urn {
            Urn::new("urn:mlhub:v1:tenant-a:agent:agent-a".into())
        }
    }

    impl NetworkAddressableResource for TestResource {
        fn target_urls(&self) -> Vec<String> {
            vec!["https://agent.example.test".into()]
        }
    }

    #[test]
    fn creates_endpoint_from_resource() {
        let endpoint =
            Endpoint::new_from_resource(&TestResource, "https://agent.example.test".into());

        assert_eq!(endpoint.id().get_version_num(), 7);
        assert_eq!(endpoint.tenant_id(), "tenant-a");
        assert_eq!(
            endpoint.target_resource_urn().as_str(),
            "urn:mlhub:v1:tenant-a:agent:agent-a"
        );
        assert_eq!(endpoint.target_url(), "https://agent.example.test");
        assert_eq!(endpoint.slug().len(), 10);
        assert!(endpoint
            .slug()
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit()));
    }

    #[test]
    fn reconstitutes_endpoint() {
        let id = uuid::Uuid::now_v7();
        let endpoint = Endpoint::reconstitute(ReconstituteEndpointProps {
            id,
            tenant_id: "tenant-a".into(),
            target_resource_urn: Urn::new("urn:mlhub:v1:tenant-a:agent:agent-a".into()),
            target_url: "https://agent.example.test".into(),
            slug: "abc123def4".into(),
        });

        assert_eq!(endpoint.id(), &id);
        assert_eq!(endpoint.tenant_id(), "tenant-a");
        assert_eq!(
            endpoint.target_resource_urn().as_str(),
            "urn:mlhub:v1:tenant-a:agent:agent-a"
        );
        assert_eq!(endpoint.target_url(), "https://agent.example.test");
        assert_eq!(endpoint.slug(), "abc123def4");
    }
}
