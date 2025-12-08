#!/bin/bash

set -euo pipefail

API_BASE_URL=${API_BASE_URL:-"http://localhost:8000"}
AUTH_FILE_DEFAULT=${TAPIS_AUTH_FILE:-"tapis_auth.json"}

usage() {
    cat <<'USAGE'
Usage: run_rest_demo.sh <command> [options]

Commands:
    auth <user> <pass> [--tenant=dev.develop.tapis.io]
                                                                        Retrieve Tapis tokens (POST /tapis_auth).
    search <query> [--author=...] [--filter=...] [--sort=...] [--direction=±1] [--limit=1-1000] [--full] [--config]
                                                                        Search Hugging Face models (passes all supported parameters).
    info <repo_id> [revision]          
                                                                        Fetch metadata for a repo (and optional revision).
    deploy <repo_id> <revision> [--auth-file=tapis_auth.json] [--tenant=override]
                                                                        Trigger a pod deployment for the model using the token+tenant stored in the auth file.
    deployments                        
                                                                        List all deployments.
    status <deployment_id>             
                                                                        Fetch the status for a specific deployment.
    cancel <deployment_id> [--auth-file=tapis_auth.json] [--tenant=override] [--delete-cache]
                                                                        Delete the running pod (and optionally cache) using the stored token info.
Environment:
    API_BASE_URL   Base URL for the REST server (default: http://localhost:8000)
USAGE
}

load_auth_from_file() {
    local file="$1"
    if [[ ! -f "$file" ]]; then
        echo "Error: auth file '$file' not found. Run ./run_rest_demo.sh auth <user> <pass> > $file" >&2
        exit 1
    fi
    local token tenant
    token=$(jq -r '.tokens.access_token // empty' "$file")
    tenant=$(jq -r '.tenant_host // empty' "$file")
    if [[ -z "$token" ]]; then
        echo "Error: access_token missing in '$file'. Did you capture the auth response?" >&2
        exit 1
    fi
    AUTH_TOKEN="$token"
    AUTH_TENANT="${tenant:-dev.develop.tapis.io}"
}

if [[ $# -lt 1 ]]; then
    usage
    exit 1
fi

command=$1
shift

case "$command" in
    search)
        if [[ $# -lt 1 ]]; then
            echo "Error: search command requires a query string." >&2
            usage
            exit 1
        fi

        search_query=$1
        shift

        author=""
        filter_param=""
        sort_param=""
        direction_param=""
        limit_param=""
        full_flag=""
        config_flag=""

        while [[ $# -gt 0 ]]; do
            case "$1" in
                --author=*) author="${1#--author=}" ;;
                --filter=*) filter_param="${1#--filter=}" ;;
                --sort=*) sort_param="${1#--sort=}" ;;
                --direction=*) direction_param="${1#--direction=}" ;;
                --limit=*) limit_param="${1#--limit=}" ;;
                --full) full_flag="true" ;;
                --config) config_flag="true" ;;
                *)
                    echo "Error: unknown option '$1' for search command." >&2
                    usage
                    exit 1
                    ;;
            esac
            shift
        done

        curl_args=("${API_BASE_URL}/models" --get --silent --show-error)
        curl_args+=(--data-urlencode "search=${search_query}")
        [[ -n $author ]] && curl_args+=(--data-urlencode "author=${author}")
        [[ -n $filter_param ]] && curl_args+=(--data-urlencode "filter=${filter_param}")
        [[ -n $sort_param ]] && curl_args+=(--data-urlencode "sort=${sort_param}")
        [[ -n $direction_param ]] && curl_args+=(--data-urlencode "direction=${direction_param}")
        [[ -n $limit_param ]] && curl_args+=(--data-urlencode "limit=${limit_param}")
        [[ -n $full_flag ]] && curl_args+=(--data-urlencode "full=true")
        [[ -n $config_flag ]] && curl_args+=(--data-urlencode "config=true")

        curl "${curl_args[@]}" | jq .
        ;;
    info)
        if [[ $# -lt 1 ]]; then
            echo "Error: info command requires repo_id (and optional revision)." >&2
            usage
            exit 1
        fi
        repo_id=$1
        revision=${2:-}
        if [[ -n $revision ]]; then
            curl -sS "${API_BASE_URL}/models/${repo_id}/revisions/${revision}" | jq .
        else
            curl -sS "${API_BASE_URL}/models/${repo_id}" | jq .
        fi
        ;;
    deploy)
        if [[ $# -lt 2 ]]; then
            echo "Error: deploy command requires repo_id revision." >&2
            usage
            exit 1
        fi
        repo_id=$1
        revision=$2
        shift 2

        auth_file="$AUTH_FILE_DEFAULT"
        tenant_override=""
        force_model=false
        force_dataset=false
        while [[ $# -gt 0 ]]; do
            case "$1" in
                --auth-file=*) auth_file="${1#--auth-file=}" ;;
                --tenant=*) tenant_override="${1#--tenant=}" ;;
                --force-model) force_model=true ;;
                --force-dataset) force_dataset=true ;;
                *)
                    echo "Error: unknown option '$1' for deploy command." >&2
                    usage
                    exit 1
                    ;;
            esac
            shift
        done

        load_auth_from_file "$auth_file"
        tapis_token="$AUTH_TOKEN"
        tenant_host="$AUTH_TENANT"
        if [[ -n "$tenant_override" ]]; then
            tenant_host="$tenant_override"
        fi

        payload=$(jq -n \
            --arg repo "$repo_id" \
            --arg rev "$revision" \
            --arg token "$tapis_token" \
            --arg tenant "$tenant_host" \
            --argjson force_model "$force_model" \
            --argjson force_dataset "$force_dataset" \
            '{repo_id: $repo, revision: $rev, tenant_host: $tenant, tapis_token: $token, force_model: $force_model, force_dataset: $force_dataset}')

        curl -sS -X POST "${API_BASE_URL}/pod_deployment" \
            -H "Content-Type: application/json" \
            -d "$payload" | jq .
        ;;
    deployments)
        curl -sS "${API_BASE_URL}/deployments" | jq .
        ;;
    status)
        if [[ $# -lt 1 ]]; then
            echo "Error: status command requires deployment_id." >&2
            usage
            exit 1
        fi
        deployment_id=$1
        curl -sS "${API_BASE_URL}/deployments/${deployment_id}" | jq .
        ;;
    cancel)
        if [[ $# -lt 1 ]]; then
            echo "Error: cancel command requires deployment_id." >&2
            usage
            exit 1
        fi
        deployment_id=$1
        shift 1

        auth_file="$AUTH_FILE_DEFAULT"
        tenant_override=""
        delete_cache=false
        while [[ $# -gt 0 ]]; do
            case "$1" in
                --auth-file=*) auth_file="${1#--auth-file=}" ;;
                --tenant=*) tenant_override="${1#--tenant=}" ;;
                --delete-cache) delete_cache=true ;;
                *)
                    echo "Error: unknown option '$1' for cancel command." >&2
                    usage
                    exit 1
                    ;;
            esac
            shift
        done

        load_auth_from_file "$auth_file"
        tapis_token="$AUTH_TOKEN"
        tenant_host="$AUTH_TENANT"
        if [[ -n "$tenant_override" ]]; then
            tenant_host="$tenant_override"
        fi

        payload=$(jq -n \
            --arg token "$tapis_token" \
            --arg tenant "$tenant_host" \
            --argjson delete_cache "$delete_cache" \
            '{tapis_token: $token, tenant_host: $tenant, delete_cache: $delete_cache}')

        curl -sS -X DELETE "${API_BASE_URL}/deployments/${deployment_id}" \
            -H "Content-Type: application/json" \
            -d "$payload" | jq .
        ;;
    auth)
        if [[ $# -lt 2 ]]; then
            echo "Error: auth command requires tapis_username tapis_password." >&2
            usage
            exit 1
        fi
        tapis_username=$1
        tapis_password=$2
        shift 2

        tenant="dev.develop.tapis.io"
        while [[ $# -gt 0 ]]; do
            case "$1" in
                --tenant=*) tenant="${1#--tenant=}" ;;
                *)
                    echo "Error: unknown option '$1' for auth command." >&2
                    usage
                    exit 1
                    ;;
            esac
            shift
        done

        payload=$(jq -n \
            --arg user "$tapis_username" \
            --arg pass "$tapis_password" \
            --arg tenant "$tenant" \
            '{tapis_username: $user, tapis_password: $pass, tenant_host: $tenant}')

        curl -sS -X POST "${API_BASE_URL}/tapis_auth" \
            -H "Content-Type: application/json" \
            -d "$payload" | jq .
        ;;
    *)
        echo "Error: unknown command '$command'" >&2
        usage
        exit 1
        ;;
esac