# MLHub Developer Guide

## Project Structure & Components 📁

This project is structured as a monorepo. Each component of this project is grouped into one of four categories.
APIs, libraries, binaries, and infrastructure.

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

The presentation layer the outermost layer responsible processing the user's requests, serving the responses, and calling to the application layer to perform the operations related to the request. The input DTOs to this layer are called **requests** and the output DTOs are called **responses**. Requests represent data sent by a user to one of this projects APIs or services. Responses represent the data sent back to those users. The presentation layer makes call to services in the application layer to perform the operation requested by the user.

### 2. The Application Layer

The application layer is responsible for orchestrating application logic i.e. the sequencing of tasks needed to fulfill a user's request. This layer is broken up into three parts: Services, ports, and input/output DTOs. First, the presentation layer calls into the application layer by invoking a service's method with user's validated request data (inputs). The service then executes the sequences of tasks that fulfill that request. Some such tasks include calls to databases and message queues (the infrastructure layer). Invocation of the infrastructure happens through repositories (to access databases) and event handlers (to send messages to brokers). These infrastructure components are furnished to the service via dependency injection (DI) in the bootstrap layer (discussed later). The concrete implementations of these repositories and event handlers are not known to the service directly, but known only through contracts. Traits are used to describe the contracts that these concrete implementations will abide by. These contracts are called "ports". The application layer only knows about the domain layer (discussed later) which is a pure representation of the business space, its entities, and the rules that govern these entities. These entities are the values returned from a service's methods. We call the entities returned from the methods "outputs". In pure DDD, we would first translate these entities into an application layer-specific struct and pass that back to the presentation layer, but in an effort to reduce the amount of code written, we are simply returning entities. This is indeed a leaky abstraction, but by removing this layer of indirection, we can improve the speed of development. Additionally, it is a relatively simple refactor when we want to implement DDD in a more complete way.

### 3. The Domain Layer

The domain layer defines core business concepts and their relationships in the problem space. This layer is divided into two parts: Entities and services. Entities are discrete representations of core business concepts, their data, and domain invariants (business rules that must always hold). An example of an entity from this project is the Artifact Ingestion. As the name implies, this entity represents the process of ingesting a machine learning artifact (models and datasets) into MLHub. In code, this entity is represented as a struct with fields such as `id`, `created_at`, `last_modified`, and `artifact_id`. The Artifact Ingestion also has a field `status` that tracks the current ingestion state. This state is one of an enumeration of possible states including `Submitted`, `Completed`, and `Failed` (there are more but this is simplified for explanation). The domain dictates that there are only a certain set of valid state transitions. For example, an ingestion cannot transition from a status of `Completed` to `Submitted`. This would be a violation of the invariant that states "there are no permitted state transitions once an Artifact Ingestion has reached the `Completed` status.

Services encapsulate domain invariants and operations involving multiple entities. For example, before creating ModelMetadata for an Artifact, we pass those entity structs into the ModelMetadataService's `create` method which verifies that the artifact exists and that the artifact has been fully ingested or uploaded.

### 4. The Infrastructure Layer

This layer encapsulates the implementation details of the actual technologies used in this project such as databases, message brokers, and external service calls. This will include repositories (see repository pattern) that handle persistence concerns, event handlers that handle passing messages to message brokers and external APIs, and making system calls.

### 5. The Bootsrap Layer

This layer ties the presentation, application, and infrastructure layers together. The concrete repositories and event handlers defined in the infrastructure layer are provided to the application layer though dependency injection and the factory pattern. This means that the bootstrap layer determines which implementations these application layer services can use. For example, let's say your application supports both Postgres and MongoDB. The decision of which application services use MongoDB and which use Postgres are decided at the bootstrap layer. It's job is to instantiate application services, furnish them (via DI) with the correct implementation details they need to perform their job, and inject those services in the presentation layer so that they can be invoked.

## Adding New Components

### Adding a new API

### Adding a new Library

### Adding a new Infrastructure component