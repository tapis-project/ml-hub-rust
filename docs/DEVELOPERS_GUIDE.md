# MLHub Developer Guide

For the repository-wide process for adding APIs and extending their features, see the
[API Development Playbook](./API_DEVELOPMENT_PLAYBOOK.md). AI-assisted contributors should also
follow the root [AGENTS.md](../AGENTS.md) and the [AI Agent Development Guide](./AI_AGENT_DEVELOPMENT_GUIDE.md).
Repository-wide deferred work is tracked in [TODOs](./TODO.md).

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

- [tacc-tapis-client](./libs/tapis-client/README.md) - Client for publishing and ingesting artifacts from Tapis Systems defined in the TACC Tapis deployment

### Infrastructure (`deploy/k8s`)

The `deploy/k8s` directory contains the Kubernetes resources for the infrastructure that supports
MLHub APIs, workers, and deployments. Each component has a base configuration and environment
overlays where applicable.

**Infrastructure catalog**

- [MongoDB](../deploy/k8s/mongo/) - Persistent document storage for MLHub resource metadata, identities, and other API-managed records. It is deployed as a StatefulSet with replica-set configuration and environment-specific storage.

- [RabbitMQ](../deploy/k8s/rabbit/) - Message broker for asynchronous MLHub workflows and service-to-service event delivery, with persistent storage and management access.

- [NFS Server](../deploy/k8s/nfs/) - Shared network file storage for components that require common, persistent filesystem access.

- [Traefik Reverse Proxy](../deploy/k8s/traefik/) - Edge reverse proxy that routes external HTTP requests to MLHub services using environment-specific dynamic configuration.


## Software Architecture 📐

MLHub combines Domain-Driven Design, Clean Architecture, and Hexagonal Architecture. Every API is
organized into presentation, application, domain, infrastructure, and bootstrap layers. The first
four separate responsibilities; bootstrap composes them into a running service.

```text
HTTP request
    │
    ▼
Presentation ──► Application ──► Domain
                     │
                     ▼
                   Port ◄──── Infrastructure adapter

Bootstrap creates the infrastructure adapter, injects it into the application service,
and registers the resulting service with the presentation server.
```

The application layer depends on ports, not concrete databases, brokers, or external clients.
Infrastructure implements those ports. Request, application-input, domain, persistence-document,
and response types are translated explicitly at their respective boundaries so no single type
becomes the contract for every layer.

### Shared Layer Conventions

Reusable layer code belongs in `libs/shared`; services re-export the modules they consume instead
of duplicating domain, application, or presentation types. HTTP handlers, server registration,
bootstrap factories, and deployment wiring are generally service-local. See the
[API Development Playbook](./API_DEVELOPMENT_PLAYBOOK.md) for the required implementation,
testing, deployment, and review workflow.

### Rust Readability Conventions

Separate logical statements and code blocks with blank lines. In particular, after a fallible
operation such as a `map_err` or `?` expression, leave a blank line before beginning the next
operation. For long method chains outside closures and iterator adapters, put each chained call
on its own continuation line. Follow nearby code when a more specific local pattern exists.

### 1. Presentation

Presentation is the HTTP boundary. Actix handlers receive requests, use shared request DTOs to
deserialize and validate untrusted input, map into application inputs, invoke an application
service, and map results into shared response DTOs and standard response envelopes. It also owns
Utoipa documentation and the service-local middleware/server composition.

### 2. Application

Application code implements use cases. Its services orchestrate domain behavior and interactions
with ports; they do not contain HTTP or database details. Application inputs and outputs define
the boundary around a use case. Service methods receive `RequestContext` first, so tenancy and
principal information comes from trusted middleware rather than client-supplied fields.

Ports are async interfaces owned by the application layer. Repository, messaging, and external
service adapters implement them in infrastructure. Workflows hold reusable orchestration that is
shared by more than one application service.

### 3. Domain

Domain code models MLHub business concepts, value objects, invariants, and domain-specific
operations. Entities keep state private, create new records through constructors (often just named `new`, but where appropriate, named after the action occuring in the domin), and rebuild persisted
records through props-based `reconstitute` constructors. Domain rules must hold regardless of whether a caller
is HTTP, a background worker, or a persistence adapter.

The canonical shared domain implementation is in `libs/shared/src/domain`; APIs re-export the
domain modules they use rather than defining service-specific copies.

### 4. Infrastructure

Infrastructure contains technology-specific implementations: Mongo documents and repositories,
message-broker integrations, and external clients. It maps between persistence or transport
representations and domain values, and implements application ports. It must not leak database
documents or provider-specific types into application-service interfaces.

### 5. Bootstrap

Bootstrap is the composition root. It loads configuration, creates concrete infrastructure clients
and adapters, injects those adapters into application services, and registers the resulting
services as presentation app data. Bootstrap makes dependencies explicit without placing runtime
wiring in domain or application code.

## Approved Design Patterns 🖼️

Use the following patterns when they fit the responsibility at hand. Follow the implementation
style already present in Models, Deployments, and `libs/shared`; do not introduce a new pattern or
variation without discussion and documentation.

| Pattern | Use in MLHub |
| --- | --- |
| **Domain-Driven Design** | Model business concepts as entities and value objects in the domain layer. Keep invariants and domain-specific errors with those concepts. |
| **Clean Architecture** | Organize code by presentation, application, domain, infrastructure, and bootstrap responsibilities. Keep technology and delivery details outside the domain. |
| **Hexagonal Architecture (Ports and Adapters)** | Define application-owned ports for persistence, messaging, and external capabilities; implement them with infrastructure adapters. |
| **Repository** | Encapsulate persistence access behind an application port. Repository adapters map between domain entities and infrastructure document DTOs. |
| **Application Service** | Implement one or more use cases by coordinating domain behavior and ports. Services receive `RequestContext` first and do not own HTTP or database details. |
| **Factory and Composition Root** | Build concrete clients, adapters, and application services in bootstrap factories, then inject them into the server. Keep construction logic out of handlers and domain code. |
| **DTO and Mapper** | Keep request, application-input, domain, persistence-document, and response shapes separate. Translate explicitly with dedicated `From` or `TryFrom` mapper implementations. |
| **Entity Reconstitution** | Create new entities through constructors and rebuild persisted entities through props-based `reconstitute` constructors, distinguishing invalid new input from corrupt persisted data. |
| **Workflow** | Extract repeated application-service orchestration into a workflow only when it serves more than one use case. |
| **Builder (test fixtures)** | Use optional-field builders to construct focused, readable domain test fixtures without duplicating setup. |
| **Singleton** | Use an application-scoped configuration or client instance only when a single shared instance is required; prefer explicit bootstrap injection over global mutable state. |

Related rules reinforce these patterns: preserve the Law of Demeter by exposing semantic domain
queries instead of navigable internals, validate untrusted input at the presentation boundary, and
keep domain validation authoritative for every caller.
