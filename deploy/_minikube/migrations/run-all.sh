#!/usr/bin/env bash

set -e

kubectl apply -f ./federated-identities-migrations-job.yml \
    -f ./models-migrations-job.yml \
    -f ./principals-migrations-job.yml