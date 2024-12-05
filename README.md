# Welcome to Tapis ML Hub! 🤠 🐂

Tapis ML Hub is a suite of services designed to facilitate model and dataset discovery and download, and programatically building up and deploying inference servers and training systems for Machine Learning/Artificial Intelligence models.

## Local Devevelopment Setup 🚧

Before contributing, you must first set up your local development environment with some software and tools that will allow you to run the Tapis ML Hub suite locally.
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

Now that you have all the necessary tools installed, we can start up the Tapis ML Hub suite. 

> **Note**: Before running the next script, you may want to take a look at the Kubernetes configuration files (deployment.yaml, cr.yaml, crb.yaml, etc) in the root of the project and in the root directory of each component to ensure that you will not be utilizing more resources than you want to. You can find the root configuration files in the `/deploy/local/minikube` directory and each individual service's configuration files in the `/src/<service_name>/deploy/local/minikube`

This project comes with a set of lifecycle management scripts that assist you in common or repetitive tasks you will encounter during the development of features in this project.

From the project's root directory, run the following commands to initalize the project and launch the services in Minikube.

1. `chmod +x manage`

2. `./manage start`

---

## Using the Lifecycle Management CLI

The Lifecycle Management CLI is a python tool that can be invoked from the command line to run commands and scripts that manage the lifecycle of the various microservices and components that comprise ML Hub. This is the same script invoked previously to initialize the ML Hub project locally.

### Managing ML Hub Services and Components