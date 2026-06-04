#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

source "$SCRIPT_DIR/../../utils.sh";

overlay=$1

kubectl kustomize "$SCRIPT_DIR/overlays/$overlay" | replace_template_vars | kubectl apply -f -