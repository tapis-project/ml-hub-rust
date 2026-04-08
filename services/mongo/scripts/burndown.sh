#!/bin/bash

set -e

projectRoot=$1

kubectl scale -f "$projectRoot/deploy/k8s/minikube/mongo/stateful-set.yaml" --replicas=0

kubectl delete -f "$projectRoot/deploy/k8s/minikube/mongo/cm-mongo-init-sidecar-script.yaml"