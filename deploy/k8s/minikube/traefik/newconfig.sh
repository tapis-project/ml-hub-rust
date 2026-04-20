#!/usr/bin/env bash

set -e

kubectl delete -f "./traefik-dynamic-config.yaml"
kubectl apply -f "./traefik-dynamic-config.yaml"