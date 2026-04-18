#!/usr/bin/env bash

set -e

kubectl apply -f "service.yaml"

nfsServerIp=$(kubectl get service mlhub-nfs-server-service -o jsonpath='{.spec.clusterIP}')
nfsServerIpTemplateVar="{{ NFS_SERVER_COMPONENT_IP }}"

kubectl kustomize . \
    | sed "s|${nfsServerIpTemplateVar}|${nfsServerIp}|g" \
    | kubectl apply -f -