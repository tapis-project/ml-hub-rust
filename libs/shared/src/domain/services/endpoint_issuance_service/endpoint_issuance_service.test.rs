#[cfg(test)]
mod endpoint_issuance_service_test {
    use crate::domain::entities::endpoint::NetworkAddressableResource;
    use crate::domain::entities::tenancy::TenantScopedResource;
    use crate::domain::services::endpoint_issuance_service::{
        EndpointIssuanceService, EndpointIssuanceServiceError,
    };
    use crate::shared_kernel::context::Actor;
    use crate::shared_kernel::identifiers::{traits::UrnGenerator, urn::Urn};

    struct TestResource {
        tenant_id: String,
        target_urls: Vec<String>,
    }

    impl TenantScopedResource for TestResource {
        fn tenant_id(&self) -> String {
            self.tenant_id.clone()
        }
    }

    impl UrnGenerator for TestResource {
        fn urn(&self) -> Urn {
            Urn::new(format!("urn:mlhub:v1:{}:agent:agent-a", self.tenant_id))
        }
    }

    impl NetworkAddressableResource for TestResource {
        fn target_urls(&self) -> Vec<String> {
            self.target_urls.clone()
        }
    }

    #[test]
    fn issues_one_endpoint_per_distinct_target_url() -> Result<(), EndpointIssuanceServiceError> {
        let resource = TestResource {
            tenant_id: "__GLOBAL__".into(),
            target_urls: vec![
                "https://agent.example.test".into(),
                "https://agent.example.test".into(),
                "https://agent-rpc.example.test".into(),
            ],
        };

        let endpoints = EndpointIssuanceService::issue_for_resource(&Actor::system(), &resource)?;

        assert_eq!(endpoints.len(), 2);
        assert_eq!(endpoints[0].tenant_id(), "__GLOBAL__");
        assert!(endpoints
            .iter()
            .any(|endpoint| endpoint.target_url() == "https://agent.example.test"));
        assert!(endpoints
            .iter()
            .any(|endpoint| endpoint.target_url() == "https://agent-rpc.example.test"));

        Ok(())
    }

    #[test]
    fn rejects_resource_from_another_tenant() {
        let resource = TestResource {
            tenant_id: "tenant-a".into(),
            target_urls: vec!["https://agent.example.test".into()],
        };

        let result = EndpointIssuanceService::issue_for_resource(&Actor::system(), &resource);

        assert!(matches!(
            result,
            Err(EndpointIssuanceServiceError::TenantMismatch {
                actor_tenant_id,
                resource_tenant_id,
            }) if actor_tenant_id == "__GLOBAL__" && resource_tenant_id == "tenant-a"
        ));
    }
}
