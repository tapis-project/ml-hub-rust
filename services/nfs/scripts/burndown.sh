#!/bin/bash

set -e

projectDir=$1

kubectl delete -f "$projectDir/deploy/k8s/minikube/nfs/deployment.yaml"