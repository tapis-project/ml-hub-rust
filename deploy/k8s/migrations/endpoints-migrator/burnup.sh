#!/usr/bin/env bash
set -euo pipefail

source ../../utils.sh

overlay="${1:-minikube}"

kubectl delete job mlhub-endpoints-migrator --ignore-not-found
kubectl kustomize "overlays/${overlay}" | replace_template_vars | kubectl apply -f -
kubectl wait --for=condition=complete job/mlhub-endpoints-migrator --timeout=10m
