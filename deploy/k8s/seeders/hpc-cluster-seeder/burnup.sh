#!/usr/bin/env bash

set -e

source ../../utils.sh

overlay=$1

kubectl kustomize "./overlays/$overlay" | replace_template_vars | kubectl apply -f -

kubectl wait --for=condition=complete job/mlhub-hpc-cluster-seeder --timeout=10m
