#!/bin/bash

kubectl apply -f deploy/local/minikube/service.yaml \
    -f deploy/local/minikube/pvc.yaml \
    -f deploy/local/minikube/deployment.yaml \
    