# MLHub AI Agent Development Guide

This runbook converts MLHub’s conventions into an execution sequence. Read the root [AGENTS.md](../AGENTS.md) first; it is the concise repository-wide instruction set. Use the [API Development Playbook](./API_DEVELOPMENT_PLAYBOOK.md) for the human-oriented explanation behind these steps.

## Operating rules

1. Inspect before changing. Find the closest Models or Deployments implementation, then reproduce its shape, naming, registration order, and test style.
2. Keep changes scoped. Treat existing working-tree changes as user-owned unless they are directly part of the requested work.
3. Put reusable code in `libs/shared` and re-export it from a service. Keep only HTTP composition and service-specific bootstrap code local.
4. Prefer Rust idioms: `From` for infallible mappings, `TryFrom` for fallible mappings, dedicated mapper files, scoped imports, and explicit error handling.
5. Never expose secrets. Do not add them to code, documentation, command output, or final responses.

## Feature workflow

### 1. Establish the service shape

For a new API, mirror Models and Deployments before adding behavior: Cargo workspace membership, service modules, OpenAPI binary and endpoint, Docker artifacts, component catalog entry, Kubernetes base/overlays/scripts, and proxy entries. For an existing API, compare all changed areas with their closest counterparts before editing.

### 2. Develop inside out

1. Add the domain entity/value objects in shared. Use private fields, `new`, props-based `reconstitute`, the established ID/URN pattern, and invariant-specific errors.
2. Add shared application inputs, ports, services, and dedicated `From`/`TryFrom` mappers. Context-first service methods derive tenancy and ownership from `RequestContext`.
3. Add shared presentation request DTOs, response DTOs, contracts, validation, and mappings. Re-export them from the service.
4. Add infrastructure documents, conversion code, and repositories that implement the shared ports. Use infrastructure DTOs for stored enum/state values instead of literal strings.
5. Add service-local factories, state, server app data, handlers, OpenAPI annotations, and middleware using the reference API’s exact registration pattern.
6. Add deployment/configuration changes only for capabilities the service now uses.

Do not move backward across these layers by putting HTTP DTOs in domain code or persistence concerns in application services.

## Domain decisions

- Use `NonEmpty` or another established structural type for a domain non-empty invariant. Mirror it with request validation when feasible.
- Distinguish invalid new data from corrupt persisted data: new construction returns a targeted domain error; reconstitution reports a data-integrity error.
- Avoid exposing internal nested objects solely so callers can read through them. Prefer semantic methods on the aggregate.
- Before defining how a new domain value object is exposed, ask the user if the desired Law-of-Demeter interface is not clear. Do not assume direct getters, presence methods, or semantic queries are interchangeable.
- Do not derive `Eq` or `PartialEq` unless the request or established model calls for equality semantics.

## Presentation decisions

- Use lower_snake_case fields. Preserve Rust enum variant casing over HTTP unless the API explicitly requires another representation.
- Validate nested request objects and collection invariants at the boundary; retain domain validation as the final authority.
- Use the project’s standard success/error envelopes and Utoipa contracts. Do not hand-edit generated OpenAPI output unless explicitly directed.
- Keep every handler in its own file. API route attributes follow Models/Deployments relative-path style; retain the established absolute OpenAPI-document route.
- Apply authentication, tenancy, logging, and preflight/CORS middleware in the same order as the reference services. Health and OpenAPI routes remain public when those services make them public.

## File and test organization

- A component with supporting files or adjacent tests is a directory with `mod.rs`; its tests are adjacent `*.test.rs` files included from the module. A solitary component may use a same-named file.
- Give every mapper and handler its own file. Leave blank lines between methods.
- Use optional-field builders for entity test setup. Return and handle errors explicitly; do not use `unwrap` for expected test outcomes.
- Prefer narrow checks: shared/unit tests for shared changes, service/unit tests for service changes, `cargo check -p <service>`, Kustomize rendering for manifest work, and `git diff --check` for all edits.

## Deployment and runtime checklist

- Ensure every relevant overlay inherits the base resource and environment site configuration.
- When Mongo is used, provide and render the site-config mount/path plus host, port, database, secret-backed credentials, and replica-set variables using the reference manifest pattern.
- Add Traefik host/path routing for every applicable environment and confirm the route matches the actual requested host.
- Verify a deployed service through its proxy: health endpoint first, then a protected operation with the shared authentication header scheme. Keep credentials out of all persisted artifacts and user-visible responses.

## Before handoff

- Review the diff for accidental broad formatting, copied code that should be shared, direct string persistence values, duplicate route configuration, and missing module exports.
- Confirm the requested endpoints, OpenAPI contracts, tests, deployment overlays, and runtime configuration are all wired together.
- State what changed and which focused validations passed; mention remaining limitations plainly.
