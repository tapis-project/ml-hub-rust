#!/usr/bin/env bash

set -e

kubectl apply -f "./service.yaml" \
    -f "./pvc.yaml" \
    -f "./deployment.yaml" \
    