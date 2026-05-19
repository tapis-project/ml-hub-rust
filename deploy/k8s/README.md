# Deploying MLHub to Kubernetes

# Overview

MLHub leverages the `kustomize` functionality built into `kubectl` to deploy the platform in different environments by applying environment-specifc patches to the base manifests for each service. The base manifests (deployments, services, pvc, etc) for each service are located at `./<service>/base/` and their overlays are located at `./<service>/overlays/<overlay>`.

Each overlay corresponds to an environment that MLHub can be deployed to. Currently, MLHub can be deployed to the following environments:

### Local
- `minikube`

### Tapis
- `tapisdev` - The Tapis development environment
- `tapisstage` - The Tapis staging environment
- `tapisprod` - The Tapis production environment

The scripts that control the starting and stopping of each service are located in their base directories. Ex. The `start` script for the Models API is located in the `./models/` directory.

To start the Models API for any environment, you would first:

1. `cd` into the base directory for the Models API (`cd ./models`)
2. and run `./burnup.sh <environment>`

This will apply the chosen environment's overlay to the Models Api's base manifest(s) and publish them to Kubernetes

All other services follow the same pattern

## Tapis Deployments

For all Tapis environments, the secrets used by each MLHub subsystem are assumed to have been created by the Tapis Deployer, therefore, you will find no Secrets manifests in any of the Tapis overlays.