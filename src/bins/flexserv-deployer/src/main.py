# main.py
from typing import Optional, List

from fastapi import FastAPI, Depends, Query, Header, HTTPException
from fastapi.encoders import jsonable_encoder
from huggingface_hub import HfApi

from flexserv_deployer import (
    DeploymentRegistry,
    DeploymentState,
    TapisAuthRequest,
    TapisAuthResponse,
)
from pod_deployer import (
    PodDeploymentRequest,
    PodDeploymentWorkflow,
    PodDeploymentCancelRequest,
    TenantConfig,
    TapisSession,
)

app = FastAPI(title="HF Models Wrapper API")

deployment_registry = DeploymentRegistry()


# Dependency to create an HfApi client, optionally with a token
def get_hf_api(
    hf_token: Optional[str] = Header(
        default=None,
        alias="X-HF-Token",  # custom header name, e.g. X-HF-Token: <your_token>
    ),
) -> HfApi:
    # If hf_token is None, this will just use anon or env-configured token
    return HfApi(token=hf_token)


@app.get("/models")
async def list_models(
    search: Optional[str] = Query(
        None, description="Substring search on repo or username"
    ),
    author: Optional[str] = Query(None, description="Filter by author/org name"),
    filter: Optional[str] = Query(
        None,
        alias="filter",
        description="Tag filter, e.g. text-classification, spacy",
    ),
    sort: Optional[str] = Query(
        "lastModified",
        description="Property to sort by, e.g. downloads, author",
    ),
    direction: Optional[int] = Query(
        -1,
        description="Sort direction: -1 for descending, anything else for ascending",
    ),
    limit: Optional[int] = Query(
        50,
        ge=1,
        le=1000,
        description="Maximum number of models to return",
    ),
    full: bool = Query(
        False,
        description="If True, fetch extended metadata (tags, files, etc.)",
    ),
    config: bool = Query(
        False,
        description="If True, include repo config in results",
    ),
    api: HfApi = Depends(get_hf_api),
):
    """
    Wrapper for HF GET /api/models (huggingface_hub.list_models).
    """
    try:
        models_iter = api.list_models(
            search=search,
            author=author,
            filter=filter,
            sort=sort,
            direction=direction,
            limit=limit,
            full=full,
            fetch_config=config,
        )
        models = list(models_iter)
    except Exception as e:
        raise HTTPException(status_code=502, detail=f"Hugging Face Hub error: {str(e)}")

    # Convert to JSON-serializable structures
    return jsonable_encoder(models)


@app.get("/models/{path_after_models:path}")
async def get_model_info(
    path_after_models: str,
    revision: Optional[str] = Query(
        None,
        description="Specific revision (branch, tag, or commit SHA)",
    ),
    security_status: Optional[bool] = Query(
        False,
        alias="securityStatus",
        description="Whether to include security status info",
    ),
    files_metadata: Optional[bool] = Query(
        False,
        description="Whether to include metadata for files (size, LFS metadata, etc.)",
    ),
    api: HfApi = Depends(get_hf_api),
):
    """
    Wrapper for HF GET /api/models/{repo_id} (huggingface_hub.model_info).

    Supports both `/models/{repo_id}` and `/models/{repo_id}/revisions/{revision}`
    forms by parsing the path segment manually so repo IDs with slashes remain intact.
    """
    repo_id = path_after_models
    revision_from_path: Optional[str] = None

    if "/revisions/" in path_after_models:
        repo_segment, revision_segment = path_after_models.split("/revisions/", 1)
        if not repo_segment:
            raise HTTPException(
                status_code=400, detail="Missing repo_id before /revisions/"
            )
        repo_id = repo_segment
        revision_from_path = revision_segment or None

    effective_revision = revision_from_path or revision

    try:
        info = api.model_info(
            repo_id=repo_id,
            revision=effective_revision,
            securityStatus=security_status,
            files_metadata=files_metadata,
        )
    except Exception as e:
        raise HTTPException(status_code=502, detail=f"Hugging Face Hub error: {str(e)}")

    return jsonable_encoder(info)


@app.post("/pod_deployment", response_model=DeploymentState)
def deploy_model(request: PodDeploymentRequest, api: HfApi = Depends(get_hf_api)):
    workflow = PodDeploymentWorkflow(
        registry=deployment_registry,
        poll_interval=request.poll_interval,
    )
    try:
        state = workflow.submit(request, api)
    except Exception as exc:
        raise HTTPException(status_code=500, detail=str(exc))
    return state


@app.get("/deployments", response_model=List[DeploymentState])
def list_deployments():
    return list(deployment_registry.all().values())


@app.get("/deployments/{deployment_id}", response_model=DeploymentState)
def get_deployment(deployment_id: str):
    state = deployment_registry.get(deployment_id)
    if state is None:
        raise HTTPException(status_code=404, detail="Deployment not found")
    return state


@app.delete("/deployments/{deployment_id}")
def delete_deployment(deployment_id: str, payload: PodDeploymentCancelRequest):
    workflow = PodDeploymentWorkflow(registry=deployment_registry)
    try:
        result = workflow.cancel_deployment(
            deployment_id,
            payload,
        )
    except Exception as exc:
        raise HTTPException(status_code=502, detail=str(exc))

    return result


@app.post("/tapis_auth", response_model=TapisAuthResponse)
def tapis_auth(payload: TapisAuthRequest):
    if not payload.tapis_token and (
        not payload.tapis_username or not payload.tapis_password
    ):
        raise HTTPException(
            status_code=400,
            detail="Provide either tapis_token or both tapis_username and tapis_password",
        )

    tenant_cfg = TenantConfig(payload.tenant_host)
    session = TapisSession(tenant_cfg)

    if payload.tapis_token:
        jwt = session.authenticate(None, None, payload.tapis_token)
        token_payload = {
            "access_token": payload.tapis_token,
            "token_source": "provided",
        }
    else:
        try:
            jwt = session.authenticate(
                payload.tapis_username, payload.tapis_password, None
            )
            print(type(jwt))
        except Exception as exc:
            raise HTTPException(
                status_code=502, detail=f"Tapis authentication failed: {exc}"
            )
        token_payload = {
            "access_token": jwt,
            "token_source": "password",
        }
    return TapisAuthResponse(tenant_host=payload.tenant_host, tokens=token_payload)
