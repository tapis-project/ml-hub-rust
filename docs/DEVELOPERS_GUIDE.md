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

### APIs

APIs live in the `services` directory. Here you will find the core APIs that provide MLHub functionality; The Models API, The Datasets API, The Deployments API, and the Agents API. These API's follow the standard cargo binary structure.
You can find the source code of an API in `services/<service>` where `<service>` is the name of the API. For example, The Models API source code is located at `services/models`.

Each API follows a similar pattern. In the root directory of each API you will find the following directories:
- **deploy**: This contains the files that are used to deploy the API in some environment. Each available deployment method and environment in which the API can be deployed will have their own subdirectory. For example, the files for deploying the Models API to a Minkube cluster are located in `services/models/deploy/local/minkube`.
- **docs**: 
- **scripts**:
- **spec**: Contains the API's specification. Each type of spec will be stored in a subdirectory by the name of the spec. For example, the OpenAPI spec for the Models API will be in the `services/models/spec/openapi` directory.
- **src**


**API catalog**

- [Models](./services/models/README.md)

- [Datasets](./services/datasets/README.md) - [Under Construction]

- [Deployments](./services/deployments/README.md)

- [Agents](./services/agents/README.md) - [Under Construction]

### Binaries

Binaries are standalone executables that live in the `services` directory. This is where all non-API binaries are stored. For example, the MLHub CLI, agent spec generation code, and the artifact ingestion and publishing workers all live in this directory.

**Binaries catalog**

- [Artifact Ingester](./services/artifact-ingester/README.md)

- [Artifact Publisher](./services/artifact-publisher/README.md)

- [Model Deployment Controller](./services/artifact-publisher/README.md)

### Libraries (libs)

Libraries are reusable codes that are shared between apis, binaries, and other libraries. Libraries can also be used to support development through codegen and sdks. These libraries are located in the `libs` directory. A comprehensive list of libraries developed for this project can be found below in the **Library Catalog** along with a short description.

**Library Catalog**

- [shared](./libs/shared/README.md) - Contains the shared layers, project level constants, and various utilities. 

- [clients](./libs/clients/README.md) - Contains the interfaces (traits) implemented by the various clients that provide the core features of MLHub (Ex. Model and dataset discovery, ingestion, and publishing, model deployment, etc)

- [client-provider](./libs/client-provider/README.md) - Provides concrete client interfaces.

- [huggingface-client](./libs/huggingface-client/README.md) - Client for fetching, discovering, publishing, and ingesting HuggingFace models and datasets and their metadata

- [git-lfs-client](./libs/git-lfs-client/README.md) - Client for private/public Git registries with LFS. This library can publish ingest model and dataset artifacts from git-backed sources. This can be used for artifacts on Github as well. 

- [github-lfs-client](./libs/github-lfs-client/README.md) - Client from ingesting and publishing model and dataset artifacts to Github and LFS. Essentially a specialized version of the **git-lfs-client**

- [patra-client](./libs/patra-client/README.md) - Handles model metadata listing, discovery, and publishing to the Patra platform

- [s3-client](./libs/s3-client/README.md) - Client for publishing and ingesting artifacts from s3-compatible storage

- [tacc-tapis-client](./libs/tapis-client/README.md) - Client for publishing and ingesting artifacts from Tapis Systems defined in the TACC Tapis deployment

- [mlhub-rust-sdk](./libs/mlhub-tust-sdk/README.md) - MLHub's software development kit (SDK) generated from the API's OpenAPI specifications

### Infra (services)

This directory (`services`) contains the deployment files for the infrastructural components that support MLHub operations such as databases, message brokers, remote file systems, and reverse proxies.

**Infrastructure catalog**

- [Artifact DB](./services/mongo/README.md) - 

- [Artifact MQ](./services/artifact-mq/README.md) - 

- [Inference DB](./services/inference-db/README.md) - 

- [NFS Server](./services/nfs/README.md) - 

- [Traefik Reverse Proxy](./services/traefik/README.md) - 


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

**Services** and their methods encapsulate the logic for every use case/operation and are the entrypoints to the presentation layer for calling into the application layer.

#### 2.5 Workflows

**Workflows** are encapsulations of commonly repeated logic througout the application layer. They should be invoked inside of application services.

### 3. The Domain Layer

The domain layer represents all business concepts and their relationships in the problem space. This layer is separated into two parts: Entities and Services. Entities are discrete representations of core business concepts, their data, and invariants. Services enforce the rules between entities and encapsulate domain specific operations. The domain layer is only known by the application layer. 

The canonical implementations of shared domain entities and services live in `libs/shared/src/domain`. APIs re-export the shared modules they use from their local domain layer instead of duplicating domain models in each service.

### 4. The Infrastructure Layer

This layer encapsulates the implementation details of the technologies used in this project such as databases, message brokers, and external service calls.

### 5. The Bootstrap Layer

The bootstrap layer is the composition root for a service. This is where the concrete infrastructure-layer implementations are instantiated and injected into application layer services. These implementations conform to some **port** in the application layer.
