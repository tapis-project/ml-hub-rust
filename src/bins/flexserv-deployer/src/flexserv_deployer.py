from __future__ import annotations

from asyncio.log import logger
import logging
import os
import math
import zipfile
import json
import requests
from abc import ABC, abstractmethod
from dataclasses import dataclass
from datetime import datetime
from enum import Enum
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor
from threading import Lock
from typing import Any, Dict, Optional
from uuid import uuid4

from huggingface_hub import HfApi, snapshot_download
from pydantic import BaseModel, Field
from tapipy.tapis import Tapis

logger = logging.getLogger(__name__)

def create_zip_archive(source_dir: Path, archive_path: Path) -> None:
	"""Create a zip archive for all files inside source_dir."""
	archive_path.parent.mkdir(parents=True, exist_ok=True)
	if archive_path.exists():
		archive_path.unlink()
	with zipfile.ZipFile(archive_path, "w", zipfile.ZIP_DEFLATED) as zf:
		for file_path in source_dir.rglob("*"):
			if file_path.is_file():
				zf.write(file_path, file_path.relative_to(source_dir))


def _directory_size_bytes(root: Path) -> int:
	return sum(f.stat().st_size for f in root.rglob("*") if f.is_file())


_DEFAULT_WORKER_COUNT = max(2, (os.cpu_count() or 1) * 2)
_GLOBAL_EXECUTOR = ThreadPoolExecutor(max_workers=_DEFAULT_WORKER_COUNT)


@dataclass(frozen=True)
class TenantConfig:
    tenant_host: str = "dev.develop.tapis.io"
    short_sha_id_length: int = 8

    @property
    def base_host(self) -> str:
        return self.tenant_host

    @property
    def base_url(self) -> str:
        return f"https://{self.base_host}"
    
@dataclass(frozen=True)
class ModelInfo:
    model_id: str
    sha: str
    artifact_dir: Path

    @property
    def archive_name(self) -> str:
        return f"{self.sha}.zip"

    @property
    def local_archive(self) -> Path:
        return self.artifact_dir / self.archive_name

class TapisSession:
    def __init__(self, config: TenantConfig) -> None:
        self.config = config
        self._client: Optional[Tapis] = None

    @property
    def client(self) -> Tapis:
        if self._client is None:
            raise RuntimeError("Tapis client not initialized.")
        return self._client

    def authenticate(self, username: Optional[str], password: Optional[str], token: Optional[str]) -> Any:
        if token:
            self._client = Tapis(
                base_url=self.config.base_url,
                access_token=token,
            )
        else:
            self._client = Tapis(
                base_url=self.config.base_url,
                username=username,
                password=password,
                grant_type="password",
            )
            self._client.get_tokens()
        return self._client.access_token.access_token


# def serialize_token_payload(tokens: Any) -> Dict[str, Any]:
# 	if tokens is None:
# 		return {}
# 	if isinstance(tokens, dict):
# 		return tokens
# 	for attr in ("model_dump", "dict"):
# 		if hasattr(tokens, attr):
# 			try:
# 				data = getattr(tokens, attr)()
# 				if isinstance(data, dict):
# 					return data
# 			except Exception:
# 				pass
# 	if hasattr(tokens, "json"):
# 		try:
# 			return json.loads(tokens.json())
# 		except Exception:
# 			pass
# 	if hasattr(tokens, "__dict__"):
# 		return dict(tokens.__dict__)
# 	return {"raw": str(tokens)}
    
class InferenceClient:
    def __init__(self, base_host, secret) -> None:
        self.url = f"https://{base_host}/v1/chat/completions"
        self.flexserv_secret = secret

    def run_smoke_test(self) -> Dict:
        payload = {
            "messages": [{"role": "user", "content": "Suggest a two-word team name"}],
            "max_tokens": 10,
        }
        response = requests.post(
            self.url,
            headers={"Content-Type": "application/json", "X-FlexServ-Secret": self.flexserv_secret},
            data=json.dumps(payload),
            timeout=30,
        )
        status = response.status_code
        payload = response.json()
        return {"status_code": status, "payload": payload}

class DeploymentStage(str, Enum):
	QUEUED = "queued"
	PREPARING_MODEL = "preparing-model"
	UPLOADING_MODEL = "uploading-model"
	PREPARING_DATASET = "preparing-dataset"
	UPLOADING_DATASET = "uploading-dataset"
	DEPLOYING = "deploying"
	MONITORING = "monitoring"
	COMPLETED = "completed"
	FAILED = "failed"


