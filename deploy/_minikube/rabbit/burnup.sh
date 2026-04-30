#!/usr/bin/env bash

set -e

kubectl apply -f "./rabbit-secrets.yaml" \
    -f "./service.yml" \
    -f "./pvc.yml" \
    -f "./deployment.yml"