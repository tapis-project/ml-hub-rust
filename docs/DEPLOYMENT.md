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

> **Note**: Before running the next script, you may want to take a look at the Kubernetes configuration files (deployment.yaml, cr.yaml, crb.yaml, etc) in the root of the project and in the root directory of each component to ensure that you will not be utilizing more resources than you want to. You can find the deployment config files in the root of the project in `deploy/k8s/minikube/` directory. Every component will have their own directory to houses their configs. `deploy/k8s/minikube/<component_name>/`

This project comes with a set of lifecycle management scripts that assist you in common or repetitive tasks you will encounter during the development of features in this project.

From the project's root directory, run the following commands to initalize the project and launch the services in Minikube. For all `./manage start` steps, ensure that each component pod is in the "Running" state before moving onto the next step.

0. `chmod +x manage` - Makes the lifecycle script executable

0. `./manage start nfs` - Starts the shared file system

0. `./manage start rabbit` - Starts the message broker

0. `./manage start mongo` - Starts the database

0. `./manage start artifact-ingester` - Start up the artifact ingestion workers

0. `./manage start artifact-publisher` - Start up the artifact publisher workers

0. `./manage start traefik` - Starts the reverse proxy that routes traffic to the APIs

0. `./manage buildl models-migrator -s` - Builds the Models API migrator image and loads it into minikube.

0. `./manage buildl models -s` - Builds the Models API image and loads it into minikube

0. `./manage start models` - Starts the Models API pod

Congrats! You know have a fully-functional local deployment of the MLHub Models Suite! The last step is exposing the Traefik reverse-proxy to external traffic. Once all of the pods for the MLHub components are `Running`, execute the following command:

`./manage expose traefik`

You can now make request to the IP address and port output by the last command. The section below will provide detailed instructions on how to make request to each service.

> **Note**: If you are using a Docker driver on darwin, the terminal will need to remain open in order to make requests to MLHub services

## 3. Making requests

You can use the IP address and port produced by the last command to make API calls to any service in the MLHub suite. Your url will need to be structured as follows:

`http://<ipAddress>:<port>/<serviceName>`

In the example below, we will use `curl` to list models from the HuggingFace Models API:

Example (Returns a list of machine learning models from the Models API):

`curl http://127.0.0.1:57783/models-api/platforms/huggingface/models`

---

## Using the Lifecycle Management CLI

The Lifecycle Management CLI is a python tool that can be invoked from the command line in the root of the project to run commands and scripts that control the lifecycle of the various components of MLHub. This is the same script invoked previously to initialize the MLHub project locally.

### The Components File

The `components.json` file contains and exhaustive list of every component in the MLHub suite and every command you can run against those components using the CLI.

### Using the MongoDB Compass GUI for local db administration
1. Download and install the MongoDB Compass GUI
2. Run `kubectl port-forward svc/mlhub-mongo-service 27017:27017`
3. Create a connection to the ip:port combination output by that command 