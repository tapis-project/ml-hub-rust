#!/bin/bash

set -e

projectRoot=$1

kubectl delete -f "$projectRoot/deploy/k8s/minikube/mongo/deployment.yaml"