#!/usr/bin/env bash

set -e

kubectl apply -f ./federated-identities-imgrations-job.yml \
    ./models-migrations-job.yml \
    ./principals-migrations-job.yml