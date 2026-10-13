# MLHub User Guide

MLHub provides APIs for discovering AI models, determining where they can run, creating model
deployments, registering datasets, and publishing AI agent descriptions.

This guide is for people using a deployed MLHub environment. For installation and development
instructions, see the [deployment guide](./DEPLOYMENT.md) and
[developer guide](./DEVELOPERS_GUIDE.md).

> MLHub is under active development. The checked-in OpenAPI specifications are the authoritative
> references for the currently supported request and response fields.

## API access

Ask your MLHub administrator for the base URL of your environment and obtain a valid Tapis access
token. The examples in this guide use shell variables so that credentials are not placed directly
in commands or shell history:

```shell
export MLHUB_URL="https://<mlhub-host>"
export TAPIS_TOKEN="<access-token>"
```

Send the token in the `X-Tapis-Token` header:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/deployments-api/data-centers/Tacc/hpc-clusters"
```

Do not commit access tokens to source control, configuration files, or documentation.

## Response envelopes

MLHub APIs return a common JSON envelope:

```json
{
  "status": 200,
  "message": "Successfully retrieved resources",
  "result": [],
  "metadata": {},
  "version": "..."
}
```

The resource or resource list is under `result`. Paginated operations may return these fields under
`metadata`:

```json
{
  "cursor": "<opaque-next-page-cursor>",
  "count": 1250
}
```

Pass the returned cursor unchanged to retrieve the next page. Do not parse it or construct your own
cursor. Add `include_count=true` when a total count is needed; counting may be more expensive than a
normal page request.

## Model concepts

MLHub distinguishes between two model resources:

- An **ExternalModel** is a globally available catalog entry imported from a provider such as
  Hugging Face. It contains provider information and derived searchable metadata.
- A **Model** is an optional tenant-owned collection entry that refers to an ExternalModel. Adding a
  model to your collection lets you give it a local name, description, and visibility.

A model does **not** need to be added to your collection before it can be deployed. Deployments are
created from an ExternalModel's deployment option.

## Discover and deploy a model

### 1. Search the external model catalog

An empty criteria list returns the paginated global catalog:

```shell
curl --request POST \
  --header "Content-Type: application/json" \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  --data '{"criteria":[]}' \
  "${MLHUB_URL}/models-api/external-models/search?limit=25&include_count=true"
```

Search criteria support model metadata such as provider, name, author, task type, runtime, tags,
license, size, likes, and downloads. They can also filter models by available deployment options.

This example finds text-generation models that have at least one deployment option:

```shell
curl --request POST \
  --header "Content-Type: application/json" \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  --data '{
    "criteria": [
      {
        "task_types": ["TextGeneration"],
        "has_deployment_options": true
      }
    ]
  }' \
  "${MLHUB_URL}/models-api/external-models/search?limit=25"
```

Fields within one criterion are combined with AND. Multiple criteria are combined with OR. Within
an array-valued filter, a model matches when it has any requested value.

Save an ExternalModel ID from the response:

```shell
export EXTERNAL_MODEL_ID="<external-model-uuid>"
```

Retrieve that model directly when its complete public projection is needed:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/models-api/external-models/${EXTERNAL_MODEL_ID}"
```

### 2. Find a target

List TACC HPC clusters:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/deployments-api/data-centers/Tacc/hpc-clusters?limit=100&include_count=true"
```

The list contains lightweight cluster summaries. Retrieve a cluster to inspect its queues,
container runtimes, hardware profiles, scheduling policies, and enabled state:

```shell
export HPC_CLUSTER_ID="<hpc-cluster-uuid>"

curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/deployments-api/data-centers/Tacc/hpc-clusters/${HPC_CLUSTER_ID}"
```

Cluster and queue IDs can be used directly in an ExternalModel search. This example asks for models
with a deployment option targeting a particular queue:

```shell
export QUEUE_ID="<batch-scheduler-queue-uuid>"

