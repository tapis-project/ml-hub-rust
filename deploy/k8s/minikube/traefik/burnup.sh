#!/usr/bin/env bash

set -e

kubectl apply -f "./traefik/cr.yaml" \
              -f "./traefik/service-account.yaml" \
              -f "./traefik/crb.yaml" \
              -f "./traefik/traefik-dynamic-config.yaml" \
              -f "./traefik/deployment.yaml" \
              -f "./traefik/web-service.yaml" \
              -f "./traefik/dashboard-service.yaml"