#!/bin/bash

set -e

rootDir=$1

kubectl delete -f "$rootDir/deploy/local/minikube/deployment.yaml"
kubectl delete -f "$rootDir/deploy/local/minikube/tapis-deployment-strategies-cm.yaml"