#!/usr/bin/env bash

set -euo pipefail

overlay="${1:-minikube}"

kubectl apply -k "overlays/${overlay}"
kubectl rollout status deployment/mlhub-gatus-public-deployment
kubectl rollout status deployment/mlhub-gatus-internal-deployment
