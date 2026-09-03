# Deploying MLHub Locally 🚧

Before contributing, you must first set up your local development environment with some software and tools that will allow you to run the MLHub suite locally.
> **Note**: This documentation only covers how to set up Mac or Linux machines.

## 0.1. Install Rust 🦀

Install Rust by following the instruction found in the following link: https://www.rust-lang.org/tools/install 

After installation, run `rustup default stable`. This command sets the default toolchain to the latest stable release. This is required by the API framework (Actix web) used in this project.

## 0.2. Install Docker 📦

Follow the installation guide for your local machine on the official docker website: 
https://docs.docker.com/desktop/setup/install/

> Must use version `24.0.2` or later

## 0.3. Install Minkube 📦📦

Follow the installation guide for your local machine on the official docker website:
https://minikube.sigs.k8s.io/docs/start/?arch=%2Fmacos%2Farm64%2Fstable%2Fbinary+download

## 1. Start Minikube 🔥

You will need to start Minikube with at least 2 nodes. Run the following command:
`minikube start --nodes 2 --disk-space=50g --memory=4g`

**Note** You may need to tune the disk space and memory for you machine. If you want to run the Huggingface Model ETL Pipeline (recommended), you will need more disk space than is allocated by default to the Minikube VM. Provision Minikube with at least 50gb to be safe. As the HuggingFace model metadata collection grows in size over time, you may need to allocate additional disk space to accomodate it.

## 2. Start your Engines! 🏎️

Now that you have all the necessary tools installed, we can start up the MLHub Models suite. 

> **Note**: Before running the next scripts, you may want to take a look at the Kubernetes configuration files (deployment.yaml, cr.yaml, crb.yaml, etc) in the root of the project to ensure that you will not be utilizing more resources than you want to. You can find the deployment config files in the root of the project in `deploy/k8s/minikube/` directory. Every component will have their own directory to houses their configs. `deploy/k8s/minikube/<component_name>/`

This project comes with a set of lifecycle management scripts that assist you in common or repetitive tasks you will encounter during the development of features in this project.

From the project's root directory, run the following commands to initalize the project and launch the services in Minikube. For all ` ...` steps, ensure that each component pod is in the "Running" state before moving onto the next step.

### Infrastructure (Required)

0. `chmod +x manage` - Makes the lifecycle script executable

0. `./dev start nfs` - Starts the shared file system

0. `./dev start rabbit` - Starts the message broker

0. `./dev start mongo` - Starts the database

0. `./dev start traefik` - Starts the reverse proxy that routes traffic to the APIs

### Migrations (Required)

0. `./dev buildl-all migrations` - Builds all migration images and loads them into Minikube.

0. `./dev run-all migrations` - Runs the Models, Federated Identities, and Principals migrations in order. The command stops if any migration fails.

### Models API

0. `./dev buildl models` - Builds the Models API image and loads it into minikube

0. `./dev start models` - Starts the Models API pod(s)

### Deployments API

0. `./dev buildl deployments` - Builds the Deployments API image and loads it into Minikube.

0. `./dev start deployments` - Starts the Deployments API pod(s).

### Agents API

0. `./dev buildl agents` - Builds the Agents API image and loads it into Minikube.

0. `./dev start agents` - Starts the Agents API pod(s).

### Artifacts Suite (Optional)

0. `./dev buildl artifact-ingester && ./dev start artifact-ingester` - Start up the artifact ingestion workers

0. `./dev buildl artifact-publisher && ./dev start artifact-publisher` - Start up the artifact publisher workers

### Networking

0. Add this entry to your `/etc/hosts` file:
    
    # MLHub local devleopment
    127.0.0.1       dev.local.develop.tapis.io tacc.local.develop.tapis.io
    
    Then run one of the following OS-specific commands for the changes to take effect.

    A. **MAC:** `sudo dscacheutil -flushcache; sudo killall -HUP mDNSResponder`
    
    B. **Linux (Modern Ubuntu, Fedora, Debian):** `sudo resolvectl flush-caches`

    C. **Windows:** 🤷‍♂️

Congrats! You know have a fully-functional local deployment of the MLHub Models Suite! The last step is exposing the Traefik reverse-proxy to external traffic. Once all of the pods for the MLHub components are `Running`, execute the following command:

`./dev expose traefik`

You can now make request to the IP address and port output by the last command. The section below will provide detailed instructions on how to make request to each service.

> **Note**: If you are using a Docker driver on darwin, the terminal will need to remain open in order to make requests to MLHub services

## 3. Seed the database with some huggingface models

Build and load both images used by the Hugging Face model ETL job, then run the job in Minikube:

0. `./dev buildl-extract hf-model-etl` - Builds and loads the Hugging Face metadata extraction image.

0. `./dev buildl-transform-load hf-model-etl` - Builds and loads the metadata transform/load image.

0. `./dev run hf-model-etl` - Creates the Hugging Face model ETL job to extract, transform, and load model metadata into MLHub.

## 4. Making requests

You can use the IP address and port produced by the last command to make API calls to any service in the MLHub suite. Your url will need to be structured as follows:

`http://<ipAddress>:<port>/<serviceName>`

The example below discovers models registered in MLHub. Replace `<access-token>` with a valid
Tapis access token.

```bash
curl --request POST 'http://127.0.0.1:<YOUR EXPOSED PORT>/models-api/models/search?limit=10' \
  --header 'Content-Type: application/json' \
  --header 'X-Tapis-Token: <access-token>' \
  --data '{"criteria": []}'
```

The request returns matching model metadata in the standard MLHub response envelope.

---

## Using the Lifecycle Management CLI

The Lifecycle Management CLI is a Python tool that can be invoked through `./dev` from the root of the project to run commands and scripts that control the lifecycle of the various components of MLHub. Its implementation and tests live under `tooling/lifecycle`.

### The Components File

The `components.json` file contains and exhaustive list of every component in the MLHub suite and every command you can run against those components using the CLI.

A component may define an optional `aliases` array containing alternate names for use with the lifecycle CLI:

```json
{
  "name": "deployments",
  "aliases": ["deploy", "deps"]
}
```

The canonical name and each alias select the same component. For example, `./dev start deployments`, `./dev start deploy`, and `./dev start deps` are equivalent. Aliases are case-sensitive and must be unique across all component names and aliases.

A lifecycle command must select at least one component explicitly. Provide component names or aliases, use `-A` or `--all` to select every component, or use `--labels` to select only components containing every requested label:

```shell
./dev test models deployments
./dev test --all
./dev test --labels api
./dev test --all --labels api
```

The `--all` flag cannot be combined with explicit component names or aliases. A label filter may be applied either to explicitly selected components or to all components.

### Using the MongoDB Compass GUI for local db administration
1. Download and install the MongoDB Compass GUI
2. Run `kubectl port-forward pod/mlhub-mongo-stateful-set-0 27017:27017`
3. Create a connection to the ip:port combination output by that command 
