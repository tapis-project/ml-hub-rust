# FlexServ Deployment Demo

This folder contains a lightweight FastAPI service (`main.py`) and two helper scripts for working with Hugging Face models and the FlexServ pod deployment workflow.

- `run_rest_server.sh` starts the FastAPI app (wrapping Hugging Face Hub APIs plus the pod deployment endpoint).
- `run_rest_demo.sh` is a convenience CLI built on `curl + jq` that exercises each REST endpoint.

## Prerequisites

1. **Python environment**
   ```bash
   cd deployment
   python -m venv .venv
   source .venv/bin/activate
   pip install -r requirements.txt
   ```
2. **Hugging Face token**
   - Copy `deployment/src/hf.env` to `hf.env` (or create a new file) and export your token:
     ```bash
     echo "export HF_TOKEN=hf_your_token" > deployment/src/hf.env
     ```
   - The server script will source this file before launching `uvicorn`.
3. **jq** (for the demo script). Install via `brew install jq` on macOS.

## Starting the REST server

```bash
cd deployment/src
./run_rest_server.sh
```

The script sources `hf.env` (to pick up `HF_TOKEN`) and runs `uvicorn main:app --host 0.0.0.0 --port 8000 --reload`. Leave this terminal running while you experiment with the demo CLI in another shell.

## Using `run_rest_demo.sh`

Set the `API_BASE_URL` if you exposed the server elsewhere (defaults to `http://localhost:8000`). Each command streams JSON into `jq` for readability.

```bash
cd deployment/src
export API_BASE_URL=http://localhost:8000   # optional
./run_rest_demo.sh <command> [...options]
```

### `search`
Query the Hugging Face `/models` endpoint and optionally forward every supported parameter.

```bash
./run_rest_demo.sh search 0.6B \
  --author=Qwen \
  --filter=text-generation \
  --sort=downloads \
  --direction=-1 \
  --limit=25 \
  --full --config
### `test_infer_pod.sh`
Once a deployment is live, use the `test_infer_pod.sh` helper to hit the pod endpoints directly. Provide the pod host (and optional port) via environment variables:

```bash
cd deployment/src
BASE_HOST="c1899de2.pods.dev.develop.tapis.io" ./test_infer_pod.sh responses
# commands: show_model | chat | chat_comp | responses | audio_trans
```

Use `--port=####` to override the port or set `BASE_URL`/`AUDIO_FILE` if you need custom values.

```

### `info`
Fetch metadata for a single repo (optional revision). Repo IDs that include a `/` should be quoted.

```bash
./run_rest_demo.sh info 'Qwen/Qwen3-0.6B'
./run_rest_demo.sh info 'Qwen/Qwen3-0.6B' main
```

### `auth`
Exchange a username/password pair for Tapis tokens via the `/tapis_auth` endpoint. This helper is useful for generating the token consumed by the `deploy` and `cancel` commands.

```bash
./run_rest_demo.sh auth my_tapis_user my_tapis_pass
# Capture the response to the default auth cache file (overwriting if it exists)
bash ./run_rest_demo.sh auth testuser7 testuser7 >| tapis_auth.json
```

By default the other commands read credentials from `tapis_auth.json`. Set `TAPIS_AUTH_FILE` or pass `--auth-file=/path/to/file` to point elsewhere.

### `deploy`
Trigger a FlexServ pod deployment through the REST API using a Tapis access token (strongly recommended over password auth).

```bash
./run_rest_demo.sh deploy 'Qwen/Qwen3-0.6B' main
# or specify a different auth cache
./run_rest_demo.sh deploy 'Qwen/Qwen3-0.6B' main --auth-file=/tmp/tapis_auth.json
```

The script posts `repo_id`, `revision`, `tenant_host`, and `tapis_token` (loaded from the auth file) to `/pod_deployment`. Check the server logs for deployment progress.

### `deployments`
List all tracked deployments.

```bash
./run_rest_demo.sh deployments
```

### `status`
Inspect a specific deployment by ID (use an ID returned from `deploy`).

```bash
./run_rest_demo.sh status 123e4567-e89b-12d3-a456-426614174000
```

### `cancel`
Delete a running pod (plus optionally its cached volume) using the recorded deployment ID. Pass the same Tapis token you used for deployment:

```bash
./run_rest_demo.sh cancel 123e4567-e89b-12d3-a456-426614174000 --delete-cache
```

Pass `--auth-file` or `--tenant` if you need to override the stored credentials.


## Tips

- Override `API_BASE_URL` when hitting a remote FastAPI instance: `API_BASE_URL=https://example.net ./run_rest_demo.sh search llama`.
- The FastAPI server also accepts raw HTTP requests (e.g., via Postman); the demo script simply illustrates the supported parameters and payloads.
- Logs from `run_rest_server.sh` show Hugging Face requests and deployment lifecycle messages—keep an eye there while running longer deployments.
