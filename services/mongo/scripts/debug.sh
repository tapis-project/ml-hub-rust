#!/bin/bash

set -e

projectRoot=$1

# Install the mongodb CR
kubectl apply -f "$projectRoot/deploy/k8s/minikube/mongo/debug.yaml" \
    