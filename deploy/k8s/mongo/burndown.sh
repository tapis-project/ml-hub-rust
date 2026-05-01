#!/usr/bin/env bash

set -e

kubectl scale -f "./base/stateful-set.yml" --replicas=0