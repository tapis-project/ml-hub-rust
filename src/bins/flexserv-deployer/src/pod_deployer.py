from __future__ import annotations

import time
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass
from threading import Lock
from typing import Any, Dict, Iterable, List, Optional

import logging
from pydantic import Field
from tapipy.tapis import Tapis

from flexserv_deployer import (
    BaseFlexDeploymentWorkflow,
    DatasetArtifact,
    DeploymentRegistry,
    DeploymentStage,
    FlexDeploymentRequest,
    FlexDeploymentCancelRequest,
    ModelArtifact,
    InferenceClient,
    TapisSession,
    ModelInfo,
    TenantConfig,
)

logger = logging.getLogger(__name__)

class PodDeploymentRequest(FlexDeploymentRequest):
    volume_id: Optional[str] = None
    volume_size_limit: int = 10240
    image: str = "zhangwei217245/ai-serving-py-transformers:latest"
    command: List[str] = Field(default_factory=lambda: ["python", "/app/main.py"])
    arguments: List[str] = Field(default_factory=list)
    networking: Dict[str, Dict[str, str]] = Field(
        default_factory=lambda: {
            "default": {
                "protocol": "http",
                "port": 8000,
            }
        }
    )
    model_mount_path: str = "/models"
    dataset_mount_path: str = "/datasets"
    pod_id: Optional[str] = None
    cleanup: bool = True
    poll_interval: int = 5
    extra_env: Dict[str, str] = Field(default_factory=dict)
    resources: Optional[Dict[str, int]] = None
    success_log_marker: str = "Successfully loaded model"


class PodDeploymentCancelRequest(FlexDeploymentCancelRequest):
    pass

@dataclass(frozen=True)
class VolumeSpec:
    volume_id: str
    description: str
    size_limit: int

@dataclass(frozen=True)
class PodSpec:
    pod_id: str
    image: str
    command: Iterable[str]
    arguments: Iterable[str]
    env: Dict[str, str]
    volume_mounts: Dict[str, Dict[str, str]]
    networking: Dict[str, Dict[str, str]]
    resources: Dict[str, int]
    description: str

class VolumeManager:
    def __init__(self, tapis: Tapis) -> None:
        self.tapis = tapis

    def delete_volume(self, volume_id: str) -> None:
        try:
            self.tapis.pods.delete_volume(volume_id=volume_id)
        except Exception:
            pass

    def create_volume(self, spec: VolumeSpec) -> None:
        self.tapis.pods.create_volume(
            volume_id=spec.volume_id,
            description=spec.description,
            size_limit=spec.size_limit,
        )

    def upload_model(self, spec: VolumeSpec, model: ModelInfo) -> None:
        with open(model.local_archive, "rb") as file:
            self.tapis.pods.upload_to_volume(
                volume_id=spec.volume_id,
                path=model.archive_name,
                file=file,
            )

    def list_files(self, volume_id: str) -> Dict:
        return self.tapis.pods.list_volume_files(volume_id=volume_id, path="/")


class PodManager:
    def __init__(self, tapis: Tapis, poll_interval: int = 5) -> None:
        self.tapis = tapis
        self.poll_interval = poll_interval

    def delete_pod(self, pod_id: str) -> None:
        try:
            self.tapis.pods.delete_pod(pod_id=pod_id)
        except Exception:
            pass

    def create_pod(self, spec: PodSpec) -> Dict:
        return self.tapis.pods.create_pod(
            pod_id=spec.pod_id,
            image=spec.image,
            command=list(spec.command),
            arguments=list(spec.arguments),
            environment_variables=spec.env,
            volume_mounts=spec.volume_mounts,
            networking=spec.networking,
            resources=spec.resources,
            description=spec.description,
        )

    def check_status_until_ready(self, pod_id: str, success_marker: str) -> None:
        total_timeout = 600  # 10 minutes
        elapsed = 0
        while True and elapsed < total_timeout:
            try:
                if self.tapis.pods.get_pod(pod_id=pod_id).status == "AVAILABLE":
                    logger.info(f"Pod {pod_id} is AVAILABLE.")
                    log_result = self.tapis.pods.get_pod_logs(pod_id=pod_id).logs
                    print("----- pod logs -----")
                    print(log_result)
                    if success_marker in log_result:
                        print("Inference server pod is ready.")
                        break
            except Exception as e:
                # TODO: contact Chris to see how to improve responsiveness of this API when pod is starting.
                logger.warning(f"Error while checking pod status: {e}")
            time.sleep(self.poll_interval)
            elapsed += self.poll_interval


