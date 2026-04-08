#!/bin/bash

set -e

projectRoot=$1

replicas=${2:-1}
if ! [[ "$replicas" =~ ^[0-9]+$ ]]; then
   echo "Error: Replicas must be a number"
   exit 1
fi

kubectl scale -f "$projectRoot/deploy/k8s/minikube/mongo/stateful-set.yaml" --replicas=0
kubectl delete -f "$projectRoot/deploy/k8s/minikube/mongo/cm-mongo-init-sidecar-script.yaml"
kubectl scale -f "$projectRoot/deploy/k8s/minikube/mongo/stateful-set.yaml" --replicas=$replicas
