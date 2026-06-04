#!/usr/bin/env bash

# Reads from stdin (implicitly with sed) and replaces all template vars
replace_template_vars() {
    local nfsServerIp
    nfsServerIp=$(kubectl get service mlhub-nfs-server-service -o jsonpath='{.spec.clusterIP}')

    local k8sNamespace
    k8sNamespace=$(kubectl config view --minify --output 'jsonpath={..namespace}')
    k8sNamespace=${k8sNamespace:-default}

    sed -e "s|\*\*-NFS_SERVER_COMPONENT_IP-\*\*|${nfsServerIp}|g" \
        -e "s|\*\*-K8S_NAMESPACE-\*\*|${k8sNamespace}|g"
}

get_caller_dirname() {
    basename "$(cd -- "$(dirname -- "${BASH_SOURCE[1]}")" &> /dev/null && pwd)"
}