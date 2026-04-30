#!/usr/bin/env bash

set -e

source ../utils.sh;

kubectl kustomize . | replace_template_vars | kubectl apply -f -