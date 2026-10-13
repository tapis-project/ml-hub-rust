use super::*;

#[test]
fn resolves_supported_hpc_cluster_hosts() -> Result<(), ReconciliationError> {
    let cases = [
        ("frontera.tacc.utexas.edu", "MLHub-FlexServ-Frontera"),
        ("ls6.tacc.utexas.edu", "MLHub-FlexServ-Lonestar6"),
        ("stampede3.tacc.utexas.edu", "MLHub-FlexServ-Stampede3"),
        ("vista.tacc.utexas.edu", "MLHub-FlexServ-Vista-PKI"),
    ];

    for (cluster_host, expected_system_id) in cases {
        let system_id =
            TapisJobsModelDeploymentReconciliationClient::resolve_tapis_system_id(cluster_host)?;

        assert_eq!(system_id, expected_system_id);
    }

    Ok(())
}

#[test]
fn rejects_unsupported_hpc_cluster_hosts() {
    let result = TapisJobsModelDeploymentReconciliationClient::resolve_tapis_system_id(
        "unknown.example.org",
    );

    assert!(matches!(
        result,
        Err(ReconciliationError::UnsupportedHpcCluster(host))
            if host == "unknown.example.org"
    ));
}

#[test]
fn parses_flexserv_access_information_without_retaining_token() -> Result<(), ReconciliationError> {
    let endpoint = TapisJobsModelDeploymentReconciliationClient::parse_flexserv_access_info(
        "FlexServ address: https://vista.tacc.utexas.edu:60048  FlexServ token: secret-token\n",
    )?;

    assert_eq!(endpoint.name(), Some("default"));
    assert_eq!(endpoint.protocol(), &Protocol::RestHttp);
    assert_eq!(
        endpoint.base_url(),
        Some("https://vista.tacc.utexas.edu:60048")
    );

    Ok(())
}

#[test]
fn rejects_incomplete_or_malformed_flexserv_access_information() {
    let cases = [
        "FlexServ address: https://example.test",
        "FlexServ token: secret-token",
        "FlexServ address:   FlexServ token: secret-token",
        "FlexServ address: not-a-url  FlexServ token: secret-token",
        "FlexServ address: https://example.test  FlexServ token: ",
    ];

    for contents in cases {
        let result =
            TapisJobsModelDeploymentReconciliationClient::parse_flexserv_access_info(contents);

        assert!(result.is_err());
    }
}
