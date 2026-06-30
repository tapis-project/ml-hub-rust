#[cfg(test)]
mod tapis_jobs_test {
    use super::super::*;
    use std::collections::HashMap;
    use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
    use crate::application::workflows::reconciliation::{
        ReconciliationAction, ReconciliationOutcome,
    };
    use crate::domain::entities::deployment::{
        DesiredState, ModelDeployment, ModelDeploymentMetadata, ModelDeploymentMetadataDelta,
        ModelReference, RehydrateModelDeploymentProps, State,
    };
    use crate::domain::entities::model_metadata::{fixtures::full_model_metadata, ModelMetadata};
    use crate::domain::entities::timestamp::TimeStamp;
    use crate::domain::entities::visibility::Visibility;
    use platforms::Platform;
    use serde_json::json;
    use uuid::Uuid;

    fn ts() -> TimeStamp {
        TimeStamp::now()
    }

    fn deployment_with_metadata(metadata: HashMap<String, serde_json::Value>) -> ModelDeployment {
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: Uuid::now_v7(),
            platform: Platform::TapisJobs,
            tenant_id: "test-tenant".into(),
            owner: "test-owner".into(),
            model: ModelReference {
                name: "Qwen3.5-0.8B".into(),
                author: "Qwen".into(),
                tenant_id: "test".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some("tapis-jobs:default".into()),
            visibility: Visibility::Private,
            deployment_interface: None,
            replicas: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: Some(ModelDeploymentMetadata(metadata)),
        })
    }

    fn deployment_without_metadata() -> ModelDeployment {
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: Uuid::now_v7(),
            platform: Platform::TapisJobs,
            tenant_id: "test-tenant".into(),
            owner: "test-owner".into(),
            model: ModelReference {
                name: "Qwen3.5-0.8B".into(),
                author: "Qwen".into(),
                tenant_id: "test".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: None,
            visibility: Visibility::Private,
            deployment_interface: None,
            replicas: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: None,
        })
    }

    fn minimal_model_metadata() -> ModelMetadata {
        let mut m = full_model_metadata();
        m.name = "Qwen3.5-0.8B".into();
        m.author = "Qwen".into();
        m
    }

    fn base_tapis_meta() -> HashMap<String, serde_json::Value> {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt-token"));
        meta
    }

    // ---- Unit tests: credential extraction ----

    #[test]
    fn extract_tapis_credentials_ok() {
        let deployment = deployment_with_metadata(base_tapis_meta());
        let (url, user, token) =
            TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
                .unwrap();
        assert_eq!(url, "https://tacc.tapis.io");
        assert_eq!(user, "user1");
        assert_eq!(token, "jwt-token");
    }

    #[test]
    fn extract_tapis_credentials_err_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("Deployment metadata is required"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_tenant_url_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("tapis_tenant_url"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_user_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("tapis_user"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_token_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisJobsModelDeploymentReconciliationClient::extract_tapis_credentials(
            &deployment,
        )
        .unwrap_err();
        assert!(err.to_string().contains("tapis_token"));
    }

    // ---- Unit tests: job UUID and HPC metadata ----

    #[test]
    fn extract_job_uuid_some() {
        let mut meta = base_tapis_meta();
        meta.insert("job_uuid".into(), json!("abc-123"));
        let d = deployment_with_metadata(meta);
        assert_eq!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&d).as_deref(),
            Some("abc-123")
        );
    }

    #[test]
    fn extract_hpc_options_ok() {
        let mut meta = base_tapis_meta();
        meta.insert("tapis_hpc_app_id".into(), json!("FlexServ-1.4.0"));
        meta.insert("tapis_hpc_app_version".into(), json!("1.4.0"));
        meta.insert("tapis_hpc_exec_system_id".into(), json!("vista-tapis"));
        meta.insert("tapis_hpc_exec_system_logical_queue".into(), json!("gh"));
        meta.insert("tapis_hpc_max_minutes".into(), json!(60));
        meta.insert("tapis_hpc_allocation".into(), json!("TACC-ACI-CIC"));
        let d = deployment_with_metadata(meta);
        let o = TapisJobsModelDeploymentReconciliationClient::extract_hpc_options(&d).unwrap();
        assert_eq!(o.app_id, "FlexServ-1.4.0");
        assert_eq!(o.exec_system_id, "vista-tapis");
        assert_eq!(o.max_minutes, 60);
    }

    #[test]
    fn extract_job_uuid_none_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        assert!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&deployment).is_none()
        );
    }

    #[test]
    fn extract_job_uuid_none_when_key_missing() {
        let deployment = deployment_with_metadata(base_tapis_meta());
        assert!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&deployment).is_none()
        );
    }

    #[test]
    fn extract_job_uuid_none_when_empty_string() {
        let mut meta = base_tapis_meta();
        meta.insert("job_uuid".into(), json!("  "));
        let deployment = deployment_with_metadata(meta);
        assert!(
            TapisJobsModelDeploymentReconciliationClient::extract_job_uuid(&deployment).is_none()
        );
    }

    // ---- Unit tests: error mapping ----

    #[test]
    fn map_deployment_error_tapis_auth() {
        let e = FlexServDeploymentError::TapisAuthFailed("bad token".into());
        let r = TapisJobsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("TAPIS authentication failed"));
        assert!(r.to_string().contains("bad token"));
    }

    #[test]
    fn map_deployment_error_job_creation() {
        let e = FlexServDeploymentError::JobCreationFailed("queue closed".into());
        let r = TapisJobsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Job creation failed"));
    }

    #[test]
    fn map_deployment_error_unknown() {
        let e = FlexServDeploymentError::UnknownError("something broke".into());
        let r = TapisJobsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Unknown error"));
    }

    // ---- Unit tests: metadata delta ----

    #[test]
    fn result_to_metadata_delta_no_change_when_job_uuid_empty() {
        let r = DeploymentResult::HPCResult {
            job_uuid: String::new(),
            status: None,
            job: None,
            hpc_url: None,
            flexserv_token: None,
        };
        let d = deployment_with_metadata(base_tapis_meta());
        assert!(matches!(
            TapisJobsModelDeploymentReconciliationClient::result_to_metadata_delta(&r, &d),
            ModelDeploymentMetadataDelta::NoChange
        ));
    }

    #[test]
    fn result_to_metadata_delta_merge_when_job_uuid_set() {
        let r = DeploymentResult::HPCResult {
            job_uuid: "550e8400-e29b-41d4-a716-446655440000".into(),
            status: Some("RUNNING".into()),
            job: None,
            hpc_url: Some("https://example.hpc".into()),
            flexserv_token: Some("secret-token".into()),
        };
        let mut meta = base_tapis_meta();
        meta.insert("tapis_tenant".into(), json!("tacc"));
        let d = deployment_with_metadata(meta);
        match TapisJobsModelDeploymentReconciliationClient::result_to_metadata_delta(&r, &d) {
            ModelDeploymentMetadataDelta::Merge(m) => {
                assert_eq!(
                    m.get("job_uuid").and_then(|v| v.as_str()),
                    Some("550e8400-e29b-41d4-a716-446655440000")
                );
                assert_eq!(
                    m.get("model_id").and_then(|v| v.as_str()),
                    Some("Qwen/Qwen3.5-0.8B")
                );
                assert_eq!(
                    m.get("hpc_url").and_then(|v| v.as_str()),
                    Some("https://example.hpc")
                );
                assert_eq!(
                    m.get("flexserv_token").and_then(|v| v.as_str()),
                    Some("secret-token")
                );
            }
            other => panic!("expected Merge, got {:?}", other),
        }
    }

    // ---- Unit tests: state_from_job_status ----

    #[test]
    fn state_from_job_status_running() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("RUNNING"));
        assert_eq!(s, State::Running);
    }

    #[test]
    fn state_from_job_status_finished_stopped() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("FINISHED"));
        assert_eq!(s, State::Stopped);
    }

    #[test]
    fn state_from_job_status_failed() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("FAILED"));
        assert_eq!(s, State::Failed);
    }

    #[test]
    fn state_from_job_status_pending_unknown() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("PENDING"));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_job_status_empty_unknown() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some(""));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_job_status_running_is_case_insensitive() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("rUnNiNg"));
        assert_eq!(s, State::Running);
    }

    #[test]
    fn state_from_job_status_queued_not_failed() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("QUEUED"));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_job_status_queued_case_insensitive() {
        let s = TapisJobsModelDeploymentReconciliationClient::state_from_job_status(Some("Queued"));
        assert_eq!(s, State::Unknown);
    }

    // ---- Async unit tests: reconcile returns error when metadata missing ----

    #[tokio::test]
    async fn reconcile_start_fails_without_metadata() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("metadata"));
    }

    #[tokio::test]
    async fn reconcile_start_fails_without_hpc_options_when_no_job_uuid() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_with_metadata(base_tapis_meta());
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("tapis_hpc_"));
    }

    #[tokio::test]
    async fn reconcile_stop_fails_without_metadata() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Stop,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reconcile_undeploy_fails_without_metadata() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Undeploy,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn reconcile_observe_fails_without_metadata() {
        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    // ---- Integration tests: real Tapis Jobs API ----
    // Run with: cargo test -p shared integration_reconcile_hpc_ -- --ignored --nocapture
    // Submit: TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN + TAPIS_HPC_* (see has_tapis_hpc_submit_env).
    // Observe/stop/undeploy with existing job: TEST_JOB_UUID + TAPIS_*.

    fn has_tapis_credentials() -> bool {
        std::env::var("TAPIS_TENANT_URL").is_ok()
            && std::env::var("TAPIS_USER").is_ok()
            && std::env::var("TAPIS_TOKEN").is_ok()
    }

    fn has_tapis_hpc_submit_env() -> bool {
        has_tapis_credentials()
            && std::env::var("TAPIS_HPC_APP_ID").is_ok()
            && std::env::var("TAPIS_HPC_APP_VERSION").is_ok()
            && std::env::var("TAPIS_HPC_EXEC_SYSTEM_ID").is_ok()
            && std::env::var("TAPIS_HPC_EXEC_SYSTEM_LOGICAL_QUEUE").is_ok()
            && std::env::var("TAPIS_HPC_MAX_MINUTES").is_ok()
            && std::env::var("TAPIS_HPC_ALLOCATION").is_ok()
    }

    fn has_test_job_uuid() -> bool {
        std::env::var("TEST_JOB_UUID")
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
    }

    fn deployment_with_tapis_jobs_meta(
        deployment_id: Uuid,
        tenant_id: &str,
        tenant_url: &str,
        tapis_user: &str,
        tapis_token: &str,
        job_uuid: Option<&str>,
        hpc_submit: Option<(
            String,
            String,
            String,
            String,
            i32,
            String,
        )>,
    ) -> ModelDeployment {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!(tenant_url));
        meta.insert("tapis_user".into(), json!(tapis_user));
        meta.insert("tapis_token".into(), json!(tapis_token));

        if let Some(j) = job_uuid {
            meta.insert("job_uuid".into(), json!(j));
        }

        if let Some((app_id, app_ver, exec_id, queue, max_min, alloc)) = hpc_submit {
            meta.insert("tapis_hpc_app_id".into(), json!(app_id));
            meta.insert("tapis_hpc_app_version".into(), json!(app_ver));
            meta.insert("tapis_hpc_exec_system_id".into(), json!(exec_id));
            meta.insert("tapis_hpc_exec_system_logical_queue".into(), json!(queue));
            meta.insert("tapis_hpc_max_minutes".into(), json!(max_min));
            meta.insert("tapis_hpc_allocation".into(), json!(alloc));
        }
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: deployment_id,
            platform: Platform::TapisJobs,
            tenant_id: tenant_id.to_string(),
            owner: tapis_user.to_string(),
            model: ModelReference {
                name: std::env::var("FLEXSERV_MODEL_NAME")
                    .unwrap_or_else(|_| "Qwen3.5-0.8B".into()),
                author: std::env::var("FLEXSERV_MODEL_AUTHOR").unwrap_or_else(|_| "Qwen".into()),
                tenant_id: "test".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some("tapis-jobs:default".into()),
            visibility: Visibility::Private,
            deployment_interface: None,
            replicas: None,
            revision: 0,
            last_modified: ts(),
            last_state_change: ts(),
            last_desired_state_change: ts(),
            created_at: ts(),
            metadata: Some(ModelDeploymentMetadata(meta)),
        })
    }

    fn assert_and_print_metadata_delta_hpc(
        label: &str,
        metadata_delta: &Option<ModelDeploymentMetadataDelta>,
    ) {
        match metadata_delta {
            Some(ModelDeploymentMetadataDelta::Merge(m)) => {
                let job_uuid = m.get("job_uuid").and_then(|v| v.as_str());
                let model_id = m.get("model_id").and_then(|v| v.as_str());
                let job_info = m.get("job_info").map(|v| {
                    const PREVIEW: usize = 400;
                    let s = v.to_string();
                    let mut out: String = s.chars().take(PREVIEW).collect();
                    if s.chars().count() > PREVIEW {
                        out.push_str(" ...<truncated>");
                    }
                    out
                });
                eprintln!(
                    "{} response: job_uuid={:?} model_id={:?} job_info_preview={:?}",
                    label, job_uuid, model_id, job_info
                );
                assert!(m.get("job_uuid").is_some(), "metadata should have job_uuid");
                assert!(m.get("model_id").is_some(), "metadata should have model_id");
                assert!(m.get("tapis_user").is_some(), "metadata should have tapis_user");
                assert!(
                    m.get("tapis_tenant").is_some() || m.get("tapis_tenant_url").is_some(),
                    "metadata should have tapis_tenant or tapis_tenant_url"
                );
            }
            Some(ModelDeploymentMetadataDelta::Delete) => {
                eprintln!("{} response: metadata marked for deletion", label);
            }
            Some(ModelDeploymentMetadataDelta::NoChange) => {
                eprintln!("{} response: no metadata changes", label);
            }
            None => panic!("Expected metadata delta in outcome"),
        }
    }

    /// First submission: no `job_uuid`; requires full TAPIS_HPC_* env set.
    #[tokio::test]
    #[ignore = "requires TAPIS_* + TAPIS_HPC_* env and real Tapis Jobs API"]
    async fn integration_reconcile_hpc_submit_only() {
        if !has_tapis_hpc_submit_env() {
            eprintln!(
                "Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN and TAPIS_HPC_APP_ID, TAPIS_HPC_APP_VERSION, TAPIS_HPC_EXEC_SYSTEM_ID, TAPIS_HPC_EXEC_SYSTEM_LOGICAL_QUEUE, TAPIS_HPC_MAX_MINUTES, TAPIS_HPC_ALLOCATION"
            );
            return;
        }
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();
        let hpc = (
            std::env::var("TAPIS_HPC_APP_ID").unwrap(),
            std::env::var("TAPIS_HPC_APP_VERSION").unwrap(),
            std::env::var("TAPIS_HPC_EXEC_SYSTEM_ID").unwrap(),
            std::env::var("TAPIS_HPC_EXEC_SYSTEM_LOGICAL_QUEUE").unwrap(),
            std::env::var("TAPIS_HPC_MAX_MINUTES")
                .unwrap()
                .parse::<i32>()
                .expect("TAPIS_HPC_MAX_MINUTES must be i32"),
            std::env::var("TAPIS_HPC_ALLOCATION").unwrap(),
        );

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            "test-tenant",
            &tenant_url,
            &tapis_user,
            &tapis_token,
            None,
            Some(hpc),
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile submit");
        match &outcome {
            ReconciliationOutcome::Started(p) => {
                assert!(p.message.is_some());
                assert_eq!(p.state, State::Unknown);
                assert_and_print_metadata_delta_hpc("submit", &p.metadata);
            }
            other => panic!("expected Started, got {:?}", other),
        }
    }

    /// Observe an existing job. Set `TEST_JOB_UUID` to a valid Tapis job id.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and TEST_JOB_UUID"]
    async fn integration_reconcile_hpc_observe_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        if !has_test_job_uuid() {
            eprintln!("Skipping: set TEST_JOB_UUID to a Tapis Jobs UUID (e.g. from a successful submit)");
            return;
        }
        let job_uuid = std::env::var("TEST_JOB_UUID").unwrap();
        let job_uuid = job_uuid.trim();
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            "test-tenant",
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(job_uuid),
            None,
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile observe");
        match &outcome {
            ReconciliationOutcome::Observed(p) => {
                assert!(p.message.is_some());
                eprintln!("observed state: {:?}", p.state);
                assert_and_print_metadata_delta_hpc("observe", &p.metadata);
            }
            other => panic!("expected Observed, got {:?}", other),
        }
    }

    /// Cancel/stop an existing job (`TEST_JOB_UUID`).
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and TEST_JOB_UUID"]
    async fn integration_reconcile_hpc_stop_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        if !has_test_job_uuid() {
            eprintln!("Skipping: set TEST_JOB_UUID to a Tapis Jobs UUID");
            return;
        }
        let job_uuid = std::env::var("TEST_JOB_UUID").unwrap();
        let job_uuid = job_uuid.trim();
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            "test-tenant",
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(job_uuid),
            None,
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Stop,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile stop");
        match &outcome {
            ReconciliationOutcome::Stopped(p) => {
                assert!(p.message.is_some());
                assert_and_print_metadata_delta_hpc("stop", &p.metadata);
            }
            other => panic!("expected Stopped, got {:?}", other),
        }
    }

    /// Undeploy maps to cancel (same as stop for HPC).
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and TEST_JOB_UUID"]
    async fn integration_reconcile_hpc_undeploy_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        if !has_test_job_uuid() {
            eprintln!("Skipping: set TEST_JOB_UUID to a Tapis Jobs UUID");
            return;
        }
        let job_uuid = std::env::var("TEST_JOB_UUID").unwrap();
        let job_uuid = job_uuid.trim();
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_jobs_meta(
            deployment_id,
            "test-tenant",
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(job_uuid),
            None,
        );

        let client = TapisJobsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Undeploy,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile undeploy");
        match &outcome {
            ReconciliationOutcome::Undeployed(p) => {
                assert!(p.message.is_some());
            }
            other => panic!("expected Undeployed, got {:?}", other),
        }
    }
}
