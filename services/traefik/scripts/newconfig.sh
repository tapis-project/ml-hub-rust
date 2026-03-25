#!/bin/bash

set -e

projectRoot=$1

kubectl delete -f "$projectRoot/deploy/k8s/minikube/traefik/traefik-dynamic-config.yaml"
kubectl apply -f "$projectRoot/deploy/k8s/minikube/traefik/traefik-dynamic-config.yaml"