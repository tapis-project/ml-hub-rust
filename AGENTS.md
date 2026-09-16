# MLHub Agent Instructions

These instructions apply throughout this repository.

## Start with established patterns

- Inspect the closest Models and Deployments implementation before adding a service, layer, endpoint, DTO, repository, bootstrap component, or deployment manifest. Follow the established pattern exactly unless a change is explicitly requested.
- Preserve unrelated working-tree changes. Do not reset, restore, reformat, or regenerate broad areas of the repository to complete a focused task.
- Make a small, coherent change and validate the affected packages and manifests before handoff.

## Architecture and placement

- MLHub uses presentation, application, domain, infrastructure, and bootstrap layers. Reusable implementations belong in `libs/shared`; services re-export the shared modules they consume rather than duplicating them.
- Keep HTTP handlers and route registration service-local. Give every handler its own file and register handlers following the Models and Deployments pattern; do not introduce a route-configuration function where those services do not use one.
- Put each mapper in its own file. Prefer `From` for infallible conversion and `TryFrom` for fallible conversion over named mapping functions.
- Across every architectural layer and every kind of Rust code—including entities, services, repositories, handlers, DTOs, mappers, clients, and utilities—a component with any adjacent test or supporting file must live in its own same-named directory with the implementation in `mod.rs`. Keep the test and every supporting file inside that directory; never place `component.rs` and `component.test.rs` as siblings in their parent module. A component with no tests or support files may remain in a same-named file.

## Domain and application rules

- Keep domain state private. Construct new entities with `new`, rehydrate persisted data with props-based `reconstitute`, and use UUID v7 for new identities where the established entity pattern does so.
- Enforce invariants in the domain. New input should receive a specific domain error; invalid persisted data should receive a data-integrity error. Use structural types such as `NonEmpty` where the domain requires them.
- Prefer semantic domain queries over exposing nested value objects. When a new value object needs an access interface and the desired encapsulation is unclear, ask before choosing it.
- Application-service methods take `&RequestContext` as their first argument. Derive tenancy and principal information from that context, not HTTP input.
- Infrastructure uses its own persistence document DTOs and conversions. Do not query or persist enum/string storage literals directly when a document DTO exists.
- Persistence and presentation DTOs must not use polymorphic enum payloads for concepts that can grow by adding explicitly named fields. Model each supported variant as its own optional field, following the Dataset provider/locator pattern, and add a new field when support expands. Include a separate enum discriminator when serialization or deserialization must select one of those fields, and validate that the discriminator and populated field agree. Domain enums may still enforce the internal invariant; map them explicitly at the DTO boundary.

## Presentation and API rules

- Put request/response DTOs and OpenAPI contracts in shared presentation modules; use validation at the request boundary to fail fast while retaining domain validation as authoritative.
- Use lower_snake_case for field names. Preserve Rust enum variant casing on the wire unless the API explicitly requires a transformation.
- Do not expose tagged or untagged polymorphic unions in response DTOs when the contract can use one field per supported type. For example, hardware profiles expose `gpu`, and future accelerator support adds sibling fields rather than changing the shape behind a generic `accelerator` field.
- Document handlers with Utoipa. Generated OpenAPI is source-derived: do not manually edit checked-in generated specifications unless explicitly requested.
- Write `#[utoipa::path(...)]` attributes across multiple lines. Every comma-separated argument gets its own properly indented line: put the HTTP method, path, tag, summary or description, parameters, request body, and response block on separate lines; within parameter and response tuples, put each comma-separated field on its own nested line; and put each response entry in its own indented block. Never use a one-line Utoipa path attribute.
- Utoipa does not reliably expose an enum's variants when an `IntoParams` query or path field is emitted as a component reference. Add `#[param(inline)]` to enum-valued parameter fields and test the generated operation parameter for its inline `schema.enum` values; follow the Agents scope-query example.
- Apply the established authentication, tenancy, logging, and CORS/preflight middleware. Keep explicitly public routes public.

## Tests, formatting, and delivery

- When writing an implementation plan, group Key Changes by entity or bounded feature, then by architectural layer where relevant; keep one consolidated Test Plan section at the end.
- Co-locate tests as adjacent `*.test.rs` files and include them through the module convention used by nearby code. Use builders with optional fields for entity tests. Return and handle errors explicitly in tests; do not use `unwrap` for expected success or failure paths.
- Keep a blank line between methods, logical statements, and code blocks; imports stay scoped at the top of a file. Alias imports rather than using inline fully qualified paths.
- Treat each change in purpose as a new logical block. In particular, leave a blank line between setup and execution, separate repository calls, validation and mutation, mutation and persistence, persistence and response construction, and each arrange/act/assert section in tests. Consecutive statements are allowed without a blank line only when they form one inseparable operation, such as populating the same map or builder.
- Do not rely on rustfmt to create semantic whitespace; it cannot infer logical boundaries. After formatting, manually review every changed function for mashed-together logical blocks and add the required blank lines before delivery.
- For a long method chain outside a closure or iterator adapter, put each chained call on its own continuation line. Leave a blank line after a completed fallible operation before beginning the next logical operation.
- Run focused `cargo test`, `cargo check`, formatting checks, Kustomize rendering, and `git diff --check` in proportion to the change. Never expose credentials, tokens, or secrets in source, documentation, commands, or handoff messages.