class DeploymentState(BaseModel):
	deployment_id: str
	stage: DeploymentStage = DeploymentStage.QUEUED
	message: str = ""
	success: Optional[bool] = None
	result: Optional[Dict[str, Any]] = None
	error: Optional[str] = None
	metadata: Dict[str, Any] = Field(default_factory=dict)
	created_at: datetime = Field(default_factory=datetime.utcnow)
	updated_at: datetime = Field(default_factory=datetime.utcnow)


class DeploymentRegistry:
	def __init__(self) -> None:
		self._statuses: Dict[str, DeploymentState] = {}
		self._lock = Lock()

	def create(self, deployment_id: Optional[str] = None) -> DeploymentState:
		deployment_id = deployment_id or str(uuid4())
		state = DeploymentState(deployment_id=deployment_id, message="Deployment queued")
		with self._lock:
			self._statuses[deployment_id] = state
		return state

	def update(self, deployment_id: str, **updates: Any) -> DeploymentState:
		with self._lock:
			if deployment_id not in self._statuses:
				raise KeyError(f"Unknown deployment id {deployment_id}")
			state = self._statuses[deployment_id]
			metadata = updates.pop("metadata", None)
			if metadata is not None:
				merged_metadata = {**state.metadata, **metadata}
				updates["metadata"] = merged_metadata
			updates.setdefault("updated_at", datetime.now())
			new_state = state.copy(update=updates)
			self._statuses[deployment_id] = new_state
			return new_state

	def get(self, deployment_id: str) -> Optional[DeploymentState]:
		with self._lock:
			return self._statuses.get(deployment_id)

	def all(self) -> Dict[str, DeploymentState]:
		with self._lock:
			return dict(self._statuses)

	def delete(self, deployment_id: str) -> Optional[DeploymentState]:
		with self._lock:
			return self._statuses.pop(deployment_id, None)


@dataclass
class ModelArtifact:
	repo_id: str
	revision: str
	sha: str
	download_dir: Path
	archive_path: Path
	total_size_bytes: int
	total_archive_size_bytes: int

	@property
	def unpacked_size_mb(self) -> int:
		return math.ceil(self.total_size_bytes / (1024 * 1024))

	@property
	def archive_size_mb(self) -> int:
		return math.ceil(self.total_archive_size_bytes / (1024 * 1024))


@dataclass
class DatasetArtifact:
	repo_id: str
	revision: str
	download_dir: Path
	archive_path: Path
	total_size_bytes: int
	total_archive_size_bytes: int

	@property
	def unpacked_size_mb(self) -> int:
		return math.ceil(self.total_size_bytes / (1024 * 1024))

	@property
	def archive_size_mb(self) -> int:
		return math.ceil(self.total_archive_size_bytes / (1024 * 1024))


class TapisAuthRequest(BaseModel):
	tenant_host: str = "dev.develop.tapis.io"
	tapis_username: Optional[str] = None
	tapis_password: Optional[str] = None
	tapis_token: Optional[str] = None


class TapisAuthResponse(BaseModel):
	tenant_host: str
	tokens: Dict[str, Any]


class FlexDeploymentRequest(BaseModel):
	tenant_host: str = "dev.develop.tapis.io"
	repo_id: str
	revision: str
	force_update_model: bool = False
	force_update_dataset: bool = False
	tapis_username: Optional[str] = None
	tapis_password: Optional[str] = None
	tapis_token: Optional[str] = None
	deployment_id: Optional[str] = None
	deployment_secret: Optional[str] = "flexserv_deployment_secret"
	model_cache_dir: Optional[Path] = None # place for holding downloaded model files. this directory is a copy of the model.
	model_archive_dir: Optional[Path] = None # place for holding zipped model archive
	dataset_repo_id: Optional[str] = None 
	dataset_revision: Optional[str] = None
	dataset_cache_dir: Optional[Path] = None  
	dataset_archive_dir: Optional[Path] = None
	
	
class FlexDeploymentCancelRequest(BaseModel):
	tenant_host: str = "dev.develop.tapis.io"
	delete_model_cache: bool = False
	tapis_username: Optional[str] = None
	tapis_password: Optional[str] = None
	tapis_token: Optional[str] = None

