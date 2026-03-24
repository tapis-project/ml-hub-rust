#!/bin/bash

set -e

projectRoot=$1

kubectl delete -f "$projectRoot/deploy/k8s/minikube/traefik/deployment.yaml" \
    -f "$projectRoot/deploy/k8s/minikube/traefik/traefik-dynamic-config.yaml"