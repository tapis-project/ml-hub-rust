#!/usr/bin/env bash

set -e

kubectl kustomize . | kubectl delete -f -