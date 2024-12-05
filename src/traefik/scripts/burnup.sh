#!/bin/bash

kubectl apply -f deploy/local/minikube/cr.yaml \
              -f deploy/local/minikube/service-account.yaml \
              -f deploy/local/minikube/crb.yaml \
              -f deploy/local/minikube/deployment.yaml \
              -f deploy/local/minikube/web-service.yaml \
              -f deploy/local/minikube/dashboard-service.yaml