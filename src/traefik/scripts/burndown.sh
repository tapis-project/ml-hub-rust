#!/bin/bash

kubectl delete -f "deploy/local/minikube/deployment.yaml" \
    -f "deploy/local/minikube/traefik-dynamic-config.yaml"