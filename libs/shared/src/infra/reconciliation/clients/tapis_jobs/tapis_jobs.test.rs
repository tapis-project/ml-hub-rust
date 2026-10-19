use super::*;

#[test]
fn classifies_retryable_tapis_status_codes() {
    for status in [408, 425, 429, 500, 503, 599] {
        assert!(TapisJobsModelDeploymentReconciliationClient::is_retryable_status(status));
    }

    for status in [400, 401, 403, 404, 409, 422] {
        assert!(!TapisJobsModelDeploymentReconciliationClient::is_retryable_status(status));
    }
}

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

#[test]
fn sanitizes_sensitive_values_in_json_response_bodies() {
    let response = r#"{
        "message": "submission failed",
        "access_token": "secret-token",
        "details": {
            "password": "secret-password",
            "reason": "invalid queue"
        },
        "environment_variable": {
            "name": "HF_TOKEN",
            "value": "secret-environment-value"
        }
    }"#;

    let sanitized = TapisJobsModelDeploymentReconciliationClient::sanitized_response_body(response);

    assert!(sanitized.contains("submission failed"));
    assert!(sanitized.contains("invalid queue"));
    assert!(!sanitized.contains("secret-token"));
    assert!(!sanitized.contains("secret-password"));
    assert!(!sanitized.contains("secret-environment-value"));
    assert!(sanitized.contains("[REDACTED]"));
}

#[test]
fn omits_non_json_response_bodies_from_diagnostic_logs() {
    let response = "upstream included an opaque error response";

    let sanitized = TapisJobsModelDeploymentReconciliationClient::sanitized_response_body(response);

    assert_eq!(
        sanitized,
        format!(
            "<non-JSON response body omitted; byte_length={}>",
            response.len()
        )
    );
}

#[test]
fn maps_successful_batch_completion_without_terminal_failure() {
    for status in ["FINISHED", "COMPLETED"] {
        let (state, outcome, disposition) =
            TapisJobsModelDeploymentReconciliationClient::observation_from_job_status(
                &DeploymentModality::Batch,
                &DesiredState::Running,
                status,
            );

        assert_eq!(state, State::Stopped);
        assert_eq!(outcome, Some(BatchExecutionOutcome::Succeeded));
        assert_eq!(disposition, None);
    }
}

#[test]
fn maps_cancelled_batch_execution_to_terminal_failure_while_running_is_desired() {
    for status in ["CANCELLED", "CANCELED", "STOPPED"] {
        let (state, outcome, disposition) =
            TapisJobsModelDeploymentReconciliationClient::observation_from_job_status(
                &DeploymentModality::Batch,
                &DesiredState::Running,
                status,
            );

        assert_eq!(state, State::Stopped);
        assert_eq!(outcome, Some(BatchExecutionOutcome::Cancelled));
        assert_eq!(disposition, Some(FailureDisposition::Terminal));
    }
}

#[test]
fn maps_failed_batch_execution_to_terminal_failure_while_running_is_desired() {
    let (state, outcome, disposition) =
        TapisJobsModelDeploymentReconciliationClient::observation_from_job_status(
            &DeploymentModality::Batch,
            &DesiredState::Running,
            "FAILED",
        );

    assert_eq!(state, State::Stopped);
    assert_eq!(outcome, Some(BatchExecutionOutcome::Failed));
    assert_eq!(disposition, Some(FailureDisposition::Terminal));
}

#[test]
fn maps_expected_batch_termination_without_terminal_failure() {
    let (state, outcome, disposition) =
        TapisJobsModelDeploymentReconciliationClient::observation_from_job_status(
            &DeploymentModality::Batch,
            &DesiredState::Stopped,
            "CANCELLED",
        );

    assert_eq!(state, State::Stopped);
    assert_eq!(outcome, Some(BatchExecutionOutcome::Cancelled));
    assert_eq!(disposition, None);
}

#[test]
fn maps_service_completion_to_terminal_failure_while_running_is_desired() {
    let (state, outcome, disposition) =
        TapisJobsModelDeploymentReconciliationClient::observation_from_job_status(
            &DeploymentModality::Service,
            &DesiredState::Running,
            "FINISHED",
        );

    assert_eq!(state, State::Failed);
    assert_eq!(outcome, None);
    assert_eq!(disposition, Some(FailureDisposition::Terminal));
}

#[test]
fn only_terminal_tapis_statuses_are_eligible_for_resubmission() {
    for status in [
        "FINISHED",
        "COMPLETED",
        "CANCELLED",
        "CANCELED",
        "STOPPED",
        "FAILED",
    ] {
        assert!(TapisJobsModelDeploymentReconciliationClient::is_terminal_job_status(status));
    }

    for status in ["PENDING", "QUEUED", "RUNNING", "BLOCKED", "PAUSED"] {
        assert!(!TapisJobsModelDeploymentReconciliationClient::is_terminal_job_status(status));
    }
}
