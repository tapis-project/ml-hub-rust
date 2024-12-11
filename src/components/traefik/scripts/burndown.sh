#!/bin/bash

set -e

rootDir=$1

kubectl delete -f "$rootDir/deploy/local/minikube/deployment.yaml" \
    -f "$rootDir/deploy/local/minikube/traefik-dynamic-config.yaml"