curl --request POST \
  --header "Content-Type: application/json" \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  --data "{
    \"criteria\": [
      {
        \"task_types\": [\"TextGeneration\"],
        \"batch_scheduler_queue_ids\": [\"${QUEUE_ID}\"],
        \"serving_runtimes\": [\"FlexServ\"]
      }
    ]
  }" \
  "${MLHUB_URL}/models-api/external-models/search?limit=25"
```

Use `hpc_cluster_ids` instead of `batch_scheduler_queue_ids` to search at cluster level.

### 3. Select a deployment option

A deployment option describes a compatible combination of ExternalModel, serving runtime, target,
and supported deployment modalities. List an ExternalModel's options:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/models-api/external-models/${EXTERNAL_MODEL_ID}/deployment-options?limit=100"
```

Each option identifies its HPC cluster and queue and includes an `available` field. Compatibility
and current availability are different:

- A compatible option remains in the list when its target is disabled.
- `available: false` means its cluster or queue is currently disabled.
- Deployment requests using an unavailable option are rejected.

Save the selected option ID:

```shell
export DEPLOYMENT_OPTION_ID="<deployment-option-uuid>"
```

Retrieve the option detail before deployment. The detail includes required and optional parameters,
their types, defaults, choices, and whether a value is secret:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/models-api/external-models/${EXTERNAL_MODEL_ID}/deployment-options/${DEPLOYMENT_OPTION_ID}"
```

### 4. Create the deployment

Provide every required parameter returned by the option-detail request. Argument values are sent as
strings:

```shell
curl --request POST \
  --header "Content-Type: application/json" \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  --data "{
    \"name\": \"text-generation-example\",
    \"description\": \"Example FlexServ deployment\",
    \"deployment_option_id\": \"${DEPLOYMENT_OPTION_ID}\",
    \"deployment_modality\": \"Batch\",
    \"replicas\": 1,
    \"arguments\": [
      {
        \"parameter_name\": \"allocation\",
        \"value\": \"<project-allocation>\"
      }
    ]
  }" \
  "${MLHUB_URL}/deployments-api/deployments"
```

The option determines the ExternalModel, serving runtime, cluster, and queue. Do not send those
values separately in the deployment request.

Deployment is asynchronous. `desired_state` represents what MLHub is trying to achieve, while
`state` represents the latest state observed from downstream infrastructure. For example:

```json
{
  "desired_state": "Running",
  "state": "Unknown"
}
```

This means MLHub has accepted or initiated the deployment but has not yet observed its running
state. The currently supported observed states are `NotDeployed`, `Running`, `Stopped`, `Failed`,
`Blocked`, and `Unknown`.

### 5. Manage deployments

List your deployments:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/deployments-api/deployments"
```

Save a deployment ID from the response:

```shell
export DEPLOYMENT_ID="<model-deployment-uuid>"
```

Request a desired-state change:

```shell
# Start
curl --request POST \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/deployments-api/deployments/${DEPLOYMENT_ID}/start"

# Stop
curl --request POST \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/deployments-api/deployments/${DEPLOYMENT_ID}/stop"

# Remove the downstream deployment
curl --request POST \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/deployments-api/deployments/${DEPLOYMENT_ID}/undeploy"
```

These operations express desired state. The MLHub control plane performs the infrastructure action
and observation asynchronously, so `state` may temporarily differ from `desired_state`.

## Add a model to your collection

Creating a Model adds an ExternalModel to your personal tenant collection. This step is optional for
deployment.

```shell
curl --request POST \
  --header "Content-Type: application/json" \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  --data "{
    \"name\": \"my-text-generation-model\",
    \"description\": \"Model used by my team\",
    \"external_model_id\": \"${EXTERNAL_MODEL_ID}\",
    \"visibility\": \"Private\"
  }" \
  "${MLHUB_URL}/models-api/models"
```

List owned models:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/models-api/models?scope=Owned&limit=100"
```

List public models shared within your tenant:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/models-api/models?scope=Shared&limit=100"
```

The same owner cannot add the same ExternalModel to the same tenant collection twice. A duplicate
request returns `409 Conflict` with `Model already in your collection`.

## Work with datasets

