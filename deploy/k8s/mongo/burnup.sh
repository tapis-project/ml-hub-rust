#!/usr/bin/env bash

set -e

source ../utils.sh;

chmod +x ./generate-keyfile.sh

./generate-keyfile.sh

overlay=$1

kubectl kustomize "./overlays/$overlay" | replace_template_vars | kubectl apply -f -
    