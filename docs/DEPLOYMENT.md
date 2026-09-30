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

**Note** You may need to tune the disk space and memory for you machine. If you want to run the Huggingface Model ETL Pipeline (recommended), you will need more disk space than is allocated by default to the Minikube VM. Provision Minikube with at least 50gb to be safe. As the HuggingFace model collection grows in size over time, you may need to allocate additional disk space to accomodate it.

## 2. Start your Engines! 🏎️

### Deploy the complete local stack

Now that the development environment and Minikube are ready, open a terminal at the repository
root and run:

```shell
bash dev deploy stack
```

This builds every deployable image, loads it into Minikube, and starts the complete MLHub stack in
dependency order. Invoking `dev` through Bash makes the first run work even when the file is not
yet executable; the stack deployment makes it executable for subsequent `./dev` commands.

The deployment waits for infrastructure and services to become ready and for each migration and ETL
job to complete before continuing. It stops on the first failure and does not roll back resources
that have already started.

Migration and ETL Jobs are never deleted or reused automatically. If one of their Kubernetes Job
resources already exists, remove it explicitly before retrying the failed stage.

This complete deployment is intended primarily to bootstrap a new local environment and should
normally be run only once. After the stack is available, use lifecycle commands on individual
components for routine development, for example:

```shell
./dev buildl models
./dev start models
./dev stop models
```

The `stack` component also provides grouped build and start commands for recovering or completing a
partial initial deployment. Grouped start commands assume the preceding infrastructure and
migration stages have already completed. Stack deployment targets Minikube and uses the `minikube`
overlay by default.

### Expose the local stack

The stack command starts Traefik but does not update the host machine's networking configuration.
Add this entry to `/etc/hosts`:

```text
# MLHub local development
127.0.0.1 dev.local.develop.tapis.io tacc.local.develop.tapis.io
```

Then flush the local DNS cache:

- macOS: `sudo dscacheutil -flushcache; sudo killall -HUP mDNSResponder`
- Modern Ubuntu, Fedora, or Debian: `sudo resolvectl flush-caches`

Expose the Traefik reverse proxy after the stack deployment completes:

```shell
./dev expose traefik
```

If Minikube uses the Docker driver on macOS, keep this terminal open while accessing MLHub.

## 3. Making requests

You can use the IP address and port produced by the last command to make API calls to any service in the MLHub suite. Your url will need to be structured as follows:

`http://<ipAddress>:<port>/<serviceName>`

The example below discovers models in MLHub's global external-model catalog. Replace
`<access-token>` with a valid Tapis access token.

```bash
curl --request POST 'http://127.0.0.1:<YOUR EXPOSED PORT>/models-api/external-models/search?limit=10' \
  --header 'Content-Type: application/json' \
  --header 'X-Tapis-Token: <access-token>' \
  --data '{"criteria": []}'
```

The request returns matching external models in the standard MLHub response envelope.

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
