#!/bin/bash

set -e

projectRoot=$1

replicas=${2:-1}
if ! [[ "$replicas" =~ ^[0-9]+$ ]]; then
   echo "Error: Replicas must be a number"
   exit 1
fi

kubectl scale -f "$projectRoot/deploy/k8s/minikube/mongo/stateful-set.yaml" --replicas=$replicas
