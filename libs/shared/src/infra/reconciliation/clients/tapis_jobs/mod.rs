use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_json::json;
use tapis_jobs::models::{JobArgSpec, ReqSubmitJob};
use tapis_jobs::{with_headers, TapisJobs};
use tapis_tokens::models::RefreshToken201Response;
use thiserror::Error;

use crate::application::inputs::deployment::ReconcileModelDeploymentInput;
use crate::application::ports::deployment::ModelDeploymentReconciliationClient;
use crate::application::ports::errors::InfrastructureError;
use crate::application::services::deployment_argument_service::DecryptedArgument;
use crate::application::workflows::reconciliation::{
    FailedOutcome, ObservedOutcome, ReconcilerError, ReconciliationAction, ReconciliationOutcome,
    StartedOutcome, StoppedOutcome, UndeployedOutcome,
};
use crate::domain::entities::deployment::{
    DeploymentTargetSnapshot, ModelDeployment, ModelDeploymentEndpoint,
    ModelDeploymentEndpointDelta, ModelDeploymentMetadata, ModelDeploymentMetadataDelta, Protocol,
    State,
};
use crate::domain::entities::model::external_model::{ExternalModel, ModelLocator, ModelProvider};
use crate::domain::entities::site::SiteContext;

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

        // TODO Check if Job UUID is in the metadata. If so, resubmit instead of submit

        let service_jwt = self.generate_service_token().await?;

        let jobs_client = TapisJobs::new(&self.get_target_base_url(deployment), Some(&service_jwt))
            .map_err(|e| {
                let error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Failed to initialize TapisJobs client: {}",
                    error.error_id(),
                    e.to_string()
                );
                error
            })?;

        let custom_headers = Self::obo_headers(deployment)?;

        let job_def = self.build_job_request(model_id, model_revision, deployment, arguments)?;

        let job_uuid = with_headers(custom_headers, async {
            jobs_client.jobs.submit_job(job_def).await
        })
        .await
        .map_err(|e| {
            let error = InfrastructureError::new_internal();

            log::error!(
                "[{}] Error when submitting FlexServ job to Tapis: {}",
                error.error_id(),
                e.to_string()
            );
            error
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

    async fn resubmit_job() -> Result<ReconciliationOutcome, ReconciliationError> {
        todo!();
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
        let job_uuid = Self::extract_job_uuid(deployment).ok_or_else(|| {
            ReconciliationError::Recoverable(
                "Tapis job identifier is unavailable for observation".into(),
            )
        })?;

        let service_jwt = self.generate_service_token().await?;

        let jobs_client = TapisJobs::new(&self.get_target_base_url(deployment), Some(&service_jwt))
            .map_err(|error| {
                let infrastructure_error = InfrastructureError::new_internal();

                log::error!(
                    "[{}] Failed to initialize Tapis Jobs observation client: {}",
                    infrastructure_error.error_id(),
                    error
                );

                infrastructure_error
            })?;

        let status_response = with_headers(Self::obo_headers(deployment)?, async {
            jobs_client.jobs.get_job_status(&job_uuid).await
        })
        .await
        .map_err(|error| {
            let infrastructure_error = InfrastructureError::new_internal();

            log::error!(
                "[{}] Failed to observe Tapis job status: {}",
                infrastructure_error.error_id(),
                error
            );

            infrastructure_error
        })?;

        let status = status_response
            .result
            .and_then(|result| result.status)
            .ok_or_else(|| {
                ReconciliationError::Recoverable(
                    "Tapis job status response did not contain a status".into(),
                )
            })?;

        let state = Self::state_from_job_status(&status);

        if state != State::Running {
            return Ok(ObservedOutcome {
                message: Some(format!("Observed Tapis job status {status}")),
                state,
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

        Ok(resp)
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

    fn state_from_job_status(status: &str) -> State {
        match status {
            "RUNNING" => State::Running,
            "QUEUED" | "PENDING" | "PROCESSING_INPUTS" | "STAGING_INPUTS" | "STAGING_JOB"
            | "SUBMITTING_JOB" | "ARCHIVING" => State::Unknown,
            "FINISHED" | "COMPLETED" | "CANCELLED" | "CANCELED" | "STOPPED" => State::Stopped,
            "FAILED" => State::Failed,
            "BLOCKED" | "PAUSED" => State::Blocked,
            _ => State::Unknown,
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
        let outcome = match input.action {
            ReconciliationAction::Start { payload } => {
                self.handle_start(&input.deployment, &input.external_model, &payload)
                    .await
            }
            ReconciliationAction::Stop => self.handle_stop(&input).await,
            ReconciliationAction::Undeploy => self.handle_undeploy(&input).await,
        };

        match outcome {
            Ok(o) => o,
            Err(e) => ReconciliationOutcome::Failed(FailedOutcome {
                message: Some(e.to_string().clone()),
                metadata: None,
                replicas: None,
                target_endpoint: Some(ModelDeploymentEndpointDelta::Delete),
            }),
        }
    }

    async fn observe(&self, deployment: ModelDeployment) -> ObservedOutcome {
        let outcome = self.handle_observe(&deployment).await;

        match outcome {
            Ok(o) => o,
            Err(e) => ObservedOutcome {
                message: Some(e.to_string().clone()),
                state: State::Failed,
                metadata: None,
                replicas: None,
                target_endpoint: Some(Self::target_endpoint_deletion(&deployment)),
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
