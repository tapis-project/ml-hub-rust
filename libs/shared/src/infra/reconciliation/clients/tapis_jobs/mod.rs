use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::{json, Value};
use tapis_jobs::models::{JobArgSpec, ReqSubmitJob};
use tapis_jobs::{with_headers, TapisJobs};
use tapis_tokens::models::RefreshToken201Response;
use thiserror::Error;

use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::ports::deployment::ModelDeploymentReconciliationClient;
use crate::application::ports::errors::InfrastructureError;
use crate::application::services::deployment_argument_service::DecryptedArgument;
use crate::application::workflows::reconciliation::{
    FailedOutcome, FailureDisposition, ObservedOutcome, ReconcilerError, ReconciliationAction,
    ReconciliationOutcome, StartedOutcome, StoppedOutcome, UndeployedOutcome,
};
use crate::domain::entities::deployment::{
    BatchExecutionOutcome, DeploymentTargetSnapshot, DesiredState, ModelDeployment,
    ModelDeploymentEndpoint, ModelDeploymentEndpointDelta, ModelDeploymentMetadata,
    ModelDeploymentMetadataDelta, Protocol, State,
};
use crate::domain::entities::model::external_model::{ExternalModel, ModelLocator, ModelProvider};
use crate::domain::entities::site::SiteContext;
use crate::shared_kernel::enums::DeploymentModality;

use std::collections::HashMap;
use std::env;

#[derive(Debug, Clone, Error)]
enum ReconciliationError {
    #[error(transparent)]
    Fatal(#[from] InfrastructureError),

    #[error("{0}")]
    Recoverable(String),

    #[error("Unsupported model provider for Tapis Jobs: {0}")]
    UnsupportedModelProvider(String),

    #[error("Unsupported FlexServ version for Tapis Jobs: {0}")]
    UnsupportedFlexServVersion(String),

    #[error("Tapis Jobs reconciliation does not support HPC cluster host {0}")]
    UnsupportedHpcCluster(String),
}

struct TapisJobStatus {
    status: String,
    condition: Option<String>,
}

pub struct TapisJobsModelDeploymentReconciliationClient {
    mlhub_service_password: String,
    site_context: SiteContext,
    client: Client,
    base_job_definition: ReqSubmitJob,
}

impl TapisJobsModelDeploymentReconciliationClient {
    const FLEXSERV_APP_ID: &'static str = "FlexServ-1.4.0";
    const FLEXSERV_APP_VERSION: &'static str = "1.4.0";
    const FLEXSERV_JOB_DEF_URL: &'static str = "https://raw.githubusercontent.com/tapis-project/FlexServ-Deployer/refs/heads/main/tapis_def/1.4.0/job.json";
    const TAPIS_JOB_UUID_KEY: &'static str = "tapis_job_uuid";
    const FLEXSERV_ACCESS_INFO_PATH: &'static str = "flexserv_access_info.txt";
    const FLEXSERV_ENDPOINT_NAME: &'static str = "default";

    fn is_retryable_status(status: u16) -> bool {
        matches!(status, 408 | 425 | 429 | 500..=599)
    }

