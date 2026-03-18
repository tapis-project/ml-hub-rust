#!/bin/bash

set -e

projectDir=$1

kubectl delete -f "$projectDir/deploy/k8s/minikube/models/deployment.yaml"
kubectl delete -f "$projectDir/deploy/k8s/minikube/tapis-deployment-strategies-cm.yaml"