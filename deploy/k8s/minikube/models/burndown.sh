#!/bin/bash

set -e

kubectl kustomize . | kubectl delete -f -