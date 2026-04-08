#!/bin/bash

set -e

projectRoot=$1

mongoSecretsDir=~/mlhub
mongoKeyfileFilename=mongo-keyfile
mongoKeyfilePath="$mongoSecretsDir/$mongoKeyfileFilename"

mkdir -p "$mongoSecretsDir"

timestamp=$(date +"%Y%m%d%H%M%S")
mongoKeyfileBackupPath="$mongoKeyfilePath.bak.$timestamp"

if [[ ! -f "$mongoKeyfilePath" ]]; then
    # Generate a random key via openssl
    openssl rand -base64 756 | tr -d '\n' > "$mongoKeyfilePath"

    # Create a timestamped copy of the keyfile
    cp $mongoKeyfilePath $mongoKeyfileBackupPath
fi

kubectl create secret generic mlhub-mongo-keyfile-secret \
  --from-file=mongo-keyfile="$mongoKeyfilePath" \
  --dry-run=client -o yaml | kubectl apply -f -

# Install the mongodb CR
kubectl apply -f "$projectRoot/deploy/k8s/minikube/mongo/headless-service.yaml" \
    -f "$projectRoot/deploy/k8s/minikube/mongo/cm-mongo-init-sidecar-script.yaml" \
    -f "$projectRoot/deploy/k8s/minikube/mongo/stateful-set.yaml" \
    