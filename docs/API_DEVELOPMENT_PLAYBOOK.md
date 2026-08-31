# MLHub API Development Playbook

This is the repository-wide process for adding an MLHub API or extending an existing one. It applies the architectural and implementation patterns already established by the Models and Deployments APIs.

## 1. Begin from an existing service

Use Models and Deployments as the primary references. Before introducing a module, handler, dependency, Dockerfile, configuration value, deployment resource, or test style, find its equivalent in those services and reproduce its structure and registration order.

For a new Cargo API binary:

1. Add the service under `services/<service>` and register its package in the workspace.
2. Mirror the standard service structure: `presentation`, `application`, `domain`, `infra`, and `bootstrap`, plus configuration, main/lib entry points, OpenAPI binary, Docker assets, ignores, and tests.
3. Register the component in the root component catalog with commands and template variables modeled on Models and Deployments. Use the agreed lifecycle value.
4. Add Kubernetes base resources, the Minikube and Tapis environment overlays, and deployment scripts following the other APIs.

Do not invent a service-local alternative to an existing pattern. For example, register independent Actix handlers directly as Models and Deployments do; do not add a route-configuration function merely because it is a reasonable general pattern.

## 2. Place code in the correct layer

MLHub uses presentation, application, domain, infrastructure, and bootstrap layers. Shared implementations are canonical: code in `libs/shared` is re-exported by the consuming API rather than copied into it.

| Layer | Responsibility | Usual location |
| --- | --- | --- |
| Presentation | Actix handlers, HTTP DTOs, validation, response contracts, OpenAPI | Shared DTOs/contracts; service-local Actix server and handlers |
| Application | use-case inputs/outputs, services, ports, workflows | `libs/shared/src/application` |
| Domain | entities, value objects, invariants, domain errors | `libs/shared/src/domain` |
| Infrastructure | Mongo documents, persistence mapping, repository adapters, external clients | `libs/shared/src/infra` |
| Bootstrap | compose concrete adapters into application services and app state | `services/<service>/src/bootstrap` |

Keep service-local code narrow: HTTP registration, response helpers, service re-exports, factories, state, and runtime composition. A service should not duplicate a shared domain entity, shared port, shared repository implementation, or shared HTTP contract.

Use a directory containing `mod.rs` for a component with adjacent tests or supporting files. Keep a solitary, unsupported component in a same-named file. Each handler and mapper gets its own file.

## 3. Model the domain before the transport

Create domain entities and value objects first. Keep their fields private and expose only the query operations callers actually need.

- New entities use `new`; persisted entities use a props-based `reconstitute` constructor. New identities use `Uuid::now_v7()` when the established entity pattern generates an ID.
- Enforce domain invariants in both constructors. Return a specific domain error for invalid new input; report invalid persisted data as `DataIntegrityError` (or the established equivalent).
- Use structural domain types such as `NonEmpty<T>` for non-empty collections. Do not rely only on request validation.
- Generate URNs with the shared URN generator when the entity is a canonical addressable MLHub resource and a matching existing pattern applies.
- Favor Law-of-Demeter-friendly semantic queries over exposing nested objects. For example, an entity should answer a capability question rather than require callers to navigate into a capabilities value object. Ask for direction when a new field’s public access semantics are not clear.

Do not add broad derives merely for test convenience. Follow the entity’s established derives; do not derive equality traits unless equality is part of the domain need.

## 4. Build the application boundary

Application input types are independent of HTTP DTOs. Put conversions in a dedicated mapper file and implement `From` for infallible mappings and `TryFrom` for mappings that can fail.

Application services implement use cases. Their first method argument is `&RequestContext`; derive tenant, owner, and principal data from it rather than accepting those values from the request. Services construct domain objects, invoke ports, apply the established retry policy, and return domain objects or application errors.

Define ports in the shared application layer as async `Send + Sync` traits. Keep infrastructure failures behind the port’s error type. Infrastructure adapters implement ports using persistence-specific documents and conversions; document enums and other persistence shapes belong in infrastructure rather than appearing as literal values in repository queries.

## 5. Add the HTTP boundary

Request and response DTOs live in shared presentation modules and are re-exported by the API. Keep requests, responses, and success-envelope contracts separate in the same style as Models and Deployments.

