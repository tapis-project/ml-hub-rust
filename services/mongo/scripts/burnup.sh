#!/bin/bash

set -e

projectRoot=$1

kubectl apply -f "$projectRoot/deploy/k8s/minikube/mongo/service.yaml" \
    -f "$projectRoot/deploy/k8s/minikube/mongo/pvc.yaml" \
    -f "$projectRoot/deploy/k8s/minikube/mongo/cm-init-mongo-script.yaml" \
    -f "$projectRoot/deploy/k8s/minikube/mongo/deployment.yaml" \
    