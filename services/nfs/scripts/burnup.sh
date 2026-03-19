#!/bin/bash

set -e

projectDir=$1

kubectl apply -f "$projectDir/deploy/k8s/minikube/nfs/service.yaml" \
    -f "$projectDir/deploy/k8s/minikube/nfs/pvc.yaml" \
    -f "$projectDir/deploy/k8s/minikube/nfs/deployment.yaml" \
    