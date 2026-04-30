#!/usr/bin/env bash

set -e

kubectl scale -f "./stateful-set.yaml" --replicas=0

kubectl delete -f "./cm-mongo-init-sidecar-script.yaml"