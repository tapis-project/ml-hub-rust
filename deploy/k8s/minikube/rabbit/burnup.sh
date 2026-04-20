#!/usr/bin/env bash

set -e

kubectl apply -f "./service.yml"
kubectl apply -f "./pvc.yml"
kubectl apply -f "./deployment.yml"