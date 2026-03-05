#!/bin/bash

set -e

rootDir=$1

kubectl apply -f "$rootDir/deploy/local/minikube/cr.yaml" \
              -f "$rootDir/deploy/local/minikube/service-account.yaml" \
              -f "$rootDir/deploy/local/minikube/crb.yaml" \
              -f "$rootDir/deploy/local/minikube/traefik-dynamic-config.yaml" \
              -f "$rootDir/deploy/local/minikube/deployment.yaml" \
              -f "$rootDir/deploy/local/minikube/web-service.yaml" \
              -f "$rootDir/deploy/local/minikube/dashboard-service.yaml"