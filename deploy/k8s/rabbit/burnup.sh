#!/usr/bin/env bash

set -e

source ../utils.sh;

kubectl apply -f "./base/service.yml" \
              -f "./base/pvc.yml"

overlay=$1

kubectl kustomize "./overlays/$overlay" | replace_template_vars | kubectl apply -f -