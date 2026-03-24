#!/bin/bash

set -e

projectRoot=$1

kubectl apply -f "$projectRoot/deploy/k8s/minikube/traefik/cr.yaml" \
              -f "$projectRoot/deploy/k8s/minikube/traefik/service-account.yaml" \
              -f "$projectRoot/deploy/k8s/minikube/traefik/crb.yaml" \
              -f "$projectRoot/deploy/k8s/minikube/traefik/traefik-dynamic-config.yaml" \
              -f "$projectRoot/deploy/k8s/minikube/traefik/deployment.yaml" \
              -f "$projectRoot/deploy/k8s/minikube/traefik/web-service.yaml" \
              -f "$projectRoot/deploy/k8s/minikube/traefik/dashboard-service.yaml"