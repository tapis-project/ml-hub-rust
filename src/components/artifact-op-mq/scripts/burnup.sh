#!/bin/bash

set -e

rootDir=$1

kubectl apply -f "$rootDir/deploy/local/minikube/service.yml"
kubectl apply -f "$rootDir/deploy/local/minikube/pvc.yml"
kubectl apply -f "$rootDir/deploy/local/minikube/deployment.yml"