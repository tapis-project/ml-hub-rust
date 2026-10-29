#!/usr/bin/env bash

set -euo pipefail

overlay="${1:-minikube}"

kubectl delete -k "overlays/${overlay}" --ignore-not-found
