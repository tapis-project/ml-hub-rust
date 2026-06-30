#[cfg(test)]
mod tapis_pods_test {
    use super::super::*;
    use std::collections::HashMap;
    use crate::domain::entities::deployment::{
        ModelDeployment, ModelDeploymentMetadata, ModelReference, State, DesiredState, RehydrateModelDeploymentProps,
    };
    use crate::domain::entities::visibility::Visibility;
    use crate::domain::entities::model_metadata::{ModelMetadata, fixtures::full_model_metadata};
    use crate::domain::entities::timestamp::TimeStamp;
    use platforms::Platform;
    use uuid::Uuid;
    use serde_json::json;

    fn ts() -> TimeStamp {
        TimeStamp::now()
    }

    fn deployment_with_metadata(metadata: HashMap<String, serde_json::Value>) -> ModelDeployment {
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: Uuid::now_v7(),
            tenant_id: "test-tenant".into(),
            platform: Platform::TapisPods,
            owner: "test-owner".into(),
            model: ModelReference {
                name: "gpt2".into(),
                author: "openai-community".into(),
                tenant_id: "test".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some("tapis-pods:default".into()),
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
            tenant_id: "test-tenant".into(),
            platform: Platform::TapisPods,
            owner: "test-owner".into(),
            model: ModelReference {
                name: "gpt2".into(),
                author: "openai-community".into(),
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
        m.name = "gpt2".into();
        m.author ="openai-community".into();
        m
    }

    // ---- Unit tests: credential extraction ----

    #[test]
    fn extract_tapis_credentials_ok() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt-token"));
        let deployment = deployment_with_metadata(meta);
        let (url, user, token) =
            TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
                .unwrap();
        assert_eq!(url, "https://tacc.tapis.io");
        assert_eq!(user, "user1");
        assert_eq!(token, "jwt-token");
    }

    #[test]
    fn extract_tapis_credentials_err_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("Deployment metadata is required"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_tenant_url_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_user".into(), json!("user1"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("tapis_tenant_url"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_user_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_token".into(), json!("jwt"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("tapis_user"));
    }

    #[test]
    fn extract_tapis_credentials_err_when_tapis_token_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        meta.insert("tapis_user".into(), json!("user1"));
        let deployment = deployment_with_metadata(meta);
        let err = TapisPodsModelDeploymentReconciliationClient::extract_tapis_credentials(&deployment)
            .unwrap_err();
        assert!(err.to_string().contains("tapis_token"));
    }

    // ---- Unit tests: pod info extraction ----

    #[test]
    fn extract_pod_info_some() {
        let mut meta = HashMap::new();
        meta.insert("pod_id".into(), json!("pabc123"));
        meta.insert("volume_id".into(), json!("vabc123"));
        let deployment = deployment_with_metadata(meta);
        let (pod_id, volume_id) =
            TapisPodsModelDeploymentReconciliationClient::extract_pod_info(&deployment).unwrap();
        assert_eq!(pod_id, "pabc123");
        assert_eq!(volume_id, "vabc123");
    }

    #[test]
    fn extract_pod_info_none_when_metadata_missing() {
        let deployment = deployment_without_metadata();
        assert!(TapisPodsModelDeploymentReconciliationClient::extract_pod_info(&deployment).is_none());
    }

