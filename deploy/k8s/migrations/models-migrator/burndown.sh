#!/usr/bin/env bash

set -e

overlay=$1

kubectl kustomize "./overlays/$overlay" | kubectl delete -f -