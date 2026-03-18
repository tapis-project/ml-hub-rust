#!/bin/bash

set -e

projectDir=$1

kubectl delete -f "$projectDir/deploy/k8s/minikube/artifact-publisher/deployment.yaml"