# MLHub Developer Guide

## Development Environement Setup

You may use whichever IDE you please! But it is recommended to use VSCode. This project
makes use of features from the latest stable version of Rust (**1.91.1** as of **Nov 25, 2025**).
The VSCode `rust-analyzer` extension may be on an older version that is incompatible
with this version of rust.

First, ensure that the version of `rustc` on your machine is **1.91.1** or higher. Next, install `rust-analyzer` using the `rustup` toolchain. Check that the version is also **1.91.1** or higher with `rust-analyzer --version`. Then in your VSCode settings, change the path of `rust-analyzer` to the path of the rust-analyzer binary (`which rust-analyzer`).

## Project Structure & Components 📁

This project is structured as a monorepo. Each component of this project is grouped into one of the following categories.
APIs, libraries, binaries, infrastructure, dev tools, and sdks.

### APIs (src/apis)

APIs live in the src/apis directory. Here you will find the core APIs that provide MLHub functionality; The Models API, The Datasets API, The Deployments API, and the Agents API. These API's follow the standard cargo binary structure.
You can find the source code of an API in `src/apis/<service>` where `<service>` is the name of the API. For example, The Models API source code is located at `src/apis/models`.

Each API follows a similar pattern. In the root directory of each API you will find the following directories:
- **deploy**: This contains the files that are used to deploy the API in some environment. Each available deployment method and environment in which the API can be deployed will have their own subdirectory. For example, the files for deploying the Models API to a local Minkube cluster are located in `src/apis/models/deploy/local/minkube`.
- **docs**: 
- **scripts**:
- **spec**: Contains the API's specification. Each type of spec will be stored in a subdirectory by the name of the spec. For example, the OpenAPI spec for the Models API will be in the `src/apis/models/spec/openapi` directory.
- **src**


**API catalog**

- [Models](./src/apis/models/README.md)

- [Datasets](./src/apis/datasets/README.md) - [Under Construction]

- [Deployments](./src/apis/deployments/README.md) - [Under Construction]

- [Agents](./src/apis/agents/README.md) - [Under Construction]

### Binaries (src/bins)

Binaries are standalone executables that live in the `src/bins` directory. This is where all non-API binaries are stored. For example, the MLHub CLI, agent spec generation code, and the artifact ingestion and publishing workers all live in this directory.

**Binaries catalog**

- [Artifact Ingester](./src/bins/artifact-ingester/README.md)

- [Artifact Publisher](./src/bins/artifact-publisher/README.md)

- [MLHub CLI](./src/bins/cli/README.md)

### Libraries (src/libs)

Libraries are reusable codes that are shared between apis, binaries, and other libraries. Libraries can also be used to support development through codegen and sdks. These libraries are located in the `src/libs` directory. A comprehensive list of libraries developed for this project can be found below in the **Library Catalog** along with a short description.

**Library Catalog**

- [shared](./src/libs/shared/README.md) - Contains the shared layers, project level constants, and various utilities. 

- [clients](./src/libs/clients/README.md) - Contains the interfaces (traits) implemented by the various clients that provide the core features of MLHub (Ex. Model and dataset discovery, ingestion, and publishing, model deployment, etc)

- [client-provider](./src/libs/client-provider/README.md) - Provides concrete client interfaces.

- [huggingface-client](./src/libs/huggingface-client/README.md) - Client for fetching, discovering, publishing, and ingesting HuggingFace models and datasets and their metadata

- [git-lfs-client](./src/libs/git-lfs-client/README.md) - Client for private/public Git registries with LFS. This library can publish ingest model and dataset artifacts from git-backed sources. This can be used for artifacts on Github as well. 

- [github-lfs-client](./src/libs/github-lfs-client/README.md) - Client from ingesting and publishing model and dataset artifacts to Github and LFS. Essentially a specialized version of the **git-lfs-client**

- [patra-client](./src/libs/patra-client/README.md) - Handles model metadata listing, discovery, and publishing to the Patra platform

- [s3-client](./src/libs/s3-client/README.md) - Client for publishing and ingesting artifacts from s3-compatible storage

- [tacc-tapis-client](./src/libs/tacc-tapis-client/README.md) - Client for publishing and ingesting artifacts from Tapis Systems defined in the TACC Tapis deployment

- [mlhub-rust-sdk](./src/libs/mlhub-tust-sdk/README.md) - MLHub's software development kit (SDK) generated from the API's OpenAPI specifications

### Infra (src/infra)

This directory (`src/infra`) contains the deployment files for the infrastructural components that support MLHub operations such as databases, message brokers, remote file systems, and reverse proxies.

**Infrastructure catalog**

- [Artifact DB](./src/infra/artifact-db/README.md) - 

- [Artifact MQ](./src/infra/artifact-mq/README.md) - 

- [Inference DB](./src/infra/inference-db/README.md) - 

- [NFS Server](./src/infra/nfs/README.md) - 

- [Traefik Reverse Proxy](./src/infra/traefik/README.md) - 


## Software Architecture 📐

This project takes a Domain Driven Design (DDD)-styled architectural approach. Each API and service in this project are composed of four structural layers (presentation, application, domain, and infrastructure) and a fifth bootstrap layer. Their purposes will be described in detail below and explained from outermost (upper) to the innermost (lower), with the bootstrap layer explained last. Each layer is connected by a set of "input" and "output" DTOs (data transfer objects) and translation logic that will convert one layer's DTOs into another's. These DTOs will be described in detail in each section.

### 1. The Presentation Layer

The presentation layer the outermost layer responsible receiving and validating the user's requests, serving the responses, and calling out to the application layer to perform the operations related to the request. The inputs to this layer are called **requests** and the outputs are called **responses**. Requests represent data sent by a user to one of the APIs or services. Responses represent the data sent back to those users.

### 2. The Application Layer

The application layer is responsible for orchestrating business logic. Service's in this layer are invoked by handlers in the presentation layer to perform the work for a given request. This layer is comprises 4 components: **inputs**, **outputs**, **ports**, and **services**,

#### 2.1 Inputs

**Inputs** are the values that are passed into the **Application Layer** from the **Presentation Layer**.

#### 2.2 Outputs

**Outputs** are the values that are returned from the **Application Layer** back to the **Presentation Layer**.

#### 2.3 Ports

**Ports** are the interfaces that the **Infrastructure Layer** implements. These interfaces allow the **Application Layer** to call into the **Infrastructure Layer** in a way that decouples the **Appliaction Layer** from the concrete implementations of the **Infrastructure Layer**.

#### 2.4 Services

**Services** and their methods encapsulate the logic for every use case/operation.

### 3. The Domain Layer

The domain layer represents all business concepts and their relationships in the problem space. This layer is separated into two parts: Entities and Services. Entities are discrete representations of core business concepts, their data, and invariants. Services enforce the rules between entities and encapsulate domain specific operations. The domain layer is only known by the application layer. 

### 4. The Infrastructure Layer

This layer encapsulates the implementation details of the technologies used in this project such as databases, message brokers, and external service calls.

### 5. The Bootstrap Layer

## Adding New Components

### Adding a new API

### Adding a new Library

### Adding a new Infrastructure component