#!/bin/bash

set -e

projectRoot=$1

kubectl apply -f "$projectRoot/deploy/k8s/minikube/rabbit/service.yml"
kubectl apply -f "$projectRoot/deploy/k8s/minikube/rabbit/pvc.yml"
kubectl apply -f "$projectRoot/deploy/k8s/minikube/rabbit/deployment.yml"