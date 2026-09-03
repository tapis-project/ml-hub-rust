# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Features

- Added the experimental Agents API with authenticated create and scoped list operations for Agent records.
- Added the shared AgentRecord domain model, request/response contracts, application service, Mongo repository, OpenAPI documentation, and URN generation.
- Added Agents API Docker, Kubernetes, component-catalog, site-configuration, Mongo-configuration, and environment-overlay support.

### Non-Breaking Changes

- Added repository-wide API development, architecture, AI-agent, deployment, migration, and approved-design-pattern documentation.
- Added lifecycle commands to build all migration images and run all migrations locally.

### Fixed

- Corrected the Minikube Traefik route for the Agents API.

## [v0.1.0] - 2025-08-20

### Features
Model listing by platform (huggingface, patra)
Model fetch by platform (huggingface, patra)
Model publication by platform (huggingface)
Model discovery by platform (huggingface, patra)
Model ingestion by platform (huggingface, github, private git repos)
Native model metadata creation
Model upload
Model download 

### Breaking Changes

### Non-Breaking Changes

### Fixed

### Removed



