#!/usr/bin/env bash

set -e

overlay=$1

kubectl apply -f "./base/service.yaml"

# Create the pvc. It has it's own overlay seperate to the rest of the manifests
# to ensure it is not deleted on burndown
./storage/burnup.sh "$overlay"

source ../utils.sh;

kubectl kustomize "./overlays/$overlay" | replace_template_vars | kubectl apply -f -