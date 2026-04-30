#!/usr/bin/env bash

set -e

source ../utils.sh;

kubectl apply -f "./base/service.yaml" \
              -f "./base/pvc.yaml"

overlay=$1

kubectl kustomize "./overlays/$overlay" | replace_template_vars | kubectl apply -f -