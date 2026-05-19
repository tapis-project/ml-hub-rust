#!/usr/bin/env bash

set -e

source ../utils.sh;

overlay=$1

kubectl kustomize "./overlays/$overlay" | kubectl delete -f -