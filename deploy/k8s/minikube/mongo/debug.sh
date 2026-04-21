#!/usr/bin/env bash

set -e

# Install the mongodb CR
kubectl apply -f "./debug.yaml"