    fn map_job_request_error<T>(
        deployment: &ModelDeployment,
        operation: &str,
        tapis_error: tapis_jobs::apis::Error<T>,
    ) -> ReconciliationError {
        match tapis_error {
            tapis_jobs::apis::Error::ResponseError(response) => {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Tapis rejected job {}; deployment_id='{}'; status={}; response_body={}",
                    error.error_id(),
                    operation,
                    deployment.id(),
                    response.status,
                    Self::sanitized_response_body(&response.content)
                );

                if Self::is_retryable_status(response.status.as_u16()) {
                    return ReconciliationError::Recoverable(format!(
                        "Tapis Jobs {} failed with retryable status {}",
                        operation, response.status
                    ));
                }

                ReconciliationError::Fatal(error)
            }
            tapis_error => {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Tapis job {} failed before receiving a response; deployment_id='{}'; error={}",
                    error.error_id(),
                    operation,
                    deployment.id(),
                    tapis_error
                );

                ReconciliationError::Recoverable(format!(
                    "Tapis Jobs {} failed before receiving a response",
                    operation
                ))
            }
        }
    }

    fn resolve_tapis_system_id(cluster_host: &str) -> Result<&'static str, ReconciliationError> {
        match cluster_host {
            "frontera.tacc.utexas.edu" => Ok("MLHub-FlexServ-Frontera"),
            "ls6.tacc.utexas.edu" => Ok("MLHub-FlexServ-Lonestar6"),
            "stampede3.tacc.utexas.edu" => Ok("MLHub-FlexServ-Stampede3"),
            "vista.tacc.utexas.edu" => Ok("MLHub-FlexServ-Vista-PKI"),
            host => Err(ReconciliationError::UnsupportedHpcCluster(host.into())),
        }
    }

    pub async fn new(site_context: &SiteContext) -> Result<Self, ReconcilerError> {
        log::debug!(
            "Initializing Tapis Jobs reconciliation client; site_id='{}'; site_base_url='{}'",
            site_context.site_id,
            site_context.base_url
        );

        let mlhub_service_password = match env::var("MLHUB_SERVICE_PASSWORD") {
            Ok(p) => p,
            Err(_) => {
                let msg = "Could not initialize Tapis Jobs deployment reconciler";

                log::error!("{}", msg);
                return Err(ReconcilerError::InitializationFailed(msg.into()));
            }
        };

        // Initialize http client
        let client = Client::new();

        log::debug!(
            "Fetching FlexServ base job definition from '{}'",
            Self::FLEXSERV_JOB_DEF_URL
        );

        // Fetch the base tapis job def
        let base_job_definition = client
            .get(Self::FLEXSERV_JOB_DEF_URL)
            .send()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Error fetching FlexServ job definition: {}",
                    error.error_id(),
                    e.to_string()
                );
                ReconcilerError::InitializationFailed(error.to_string())
            })?
            .json::<ReqSubmitJob>()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Error deserializing FlexServ job definition: {}",
                    error.error_id(),
                    e.to_string()
                );
                ReconcilerError::InitializationFailed(error.to_string())
            })?;

        log::debug!("FlexServ base job definition loaded successfully");

        Ok(Self {
            client,
            base_job_definition,
            mlhub_service_password,
            site_context: site_context.clone(),
        })
    }

    async fn handle_start(
        &self,
        deployment: &ModelDeployment,
        external_model: &ExternalModel,
        arguments: &[DecryptedArgument],
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        log::debug!(
            "Preparing Tapis Jobs start reconciliation; deployment_id='{}'; tenant_id='{}'; owner='{}'; external_model_id='{}'; argument_count={}",
            deployment.id(),
            deployment.tenant_id,
            deployment.owner,
            external_model.id(),
            arguments.len()
        );

        let (model_id, model_revision) = match (external_model.provider(), external_model.locator())
        {
            (ModelProvider::HuggingFace, ModelLocator::HuggingFace(locator)) => {
                (locator.id(), locator.sha())
            }
            (provider, _) => {
                return Err(ReconciliationError::UnsupportedModelProvider(
                    provider.to_string(),
                ));
            }
        };

        log::debug!(
            "Resolved Hugging Face source for model deployment '{}'; model_id='{}'; model_revision='{}'",
            deployment.id(),
            model_id,
            model_revision
        );

        log::debug!(
            "Requesting MLHub service token for model deployment '{}'",
            deployment.id()
        );

        let service_jwt = self.generate_service_token().await?;

        log::debug!(
            "MLHub service token acquired for model deployment '{}'",
            deployment.id()
        );

        let target_base_url = self.get_target_base_url(deployment);

        log::debug!(
            "Initializing Tapis Jobs SDK for model deployment '{}'; base_url='{}'",
            deployment.id(),
            target_base_url
        );

        let jobs_client = TapisJobs::new(&target_base_url, Some(&service_jwt)).map_err(|e| {
            let error = InfrastructureError::new_internal();

            log::error!(
                "[{}] Failed to initialize TapisJobs client: {}",
                error.error_id(),
                e.to_string()
            );
            error
        })?;

        let custom_headers = Self::obo_headers(deployment)?;

        if let Some(job_uuid) = Self::extract_job_uuid(deployment) {
            let job_status = self
                .fetch_job_status(&jobs_client, deployment, &job_uuid)
                .await?;

            if Self::is_terminal_job_status(&job_status.status) {
                return self.resubmit_job(&jobs_client, deployment, &job_uuid).await;
            }

            log::info!(
                "Retaining active Tapis job for model deployment '{}'; job_uuid='{}'; status='{}'; condition={:?}",
                deployment.id(),
                job_uuid,
                job_status.status,
                job_status.condition
            );

            return Ok(ReconciliationOutcome::NoOp);
        }

        let job_def = self.build_job_request(model_id, model_revision, deployment, arguments)?;

        log::debug!(
            "Built FlexServ Tapis job request for model deployment '{}'; job_name='{}'; app_id='{}'; app_version='{}'; exec_system_id={:?}; exec_system_logical_queue={:?}",
            deployment.id(),
            job_def.name,
            job_def.app_id,
            job_def.app_version,
            job_def.exec_system_id,
            job_def.exec_system_logical_queue
        );

        if log::log_enabled!(log::Level::Debug) {
            match serde_json::to_value(&job_def) {
                Ok(mut payload) => {
                    Self::redact_sensitive_json(&mut payload);

                    log::debug!(
                        "Submitting FlexServ Tapis job request for model deployment '{}'; payload={}",
                        deployment.id(),
                        payload
                    );
                }
                Err(error) => {
                    log::warn!(
                        "Unable to serialize FlexServ Tapis job request for diagnostic logging; deployment_id='{}'; error={}",
                        deployment.id(),
                        error
                    );
                }
            }
        }

        let job_uuid = with_headers(custom_headers, async {
            jobs_client.jobs.submit_job(job_def).await
        })
        .await
        .map_err(|tapis_error| {
            match &tapis_error {
                tapis_jobs::apis::Error::ResponseError(response) => {
                    let error = InfrastructureError::new_internal();

                    log::error!(
                        "[{}] Tapis rejected FlexServ job submission; deployment_id='{}'; status={}; response_body={}",
                        error.error_id(),
                        deployment.id(),
                        response.status,
                        Self::sanitized_response_body(&response.content)
                    );

                    if Self::is_retryable_status(response.status.as_u16()) {
                        return ReconciliationError::Recoverable(format!(
                            "Tapis Jobs request failed with retryable status {}",
                            response.status
                        ));
                    }

                    ReconciliationError::Fatal(error)
                }
                _ => {
                    let error = InfrastructureError::new_internal();

                    log::error!(
                        "[{}] Error when submitting FlexServ job to Tapis; deployment_id='{}'; error={}",
                        error.error_id(),
                        deployment.id(),
                        tapis_error
                    );

                    ReconciliationError::Recoverable(
                        "Tapis Jobs request failed before receiving a response".into(),
                    )
                }
            }
        })?
        .result
        .ok_or_else(|| {
            let error = InfrastructureError::new_internal();

            log::error!("[{}] Job is None", error.error_id());
            error
        })?
        .uuid
        .ok_or_else(|| {
            let error = InfrastructureError::new_internal();

            log::error!("[{}] Uuid is None", error.error_id());
            error
        })?;

        log::info!(
            "Tapis accepted FlexServ job for model deployment '{}'; job_uuid='{}'",
            deployment.id(),
            job_uuid
        );

        let mut map = HashMap::new();

        map.insert(Self::TAPIS_JOB_UUID_KEY.to_string(), json!(job_uuid));

        Ok(ReconciliationOutcome::Started(StartedOutcome {
            message: Some("Deployment submitted".to_string()),
            state: State::Unknown,
            metadata: Some(ModelDeploymentMetadataDelta::Merge(
                ModelDeploymentMetadata(map),
            )),
            replicas: None,
            target_endpoint: None,
        }))
    }

    async fn resubmit_job(
        &self,
        jobs_client: &TapisJobs,
        deployment: &ModelDeployment,
        previous_job_uuid: &str,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        log::debug!(
            "Resubmitting terminal Tapis job for model deployment '{}'; previous_job_uuid='{}'",
            deployment.id(),
            previous_job_uuid
        );

        let response = with_headers(Self::obo_headers(deployment)?, async {
            jobs_client.jobs.resubmit_job(previous_job_uuid, None).await
        })
        .await
        .map_err(|tapis_error| {
            Self::map_job_request_error(deployment, "resubmission", tapis_error)
        })?;

        let job_uuid = response
            .result
            .ok_or_else(|| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Tapis job resubmission response did not contain a job; deployment_id='{}'",
                    error.error_id(),
                    deployment.id()
                );

                error
            })?
            .uuid
            .ok_or_else(|| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Tapis job resubmission response did not contain a UUID; deployment_id='{}'",
                    error.error_id(),
                    deployment.id()
                );

                error
            })?;

        log::info!(
            "Tapis accepted FlexServ job resubmission for model deployment '{}'; job_uuid='{}'",
            deployment.id(),
            job_uuid
        );

        let mut metadata = HashMap::new();

        metadata.insert(Self::TAPIS_JOB_UUID_KEY.to_string(), json!(job_uuid));

        Ok(ReconciliationOutcome::Started(StartedOutcome {
            message: Some("Deployment resubmitted".into()),
            state: State::Unknown,
            metadata: Some(ModelDeploymentMetadataDelta::Merge(
                ModelDeploymentMetadata(metadata),
            )),
            replicas: None,
            target_endpoint: Some(ModelDeploymentEndpointDelta::Delete),
        }))
    }

    async fn handle_stop(
        &self,
        _input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        Ok(ReconciliationOutcome::Stopped(StoppedOutcome {
            message: Some("Deployment stopped successfully".to_string()),
            metadata: None,
            replicas: None,
            target_endpoint: Some(ModelDeploymentEndpointDelta::Delete),
        }))
    }

    async fn handle_undeploy(
        &self,
        _input: &ReconcileModelDeploymentInput,
    ) -> Result<ReconciliationOutcome, ReconciliationError> {
        Ok(ReconciliationOutcome::Undeployed(UndeployedOutcome {
            message: Some("Deployment canceled successfully".to_string()),
            metadata: Some(ModelDeploymentMetadataDelta::Delete),
            target_endpoint: Some(ModelDeploymentEndpointDelta::Delete),
        }))
    }

    async fn handle_observe(
        &self,
        deployment: &ModelDeployment,
    ) -> Result<ObservedOutcome, ReconciliationError> {
        log::debug!(
            "Observing Tapis Jobs deployment; deployment_id='{}'; revision={}; state='{:?}'",
            deployment.id(),
            deployment.revision(),
            deployment.state
        );

        let job_uuid = Self::extract_job_uuid(deployment).ok_or_else(|| {
            ReconciliationError::Recoverable(
                "Tapis job identifier is unavailable for observation".into(),
            )
        })?;

        log::debug!(
            "Resolved Tapis job identifier for model deployment '{}'; job_uuid='{}'",
            deployment.id(),
            job_uuid
        );

        let service_jwt = self.generate_service_token().await?;

        let target_base_url = self.get_target_base_url(deployment);

        log::debug!(
            "Fetching Tapis job status for model deployment '{}'; job_uuid='{}'; base_url='{}'",
            deployment.id(),
            job_uuid,
            target_base_url
        );

        let jobs_client =
            TapisJobs::new(&target_base_url, Some(&service_jwt)).map_err(|error| {
                let infrastructure_error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Failed to initialize Tapis Jobs observation client: {}",
                    infrastructure_error.error_id(),
                    error
                );

                infrastructure_error
            })?;

        let job_status = self
            .fetch_job_status(&jobs_client, deployment, &job_uuid)
            .await?;

        let (state, batch_execution_outcome, failure_disposition) =
            Self::observation_from_job_status(
                &deployment.deployment_modality,
                &deployment.desired_state,
                &job_status.status,
                job_status.condition.as_deref(),
            );

        log::debug!(
            "Observed Tapis job status for model deployment '{}'; job_uuid='{}'; tapis_status='{}'; tapis_condition={:?}; mapped_state='{:?}'",
            deployment.id(),
            job_uuid,
            job_status.status,
            job_status.condition,
            state
        );

        if state != State::Running {
            let message = match job_status.condition {
                Some(condition) => format!(
                    "Observed Tapis job status {} with condition {}",
                    job_status.status, condition
                ),
                None => format!("Observed Tapis job status {}", job_status.status),
            };

            return Ok(ObservedOutcome {
                message: Some(message),
                state,
                batch_execution_outcome,
                failure_disposition,
                metadata: None,
                replicas: None,
                target_endpoint: Some(Self::target_endpoint_deletion(deployment)),
            });
        }

        let target_endpoint = self
            .discover_flexserv_endpoint(&jobs_client, deployment, &job_uuid)
            .await;

        Ok(ObservedOutcome {
            message: Some("Observed running Tapis job".into()),
            state,
            batch_execution_outcome,
            failure_disposition,
            metadata: None,
            replicas: None,
            target_endpoint,
        })
    }

    async fn discover_flexserv_endpoint(
        &self,
        jobs_client: &TapisJobs,
        deployment: &ModelDeployment,
        job_uuid: &str,
    ) -> Option<ModelDeploymentEndpointDelta> {
        let response = with_headers(Self::obo_headers(deployment).ok()?, async {
            jobs_client
                .jobs
                .get_job_output_download(
                    job_uuid,
                    Self::FLEXSERV_ACCESS_INFO_PATH,
                    Some(false),
                    None,
                    Some(true),
                )
                .await
        })
        .await;

        let Ok(response) = response else {
            log::warn!(
                "FlexServ access information is not yet available for model deployment '{}'",
                deployment.id()
            );

            return Some(ModelDeploymentEndpointDelta::NoChange);
        };

        let Ok(contents) = response.text().await else {
            log::warn!(
                "FlexServ access information could not be read for model deployment '{}'",
                deployment.id()
            );

            return Some(ModelDeploymentEndpointDelta::NoChange);
        };

        let Ok(endpoint) = Self::parse_flexserv_access_info(&contents) else {
            log::warn!(
                "FlexServ access information is incomplete or malformed for model deployment '{}'",
                deployment.id()
            );

            return Some(ModelDeploymentEndpointDelta::NoChange);
        };

        if deployment.target_endpoint() == Some(&endpoint) {
            return Some(ModelDeploymentEndpointDelta::NoChange);
        }

        Some(ModelDeploymentEndpointDelta::Replace(endpoint))
    }

    fn parse_flexserv_access_info(
        contents: &str,
    ) -> Result<ModelDeploymentEndpoint, ReconciliationError> {
        let contents = contents.trim();

        let remainder = contents.strip_prefix("FlexServ address: ").ok_or_else(|| {
            ReconciliationError::Recoverable("Invalid FlexServ access information".into())
        })?;

        let (address, token) = remainder.split_once("  FlexServ token: ").ok_or_else(|| {
            ReconciliationError::Recoverable("Invalid FlexServ access information".into())
        })?;

        if address.trim().is_empty()
            || token.trim().is_empty()
            || token.split_whitespace().count() != 1
        {
            return Err(ReconciliationError::Recoverable(
                "Invalid FlexServ access information".into(),
            ));
        }

        ModelDeploymentEndpoint::new(
            Some(Self::FLEXSERV_ENDPOINT_NAME.into()),
            Protocol::RestHttp,
            Some(address.trim().into()),
        )
        .map_err(|_| ReconciliationError::Recoverable("Invalid FlexServ access information".into()))
    }

    fn obo_headers(deployment: &ModelDeployment) -> Result<HeaderMap, ReconciliationError> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "X-Tapis-Tenant",
            HeaderValue::from_str(deployment.tenant_id.as_str()).map_err(|error| {
                let infrastructure_error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Invalid Tapis tenant header: {}",
                    infrastructure_error.error_id(),
                    error
                );

                infrastructure_error
            })?,
        );

        headers.insert(
            "X-Tapis-User",
            HeaderValue::from_str(deployment.owner.as_str()).map_err(|error| {
                let infrastructure_error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Invalid Tapis user header: {}",
                    infrastructure_error.error_id(),
                    error
                );

                infrastructure_error
            })?,
        );

        Ok(headers)
    }

    fn target_endpoint_deletion(deployment: &ModelDeployment) -> ModelDeploymentEndpointDelta {
        if deployment.target_endpoint().is_some() {
            ModelDeploymentEndpointDelta::Delete
        } else {
            ModelDeploymentEndpointDelta::NoChange
        }
    }

    async fn generate_service_token(&self) -> Result<String, ReconciliationError> {
        let payload = json!({
            "account_type": "service",
            "token_tenant_id": "admin",
            "token_username": "mlhub",
            "target_site_id": self.site_context.site_id.clone(),
        });

        log::debug!(
            "Requesting MLHub service token; site_id='{}'; token_service_base_url='{}'",
            self.site_context.site_id,
            self.get_site_context().base_url
        );

        let resp = self
            .client
            .post(format!("{}/v3/tokens", self.get_site_context().base_url))
            .basic_auth("mlhub", Some(&self.mlhub_service_password))
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] External service call error: {}",
                    error.error_id(),
                    e.to_string()
                );
                error
            })?
            .json::<RefreshToken201Response>()
            .await
            .map_err(|e| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Deserialization error: {}",
                    error.error_id(),
                    e.to_string()
                );
                error
            })?
            .result
            .ok_or_else(|| InfrastructureError::new_internal())
            .map_err(|e| {
                log::error!(
                    "[{}] Missing result field in response: {}",
                    e.error_id(),
                    e.to_string()
                );
                e
            })?
            .access_token
            .ok_or_else(|| InfrastructureError::new_internal())
            .map_err(|e| {
                log::error!(
                    "[{}] Missing access token field in result: {}",
                    e.error_id(),
                    e.to_string()
                );
                e
            })?
            .access_token
            .ok_or_else(|| InfrastructureError::new_internal())
            .map_err(|e| {
                log::error!(
                    "[{}] Missing access token field inside of NewAccessTokenResponse: {}",
                    e.error_id(),
                    e.to_string()
                );
                e
            })?;

        log::debug!(
            "MLHub service token request succeeded; site_id='{}'",
            self.site_context.site_id
        );

        Ok(resp)
    }

    fn sanitized_response_body(content: &str) -> String {
        let Ok(mut value) = serde_json::from_str::<Value>(content) else {
            return format!(
                "<non-JSON response body omitted; byte_length={}>",
                content.len()
            );
        };

        Self::redact_sensitive_json(&mut value);

        value.to_string()
    }

    fn redact_sensitive_json(value: &mut Value) {
        match value {
            Value::Object(object) => {
                let contains_sensitive_identifier = object
                    .iter()
                    .filter(|(key, _)| matches!(key.as_str(), "name" | "key" | "id"))
                    .filter_map(|(_, value)| value.as_str())
                    .any(Self::is_sensitive_name);

                for (key, value) in object {
                    let is_sensitive_key = Self::is_sensitive_name(key);
                    let is_sensitive_value =
                        contains_sensitive_identifier && matches!(key.as_str(), "value" | "arg");

                    if is_sensitive_key || is_sensitive_value {
                        *value = Value::String("[REDACTED]".into());
                    } else {
                        Self::redact_sensitive_json(value);
                    }
                }
            }
            Value::Array(values) => {
                for value in values {
                    Self::redact_sensitive_json(value);
                }
            }
            _ => {}
        }
    }

    fn is_sensitive_name(name: &str) -> bool {
        let normalized_name = name.to_ascii_lowercase();

        ["authorization", "credential", "password", "secret", "token"]
            .iter()
            .any(|candidate| normalized_name.contains(candidate))
    }

    /// `job_uuid` from metadata when the deployment already submitted a Tapis job.
    fn extract_job_uuid(deployment: &ModelDeployment) -> Option<String> {
        deployment.metadata.as_ref().and_then(|m| {
            m.get("tapis_job_uuid")?
                .as_str()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(String::from)
        })
    }

    async fn fetch_job_status(
        &self,
        jobs_client: &TapisJobs,
        deployment: &ModelDeployment,
        job_uuid: &str,
    ) -> Result<TapisJobStatus, ReconciliationError> {
        let response = with_headers(Self::obo_headers(deployment)?, async {
            jobs_client.jobs.get_job_status(job_uuid).await
        })
        .await
        .map_err(|error| Self::map_job_request_error(deployment, "status request", error))?;

        let result = response.result.ok_or_else(|| {
            ReconciliationError::Recoverable(
                "Tapis job status response did not contain a result".into(),
            )
        })?;

        let status = result.status.ok_or_else(|| {
            ReconciliationError::Recoverable(
                "Tapis job status response did not contain a status".into(),
            )
        })?;

        let condition = result
            .condition
            .map(|condition| condition.trim().to_owned())
            .filter(|condition| !condition.is_empty());

        Ok(TapisJobStatus { status, condition })
    }

    fn is_terminal_job_status(status: &str) -> bool {
        matches!(
            status,
            "FINISHED" | "COMPLETED" | "CANCELLED" | "CANCELED" | "STOPPED" | "FAILED"
        )
    }

    fn observation_from_job_status(
        deployment_modality: &DeploymentModality,
        desired_state: &DesiredState,
        status: &str,
        condition: Option<&str>,
    ) -> (
        State,
        Option<BatchExecutionOutcome>,
        Option<FailureDisposition>,
    ) {
        let desires_running = desired_state == &DesiredState::Running;

        if deployment_modality == &DeploymentModality::Batch
            && status == "FAILED"
            && condition == Some("SCHEDULER_TIMEOUT")
        {
            return (State::Stopped, Some(BatchExecutionOutcome::TimedOut), None);
        }

        match (deployment_modality, status) {
            (_, "RUNNING") => (State::Running, None, None),
            (
                _,
                "QUEUED" | "PENDING" | "PROCESSING_INPUTS" | "STAGING_INPUTS" | "STAGING_JOB"
                | "SUBMITTING_JOB" | "ARCHIVING",
            ) => (State::Unknown, None, None),
            (DeploymentModality::Batch, "FINISHED" | "COMPLETED") => {
                (State::Stopped, Some(BatchExecutionOutcome::Succeeded), None)
            }
            (DeploymentModality::Batch, "CANCELLED" | "CANCELED" | "STOPPED") => (
                State::Stopped,
                Some(BatchExecutionOutcome::Cancelled),
                desires_running.then_some(FailureDisposition::Terminal),
            ),
            (DeploymentModality::Batch, "FAILED") => (
                State::Stopped,
                Some(BatchExecutionOutcome::Failed),
                desires_running.then_some(FailureDisposition::Terminal),
            ),
            (
                DeploymentModality::Service,
                "FINISHED" | "COMPLETED" | "CANCELLED" | "CANCELED" | "STOPPED" | "FAILED",
            ) if desires_running => (State::Failed, None, Some(FailureDisposition::Terminal)),
            (
                DeploymentModality::Service,
                "FINISHED" | "COMPLETED" | "CANCELLED" | "CANCELED" | "STOPPED" | "FAILED",
            ) => (State::Stopped, None, None),
            (_, "BLOCKED" | "PAUSED") => (State::Blocked, None, None),
            _ => (State::Unknown, None, None),
        }
    }

    fn get_target_base_url(&self, deployment: &ModelDeployment) -> String {
        let target_base_url = self
            .get_site_context()
            .base_url
            .clone()
            .replace("admin", &deployment.tenant_id.clone());

        format!("{}/{}", target_base_url, "v3")
    }

    fn build_job_request(
        &self,
        model_id: &str,
        model_revision: &str,
        deployment: &ModelDeployment,
        args: &[DecryptedArgument],
    ) -> Result<ReqSubmitJob, ReconciliationError> {
        let mut job_def = self.base_job_definition.clone();

        let job_name = format!("MLHub-FlexServ-{}", deployment.id());

        job_def.name = job_name.clone();
        job_def.app_id = Self::FLEXSERV_APP_ID.into();
        job_def.app_version = Self::FLEXSERV_APP_VERSION.into();
        job_def.tenant = Some(deployment.tenant_id.clone());
        job_def.owner = Some(deployment.owner.clone());

        // Set the model id
        let target_spec = job_def
            .parameter_set
            .as_mut()
            .and_then(|params| params.app_args.as_mut())
            .and_then(|args| {
                args.iter_mut()
                    .find(|spec| spec.name.as_deref() == Some("modelName"))
            });

        if let Some(spec) = target_spec {
            spec.arg = Some(format!("--model-name {}", model_id));
        }

        let target_spec = job_def
            .parameter_set
            .as_mut()
            .and_then(|params| params.app_args.as_mut())
            .and_then(|args| {
                args.iter_mut()
                    .find(|spec| spec.name.as_deref() == Some("modelRevision"))
            });

        if let Some(spec) = target_spec {
            spec.arg = Some(format!("--model-revision {}", model_revision));
        }

        let flexserv_version = args
            .iter()
            .find(|argument| argument.parameter_name == "flexserv_version")
            .map(|arg| &arg.value)
            .ok_or_else(|| {
                let message: String = "'flexserv_version' not provided in arguments".into();

                let error = InfrastructureError::new_internal();

                log::error!("[{}] Missing argument: {}", error.error_id(), &message);

                ReconciliationError::Fatal(error)
            })?;

        match flexserv_version.as_str() {
            "1.4" => {}
            version => {
                return Err(ReconciliationError::UnsupportedFlexServVersion(
                    version.into(),
                ));
            }
        }

        let target = match &deployment.deployment_option_snapshot.target {
            DeploymentTargetSnapshot::HpcClusterQueue(target) => target,
        };

        let tapis_system_id = Self::resolve_tapis_system_id(&target.cluster_host)?;

        log::debug!(
            "Resolved Tapis execution target for model deployment '{}'; cluster_host='{}'; tapis_system_id='{}'; queue='{}'",
            deployment.id(),
            target.cluster_host,
            tapis_system_id,
            target.queue_name
        );

        job_def.exec_system_id = Some(tapis_system_id.into());
        job_def.exec_system_logical_queue = Some(target.queue_name.clone());

        // Set the slurm allocation
        let slurm_allocation = args
            .iter()
            .find(|a| a.parameter_name == "project_allocation")
            .map(|arg| &arg.value)
            .ok_or_else(|| {
                let message: String = "'project_allocation' not provided in arguments".into();

                let error = InfrastructureError::new_internal();

                log::error!("[{}] Missing argument: {}", error.error_id(), &message);

                ReconciliationError::Fatal(error)
            })?;

        let target_spec = job_def
            .parameter_set
            .as_mut()
            .and_then(|params| params.scheduler_options.as_mut())
            .and_then(|opts| {
                opts.iter_mut()
                    .find(|spec| spec.name.as_deref() == Some("TACC Resource Allocation"))
            });

        if let Some(spec) = target_spec {
            spec.arg = Some(format!("-A {}", slurm_allocation));
        }

        // Set the slurm reservation
        let maybe_slurm_reservation = args
            .iter()
            .find(|a| a.parameter_name == "reservation" && !a.value.is_empty())
            .map(|arg| &arg.value);

        if let Some(slurm_reservation) = maybe_slurm_reservation {
            let scheduler_opts = job_def
                .parameter_set
                .as_mut()
                .and_then(|params| params.scheduler_options.as_mut());

            if let Some(opts) = scheduler_opts {
                opts.push(JobArgSpec {
                    name: Some("Reservation Name".to_string()),
                    arg: Some(format!("-R {}", slurm_reservation)),
                    description: Some("The Slurm Reservation (set by MLHub)".into()),
                    include: Some(true),
                    notes: None,
                });
            }
        }

        // Update the slurm job name.
        // TODO Remove this when the following issue is resolved.
        // https://github.com/tapis-project/tapis-jobs/issues/181
        job_def
            .parameter_set
            .as_mut()
            .and_then(|params| params.scheduler_options.as_mut())
            .map(|opts| {
                opts.push(JobArgSpec {
                    name: Some("Slurm job name".into()),
                    description: Some("The name of the slurm job (overwritten by MLHub).".into()),
                    arg: Some(format!("--job-name tap_{}", &job_name)),
                    notes: None,
                    include: Some(true),
                });
            });

        Ok(job_def)
    }
}

