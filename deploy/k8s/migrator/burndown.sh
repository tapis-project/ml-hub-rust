#!/usr/bin/env bash

set -euo pipefail

kubectl delete job mlhub-migrator --ignore-not-found

