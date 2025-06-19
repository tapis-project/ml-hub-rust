#!/bin/bash

set -e

rootDir=$1

kubectl apply -f "$rootDir/deploy/local/minikube/service.yaml" \
    -f "$rootDir/deploy/local/minikube/pvc.yaml" \
    -f "$rootDir/deploy/local/minikube/cm-init-db-script.yaml" \
    -f "$rootDir/deploy/local/minikube/deployment.yaml" \
    