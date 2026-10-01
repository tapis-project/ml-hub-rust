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