class BaseFlexDeploymentWorkflow(ABC):
	def __init__(
		self,
		registry: DeploymentRegistry,
		poll_interval: int = 5,
		executor: Optional[ThreadPoolExecutor] = None,
		monitor_executor: Optional[ThreadPoolExecutor] = None,
	) -> None:
		self.registry = registry
		self.poll_interval = poll_interval
		self._executor = executor or _GLOBAL_EXECUTOR
		self._monitor_executor = monitor_executor or self._executor

	def submit(self, request: FlexDeploymentRequest, api: HfApi) -> DeploymentState:
		deployment_id = request.deployment_id or str(uuid4())
		request.deployment_id = deployment_id
		state = self.registry.create(deployment_id)
		self._executor.submit(self._run_workflow, deployment_id, request, api)
		return state

	def _run_workflow(self, deployment_id: str, request: FlexDeploymentRequest, api: HfApi) -> None:
		try:
			model = self._prepare_model_artifact(deployment_id, request, api)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.UPLOADING_MODEL,
				message="Uploading model artifacts",
				metadata={"model_sha": model.sha},
			)
			self.upload_model_artifact(deployment_id, request, model)

			dataset: Optional[DatasetArtifact] = None
			if request.dataset_repo_id:
				dataset = self._prepare_dataset_artifact(deployment_id, request, api)
				self.registry.update(
					deployment_id,
					stage=DeploymentStage.UPLOADING_DATASET,
					message="Uploading dataset artifacts",
				)
				self.upload_dataset_artifact(deployment_id, request, dataset)

			self.registry.update(
				deployment_id,
				stage=DeploymentStage.DEPLOYING,
				message="Executing deployment",
			)
			handle = self.execute_deployment(deployment_id, request, model, dataset)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.MONITORING,
				message="Monitoring deployment",
			)
			self._start_monitoring_thread(deployment_id, request, model, dataset, handle)
		except Exception as exc:  
			logger.exception("Deployment %s failed", deployment_id)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.FAILED,
				success=False,
				error=str(exc),
				message="Deployment failed",
			)

	def _start_monitoring_thread(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		model: ModelArtifact,
		dataset: Optional[DatasetArtifact],
		handle: Any,
	) -> None:
		self._monitor_executor.submit(
			self._monitor_wrapper,
			deployment_id,
			request,
			model,
			dataset,
			handle,
		)

	def _monitor_wrapper(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		model: ModelArtifact,
		dataset: Optional[DatasetArtifact],
		handle: Any,
	) -> None:
		try:
			result = self.monitor_deployment(deployment_id, request, model, dataset, handle)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.COMPLETED,
				success=True,
				message="Deployment completed",
				result=result,
			)
		except Exception as exc:  
			logger.exception("Monitoring for deployment %s failed", deployment_id)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.FAILED,
				success=False,
				error=str(exc),
				message="Deployment failed during monitoring",
			)

	def _prepare_model_artifact(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		api: HfApi,
	) -> ModelArtifact:
		self.registry.update(
			deployment_id,
			stage=DeploymentStage.PREPARING_MODEL,
			message="Fetching model metadata",
		)
		info = api.model_info(repo_id=request.repo_id, revision=request.revision)
		sha = getattr(info, "sha", None)
		if not sha:
			raise ValueError("Model metadata lacks a sha field")

		cache_root = request.model_cache_dir or (Path.home() / ".models" / sha)
		cache_root.mkdir(parents=True, exist_ok=True)
		if cache_root.exists() and any(cache_root.iterdir()) and not request.force_update_model:
			download_dir = cache_root
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_MODEL,
				message="Model files seem to be cached already; using cached model files",
			)
		else:
			download_dir = Path(
				snapshot_download(
					request.repo_id,
					revision=request.revision,
					cache_dir=str(cache_root),
					local_dir=str(cache_root),
					local_dir_use_symlinks=False,
				)
			)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_MODEL,
				message="Downloading model files",
			)

		archive_dir = request.model_archive_dir or (Path.home() / ".models")
		archive_path = archive_dir / f"{sha}.zip"

		self.registry.update(
			deployment_id,
			stage=DeploymentStage.PREPARING_MODEL,
			message="Creating model archive",
		)
		# test if the model archive already exists
		if archive_path.exists() and not request.force_update_model:
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_MODEL,
				message="Model archive seems to exist already; using cached archive",
			)
		else:
			create_zip_archive(download_dir, archive_path)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_MODEL,
				message="Model archive created",
			)

		# record model archive path and model download_dir in deployment metadata
		self.registry.update(
			deployment_id,
			metadata={
				"model_archive_path": str(archive_path),
				"model_download_dir": str(download_dir),
			},
		)

		artifact = ModelArtifact(
			repo_id=request.repo_id,
			revision=request.revision,
			sha=sha,
			download_dir=download_dir,
			archive_path=archive_path,
			total_size_bytes=_directory_size_bytes(download_dir),
			total_archive_size_bytes=archive_path.stat().st_size,
		)
		self.registry.update(
			deployment_id,
			message="Model artifact prepared",
			metadata={"model_sha": sha, "model_size_mb": artifact.unpacked_size_mb + artifact.archive_size_mb},
		)
		return artifact

	def _prepare_dataset_artifact(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		api: HfApi,
	) -> DatasetArtifact:
		if not request.dataset_repo_id:
			raise ValueError("dataset_repo_id not provided")
		self.registry.update(
			deployment_id,
			stage=DeploymentStage.PREPARING_DATASET,
			message="Fetching dataset metadata",
		)
		revision = request.dataset_revision or "main"
		safe_repo = request.dataset_repo_id.replace("/", "_")
		cache_root = request.dataset_cache_dir or (Path.home() / ".datasets" / safe_repo / revision)
		cache_root.mkdir(parents=True, exist_ok=True)
		if cache_root.exists() and any(cache_root.iterdir()) and not request.force_update_dataset:
			download_dir = cache_root
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_DATASET,
				message="Dataset files seem to be cached already; using cached dataset files",
			)
		else:
			download_dir = Path(
				snapshot_download(
					request.dataset_repo_id,
					revision=revision,
					cache_dir=str(cache_root),
					local_dir=str(cache_root),
					local_dir_use_symlinks=False,
					repo_type="dataset",
				)
			)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_DATASET,
				message="Downloading dataset files",
			)

		archive_dir = request.dataset_archive_dir or (Path.home() / ".datasets" / safe_repo)
		archive_dir.mkdir(parents=True, exist_ok=True)
		archive_path = archive_dir / f"{revision}.zip"
		if archive_path.exists() and not request.force_update_dataset:
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_DATASET,
				message="Dataset archive seems to exist already; using cached archive",
			)
		else:
			create_zip_archive(download_dir, archive_path)
			self.registry.update(
				deployment_id,
				stage=DeploymentStage.PREPARING_DATASET,
				message="Dataset archive created",
			)

		# record model archive path and model download_dir in deployment metadata
		self.registry.update(
			deployment_id,
			metadata={
				"dataset_archive_path": str(archive_path),
				"dataset_download_dir": str(download_dir),
			},
		)

		artifact = DatasetArtifact(
			repo_id=request.dataset_repo_id,
			revision=revision,
			download_dir=download_dir,
			archive_path=archive_path,
			total_size_bytes=_directory_size_bytes(download_dir),
			total_archive_size_bytes=archive_path.stat().st_size,
		)
		self.registry.update(
			deployment_id,
			message="Dataset artifact prepared",
			metadata={
				"dataset_revision": revision,
				"dataset_size_mb": artifact.unpacked_size_mb + artifact.archive_size_mb,
			},
		)
		return artifact

	@abstractmethod
	def upload_model_artifact(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		artifact: ModelArtifact,
	) -> None:
		raise NotImplementedError

	def upload_dataset_artifact(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		artifact: Optional[DatasetArtifact],
	) -> None:
		# Optional for subclasses
		if artifact:
			raise NotImplementedError("Dataset uploads not implemented for this workflow")

	def cancel_deployment(
		self,
		deployment_id: str,
		request: FlexDeploymentCancelRequest,
	) -> Dict[str, Any]:
		if request.delete_model_cache:
			# delete model cache directory
			state = self.registry.get(deployment_id)
			if state and "model_download_dir" in state.metadata:
				model_dir = Path(state.metadata["model_download_dir"])
				if model_dir.exists() and model_dir.is_dir():
					for item in model_dir.rglob("*"):
						if item.is_file():
							item.unlink()
					model_dir.rmdir()
			# delete model archive file
			if state and "model_archive_path" in state.metadata:
				archive_path = Path(state.metadata["model_archive_path"])
				if archive_path.exists() and archive_path.is_file():
					archive_path.unlink()
			# delete dataset cache directory
			if state and "dataset_download_dir" in state.metadata:
				dataset_dir = Path(state.metadata["dataset_download_dir"])
				if dataset_dir.exists() and dataset_dir.is_dir():
					for item in dataset_dir.rglob("*"):
						if item.is_file():
							item.unlink()
					dataset_dir.rmdir()
			# delete dataset archive file
			if state and "dataset_archive_path" in state.metadata:
				dataset_archive_path = Path(state.metadata["dataset_archive_path"])
				if dataset_archive_path.exists() and dataset_archive_path.is_file():
					dataset_archive_path.unlink()
		return self.revoke_deployment(deployment_id, request)

	@abstractmethod
	def execute_deployment(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		model: ModelArtifact,
		dataset: Optional[DatasetArtifact],
	) -> Any:
		raise NotImplementedError

	@abstractmethod
	def monitor_deployment(
		self,
		deployment_id: str,
		request: FlexDeploymentRequest,
		model: ModelArtifact,
		dataset: Optional[DatasetArtifact],
		handle: Any,
	) -> Dict[str, Any]:
		raise NotImplementedError

	@abstractmethod
	def revoke_deployment(
		self,
		deployment_id: str,
		request: FlexDeploymentCancelRequest,
	) -> Dict[str, Any]:
		raise NotImplementedError
