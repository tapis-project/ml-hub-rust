#!/usr/bin/env bash

set -e

source ../utils.sh;

chmod +x ./generate-keyfile.sh

./generate-keyfile.sh

overlay=$1

kubectl kustomize "./overlays/$overlay" | replace_template_vars | kubectl apply -f -

# kubectl apply -f ./mongo-initdb-secrets.yaml \
#               -f ./mongo-secrets.yaml

# # Install the mongodb CR
# kubectl apply -f "./headless-service.yaml" \
#               -f "./cm-mongo-init-sidecar-script.yaml" \
#               -f "./stateful-set.yaml" \
    