#[async_trait::async_trait]
impl ModelDeploymentReconciliationClient for TapisJobsModelDeploymentReconciliationClient {
    async fn reconcile(&self, input: ReconcileModelDeploymentInput) -> ReconciliationOutcome {
        let deployment_id = *input.deployment.id();
        let action_name = match &input.action {
            ReconciliationAction::Start { .. } => "Start",
            ReconciliationAction::Stop => "Stop",
            ReconciliationAction::Undeploy => "Undeploy",
        };

        log::debug!(
            "Tapis Jobs reconciler received action for model deployment '{}'; action='{}'",
            deployment_id,
            action_name
        );

        let outcome = match input.action {
            ReconciliationAction::Start { payload } => {
                self.handle_start(&input.deployment, &input.external_model, &payload)
                    .await
            }
            ReconciliationAction::Stop => self.handle_stop(&input).await,
            ReconciliationAction::Undeploy => self.handle_undeploy(&input).await,
        };

        match outcome {
            Ok(outcome) => {
                log::debug!(
                    "Tapis Jobs reconciliation completed for model deployment '{}'",
                    deployment_id
                );

                outcome
            }
            Err(error) => {
                log::error!(
                    "Tapis Jobs reconciliation failed for model deployment '{}': {}",
                    deployment_id,
                    error
                );

                ReconciliationOutcome::Failed(FailedOutcome {
                    message: Some(error.to_string()),
                    disposition: match error {
                        ReconciliationError::Recoverable(_) => FailureDisposition::Recoverable,
                        _ => FailureDisposition::Terminal,
                    },
                    metadata: None,
                    replicas: None,
                    target_endpoint: Some(ModelDeploymentEndpointDelta::Delete),
                })
            }
        }
    }

    async fn observe(&self, deployment: ModelDeployment) -> ObservedOutcome {
        let outcome = self.handle_observe(&deployment).await;

        match outcome {
            Ok(o) => o,
            Err(e) => ObservedOutcome {
                message: Some(e.to_string().clone()),
                state: deployment.state.clone(),
                batch_execution_outcome: deployment.batch_execution_outcome().cloned(),
                failure_disposition: Some(FailureDisposition::Recoverable),
                metadata: None,
                replicas: None,
                target_endpoint: Some(ModelDeploymentEndpointDelta::NoChange),
            },
        }
    }

    fn get_site_context(&self) -> &SiteContext {
        return &self.site_context;
    }
}

#[cfg(test)]
#[path = "tapis_jobs.test.rs"]
mod tapis_jobs_test;
