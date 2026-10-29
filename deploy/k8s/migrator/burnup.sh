#!/usr/bin/env bash

set -euo pipefail

source ../utils.sh

overlay=${1:-minikube}

kubectl kustomize "./overlays/$overlay" | replace_template_vars | kubectl apply -f -

