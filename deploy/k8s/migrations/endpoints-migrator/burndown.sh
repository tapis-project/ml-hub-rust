#!/usr/bin/env bash
set -euo pipefail

kubectl delete job mlhub-endpoints-migrator --ignore-not-found