    #[test]
    fn extract_pod_info_none_when_keys_missing() {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!("https://tacc.tapis.io"));
        let deployment = deployment_with_metadata(meta);
        assert!(TapisPodsModelDeploymentReconciliationClient::extract_pod_info(&deployment).is_none());
    }

    #[test]
    fn extract_pod_info_pod_only_no_volume() {
        let mut meta = HashMap::new();
        meta.insert("pod_id".into(), json!("pmingyutest"));
        let deployment = deployment_with_metadata(meta);
        let (pod_id, volume_id) =
            TapisPodsModelDeploymentReconciliationClient::extract_pod_info(&deployment).unwrap();
        assert_eq!(pod_id, "pmingyutest");
        assert_eq!(volume_id, "");
    }

    // ---- Unit tests: error mapping ----

    #[test]
    fn map_deployment_error_tapis_auth() {
        let e = FlexServDeploymentError::TapisAuthFailed("bad token".into());
        let r = TapisPodsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("TAPIS authentication failed"));
        assert!(r.to_string().contains("bad token"));
    }

    #[test]
    fn map_deployment_error_pod_creation() {
        let e = FlexServDeploymentError::PodCreationFailed("quota exceeded".into());
        let r = TapisPodsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Pod creation failed"));
    }

    #[test]
    fn map_deployment_error_unknown() {
        let e = FlexServDeploymentError::UnknownError("something broke".into());
        let r = TapisPodsModelDeploymentReconciliationClient::map_deployment_error(e);
        assert!(r.to_string().contains("Unknown error"));
    }

    // ---- Unit tests: state_from_status (canonical TAPIS pod status strings) ----

    #[test]
    fn state_from_status_available() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(Some("AVAILABLE"));
        assert_eq!(s, State::Running);
    }

    #[test]
    fn state_from_status_running_case_insensitive() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(Some("running"));
        assert_eq!(s, State::Running);
    }

    #[test]
    fn state_from_status_stopped() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(Some("STOPPED"));
        assert_eq!(s, State::Stopped);
    }

    #[test]
    fn state_from_status_failed() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(Some("FAILED"));
        assert_eq!(s, State::Failed);
    }

    #[test]
    fn state_from_status_pending_unknown() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(Some("PENDING"));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_status_creating_unknown() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(Some("CREATING"));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_status_empty_unknown() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(Some(""));
        assert_eq!(s, State::Unknown);
    }

    #[test]
    fn state_from_status_none_unknown() {
        let s = TapisPodsModelDeploymentReconciliationClient::state_from_status(None);
        assert_eq!(s, State::Unknown);
    }

    // ---- Async unit tests: reconcile returns error when metadata missing ----

    #[tokio::test]
    async fn reconcile_start_fails_without_metadata() {
        let client = TapisPodsModelDeploymentReconciliationClient::new();
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
    async fn reconcile_stop_fails_without_metadata() {
        let client = TapisPodsModelDeploymentReconciliationClient::new();
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
        let client = TapisPodsModelDeploymentReconciliationClient::new();
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
        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let deployment = deployment_without_metadata();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let result = client.reconcile(input).await;
        assert!(result.is_err());
    }

    // ---- Integration tests: real Tapis (one test per action) ----
    // Run with: cargo test -p shared integration_reconcile_<create|start|stop|terminate|monitor> -- --ignored
    // Create: needs only TAPIS_* env. Start/Stop/Terminate/Monitor: need deployment.metadata with pod_id and volume_id.

    fn has_tapis_credentials() -> bool {
        std::env::var("TAPIS_TENANT_URL").is_ok()
            && std::env::var("TAPIS_USER").is_ok()
            && std::env::var("TAPIS_TOKEN").is_ok()
    }

    fn deployment_with_tapis_meta(
        deployment_id: Uuid,
        tenant_url: &str,
        tapis_user: &str,
        tapis_token: &str,
        pod_id: Option<&str>,
        volume_id: Option<&str>,
    ) -> ModelDeployment {
        let mut meta = HashMap::new();
        meta.insert("tapis_tenant_url".into(), json!(tenant_url));
        meta.insert("tapis_user".into(), json!(tapis_user));
        meta.insert("tapis_token".into(), json!(tapis_token));
        if let Some(p) = pod_id {
            meta.insert("pod_id".into(), json!(p));
        }
        if let Some(v) = volume_id {
            meta.insert("volume_id".into(), json!(v));
        }
        ModelDeployment::rehydrate(RehydrateModelDeploymentProps {
            id: deployment_id,
            tenant_id: "test-tenant".into(),
            platform: Platform::TapisPods,
            owner: tapis_user.to_string(),
            model: ModelReference {
                name: "gpt2".into(),
                author: "openai-community".into(),
                tenant_id: "test".into(),
            },
            state: State::NotDeployed,
            desired_state: DesiredState::Running,
            last_message: None,
            deployment_strategy: Some("tapis-pods:default".into()),
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

    fn assert_and_print_metadata_delta(label: &str, metadata_delta: &Option<ModelDeploymentMetadataDelta>) {
        match metadata_delta {
            Some(ModelDeploymentMetadataDelta::Merge(m)) => {
                let pod_id = m.get("pod_id").and_then(|v| v.as_str());
                let volume_id = m.get("volume_id").and_then(|v| v.as_str());
                let tapis_user = m.get("tapis_user").and_then(|v| v.as_str());
                let tapis_tenant = m.get("tapis_tenant").and_then(|v| v.as_str());
                let model_id = m.get("model_id").and_then(|v| v.as_str());
                let pod_url = m.get("pod_url").and_then(|v| v.as_str());
                eprintln!("{} response: pod_id={:?} volume_id={:?} pod_url={:?}", label, pod_id, volume_id, pod_url);
                assert!(m.get("pod_id").is_some(), "metadata should have pod_id");
                assert!(m.get("volume_id").is_some(), "metadata should have volume_id");
                assert!(m.get("tapis_user").is_some(), "metadata should have tapis_user");
                assert!(m.get("tapis_tenant").is_some(), "metadata should have tapis_tenant");
                assert!(m.get("model_id").is_some(), "metadata should have model_id");
            },
            Some(ModelDeploymentMetadataDelta::Delete) => {
                eprintln!("{} response: metadata marked for deletion", label);
            },
            Some(ModelDeploymentMetadataDelta::NoChange) => {
                eprintln!("{} response: no metadata changes", label);
            },
            None => {
                panic!("Expected metadata delta in outcome");
            }
        }
    }

    /// Create pod only. No pod_id/volume_id needed. Response includes pod_id, volume_id, pod_url.
    #[tokio::test]
    #[ignore = "requires TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN and real Tapis Pods API"]
    async fn integration_reconcile_create_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            None,
            None,
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile create");
        match &outcome {
            ReconciliationOutcome::Started(p) => {
                assert!(p.message.is_some());
                assert_eq!(p.state, State::Unknown);
                assert_and_print_metadata_delta("create", &p.metadata);
            }
            other => panic!("expected Started, got {:?}", other),
        }
    }

    /// Start an existing pod. Requires pod_id and volume_id in deployment.metadata.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and deployment.metadata with pod_id, volume_id"]
    async fn integration_reconcile_start_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for start test");
        let volume_id = std::env::var("TEST_VOLUME_ID").expect("TEST_VOLUME_ID required for start test");
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            Some(&volume_id),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Start { strategy: None },
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile start");
        match &outcome {
            ReconciliationOutcome::Started(p) => {
                assert!(p.message.is_some());
                assert_and_print_metadata_delta("start", &p.metadata);
            }
            other => panic!("expected Started, got {:?}", other),
        }
    }

    /// Stop a running pod. Requires pod_id and volume_id in deployment.metadata.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and deployment.metadata with pod_id, volume_id"]
    async fn integration_reconcile_stop_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for stop test");
        let volume_id = std::env::var("TEST_VOLUME_ID").expect("TEST_VOLUME_ID required for stop test");
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            Some(&volume_id),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Stop,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile stop");
        match &outcome {
            ReconciliationOutcome::Stopped(p) => {
                assert!(p.message.is_some());
                assert_and_print_metadata_delta("stop", &p.metadata);
            }
            other => panic!("expected Stopped, got {:?}", other),
        }
    }

    /// Terminate (delete) pod and volume. Requires pod_id and volume_id in deployment.metadata.
    #[tokio::test]
    #[ignore = "requires TAPIS_* env and deployment.metadata with pod_id, volume_id"]
    async fn integration_reconcile_terminate_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for terminate test");
        let volume_id = std::env::var("TEST_VOLUME_ID").expect("TEST_VOLUME_ID required for terminate test");
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            Some(&volume_id),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Undeploy,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile terminate");
        match &outcome {
            ReconciliationOutcome::Undeployed(p) => {
                assert!(p.message.is_some());
            }
            other => panic!("expected Undeployed, got {:?}", other),
        }
    }

    #[tokio::test]
    #[ignore = "requires TAPIS_* env and TEST_POD_ID"]
    async fn integration_reconcile_monitor_only() {
        if !has_tapis_credentials() {
            eprintln!("Skipping: set TAPIS_TENANT_URL, TAPIS_USER, TAPIS_TOKEN to run");
            return;
        }
        let pod_id = std::env::var("TEST_POD_ID").expect("TEST_POD_ID required for monitor test");
        let volume_id = std::env::var("TEST_VOLUME_ID").ok();
        let tenant_url = std::env::var("TAPIS_TENANT_URL").unwrap();
        let tapis_user = std::env::var("TAPIS_USER").unwrap();
        let tapis_token = std::env::var("TAPIS_TOKEN").unwrap();

        let deployment_id = Uuid::now_v7();
        let deployment = deployment_with_tapis_meta(
            deployment_id,
            &tenant_url,
            &tapis_user,
            &tapis_token,
            Some(&pod_id),
            volume_id.as_deref(),
        );

        let client = TapisPodsModelDeploymentReconciliationClient::new();
        let input = ReconcileModelDeploymentInput {
            action: ReconciliationAction::Observe,
            deployment,
            model_metadata: minimal_model_metadata(),
        };
        let outcome = client.reconcile(input).await.expect("reconcile monitor");
        match &outcome {
            ReconciliationOutcome::Observed(p) => {
                assert!(p.message.is_some());
                eprintln!("observed state: {:?}", p.state);
                assert_and_print_metadata_delta("monitor", &p.metadata);
            }
            other => panic!("expected Observed, got {:?}", other),
        }
    }
}