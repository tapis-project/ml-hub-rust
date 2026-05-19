#!/bin/bash

set -e

rootDir=$1
nfsServerIp=$2
nfsServerIpTemplate="{{ NFS_SERVER_COMPONENT_IP }}"

sed -i.bak "s|${nfsServerIpTemplate}|${nfsServerIp}|g" "$rootDir/deploy/local/minikube/deployment.yaml"
rm "$rootDir/deploy/local/minikube/deployment.yaml.bak"

kubectl apply -f "$rootDir/deploy/local/minikube/service.yaml" \
    -f "$rootDir/deploy/local/minikube/deployment.yaml"