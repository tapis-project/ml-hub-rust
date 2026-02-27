#!/bin/bash

set -e

rootDir=$1
projectDir=$2

kubectl delete -f "$rootDir/deploy/local/minikube/deployment.yaml"
kubectl delete -f "$projectDir/deploy/local/minikube/tapis-deployment-strategies-cm.yaml"