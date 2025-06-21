# Welcome to MLHub! 🤠 🐂

MLHub is a suite of services designed to facilitate model and dataset discovery and download, and programatically building and deploying inference servers and training systems for Machine Learning/Artificial Intelligence models.

## Deploying MLHub 🚧

Before contributing, you must first set up your local development environment with some software and tools that will allow you to run the MLHub suite locally.
> **Note**: This documentation only covers how to set up Mac or Linux machines.

### 0.1. Install Rust 🦀

Install Rust by following the instruction found in the following link: https://www.rust-lang.org/tools/install 

After installation, run `rustup default stable`. This command sets the default toolchain to the latest stable release. This is required by the API framework (Actix web) used in this project.

### 0.2. Install Docker 📦

Follow the installation guide for your local machine on the official docker website: 
https://docs.docker.com/desktop/setup/install/mac-install/

> **Note** Must use version `24.0.2` or later

### 0.3. Install Minkube 📦📦

Follow the installation guide for your local machine on the official docker website:
https://minikube.sigs.k8s.io/docs/start/?arch=%2Fmacos%2Farm64%2Fstable%2Fbinary+download

### 1. Start Minikube 🔥

You will need to start Minikube with at least 2 nodes. Run the following command:
`minikube start --nodes 2`

### 2. Start your Engines! 🏎️

Now that you have all the necessary tools installed, we can start up the MLHub suite. 

> **Note**: Before running the next script, you may want to take a look at the Kubernetes configuration files (deployment.yaml, cr.yaml, crb.yaml, etc) in the root of the project and in the root directory of each component to ensure that you will not be utilizing more resources than you want to. You can find the root configuration files in the `/deploy/local/minikube` directory and each individual service's configuration files in the `/src/<service_name>/deploy/local/minikube`

This project comes with a set of lifecycle management scripts that assist you in common or repetitive tasks you will encounter during the development of features in this project.

From the project's root directory, run the following commands to initalize the project and launch the services in Minikube. For all `./manage start` steps, ensure that each component pod is in the "Running" state before moving onto the next step.

1. `chmod +x manage` - Makes the lifecycle script executable

2. `./manage start nfs` - Starts the shared file system

3. `./manage start artifact-mq` - Starts the artifact message broker

4. `./manage start artifact-db` - Starts the artifact database

5. `./manage start artifact-ingester` - Start up the artifact ingestion workers

6. `./manage start traefik` - Starts the revers proxy

7. `./manage buildl models` - Builds the Models API image with the `local` tag

8. `./manage start models` - Starts the Models API pod

Congrats! You know have a fully-functional local deployment of the MLHub Suite! The last step is exposing the Traefik reverse-proxy to external traffic. Once all of the pods for the MLHub components are `Running`, execute the following command:

`./manage expose traefik`

You can now make request to the IP address and port output by the last command. The section below will provide detailed instructions on how to make request to each service.

> **Note**: If you are using a Docker driver on darwin, the terminal will need to remain open in order to make requests to MLHub services

### 3. Making requests

You can use the IP address and port produced by the last command to make API calls to any service in the MLHub suite. Your url will need to be structured as follows:

`http://<ipAddress>:<port>/<serviceName>`

In the example below, we will use `curl` to list models from the HuggingFace Models API:

Example (Returns a list of machine learning models from the Models API):

`curl http://127.0.0.1:57783/models-api/platforms/huggingface/models`

---

## Using the Lifecycle Management CLI

The Lifecycle Management CLI is a python tool that can be invoked from the command line to run bash commands and scripts that control the lifecycle of the various microservices and components of MLHub. This is the same script invoked previously to initialize the MLHub project locally.

### Managing MLHub Services and Components

In this section, we will describe the service and components comprising MLHub and their relationships to eachother.

**APIs**
Models API
Datasets API
Inference API
Training API

**Components**
NFS Server
Artifacts Database - MongoDB
Artifact Operations Message Broker - RabbitMQ
Artifact Ingester - Asynch workers for model/dataset ingestion
Inference Database - MongoDB
Traefik

## Developing in MLHub

### Using the MongoDB Compass GUI for local db administration
1. Download and install the MongoDB Compass GUI
2. Run `kubectl port-forward svc/mlhub-artifact-db-service 27017:27017`
3. Create a connection to the ip:port combination output by that command 