Datasets are tenant-owned or globally readable records backed by a provider locator. List datasets
by scope:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/datasets-api/datasets?scope=Owned&limit=100&include_count=true"
```

Supported scopes are `Owned`, `Shared`, and `Global`.

Register a Hugging Face dataset by providing its immutable provider locator, complete item list, and
total size in bytes:

```shell
curl --request POST \
  --header "Content-Type: application/json" \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  --data '{
    "name": "example-dataset",
    "description": "Example dataset registration",
    "provider": "HuggingFace",
    "huggingface_repo_locator": {
      "id": "organization/dataset-name",
      "sha": "<repository-sha>"
    },
    "items": [
      {
        "path": "data/train.jsonl",
        "size": 1024
      }
    ],
    "size": 1024,
    "tags": ["example"],
    "visibility": "Private"
  }' \
  "${MLHUB_URL}/datasets-api/datasets"
```

Retrieve a dataset by ID:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/datasets-api/datasets/<dataset-uuid>"
```

Dataset list and detail responses return at most the first 50 items for each dataset.

## Work with agents

MLHub separates an agent's portable description from a deployed agent:

- An **AgentRecord** describes the agent version, provider, skills, capabilities, interfaces, and
  artifacts.
- An **Agent** represents a usable agent and may refer to an AgentRecord and expose concrete
  endpoints.

List owned or shared agent records:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/agents-api/agent-records?scope=Owned"
```

List agents, optionally including their endpoints:

```shell
curl --request GET \
  --header "X-Tapis-Token: ${TAPIS_TOKEN}" \
  "${MLHUB_URL}/agents-api/agents?scope=Owned&include_endpoints=true"
```

Create the AgentRecord before creating an Agent that refers to it. Consult the Agents OpenAPI
schema for the interface-specific REST, RPC, and stdio request shapes.

## Visibility and scope

User-owned resources use `Private` or `Public` visibility:

- `Private` resources are readable by their owner.
- `Public` resources can be read by other users in the same tenant through `Shared` scope.
- Global catalog resources such as ExternalModels and HPC targets are not owned by an individual
  tenant user.

Mutation operations remain owner-controlled even when a resource is publicly readable.

## Errors and troubleshooting

Common HTTP statuses include:

| Status | Meaning |
|---|---|
| `200 OK` | Read or update succeeded. |
| `201 Created` | A resource such as a Model or ModelDeployment was created. |
| `400 Bad Request` | An identifier, cursor, parameter, or request field is invalid. |
| `401 Unauthorized` | The access token is absent or invalid. |
| `403 Forbidden` | The authenticated principal cannot perform the operation. |
| `404 Not Found` | The requested resource does not exist or is not visible in the requested scope. |
| `409 Conflict` | The request conflicts with current state. |
| `500 Internal Server Error` | An internal, persistence, or integration failure occurred. |

When a deployment does not start:

1. Retrieve its deployment option again and check `available`.
2. Verify the selected deployment modality appears in `supported_deployment_modalities`.
3. Retrieve the option detail and supply all required parameters.
4. Inspect the deployment's `state`, `desired_state`, and `last_message`.
5. If the state is `Unknown`, allow the observation loop time to query downstream infrastructure.
6. Give the deployment ID and response message to an MLHub administrator if it remains blocked or
   failed.

## API and SDK references

The complete source-derived API contracts are available here:

- [Models OpenAPI specification](../specs/models/openapi.json)
- [Deployments OpenAPI specification](../specs/deployments/openapi.json)
- [Datasets OpenAPI specification](../specs/datasets/openapi.json)
- [Agents OpenAPI specification](../specs/agents/openapi.json)

Generated TypeScript SDK documentation is available here:

- [Models TypeScript SDK](../sdks/models-ts-sdk/README.md)
- [Deployments TypeScript SDK](../sdks/deployments-ts-sdk/README.md)
- [Datasets TypeScript SDK](../sdks/datasets-ts-sdk/README.md)
- [Agents TypeScript SDK](../sdks/agents-ts-sdk/README.md)