- Validate untrusted HTTP input with the project validator before application handling. Use nested validation, required/non-empty checks, URL checks, and custom collection validation where they mirror domain invariants.
- Validation fails fast, but the domain remains authoritative for every caller and for reconstitution.
- Use lower_snake_case field names. Keep enum values in their Rust variant casing unless an explicit wire-format requirement says otherwise.
- Use blank lines to separate logical statements and blocks. Outside closures and iterator adapters, format long method chains with one chained call per continuation line.
- Normalize optional request collections only when the contract requires it; responses that promise an array should serialize an empty array rather than `null`.
- Map DTOs through `From`/`TryFrom`, never through handler-local conversion functions.
- Give every Actix handler its own file. Keep handler route attributes consistent with Models and Deployments: API handler attributes are relative, while the OpenAPI document endpoint retains its established absolute form.
- Describe every public handler with Utoipa request, query, response, and error contracts. The OpenAPI endpoint and binary derive their output from those annotations. Do not manually modify a checked-in generated specification unless the task explicitly asks for it.
- For enum-valued query or path fields derived with `IntoParams`, annotate the field with `#[param(inline)]`. Without it, Utoipa can emit only a component reference and omit the usable enum choices from the operation parameter. Add a generated-OpenAPI test that asserts the parameter's inline `schema.enum` values, following the Agents scope-query test.

Apply the established logger, preflight/CORS behavior, authentication, and tenant-resolution middleware. Explicitly public health and OpenAPI routes remain outside the protected scope; protected handlers receive `RequestContext` through the shared extractor.

## 6. Compose and deploy the service

Bootstrap is the composition root. Initialize shared site configuration and identity context, create the Mongo client, build concrete repositories and application services in service factories, and register the same app data that shared middleware requires.

For Kubernetes, use the Models and Deployments manifest pattern:

1. Define the API deployment and service in the base and reference it from each environment overlay.
2. Mount the generated site-config ConfigMap at the shared configuration path and set `SITE_CONFIG_PATH` explicitly.
3. Provide all Mongo variables used by the server: host, port, database, credentials, and replica set. Obtain credentials from the existing secret pattern.
4. Verify every overlay includes its environment site-config overlay and renders the expected configuration.
5. Add or update Traefik service/router entries for the external route in every applicable environment. Match the real host and path prefix, not an assumed local hostname pattern.

Do not add unrelated messaging, storage, or environment configuration until the service actually consumes it.

## 7. Test, review, and verify

Co-locate tests with the code they exercise as `*.test.rs` and include them using the adjacent module convention. Use the project’s entity builders, whose fields are normally optional to make individual test setup focused. Tests return and explicitly handle errors rather than calling `unwrap` on expected results.

Verify the appropriate scope of work:

- domain construction, reconstitution, invariants, and semantic queries;
- request deserialization and validation, DTO mappings, and response serialization;
- application services with test ports, including context-derived tenant and principal behavior and propagated failures;
- handler route registration, documented OpenAPI paths/schemas, success envelopes, and error paths;
- Mongo document/entity mappings and repository filters;
- service checks/tests, rendered Kustomize overlays, and `git diff --check`.

Use focused commands first, such as `cargo test -p shared --lib`, `cargo test -p <service> --lib`, and `cargo check -p <service>`. Run `./manage test <component>` or broader checks when the change warrants them. Avoid formatters that rewrite unrelated files; use formatting checks or a targeted formatter invocation.

For runtime verification, use the deployed route through the intended proxy. Confirm health first, then invoke protected endpoints with the header format expected by shared authentication. Supply secrets only through secure local execution, never source control or documentation, and never repeat tokens in a handoff. Report the HTTP status and non-sensitive resource identifiers only.

## 8. Review checklist

- Did the change follow the closest existing API rather than add a new local convention?
- Is reusable layer code in shared and re-exported by services?
- Are domain invariants enforced in both new and reconstitution paths?
- Are domain APIs semantic and encapsulated?
- Are mappings idiomatic `From`/`TryFrom` implementations in dedicated mapper files?
- Are handlers, tests, modules, imports, document DTOs, and route style consistent with project conventions?
- Are deployment manifests complete for every overlay and do they render correctly?
- Did validation cover only the changed scope, with unrelated changes left untouched?