@dataclass
class PodDeploymentContext:
    tenant_cfg: TenantConfig
    model_info: ModelInfo
    request: PodDeploymentRequest
    model_volume_spec: VolumeSpec
    dataset_volume_spec: Optional[VolumeSpec] = None
    pod_spec: Optional[PodSpec] = None
    pod_result: Optional[Any] = None
    pod_summary: Optional[Dict[str, Any]] = None

class PodDeploymentWorkflow(BaseFlexDeploymentWorkflow):
    def __init__(
        self,
        registry: DeploymentRegistry,
        poll_interval: int = 5,
        executor: Optional[ThreadPoolExecutor] = None,
        monitor_executor: Optional[ThreadPoolExecutor] = None,
    ) -> None:
        super().__init__(registry, poll_interval, executor, monitor_executor)
        self._contexts: Dict[str, PodDeploymentContext] = {}
        self._ctx_lock = Lock()
        self._volume_buffer_mb = 1024

    def upload_model_artifact(
        self,
        deployment_id: str,
        request: FlexDeploymentRequest,
        artifact: ModelArtifact,
    ) -> None:
        pod_request = self._ensure_pod_request(request)
        tenant_cfg = TenantConfig(pod_request.tenant_host)
        model_info = ModelInfo(
            model_id=pod_request.repo_id,
            sha=artifact.sha,
            artifact_dir=artifact.archive_path.parent,
        )

        volume_id = pod_request.volume_id or artifact.sha[:tenant_cfg.short_sha_id_length]
        # if volume_id is not started with an lowercase alpha, prepend 'v' to make it valid.
        if not volume_id[0].isalpha() or not volume_id[0].islower():
            volume_id = f"v{volume_id}"
        computed_limit = artifact.unpacked_size_mb + artifact.archive_size_mb + self._volume_buffer_mb
        size_limit = max(pod_request.volume_size_limit, computed_limit)
        volume_spec = VolumeSpec(
            volume_id=volume_id,
            description=f"Volume for {artifact.repo_id}@{artifact.revision}",
            size_limit=size_limit,
        )

        session = self._build_session(tenant_cfg, pod_request)
        volume_mgr = VolumeManager(session.client)
        
        # check if volume exists and if the size of the file is correct.
        # if so, we just reuse the volume. 
        # otherwise, we delete and recreate it.
        try:
            files = volume_mgr.list_files(volume_spec.volume_id)
            for f in files:
                # TODO: we need to probably run a different template pod to get the checksum of the file in a volume. 
                # we can run the pod quickly to mount the volume and once the checksum is aquired, we can delete the pod.
                # but this quick-and-dirty check should work for now.
                if f.name == model_info.archive_name and f.size == artifact.total_archive_size_bytes:
                    logger.info(f"Volume {volume_spec.volume_id} already has correct model file; skipping upload")
                    self.registry.update(
                        deployment_id,
                        message="Model already present in volume; skipping upload",
                        metadata={"volume_id": volume_spec.volume_id, "model_sha": artifact.sha},
                    )
                    with self._ctx_lock:
                        self._contexts[deployment_id] = PodDeploymentContext(
                            tenant_cfg=tenant_cfg,
                            model_info=model_info,
                            model_volume_spec=volume_spec,
                            request=pod_request,
                        )
                    return 
        except Exception:
            pass  # volume does not exist or cannot be accessed; proceed to create it
        
        try:
            if pod_request.cleanup:
                volume_mgr.delete_volume(volume_spec.volume_id)
        except Exception:
            logger.warning(f"Failed to delete existing volume {volume_spec.volume_id}; continuing")
            pass  # ignore errors during deletion

        volume_mgr.create_volume(volume_spec)
        volume_mgr.upload_model(volume_spec, model_info)

        self.registry.update(
            deployment_id,
            message="Model uploaded to volume",
            metadata={"volume_id": volume_spec.volume_id, "model_sha": artifact.sha},
        )

        with self._ctx_lock:
            self._contexts[deployment_id] = PodDeploymentContext(
                tenant_cfg=tenant_cfg,
                model_info=model_info,
                model_volume_spec=volume_spec,
                request=pod_request,
            )

    def upload_dataset_artifact(
        self,
        deployment_id: str,
        request: FlexDeploymentRequest,
        artifact: Optional[DatasetArtifact],
    ) -> None:
        # Pods currently ignore dataset uploads; hook provided for future use.
        logger.info("Dataset upload step skipped for pod deployment")
        pass

    def execute_deployment(
        self,
        deployment_id: str,
        request: FlexDeploymentRequest,
        model: ModelArtifact,
        dataset: Optional[DatasetArtifact],
    ) -> Dict[str, Any]:
        ctx = self._get_context(deployment_id)
        session = self._build_session(ctx.tenant_cfg, ctx.request)
        pod_mgr = PodManager(session.client, self.poll_interval)

        pod_spec = self._build_pod_spec(ctx, model)
        if ctx.request.cleanup:
            pod_mgr.delete_pod(pod_spec.pod_id)
        pod_creation_result = pod_mgr.create_pod(pod_spec)

        pod_summary = {
            "pod_id": pod_spec.pod_id, 
            "tenant": ctx.tenant_cfg.tenant_host, 
            "pod_host": f"{pod_creation_result.networking.default.url}"
        }
        
        with self._ctx_lock:
            self._contexts[deployment_id].pod_spec = pod_spec
            self._contexts[deployment_id].pod_result = pod_creation_result
            self._contexts[deployment_id].pod_summary = pod_summary
            
        self.registry.update(
            deployment_id,
            message=f"Pod {pod_spec.pod_id} created",
            metadata=pod_summary,
        )
        return pod_summary

    def monitor_deployment(
        self,
        deployment_id: str,
        request: FlexDeploymentRequest,
        model: ModelArtifact,
        dataset: Optional[DatasetArtifact],
        handle: Any,
    ) -> Dict[str, Any]:
        ctx = self._get_context(deployment_id)
        session = self._build_session(ctx.tenant_cfg, ctx.request)
        pod_mgr = PodManager(session.client, self.poll_interval)
        pod_spec = ctx.pod_spec
        if pod_spec is None:
            raise RuntimeError("Pod spec missing for deployment")

        self.registry.update(
            deployment_id,
            stage=DeploymentStage.MONITORING,
            message=f"checking status for pod {pod_spec.pod_id}",
        )
        pod_mgr.check_status_until_ready(pod_spec.pod_id, ctx.request.success_log_marker)
        self.registry.update(
            deployment_id,
            message=f"Pod {pod_spec.pod_id} is up and running, proceeding to smoke test",
        )
        inference = InferenceClient(ctx.pod_summary["pod_host"], ctx.pod_spec.env.get("FLEXSERV_SECRET", ""))
        try:
            smoke_result = inference.run_smoke_test()
            if smoke_result["status_code"] != 200:
                self.registry.update(
                    deployment_id,
                    stage = DeploymentStage.FAILED,
                    message="Smoke test failed",
                    metadata={"smoke_test_result": smoke_result},
                )
                raise RuntimeError(f"Smoke test failed with status {smoke_result['status_code']}")
            self.registry.update(
                deployment_id,
                stage=DeploymentStage.COMPLETED,
                message="Smoke test passed",
                metadata={"smoke_test_result": smoke_result},
            )
        finally:
            with self._ctx_lock:
                self._contexts.pop(deployment_id, None)
        return {"smoke_test": smoke_result}

    def _build_session(self, tenant_cfg: TenantConfig, request: "PodDeploymentRequest") -> TapisSession:
        session = TapisSession(tenant_cfg)
        session.authenticate(request.tapis_username, request.tapis_password, request.tapis_token)
        return session

    def _build_pod_spec(self, ctx: PodDeploymentContext, model: ModelArtifact) -> PodSpec:
        req = ctx.request
        env = {"MODEL_HASH": model.sha,  "FLEXSERV_SECRET": req.deployment_secret, **req.extra_env}
        volume_mounts = {
            ctx.model_volume_spec.volume_id: {
                "type": "tapisvolume",
                "mount_path": req.model_mount_path,
            }
        }
        if ctx.dataset_volume_spec:
            volume_mounts[ctx.dataset_volume_spec.volume_id] = {
                "type": "tapisvolume",
                "mount_path": req.dataset_mount_path,
            }
        # TODO: resource estimator here. 
        resources = req.resources or {
            "cpu_request": 10001,
            "cpu_limit": 10001,
            "mem_request": 15001,
            "mem_limit": 15001,
            "gpus": 0,
        }
        resolved_pod_id = req.pod_id or f"{model.sha[:8]}"
        if not resolved_pod_id[0].isalpha() or not resolved_pod_id[0].islower():
            resolved_pod_id = f"p{resolved_pod_id}"
        return PodSpec(
            pod_id=resolved_pod_id,
            image=req.image,
            command=req.command,
            arguments=req.arguments,
            env=env,
            volume_mounts=volume_mounts,
            networking=req.networking,
            resources=resources,
            description=f"Inference pod for {req.repo_id}@{req.revision}",
        )

    def _get_context(self, deployment_id: str) -> PodDeploymentContext:
        with self._ctx_lock:
            if deployment_id not in self._contexts:
                raise KeyError(f"Missing deployment context for {deployment_id}")
            return self._contexts[deployment_id]

    @staticmethod
    def _ensure_pod_request(request: FlexDeploymentRequest) -> "PodDeploymentRequest":
        if not isinstance(request, PodDeploymentRequest):
            raise TypeError("PodDeploymentWorkflow expects a PodDeploymentRequest")
        return request
    
    @staticmethod
    def _ensure_pod_cancel_request(request: FlexDeploymentCancelRequest) -> "PodDeploymentCancelRequest":
        if not isinstance(request, PodDeploymentCancelRequest):
            raise TypeError("PodDeploymentWorkflow expects a PodDeploymentCancelRequest")
        return request
    

    def revoke_deployment(
        self,
        deployment_id: str,
        request: PodDeploymentCancelRequest,
    ) -> Dict[str, Any]:
        request = self._ensure_pod_cancel_request(request)
        tenant_cfg = TenantConfig(request.tenant_host)
        session = TapisSession(tenant_cfg)
        session.authenticate(request.tapis_username, request.tapis_password, request.tapis_token)
        tapis_client = session.client
        
        state = self.registry.get(deployment_id)
        if state is None: # TODO: better exception types need to be defined.
            raise RuntimeError(f"Deployment {deployment_id} not found")
        
        pod_id = state.metadata.get("pod_id")
        volume_id = state.metadata.get("volume_id")
        actions: List[str] = []
        
        if pod_id:
            try:
                tapis_client.pods.delete_pod(pod_id=pod_id)
                actions.append(f"Pod {pod_id} deleted")
            except Exception as exc:  
                raise RuntimeError(f"Failed to delete pod {pod_id}: {exc}")
        else:
            actions.append("No pod_id recorded; skipping pod deletion")

        if request.delete_model_cache:
            if volume_id:
                try:
                    tapis_client.pods.delete_volume(volume_id=volume_id)
                    actions.append(f"Volume {volume_id} deleted")
                except Exception as exc:  
                    raise RuntimeError(f"Failed to delete volume {volume_id}: {exc}")
            else:
                actions.append("No volume_id recorded; skipping volume deletion")
            
        self.registry.delete(deployment_id)
        actions.append("Deployment removed from registry")

        return {
            "deployment_id": deployment_id,
            "pod_id": pod_id,
            "volume_id": volume_id if request.delete_model_cache else None,
            "actions": actions,
        }
